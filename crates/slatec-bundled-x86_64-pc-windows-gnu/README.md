# Target-specific SLATEC bundled-provider carrier

This crates.io package carries only target-specific native-provider artifacts
for `x86_64-pc-windows-gnu`; it exposes no numerical Rust API. Depend on
`slatec` (or, for an expert provider selection, `slatec-src`), not on this
carrier directly.

The carrier packages one deterministic accepted-source archive. Individual
families are available only when their entire hash-pinned closure is present;
ordinary static linking still extracts only referenced native objects. The
current receipt identifies the available families, checksums, compiler recipe,
source-unit manifest, symbol/runtime audits, SPDX SBOM, and notices.

Reduced static `libgfortran` and `libquadmath` closures are included only when
their required members are recorded. Exact GNU licence texts, source locations,
and replacement/relink instructions live in
`metadata/runtime-licenses/`. The package contains no source cache, object
intermediates, executables, or compiler logs.
