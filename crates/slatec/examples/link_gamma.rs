fn main() {
    let value = slatec::special::gamma::gamma(0.5).expect("validated gamma input");
    assert!((value - core::f64::consts::PI.sqrt()).abs() < 1.0e-12);
}
