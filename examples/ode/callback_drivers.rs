// slatec-safe-example
//! Stateful real and complex callback-driven ODE sessions.

use slatec::ode::{
    ComplexDriv1Session, ComplexDriv2Session, Driv1Session, Driv2Options, Driv2Session,
    DrivMethod, DrivStatus, OdeComplex32 as Complex32,
};

fn main() {
    let mut ordinary = Driv1Session::<f64>::new(0.0, vec![1.0], 1.0e-10)
        .expect("valid ordinary ODE session");
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
    let mut events = Driv2Session::<f32>::new(0.0, vec![0.0], options)
        .expect("valid event ODE session");
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

    let mut complex_events =
        ComplexDriv2Session::new(0.0, vec![Complex32::new(1.0, 0.0)], options)
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
