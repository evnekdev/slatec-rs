fn main() {
    let result = slatec::quadrature::integrate(
        |x| x * x,
        0.0,
        1.0,
        slatec::quadrature::IntegrationOptions::default(),
    )
    .expect("smooth finite integral");
    assert!((result.value - 1.0 / 3.0).abs() < 1.0e-10);
}
