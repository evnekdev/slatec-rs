#![no_std]

//! Target-specific crates.io carrier for bundled SLATEC native archives.
//!
//! This crate exposes no numerical Rust API. Its hash-verified build metadata
//! is consumed only by `slatec-src` when that crate's `bundled` provider is
//! selected for `x86_64-unknown-linux-gnu`.
