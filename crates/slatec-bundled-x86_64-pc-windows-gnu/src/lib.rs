#![no_std]

//! Target-specific carrier for the bundled SLATEC native archive.
//!
//! The crate deliberately exposes no numerical Rust API. Its build metadata is
//! consumed only by `slatec-src` when the `bundled` provider is selected.
