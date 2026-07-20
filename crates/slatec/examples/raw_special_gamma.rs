//! Run with `cargo run -p slatec --example raw_special_gamma --features source-build,special-gamma` on the supported GNU MinGW target.
//!
//! Do not pass a Gamma pole: the native FNLIB error path may be fatal under a
//! caller's legacy XERROR configuration.

fn main() {
    slatec_src::ensure_linked();
    let mut x = 0.5_f64;
    let result = unsafe { slatec_sys::special::gamma::dgamma(&mut x) };
    println!("Gamma({x}) = {result}");
}
