#![cfg(feature = "ode-sdrive-expert-native-tests")]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use slatec::ode::{
    OdeError, OdeInputError, OdeMethod, OdeOptions, OdeSession, OdeStatus, OdeTolerance,
    OdeTolerances,
};

fn scalar_tolerances<T>(relative: T, absolute: T) -> OdeTolerances<T> {
    OdeTolerances {
        relative,
        absolute: OdeTolerance::Scalar(absolute),
    }
}

#[test]
fn double_exponential_decay_continues_to_multiple_targets() {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -2.0 * state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        session.integrate_to(0.25).unwrap().status,
        OdeStatus::ReachedTarget
    );
    assert_eq!(
        session.integrate_to(1.0).unwrap().status,
        OdeStatus::ReachedTarget
    );
    assert!((session.state()[0] - (-2.0_f64).exp()).abs() < 3.0e-6);
}

#[test]
fn direct_and_segmented_continuation_agree() {
    let build_session = || {
        OdeSession::new(
            0.0_f64,
            vec![1.0],
            |_time, state, derivative| -> Result<(), ()> {
                derivative[0] = -state[0];
                Ok(())
            },
            scalar_tolerances(1.0e-9, 1.0e-11),
            OdeOptions::default(),
        )
        .unwrap()
    };
    let mut direct = build_session();
    let mut segmented = build_session();
    direct.integrate_to(1.0).unwrap();
    segmented.integrate_to(0.2).unwrap();
    segmented.integrate_to(0.6).unwrap();
    segmented.integrate_to(1.0).unwrap();
    assert!((direct.state()[0] - segmented.state()[0]).abs() < 2.0e-7);
    assert!((direct.state()[0] - (-1.0_f64).exp()).abs() < 2.0e-6);
}

#[test]
fn single_harmonic_oscillator_accepts_component_tolerances() {
    let mut session = OdeSession::new(
        0.0_f32,
        vec![1.0, 0.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = state[1];
            derivative[1] = -state[0];
            Ok(())
        },
        OdeTolerances {
            relative: 1.0e-5,
            absolute: OdeTolerance::Vector(vec![1.0e-7, 1.0e-7]),
        },
        OdeOptions::default(),
    )
    .unwrap();
    session.integrate_to(core::f32::consts::FRAC_PI_2).unwrap();
    assert!(session.state()[0].abs() < 3.0e-3);
    assert!((session.state()[1] + 1.0).abs() < 3.0e-3);
}

#[test]
fn backward_integration_is_supported_from_a_fresh_session() {
    let mut session = OdeSession::new(
        1.0_f64,
        vec![1.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = 1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-9, 1.0e-11),
        OdeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        session.integrate_to(0.0).unwrap().status,
        OdeStatus::ReachedTarget
    );
    assert!(session.state()[0].abs() < 1.0e-7);
}

#[test]
fn bdf_functional_iteration_remains_rhs_only() {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -10.0 * state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-7, 1.0e-9),
        OdeOptions {
            method: OdeMethod::Bdf,
            maximum_order: Some(5),
            maximum_steps: None,
            maximum_step: Some(0.01),
        },
    )
    .unwrap();
    session.integrate_to(0.2).unwrap();
    assert!((session.state()[0] - (-2.0_f64).exp()).abs() < 1.0e-4);
}

#[test]
fn callback_error_terminates_without_leaking_context() {
    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    let mut failing = OdeSession::new(
        0.0_f64,
        vec![1.0],
        move |_time, _state, derivative| -> Result<(), &'static str> {
            if callback_calls.fetch_add(1, Ordering::SeqCst) > 1 {
                return Err("stop");
            }
            derivative[0] = -1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        failing.integrate_to(1.0),
        Err(OdeError::Callback("stop"))
    ));
    assert!(matches!(
        failing.integrate_to(1.0),
        Err(OdeError::SessionFailed)
    ));

    let mut recovered = OdeSession::new(
        0.0_f64,
        vec![0.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = 1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    recovered.integrate_to(1.0).unwrap();
    assert!((recovered.state()[0] - 1.0).abs() < 1.0e-6);
}

#[test]
fn callback_panic_and_nonfinite_derivative_are_contained() {
    let mut panicking = OdeSession::new(
        0.0_f32,
        vec![1.0],
        |_time, _state, _derivative| -> Result<(), ()> { panic!("contained") },
        scalar_tolerances(1.0e-5, 1.0e-7),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        panicking.integrate_to(1.0),
        Err(OdeError::CallbackPanicked)
    ));

    let mut nonfinite = OdeSession::new(
        0.0_f32,
        vec![1.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = f32::NAN;
            Ok(())
        },
        scalar_tolerances(1.0e-5, 1.0e-7),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        nonfinite.integrate_to(1.0),
        Err(OdeError::NonFiniteDerivative { index: 0 })
    ));
}

#[test]
fn callback_failure_restores_xerror_and_allows_fresh_session() {
    let mut before = 0;
    // SAFETY: the reviewed XGETF ABI writes one valid native INTEGER.
    unsafe { slatec_sys::legacy_error::xgetf(&mut before) };
    let mut failing = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, _state, _derivative| -> Result<(), ()> { Err(()) },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        failing.integrate_to(1.0),
        Err(OdeError::Callback(()))
    ));
    let mut after = 0;
    // SAFETY: the reviewed XGETF ABI writes one valid native INTEGER.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
    assert_eq!(
        after, before,
        "SDRIV3 callback termination must restore XERROR"
    );

    let mut fresh = OdeSession::new(
        0.0_f64,
        vec![0.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = 1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    fresh.integrate_to(1.0).unwrap();
    assert!((fresh.state()[0] - 1.0).abs() < 1.0e-6);
}

#[test]
fn rejects_invalid_input_before_native_entry() {
    let result = OdeSession::new(
        0.0_f64,
        Vec::new(),
        |_time, _state, _derivative| -> Result<(), ()> { Ok(()) },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    );
    assert!(matches!(
        result,
        Err(OdeError::InvalidInput(OdeInputError::EmptyState))
    ));

    let result = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, _state, _derivative| -> Result<(), ()> { Ok(()) },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions {
            maximum_order: Some(13),
            ..OdeOptions::default()
        },
    );
    assert!(matches!(
        result,
        Err(OdeError::InvalidInput(OdeInputError::InvalidMaximumOrder))
    ));
}

#[test]
fn independent_sessions_are_serialized_without_context_cross_talk() {
    let handles = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let mut session = OdeSession::new(
                    0.0_f64,
                    vec![0.0],
                    |_time, _state, derivative| -> Result<(), ()> {
                        derivative[0] = 1.0;
                        Ok(())
                    },
                    scalar_tolerances(1.0e-8, 1.0e-10),
                    OdeOptions::default(),
                )
                .unwrap();
                session.integrate_to(1.0).unwrap();
                session.state()[0]
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        assert!((handle.join().unwrap() - 1.0).abs() < 1.0e-6);
    }
}
