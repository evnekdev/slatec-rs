#![cfg(all(
    feature = "nonlinear-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native execution tests for the reviewed GNU MinGW SNSQE/DNSQE profile.
//!
//! These tests require the explicit validated native provider. They are kept
//! separate from source-only CI because callback-bearing Fortran execution is
//! not available on every developer or documentation host.

use std::sync::{Arc, Barrier};
use std::thread;
use std::{env, process::Command};

use slatec::nonlinear::{
    NonlinearError, NonlinearOptions, NonlinearStatus, solve_system, solve_system_f32,
};
use slatec::quadrature::{IntegrationOptions, integrate};
use slatec::roots::{RootBracket, RootOptions, find_root};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn scalar_system() -> Result<slatec::nonlinear::NonlinearResult<f64>, NonlinearError> {
    solve_system(
        &[1.0],
        |x, residual| {
            residual[0] = x[0] * x[0] - 2.0;
        },
        NonlinearOptions::default(),
    )
}

#[test]
fn easy_drivers_solve_reference_systems_in_both_precisions() {
    let scalar = scalar_system().unwrap();
    close(scalar.solution[0], 2.0_f64.sqrt(), 2.0e-9);
    assert!(scalar.residual_norm < 2.0e-9);
    assert!(scalar.function_evaluations > 0);
    assert_eq!(scalar.status, NonlinearStatus::Converged);

    let pair = solve_system(
        &[0.8, 2.2],
        |x, residual| {
            residual[0] = x[0] + x[1] - 3.0;
            residual[1] = x[0] * x[0] + x[1] * x[1] - 5.0;
        },
        NonlinearOptions::default(),
    )
    .unwrap();
    close(pair.solution[0], 1.0, 2.0e-8);
    close(pair.solution[1], 2.0, 2.0e-8);
    assert!(pair.residual_norm < 2.0e-8);

    let coupled = solve_system(
        &[1.2, 1.9, 2.8],
        |x, residual| {
            residual[0] = x[0] + x[1] + x[2] - 6.0;
            residual[1] = x[0] * x[0] + x[1] * x[1] + x[2] * x[2] - 14.0;
            residual[2] = x[0] * x[1] * x[2] - 6.0;
        },
        NonlinearOptions::default(),
    )
    .unwrap();
    for (actual, expected) in coupled.solution.iter().zip([1.0, 2.0, 3.0]) {
        close(*actual, expected, 1.0e-7);
    }
    assert!(coupled.residual_norm < 1.0e-7);

    let single = solve_system_f32(
        &[1.0_f32],
        |x, residual| {
            residual[0] = x[0] * x[0] - 2.0;
        },
        NonlinearOptions::single_precision(),
    )
    .unwrap();
    close(f64::from(single.solution[0]), 2.0_f64.sqrt(), 3.0e-4);
    assert!(single.residual_norm < 4.0e-4);
}

#[test]
fn callback_failures_are_contained_and_do_not_poison_later_solves() {
    assert!(matches!(
        solve_system(
            &[1.0],
            |_, _| panic!("contained nonlinear callback panic"),
            NonlinearOptions::default(),
        ),
        Err(NonlinearError::CallbackPanicked)
    ));
    assert!(matches!(
        solve_system(
            &[1.0],
            |_, residual| {
                residual[0] = f64::NAN;
            },
            NonlinearOptions::default(),
        ),
        Err(NonlinearError::CallbackReturnedNonFinite { index: 0 })
    ));
    assert!(scalar_system().is_ok());
}

#[test]
fn invalid_inputs_are_rejected_without_native_execution() {
    assert_eq!(
        solve_system(&[], |_, _| {}, NonlinearOptions::default()),
        Err(NonlinearError::EmptySystem)
    );
    assert_eq!(
        solve_system(&[f64::INFINITY], |_, _| {}, NonlinearOptions::default(),),
        Err(NonlinearError::NonFiniteInitialValue { index: 0 })
    );
    assert_eq!(
        solve_system(&[1.0], |_, _| {}, NonlinearOptions { tolerance: -1.0 },),
        Err(NonlinearError::InvalidTolerance)
    );
}

