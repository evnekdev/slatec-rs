# Target-specific SLATEC bundled-provider carrier

This crate carries only target-specific native-provider artifacts for
`x86_64-pc-windows-gnu`; it exposes no numerical Rust API.

It currently contains the deterministic `special-elementary` SLATEC archive,
the exact static GNU runtime closure required by the reviewed final consumer,
and generated checksums, source-unit manifest, build recipe, compiler receipt,
symbol/runtime audits, SPDX SBOM, and redistribution notices. It does not
contain source caches, object intermediates, executables, or logs.

The carrier is usable only when `metadata/bundle-manifest.json` is `ready` and
every recorded checksum verifies. Other source families remain unavailable with
the bundled provider until their own hash-guarded provenance closure is
cleared. See `generated/licensing/` in the workspace for the source-level
evidence and runtime obligations.
