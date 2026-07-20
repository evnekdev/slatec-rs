//! Run with `cargo run -p slatec --example raw_airy --features source-build,special-airy`
//! on the supported GNU MinGW target.
//!
//! The raw call passes a real scalar by address. The caller remains responsible
//! for pointer validity, the FNLIB global runtime, and native diagnostic paths.

fn main() {
    slatec_src::ensure_linked();
    let mut x = 0.0_f64;
    let result = unsafe { slatec_sys::special::airy::dai(&mut x) };
    println!("Ai({x}) = {result}");
}
