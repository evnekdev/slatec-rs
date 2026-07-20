//! Generated Batch C native-symbol link probe.

#![cfg(feature = "raw-batch-c-link-tests")]

#[test]
fn links_batch_c_symbols() {
    let _ = slatec_sys::nonlinear::complex::rpzero as *const ();
}
