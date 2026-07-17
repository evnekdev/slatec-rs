#![cfg(all(
    feature = "quadrature-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native execution tests for the validated GNU MinGW quadrature profile.
//!
//! Set `SLATEC_NATIVE_LIB_DIR` and `SLATEC_GFORTRAN_RUNTIME_DIR` to the
//! explicit archive and GNU runtime directories before running this target.

use std::sync::{Arc, Barrier};
use std::thread;

use slatec::quadrature::{
    EndpointWeight, FourierOptions, InfiniteInterval, IntegrationError, IntegrationOptions,
    Nc79Options, NonAdaptiveOptions, OscillatoryOptions, OscillatoryWeight, integrate,
    integrate_f32, integrate_fourier_tail, integrate_fourier_tail_f32, integrate_infinite,
    integrate_infinite_f32, integrate_nc79, integrate_nc79_f32, integrate_non_adaptive,
    integrate_non_adaptive_f32, integrate_oscillatory, integrate_oscillatory_f32,
    integrate_principal_value, integrate_principal_value_f32, integrate_singular,
    integrate_singular_f32, integrate_weighted_endpoints, integrate_weighted_endpoints_f32,
    integrate_with_breakpoints, integrate_with_breakpoints_f32,
};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

#[test]
fn finite_and_reversed_integrals_match_reference_values() {
    let options = IntegrationOptions::default();
    let square = integrate(|x| x * x, 0.0, 1.0, options).unwrap();
    close(square.value, 1.0 / 3.0, 1.0e-12);
    assert!(square.estimated_error >= 0.0);
    assert!(square.evaluations > 0);
    assert!(square.intervals > 0);

    let sine = integrate(|x| x.sin(), 0.0, std::f64::consts::PI, options).unwrap();
    close(sine.value, 2.0, 1.0e-12);

    let reversed = integrate(|x| x * x, 1.0, 0.0, options).unwrap();
    close(reversed.value, -1.0 / 3.0, 1.0e-12);

    let single_options = IntegrationOptions::single_precision();
    let single = integrate_f32(|x| x * x, 0.0, 1.0, single_options).unwrap();
    close(f64::from(single.value), 1.0 / 3.0, 2.0e-5);
}

#[test]
fn specialized_interval_drivers_match_reference_values() {
    let options = IntegrationOptions::default();
    let infinite =
        integrate_infinite(|x| (-x).exp(), InfiniteInterval::Above(0.0), options).unwrap();
    close(infinite.value, 1.0, 1.0e-10);

    let singular = integrate_singular(|x| 1.0 / (1.0 - x * x).sqrt(), -1.0, 1.0, options).unwrap();
    close(singular.value, std::f64::consts::PI, 2.0e-10);

    let principal = integrate_principal_value(|x| x, -1.0, 1.0, 0.0, options).unwrap();
    close(principal.value, 2.0, 1.0e-11);

    let single_options = IntegrationOptions::single_precision();
    close(
        f64::from(
            integrate_infinite_f32(
                |x| (-x).exp(),
                InfiniteInterval::Above(0.0_f32),
                single_options,
            )
            .unwrap()
            .value,
        ),
        1.0,
        2.0e-4,
    );
    close(
        f64::from(
            integrate_singular_f32(|x| 1.0 / (1.0 - x * x).sqrt(), -1.0, 1.0, single_options)
                .unwrap()
                .value,
        ),
        std::f64::consts::PI,
        3.0e-4,
    );
    close(
        f64::from(
            integrate_principal_value_f32(|x| x, -1.0, 1.0, 0.0, single_options)
                .unwrap()
                .value,
        ),
        2.0,
        2.0e-4,
    );
}

