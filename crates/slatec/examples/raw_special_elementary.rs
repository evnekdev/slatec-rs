//! Run with `cargo run -p slatec --example raw_special_elementary --features source-build,special-elementary` on the supported GNU MinGW target.
//!
//! This is a raw call: the scalar is passed by address, FNLIB state is not
//! serialized for the caller, and the caller is responsible for its domain.

fn main() {
    slatec_src::ensure_linked();
    let mut x = 1.0e-8_f64;
    let result = unsafe { slatec_sys::special::elementary::dlnrel(&mut x) };
    println!("ln(1 + {x}) = {result}");
}
