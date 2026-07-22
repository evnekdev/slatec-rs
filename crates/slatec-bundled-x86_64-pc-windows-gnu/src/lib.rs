#![no_std]

//! Target-specific crates.io carrier for bundled SLATEC native archives.
//!
//! The crate deliberately exposes no numerical Rust API. Its build metadata is
//! consumed only by `slatec-src` when the `bundled` provider is selected.
//!
//! The supported target is `x86_64-pc-windows-gnu`. The crate is not a Rust
//! numerical API and must not be depended on directly; select the `bundled`
//! provider through `slatec` or `slatec-src` instead. The package contains one
//! hash-verified accepted-source archive, reduced static GNU runtime closures,
//! exact runtime licence texts, and deterministic source/relink receipts.
