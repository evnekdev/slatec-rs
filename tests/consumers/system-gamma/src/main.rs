fn main() {
    let value = slatec::special::gamma::gamma(1.0).expect("validated gamma input");
    assert_eq!(value, 1.0);
}
