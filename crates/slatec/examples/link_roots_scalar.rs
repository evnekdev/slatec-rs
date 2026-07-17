fn main() {
    use slatec::roots::{RootBracket, RootOptions, find_root};
    let result = find_root(
        |x| x * x - 2.0,
        RootBracket {
            lower: 1.0,
            upper: 2.0,
        },
        RootOptions::default(),
    )
    .expect("bracketed root");
    assert!((result.estimate - 2.0_f64.sqrt()).abs() < 1.0e-10);
}