#[test]
fn callback_failures_are_contained_and_context_is_reusable() {
    assert_eq!(
        integrate(
            |_| panic!("contained callback panic"),
            0.0,
            1.0,
            IntegrationOptions::default()
        ),
        Err(IntegrationError::CallbackPanicked)
    );
    assert_eq!(
        integrate(|_| f64::NAN, 0.0, 1.0, IntegrationOptions::default()),
        Err(IntegrationError::CallbackFailed)
    );
    assert_eq!(
        integrate(|x| 1.0 / (x - 0.5), 0.0, 1.0, IntegrationOptions::default()),
        Err(IntegrationError::CallbackFailed)
    );

    let recovered = integrate(|x| x, 0.0, 1.0, IntegrationOptions::default()).unwrap();
    close(recovered.value, 0.5, 1.0e-12);
}

#[test]
fn nested_calls_fail_without_deadlock_and_outer_call_completes() {
    let mut nested = None;
    let outer = integrate(
        |x| {
            if nested.is_none() {
                nested = Some(integrate(|y| y, 0.0, 1.0, IntegrationOptions::default()));
            }
            x
        },
        0.0,
        1.0,
        IntegrationOptions::default(),
    )
    .unwrap();
    close(outer.value, 0.5, 1.0e-12);
    assert_eq!(nested, Some(Err(IntegrationError::NestedIntegration)));
}

#[test]
fn parallel_callers_are_serialized_without_sharing_callback_state() {
    let barrier = Arc::new(Barrier::new(3));
    let mut threads = Vec::new();
    for factor in [1.0, 2.0] {
        let barrier = Arc::clone(&barrier);
        threads.push(thread::spawn(move || {
            barrier.wait();
            integrate(|x| factor * x, 0.0, 1.0, IntegrationOptions::default())
                .unwrap()
                .value
        }));
    }
    barrier.wait();
    let mut values: Vec<_> = threads
        .into_iter()
        .map(|join| join.join().unwrap())
        .collect();
    values.sort_by(f64::total_cmp);
    close(values[0], 0.5, 1.0e-12);
    close(values[1], 1.0, 1.0e-12);
}

#[test]
fn native_status_is_mapped_for_an_intentionally_tiny_limit() {
    let options = IntegrationOptions {
        absolute_tolerance: 0.0,
        relative_tolerance: 1.0e-13,
        limit: 1,
        ..IntegrationOptions::default()
    };
    let result = integrate(|x| x.sqrt(), 0.0, 1.0, options);
    assert!(matches!(
        result,
        Err(IntegrationError::MaximumSubdivisions | IntegrationError::RoundoffDetected)
    ));
}

