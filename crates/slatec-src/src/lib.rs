#![no_std]
#![deny(missing_docs)]

//! Native implementation-provider selection for `slatec`.
//!
//! This crate intentionally exposes no numerical API. Its Cargo features
//! select one native provider and keep that provider in the dependency graph.
//! The runtime library is `no_std`; acquisition, compilation, and linker
//! configuration are confined to its hosted Cargo build script.

/// Keeps the selected implementation provider in the final dependency graph.
///
/// Safe wrapper crates call this zero-cost marker internally. It performs no
/// runtime initialization and is not a numerical interface.
#[doc(hidden)]
#[inline(never)]
pub fn ensure_linked() {}
