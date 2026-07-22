# Target-specific SLATEC bundled-provider carrier

This crate carries only target-specific native-provider metadata and, after
source-level redistribution clearance, a deterministic static archive for
`x86_64-pc-windows-gnu`. It exposes no numerical Rust API.

The committed carrier is intentionally metadata-only. Its generated manifest
records the provenance gate that currently blocks archive production; a build
using `slatec-src/bundled` therefore fails before selecting a compiler, source
cache, system directory, or network path. See `generated/licensing/` in the
workspace for the source-level evidence.
