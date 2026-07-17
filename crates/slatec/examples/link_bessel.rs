fn main() {
    let value = slatec::special::bessel::bessel_j0(0.0).expect("validated input");
    assert!((value - 1.0).abs() < 1.0e-14);
}
