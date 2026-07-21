//! Link probe for the reviewed scalar special-function closure.

fn main() {
    use slatec::special::scalar_expanded::{
        carlson_rc, carlson_rd, carlson_rf, carlson_rj, logarithmic_integral, spence_integral,
    };

    let values = [
        logarithmic_integral(2.0).expect("li(2) is in the reviewed domain"),
        spence_integral(1.0).expect("Spence integral is finite at one"),
        carlson_rc(0.0, 1.0).expect("RC(0, 1) is defined"),
        carlson_rf(1.0, 1.0, 1.0).expect("RF(1, 1, 1) is defined"),
        carlson_rd(1.0, 1.0, 1.0).expect("RD(1, 1, 1) is defined"),
        carlson_rj(1.0, 1.0, 1.0, 1.0).expect("RJ(1, 1, 1, 1) is defined"),
    ];
    assert!(values.iter().all(|value| value.is_finite()));
}
