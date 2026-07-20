//! Run with `cargo run -p slatec --example raw_special_beta --features source-build,special-beta` on the supported GNU MinGW target.
//!
//! Both arguments are passed by address and must be positive to avoid the
//! native XERROR domain path.

fn main() {
    slatec_src::ensure_linked();
    let mut a = 0.5_f64;
    let mut b = 0.5_f64;
    let result = unsafe { slatec_sys::special::beta::dbeta(&mut a, &mut b) };
    println!("Beta({a}, {b}) = {result}");
}
