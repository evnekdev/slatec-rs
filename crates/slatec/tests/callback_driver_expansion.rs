use num_complex::Complex32;
use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;
use slatec::nonlinear::{
    SystemOptions, SystemTermination, solve_scalar_equations, solve_scalar_equations_f32,
};
use slatec::ode::{
    ComplexDriv1Session, ComplexDriv2Session, Driv1Session, Driv2Options, Driv2Session, DrivMethod,
    DrivStatus, OdeError,
};
use slatec::quadrature::{
    IntegrationError, PiecewiseQuadratureStatus, integrate_piecewise_polynomial,
};

#[test]
fn dpfqad_integrates_a_polynomial_times_a_checked_constant_pp() {
    let polynomial = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 1).unwrap();
    let result =
        integrate_piecewise_polynomial(&polynomial, 0, 0.0..=1.0, 1.0e-8, |x| x * x).unwrap();
    assert_eq!(result.status, PiecewiseQuadratureStatus::Converged);
    assert!((result.value - 1.0 / 3.0).abs() < 1.0e-8);
}

fn circle_f64(values: &[f64], index: usize) -> f64 {
    match index {
        0 => values[0] * values[0] + values[1] * values[1] - 1.0,
        1 => values[0] - values[1],
        _ => unreachable!(),
    }
}

fn circle_f32(values: &[f32], index: usize) -> f32 {
    match index {
        0 => values[0] * values[0] + values[1] * values[1] - 1.0,
        1 => values[0] - values[1],
        _ => unreachable!(),
    }
}

#[test]
fn sos_and_dsos_solve_the_positive_circle_system() {
    let result = solve_scalar_equations(&[0.8, 0.6], SystemOptions::default(), circle_f64).unwrap();
    assert!(matches!(
        result.termination,
        SystemTermination::IterateConverged
            | SystemTermination::ResidualConverged
            | SystemTermination::BothConverged
    ));
    assert!((result.solution[0] - 2.0_f64.sqrt() / 2.0).abs() < 1.0e-6);
    assert!((result.solution[1] - 2.0_f64.sqrt() / 2.0).abs() < 1.0e-6);

    let result =
        solve_scalar_equations_f32(&[0.8, 0.6], SystemOptions::default(), circle_f32).unwrap();
    assert!(matches!(
        result.termination,
        SystemTermination::IterateConverged
            | SystemTermination::ResidualConverged
            | SystemTermination::BothConverged
    ));
    assert!((result.solution[0] - 2.0_f32.sqrt() / 2.0).abs() < 2.0e-3);
    assert!((result.solution[1] - 2.0_f32.sqrt() / 2.0).abs() < 2.0e-3);
}

#[test]
fn real_driv1_sessions_integrate_exponentials_and_preserve_continuation() {
    let mut single = Driv1Session::<f32>::new(0.0, vec![1.0], 1.0e-5).unwrap();
    assert_eq!(
        single
            .integrate_to(0.5, |_, state, derivative| derivative[0] = state[0])
            .unwrap()
            .status,
        DrivStatus::ReachedTarget
    );
    single
        .integrate_to(1.0, |_, state, derivative| derivative[0] = state[0])
        .unwrap();
    assert!((single.state()[0] - core::f32::consts::E).abs() < 3.0e-3);

    let mut double = Driv1Session::<f64>::new(0.0, vec![1.0], 1.0e-10).unwrap();
    double
        .integrate_to(1.0, |_, state, derivative| derivative[0] = state[0])
        .unwrap();
    assert!((double.state()[0] - core::f64::consts::E).abs() < 3.0e-8);
}

#[test]
fn real_driv2_sessions_report_zero_based_event_indices() {
    let single_options = Driv2Options {
        relative_tolerance: 1.0e-5_f32,
        error_weight: 1.0,
        method: DrivMethod::Adams,
        root_count: 1,
    };
    let mut single = Driv2Session::<f32>::new(0.0, vec![0.0], single_options).unwrap();
    assert_eq!(
        single
            .integrate_to_with_events(
                2.0,
                |_, _, derivative| derivative[0] = 1.0,
                |_, state, index| {
                    assert_eq!(index, 0);
                    state[0] - 0.5
                },
            )
            .unwrap()
            .status,
        DrivStatus::RootFound { index: 0 }
    );

    let double_options = Driv2Options {
        relative_tolerance: 1.0e-10_f64,
        error_weight: 1.0,
        method: DrivMethod::Adams,
        root_count: 1,
    };
    let mut double = Driv2Session::<f64>::new(0.0, vec![0.0], double_options).unwrap();
    assert_eq!(
        double
            .integrate_to_with_events(
                2.0,
                |_, _, derivative| derivative[0] = 1.0,
                |_, state, index| {
                    assert_eq!(index, 0);
                    state[0] - 0.5
                },
            )
            .unwrap()
            .status,
        DrivStatus::RootFound { index: 0 }
    );
}

#[test]
fn complex_driv_sessions_integrate_rotation_and_report_events() {
    let mut ordinary =
        ComplexDriv1Session::new(0.0, vec![Complex32::new(1.0, 0.0)], 1.0e-5).unwrap();
    ordinary
        .integrate_to(1.0, |_, state, derivative| {
            derivative[0] = Complex32::new(-state[0].im, state[0].re);
        })
        .unwrap();
    assert!((ordinary.state()[0].re - 1.0_f32.cos()).abs() < 3.0e-3);
    assert!((ordinary.state()[0].im - 1.0_f32.sin()).abs() < 3.0e-3);

    let options = Driv2Options {
        relative_tolerance: 1.0e-5,
        error_weight: 1.0,
        method: DrivMethod::Adams,
        root_count: 1,
    };
    let mut events =
        ComplexDriv2Session::new(0.0, vec![Complex32::new(1.0, 0.0)], options).unwrap();
    assert_eq!(
        events
            .integrate_to_with_events(
                2.0,
                |_, state, derivative| {
                    derivative[0] = Complex32::new(-state[0].im, state[0].re);
                },
                |_, state, index| {
                    assert_eq!(index, 0);
                    state[0].re - 0.5
                },
            )
            .unwrap()
            .status,
        DrivStatus::RootFound { index: 0 }
    );
}

#[test]
fn callback_panics_are_contained_and_do_not_leave_stale_context() {
    let polynomial = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 1).unwrap();
    assert!(matches!(
        integrate_piecewise_polynomial(&polynomial, 0, 0.0..=1.0, 1.0e-8, |_| panic!("DPFQAD")),
        Err(IntegrationError::CallbackPanicked)
    ));
    let result =
        integrate_piecewise_polynomial(&polynomial, 0, 0.0..=1.0, 1.0e-8, |x| x * x).unwrap();
    assert!((result.value - 1.0 / 3.0).abs() < 1.0e-8);

    assert!(matches!(
        solve_scalar_equations(&[0.8, 0.6], SystemOptions::default(), |_, _| panic!("DSOS")),
        Err(slatec::nonlinear::NonlinearError::CallbackPanicked)
    ));
    assert!(solve_scalar_equations(&[0.8, 0.6], SystemOptions::default(), circle_f64).is_ok());

    let mut session = Driv1Session::<f64>::new(0.0, vec![1.0], 1.0e-10).unwrap();
    assert!(matches!(
        session.integrate_to(1.0, |_, _, _| panic!("DDRIV1")),
        Err(OdeError::CallbackPanicked)
    ));
    assert!(matches!(
        session.integrate_to(1.0, |_, state, derivative| derivative[0] = state[0]),
        Err(OdeError::SessionFailed)
    ));
}
