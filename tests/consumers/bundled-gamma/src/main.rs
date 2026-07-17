fn main() {
    let value = slatec::special::gamma::gamma(1.0).expect("ordinary gamma input");
    assert_eq!(value, 1.0);
}
