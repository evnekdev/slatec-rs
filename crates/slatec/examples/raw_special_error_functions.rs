//! Run with `cargo run -p slatec --example raw_special_error_functions --features source-build,special-error` on the supported GNU MinGW target.
//!
//! The raw call passes its scalar by address. Tail underflow and legacy FNLIB
//! diagnostics remain native behavior rather than Rust errors.

fn main() {
    slatec_src::ensure_linked();
    let mut x = 1.0_f64;
    let result = unsafe { slatec_sys::special::error::derfc(&mut x) };
    println!("erfc({x}) = {result}");
}