#[test]
fn nested_cross_family_callbacks_are_rejected_without_deadlock() {
    let nested_root = solve_system(
        &[1.0],
        |_, residual| {
            assert!(matches!(
                find_root(
                    |x| x * x - 2.0,
                    RootBracket {
                        lower: 1.0,
                        upper: 2.0
                    },
                    RootOptions::default(),
                ),
                Err(slatec::roots::RootError::NestedNativeCallback)
            ));
            residual[0] = 0.0;
        },
        NonlinearOptions::default(),
    );
    assert!(nested_root.is_ok());

    let nested_quadrature = solve_system(
        &[1.0],
        |_, residual| {
            assert!(matches!(
                integrate(|x| x, 0.0, 1.0, IntegrationOptions::default()),
                Err(slatec::quadrature::IntegrationError::NestedIntegration)
            ));
            residual[0] = 0.0;
        },
        NonlinearOptions::default(),
    );
    assert!(nested_quadrature.is_ok());

    let root = find_root(
        |x| {
            assert!(matches!(
                solve_system(
                    &[1.0],
                    |_, residual| residual[0] = 0.0,
                    NonlinearOptions::default(),
                ),
                Err(NonlinearError::NestedNativeCallback)
            ));
            x * x - 2.0
        },
        RootBracket {
            lower: 1.0,
            upper: 2.0,
        },
        RootOptions::default(),
    );
    assert!(root.is_ok());

    let integral = integrate(
        |x| {
            assert!(matches!(
                solve_system(
                    &[1.0],
                    |_, residual| residual[0] = 0.0,
                    NonlinearOptions::default(),
                ),
                Err(NonlinearError::NestedNativeCallback)
            ));
            x
        },
        0.0,
        1.0,
        IntegrationOptions::default(),
    );
    assert!(integral.is_ok());
}

#[test]
fn parallel_nonlinear_calls_share_the_native_serialization_policy() {
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            barrier.wait();
            scalar_system().unwrap().solution[0]
        }));
    }
    barrier.wait();
    for worker in workers {
        close(worker.join().unwrap(), 2.0_f64.sqrt(), 2.0e-9);
    }
}

#[test]
fn cross_family_parallel_callers_share_the_native_serialization_policy() {
    let barrier = Arc::new(Barrier::new(4));
    let nonlinear_barrier = Arc::clone(&barrier);
    let nonlinear = thread::spawn(move || {
        nonlinear_barrier.wait();
        scalar_system().unwrap().solution[0]
    });
    let roots_barrier = Arc::clone(&barrier);
    let root = thread::spawn(move || {
        roots_barrier.wait();
        find_root(
            |x| x * x - 2.0,
            RootBracket {
                lower: 1.0,
                upper: 2.0,
            },
            RootOptions::default(),
        )
        .unwrap()
        .estimate
    });
    let quadrature_barrier = Arc::clone(&barrier);
    let quadrature = thread::spawn(move || {
        quadrature_barrier.wait();
        integrate(|x| x * x, 0.0, 1.0, IntegrationOptions::default())
            .unwrap()
            .value
    });
    barrier.wait();
    close(nonlinear.join().unwrap(), 2.0_f64.sqrt(), 2.0e-9);
    close(root.join().unwrap(), 2.0_f64.sqrt(), 2.0e-9);
    close(quadrature.join().unwrap(), 1.0 / 3.0, 2.0e-12);
}

unsafe extern "C" fn negative_iflag_callback(
    _n: *const slatec_sys::FortranInteger,
    _x: *const f64,
    fvec: *mut f64,
    iflag: *mut slatec_sys::FortranInteger,
) {
    if let Some(fvec) = unsafe { fvec.as_mut() } {
        *fvec = 0.0;
    }
    if let Some(iflag) = unsafe { iflag.as_mut() } {
        *iflag = -1;
    }
}

unsafe extern "C" fn unused_child_jacobian(
    _n: *const slatec_sys::FortranInteger,
    _x: *const f64,
    _fvec: *const f64,
    _fjac: *mut f64,
    _ldfjac: *const slatec_sys::FortranInteger,
    _iflag: *mut slatec_sys::FortranInteger,
) {
}

#[test]
fn legacy_negative_iflag_is_contained_in_a_child_process() {
    if env::var_os("SLATEC_NEGATIVE_IFLAG_CHILD").is_some() {
        let mut iopt = 2;
        let mut n = 1;
        let mut x = [1.0];
        let mut fvec = [0.0];
        let mut tolerance = 1.0e-10;
        let mut nprint = 0;
        let mut info = 0;
        let mut workspace = [0.0; 8];
        let mut workspace_length = 8;
        // SAFETY: this isolated child intentionally exercises the reviewed
        // legacy negative-IFLAG path with valid scalar/vector/workspace ABI.
        unsafe {
            slatec_sys::nonlinear::dnsqe(
                negative_iflag_callback,
                unused_child_jacobian,
                &mut iopt,
                &mut n,
                x.as_mut_ptr(),
                fvec.as_mut_ptr(),
                &mut tolerance,
                &mut nprint,
                &mut info,
                workspace.as_mut_ptr(),
                &mut workspace_length,
            );
        }
        panic!("negative IFLAG unexpectedly returned from DNSQE");
    }

    let status = Command::new(env::current_exe().unwrap())
        .arg("--exact")
        .arg("legacy_negative_iflag_is_contained_in_a_child_process")
        .arg("--nocapture")
        .env("SLATEC_NEGATIVE_IFLAG_CHILD", "1")
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(70));
}
