# Target-specific SLATEC bundled-provider carrier

This crates.io package carries only native-provider artifacts for
`x86_64-unknown-linux-gnu`; it exposes no numerical Rust API. Depend on
`slatec` (or, for expert provider selection, `slatec-src`), not on this
carrier directly.

The carrier contains one deterministic archive built from exact
hash-pinned historical SLATEC sources, plus only the static GNU runtime
members recorded in its receipt. Its generated metadata records the target,
compiler profile, source closures, archive checksums, native dependency
inspection, and source-build parity probe. It contains no source cache,
object intermediates, executables, or compiler logs.
