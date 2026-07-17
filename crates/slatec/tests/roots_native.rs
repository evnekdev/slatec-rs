#![cfg(all(
    feature = "roots-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native execution tests for the reviewed GNU MinGW FZERO profile.
//!
//! Set `SLATEC_NATIVE_LIB_DIR` and `SLATEC_GFORTRAN_RUNTIME_DIR` before
//! running this target. The native archive is explicit and remains ignored.

use std::sync::{Arc, Barrier};
use std::thread;

use slatec::quadrature::{IntegrationOptions, integrate};
use slatec::roots::{RootBracket, RootError, RootOptions, RootStatus, find_root, find_root_f32};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn bracket() -> RootBracket<f64> {
    RootBracket {
        lower: 1.0,
        upper: 2.0,
    }
}

#[test]
fn scalar_roots_match_ordinary_references_in_both_precisions() {
    let square = find_root(|x| x * x - 2.0, bracket(), RootOptions::default()).unwrap();
    close(square.estimate, 2.0_f64.sqrt(), 2.0e-10);
    assert!(square.evaluations >= 3);
    assert!(matches!(
        square.status,
        RootStatus::Converged | RootStatus::EndpointRoot
    ));

    let sine = find_root(
        |x| x.sin(),
        RootBracket {
            lower: 3.0,
            upper: 4.0,
        },
        RootOptions::default(),
    )
    .unwrap();
    close(sine.estimate, std::f64::consts::PI, 2.0e-10);

    let exponential = find_root(
        |x| x.exp() - 2.0,
        RootBracket {
            lower: 0.0,
            upper: 2.0,
        },
        RootOptions::default(),
    )
    .unwrap();
    close(exponential.estimate, 2.0_f64.ln(), 2.0e-10);

    let single = find_root_f32(
        |x| x * x - 2.0,
        RootBracket {
            lower: 1.0_f32,
            upper: 2.0_f32,
        },
        RootOptions::single_precision(),
    )
    .unwrap();
    close(f64::from(single.estimate), 2.0_f64.sqrt(), 3.0e-5);
}

#[test]
fn endpoint_reversed_and_validation_cases_are_explicit() {
    let lower = find_root(
        |x| x,
        RootBracket {
            lower: 0.0,
            upper: 1.0,
        },
        RootOptions::default(),
    )
    .unwrap();
    assert_eq!(lower.status, RootStatus::LowerEndpoint);
    assert_eq!(lower.evaluations, 1);

    let upper = find_root(
        |x| x - 1.0,
        RootBracket {
            lower: 0.0,
            upper: 1.0,
        },
        RootOptions::default(),
    )
    .unwrap();
    assert_eq!(upper.status, RootStatus::UpperEndpoint);
    assert_eq!(upper.evaluations, 2);

    let reversed = find_root(
        |x| x * x - 2.0,
        RootBracket {
            lower: 2.0,
            upper: 1.0,
        },
        RootOptions {
            relative_tolerance: 1.0e-12,
            absolute_tolerance: 1.0e-14,
            initial_guess: Some(1.5),
        },
    )
    .unwrap();
    close(reversed.estimate, 2.0_f64.sqrt(), 2.0e-11);

    assert_eq!(
        find_root(
            |_| 1.0,
            RootBracket {
                lower: 0.0,
                upper: 1.0,
            },
            RootOptions::default(),
        ),
        Err(RootError::NoSignChange)
    );
}

#[test]
fn callback_failures_are_contained_and_valid_calls_recover() {
    for failure in [
        find_root(
            |_| panic!("contained root callback panic"),
            bracket(),
            RootOptions::default(),
        ),
        find_root(|_| f64::NAN, bracket(), RootOptions::default()),
        find_root(|_| f64::INFINITY, bracket(), RootOptions::default()),
        find_root(|_| f64::NEG_INFINITY, bracket(), RootOptions::default()),
    ] {
        assert!(matches!(
            failure,
            Err(RootError::CallbackPanicked | RootError::CallbackReturnedNonFinite)
        ));
    }
    let recovered = find_root(
        |x| x - 0.5,
        RootBracket {
            lower: 0.0,
            upper: 1.0,
        },
        RootOptions::default(),
    )
    .unwrap();
    close(recovered.estimate, 0.5, 1.0e-10);
}

#[test]
fn roots_and_quadrature_reject_cross_family_nesting() {
    let mut nested_root = None;
    let outer_quadrature = integrate(
        |x| {
            if nested_root.is_none() {
                nested_root = Some(find_root(
                    |y| y - 0.5,
                    RootBracket {
                        lower: 0.0,
                        upper: 1.0,
                    },
                    RootOptions::default(),
                ));
            }
            x
        },
        0.0,
        1.0,
        IntegrationOptions::default(),
    )
    .unwrap();
    close(outer_quadrature.value, 0.5, 1.0e-12);
    assert_eq!(nested_root, Some(Err(RootError::NestedNativeCallback)));

    let mut nested_quadrature = None;
    let outer_root = find_root(
        |x| {
            if nested_quadrature.is_none() {
                nested_quadrature = Some(integrate(|y| y, 0.0, 1.0, IntegrationOptions::default()));
            }
            x - 0.5
        },
        RootBracket {
            lower: 0.0,
            upper: 1.0,
        },
        RootOptions::default(),
    )
    .unwrap();
    close(outer_root.estimate, 0.5, 1.0e-10);
    assert_eq!(
        nested_quadrature,
        Some(Err(slatec::quadrature::IntegrationError::NestedIntegration))
    );
}

#[test]
fn parallel_cross_family_calls_serialize_without_sharing_callback_state() {
    let barrier = Arc::new(Barrier::new(3));
    let root_barrier = Arc::clone(&barrier);
    let root = thread::spawn(move || {
        root_barrier.wait();
        find_root(|x| x * x - 2.0, bracket(), RootOptions::default())
            .unwrap()
            .estimate
    });
    let quadrature_barrier = Arc::clone(&barrier);
    let quadrature = thread::spawn(move || {
        quadrature_barrier.wait();
        integrate(|x| x, 0.0, 1.0, IntegrationOptions::default())
            .unwrap()
            .value
    });
    barrier.wait();
    close(root.join().unwrap(), 2.0_f64.sqrt(), 2.0e-10);
    close(quadrature.join().unwrap(), 0.5, 1.0e-12);
}