#[test]
fn extended_public_drivers_match_analytic_references() {
    let adaptive = IntegrationOptions::default();
    let adaptive_f32 = IntegrationOptions::single_precision();
    close(
        integrate_with_breakpoints(|x| x.abs(), -1.0, 1.0, &[0.0], adaptive)
            .unwrap()
            .value,
        1.0,
        1.0e-12,
    );
    close(
        f64::from(
            integrate_with_breakpoints_f32(|x| x.abs(), -1.0, 1.0, &[0.0], adaptive_f32)
                .unwrap()
                .value,
        ),
        1.0,
        2.0e-5,
    );

    close(
        integrate_weighted_endpoints(
            |_| 1.0,
            0.0,
            1.0,
            -0.5,
            0.0,
            EndpointWeight::Algebraic,
            adaptive,
        )
        .unwrap()
        .value,
        2.0,
        2.0e-11,
    );
    close(
        integrate_weighted_endpoints(
            |_| 1.0,
            0.0,
            1.0,
            0.0,
            0.0,
            EndpointWeight::AlgebraicLogLower,
            adaptive,
        )
        .unwrap()
        .value,
        -1.0,
        2.0e-11,
    );
    close(
        f64::from(
            integrate_weighted_endpoints_f32(
                |_| 1.0,
                0.0,
                1.0,
                -0.5,
                0.0,
                EndpointWeight::Algebraic,
                adaptive_f32,
            )
            .unwrap()
            .value,
        ),
        2.0,
        3.0e-4,
    );

    let oscillatory = OscillatoryOptions::default();
    close(
        integrate_oscillatory(
            |_| 1.0,
            0.0,
            std::f64::consts::PI,
            1.0,
            OscillatoryWeight::Sine,
            oscillatory,
        )
        .unwrap()
        .value,
        2.0,
        2.0e-11,
    );
    close(
        integrate_oscillatory(
            |x| x,
            0.0,
            1.0,
            1.0,
            OscillatoryWeight::Cosine,
            OscillatoryOptions {
                absolute_tolerance: 1.0e-8,
                relative_tolerance: 1.0e-8,
                ..OscillatoryOptions::default()
            },
        )
        .unwrap()
        .value,
        1.0_f64.sin() + 1.0_f64.cos() - 1.0,
        2.0e-8,
    );
    close(
        f64::from(
            integrate_oscillatory_f32(
                |_| 1.0,
                0.0,
                std::f32::consts::PI,
                1.0,
                OscillatoryWeight::Sine,
                OscillatoryOptions::single_precision(),
            )
            .unwrap()
            .value,
        ),
        2.0,
        3.0e-4,
    );

    let fourier = FourierOptions::default();
    close(
        integrate_fourier_tail(|x| (-x).exp(), 0.0, 1.0, OscillatoryWeight::Cosine, fourier)
            .unwrap()
            .value,
        0.5,
        2.0e-10,
    );
    close(
        integrate_fourier_tail(|x| (-x).exp(), 0.0, 1.0, OscillatoryWeight::Sine, fourier)
            .unwrap()
            .value,
        0.5,
        2.0e-10,
    );
    close(
        f64::from(
            integrate_fourier_tail_f32(
                |x| (-x).exp(),
                0.0,
                1.0,
                OscillatoryWeight::Cosine,
                FourierOptions::single_precision(),
            )
            .unwrap()
            .value,
        ),
        0.5,
        4.0e-4,
    );

    close(
        integrate_non_adaptive(|x| x * x, 0.0, 1.0, NonAdaptiveOptions::default())
            .unwrap()
            .value,
        1.0 / 3.0,
        1.0e-12,
    );
    close(
        f64::from(
            integrate_non_adaptive_f32(|x| x * x, 0.0, 1.0, NonAdaptiveOptions::single_precision())
                .unwrap()
                .value,
        ),
        1.0 / 3.0,
        2.0e-5,
    );

    close(
        integrate_nc79(|x| x * x, 0.0, 1.0, Nc79Options::default())
            .unwrap()
            .value,
        1.0 / 3.0,
        1.0e-11,
    );
    close(
        f64::from(
            integrate_nc79_f32(|x| x * x, 0.0, 1.0, Nc79Options::single_precision())
                .unwrap()
                .value,
        ),
        1.0 / 3.0,
        3.0e-5,
    );
}

#[test]
fn extended_drivers_share_callback_containment_and_recovery() {
    let run =
        |mut call: Box<dyn FnMut(Box<dyn FnMut(f64) -> f64>) -> Result<(), IntegrationError>>| {
            assert_eq!(
                call(Box::new(|_| panic!("contained"))),
                Err(IntegrationError::CallbackPanicked)
            );
            assert_eq!(
                call(Box::new(|_| f64::INFINITY)),
                Err(IntegrationError::CallbackFailed)
            );
            call(Box::new(|x| x)).unwrap();
        };

    run(Box::new(|f| {
        integrate_with_breakpoints(f, 0.0, 1.0, &[0.5], IntegrationOptions::default()).map(|_| ())
    }));
    run(Box::new(|f| {
        integrate_weighted_endpoints(
            f,
            0.0,
            1.0,
            0.0,
            0.0,
            EndpointWeight::Algebraic,
            IntegrationOptions::default(),
        )
        .map(|_| ())
    }));
    run(Box::new(|f| {
        integrate_oscillatory(
            f,
            0.0,
            1.0,
            1.0,
            OscillatoryWeight::Sine,
            OscillatoryOptions::default(),
        )
        .map(|_| ())
    }));
    run(Box::new(|f| {
        integrate_fourier_tail(
            f,
            0.0,
            1.0,
            OscillatoryWeight::Cosine,
            FourierOptions::default(),
        )
        .map(|_| ())
    }));
    run(Box::new(|f| {
        integrate_non_adaptive(f, 0.0, 1.0, NonAdaptiveOptions::default()).map(|_| ())
    }));
    run(Box::new(|f| {
        integrate_nc79(f, 0.0, 1.0, Nc79Options::default()).map(|_| ())
    }));
}

