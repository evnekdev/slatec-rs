//! Callback-contained scalar root example for FZERO and DFZERO.
// slatec-safe-example: bracketed scalar roots in f64 and f32.

use slatec::roots::{find_root, RootBracket, RootOptions};

fn main() -> Result<(), slatec::roots::RootError> {
    let result = find_root(
        |x| x * x - 2.0,
        RootBracket { lower: 1.0, upper: 2.0 },
        RootOptions::default(),
    )?;
    assert!((result.estimate - 2.0_f64.sqrt()).abs() < 1.0e-10);
    Ok(())
}
