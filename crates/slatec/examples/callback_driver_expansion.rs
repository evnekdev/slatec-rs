//! Compile-and-link example for the safe callback-driver expansion.

use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;
use slatec::nonlinear::{SystemOptions, solve_scalar_equations};
use slatec::ode::{
    ComplexDriv1Session, ComplexDriv2Session, Driv1Session, Driv2Options, Driv2Session, DrivMethod,
    DrivStatus, OdeComplex32 as Complex32,
};
use slatec::quadrature::integrate_piecewise_polynomial;

fn main() {
    let constant = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 1)
        .expect("valid constant piecewise polynomial");
    let integral = integrate_piecewise_polynomial(&constant, 0, 0.0..=1.0, 1.0e-8, |x| x * x)
        .expect("piecewise-polynomial quadrature succeeds");
    assert!((integral.value - 1.0 / 3.0).abs() < 1.0e-8);

    let system = solve_scalar_equations(&[0.8, 0.6], SystemOptions::default(), |x, equation| {
        if equation == 0 {
            x[0] * x[0] + x[1] * x[1] - 1.0
        } else {
            x[0] - x[1]
        }
    })
    .expect("scalar equations converge");
    assert!((system.solution[0] - 2.0_f64.sqrt() / 2.0).abs() < 1.0e-6);

    let mut ordinary =
        Driv1Session::<f64>::new(0.0, vec![1.0], 1.0e-10).expect("valid ordinary ODE session");
    ordinary
        .integrate_to(1.0, |_, state, derivative| derivative[0] = state[0])
        .expect("ordinary ODE integration succeeds");
    assert!((ordinary.state()[0] - core::f64::consts::E).abs() < 3.0e-8);

    let options = Driv2Options {
        relative_tolerance: 1.0e-5,
        error_weight: 1.0,
        method: DrivMethod::Adams,
        root_count: 1,
    };
    let mut events =
        Driv2Session::<f32>::new(0.0, vec![0.0], options).expect("valid event ODE session");
    assert_eq!(
        events
            .integrate_to_with_events(
                2.0,
                |_, _, derivative| derivative[0] = 1.0,
                |_, state, index| {
                    assert_eq!(index, 0);
                    state[0] - 0.5
                },
            )
            .expect("event ODE integration succeeds")
            .status,
        DrivStatus::RootFound { index: 0 },
    );

    let mut complex = ComplexDriv1Session::new(0.0, vec![Complex32::new(1.0, 0.0)], 1.0e-5)
        .expect("valid complex ODE session");
    complex
        .integrate_to(1.0, |_, state, derivative| {
            derivative[0] = Complex32::new(-state[0].im, state[0].re);
        })
        .expect("complex ODE integration succeeds");
    let mut complex_events = ComplexDriv2Session::new(0.0, vec![Complex32::new(1.0, 0.0)], options)
        .expect("valid complex event ODE session");
    assert_eq!(
        complex_events
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
            .expect("complex event ODE integration succeeds")
            .status,
        DrivStatus::RootFound { index: 0 },
    );
}
