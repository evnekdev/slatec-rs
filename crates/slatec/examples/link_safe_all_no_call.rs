//! Ensures the full safe API feature set creates no native implementation root
//! until an operation is actually called.

fn main() {
    let _ = core::mem::size_of::<slatec::linear_algebra::blas::BlasError>();
}
