// slatec-safe-example: tabulated-data

use slatec::interpolation::tabulated::TabulatedData;
use slatec::quadrature::integrate_tabulated;

fn main() {
    let samples = TabulatedData::<f64>::from_samples(vec![0.0_f64, 1.0, 2.0], vec![1.0, 4.0, 9.0])
        .expect("finite, strictly increasing samples");
    let interpolant = samples
        .interpolating_polynomial()
        .expect("reviewed interpolation representation");
    let value = interpolant.evaluate(1.5).expect("finite query");
    let derivatives = interpolant
        .evaluate_with_derivatives(1.5, 2)
        .expect("finite query");
    let coefficients = interpolant
        .taylor_coefficients_at(1.0)
        .expect("finite expansion centre");
    let area = integrate_tabulated(&samples, 0.0..=2.0).expect("sufficient tabulated samples");

    println!(
        "value={value}, first derivative={}, coefficients={coefficients:?}, area={area}",
        derivatives.derivatives[0]
    );
}
