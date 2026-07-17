fn main() {
    let value = slatec::special::airy::airy_ai(0.0).expect("validated Airy input");
    assert!((value - 0.355_028_053_887_817).abs() < 1.0e-12);
}
