fn main() {
    use slatec::quadrature::{OscillatoryOptions, OscillatoryWeight, integrate_oscillatory};
    let result = integrate_oscillatory(
        |_| 1.0,
        0.0,
        core::f64::consts::PI,
        1.0,
        OscillatoryWeight::Sine,
        OscillatoryOptions::default(),
    )
    .expect("analytic oscillatory integral");
    assert!((result.value - 2.0).abs() < 1.0e-10);
}