#[test]
fn extended_breakpoint_weight_and_frequency_checks_avoid_native_misuse() {
    assert!(matches!(
        integrate_with_breakpoints(|x| x, -1.0, 1.0, &[0.0, 0.0], IntegrationOptions::default()),
        Err(IntegrationError::DuplicateBreakpoint { .. })
    ));
    assert!(matches!(
        integrate_weighted_endpoints(
            |x| x,
            0.0,
            1.0,
            -1.0,
            0.0,
            EndpointWeight::Algebraic,
            IntegrationOptions::default(),
        ),
        Err(IntegrationError::InvalidWeightParameter { .. })
    ));
    assert!(matches!(
        integrate_oscillatory(
            |x| x,
            0.0,
            1.0,
            f64::NAN,
            OscillatoryWeight::Sine,
            OscillatoryOptions::default(),
        ),
        Err(IntegrationError::InvalidFrequency { .. })
    ));
}

#[test]
fn extended_zero_frequency_and_callback_reentrancy_are_explicit() {
    let oscillatory = OscillatoryOptions::default();
    close(
        integrate_oscillatory(
            |_| 1.0,
            0.0,
            1.0,
            0.0,
            OscillatoryWeight::Cosine,
            oscillatory,
        )
        .unwrap()
        .value,
        1.0,
        1.0e-10,
    );
    close(
        integrate_oscillatory(|_| 1.0, 0.0, 1.0, 0.0, OscillatoryWeight::Sine, oscillatory)
            .unwrap()
            .value,
        0.0,
        1.0e-12,
    );

    let zero_sine = integrate_fourier_tail(
        |_| panic!("zero sine Fourier tail must not invoke its callback"),
        0.0,
        0.0,
        OscillatoryWeight::Sine,
        FourierOptions::default(),
    )
    .unwrap();
    assert_eq!(zero_sine.value, 0.0);
    assert_eq!(zero_sine.evaluations, 0);

    close(
        integrate_fourier_tail(
            |x| (-x).exp(),
            0.0,
            0.0,
            OscillatoryWeight::Cosine,
            FourierOptions::default(),
        )
        .unwrap()
        .value,
        1.0,
        1.0e-10,
    );

    let outer = integrate_with_breakpoints(
        |x| {
            assert_eq!(
                integrate_non_adaptive(|t| t, 0.0, 1.0, NonAdaptiveOptions::default()),
                Err(IntegrationError::NestedIntegration)
            );
            x
        },
        0.0,
        1.0,
        &[0.5],
        IntegrationOptions::default(),
    )
    .unwrap();
    close(outer.value, 0.5, 1.0e-10);
}

#[test]
fn extended_parallel_calls_are_serialized_and_independent() {
    let barrier = Arc::new(Barrier::new(2));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            barrier.wait();
            integrate_oscillatory(
                |_| 1.0,
                0.0,
                std::f64::consts::PI,
                1.0,
                OscillatoryWeight::Sine,
                OscillatoryOptions::default(),
            )
            .unwrap()
            .value
        }));
    }
    for worker in workers {
        close(worker.join().unwrap(), 2.0, 2.0e-11);
    }
}
