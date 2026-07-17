fn main() {
    let value = slatec::special::elementary::log1p(0.5).expect("validated input");
    assert!((value - 1.5_f64.ln()).abs() < 1.0e-14);
}
