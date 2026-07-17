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
    InfiniteInterval, IntegrationError, IntegrationOptions, integrate, integrate_f32,
    integrate_infinite, integrate_infinite_f32, integrate_principal_value,
    integrate_principal_value_f32, integrate_singular, integrate_singular_f32,
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
