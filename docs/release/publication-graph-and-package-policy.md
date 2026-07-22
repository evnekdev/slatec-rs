# Publication graph and package policy

## Coordinated version

The initial candidate uses the shared workspace version `0.1.0`. Publishing and
tagging are separate release operations and are not performed by readiness
work.

## Mechanically derived dependency order

The package audit reads workspace members and publishable dependency tables,
resolves inherited workspace dependencies, and requires both `version` and
`path` for every publishable workspace edge.

```text
layer 0: slatec-bundled-x86_64-pc-windows-gnu    slatec-bundled-x86_64-unknown-linux-gnu    slatec-sys
                         |                         |
layer 1:                    slatec-src        slatec-core
                              \                 /
layer 2:                         slatec
```

The target carrier and `slatec-sys` are independent and may be published in
either order. `slatec-src` depends on the carrier for its supported target;
`slatec-core` depends on `slatec-sys`; the safe `slatec` facade depends on all
three lower layers. `slatec-tools` is workspace-only and has `publish = false`.

## Native ownership

`slatec-src` alone owns Cargo `links = "slatec"`, because it selects and links
source, system, or external native providers. `slatec-sys` is declarations
only, has no build script, and reserves no native link namespace. This permits
direct raw consumers to provide compatible symbols without hidden native build
behavior while Cargo still prevents two `slatec-src` provider instances from
silently owning the same native library identity.

## Package content boundary

Each publishable crate uses an explicit `include` policy. The package validator
runs `cargo package --allow-dirty --list --offline` and requires Cargo metadata,
README, and both workspace license files. It rejects target directories,
evidence and source caches, downloaded Fortran corpus bytes, objects, DLLs and
executables, link maps, logs, credentials, and local Cargo configuration. The
target carrier is the sole exception for its generated manifest-verified static
archives; it must also package exact GNU runtime licence texts and
source/relinking instructions.

`slatec-src` packages deterministic provider metadata and reviewed machine
constant overrides. It does not package the separately acquired SLATEC source
cache. The canonical `bundled` mode uses the published target carrier only for
exactly approved family closures and never falls back to another provider.
These are technical redistribution boundaries, not new legal conclusions.

Generated evidence is committed at:

- [`workspace-publication-graph.json`](../../generated/release-readiness/workspace-publication-graph.json)
- [`package-content-audit.json`](../../generated/release-readiness/package-content-audit.json)
- [`package-dry-run-audit.json`](../../generated/release-readiness/package-dry-run-audit.json)
- [`public-api-freeze-baseline.json`](../../generated/release-readiness/public-api-freeze-baseline.json) — the safe `slatec` callable facade, derived from the function index rather than raw declarations
- [`raw-public-abi-freeze-baseline.json`](../../generated/release-readiness/raw-public-abi-freeze-baseline.json) — the separate canonical `slatec-sys` ABI surface
- [`core-support-types-api-baseline.json`](../../generated/release-readiness/core-support-types-api-baseline.json) — provider-neutral `slatec-core` support exports
- [`carrier-metadata-api-baseline.json`](../../generated/release-readiness/carrier-metadata-api-baseline.json) — target carrier package metadata and receipts
- [`scalar-api-disposition.json`](../../generated/release-readiness/scalar-api-disposition.json)
- [`scalar-accuracy-evidence.json`](../../generated/release-readiness/scalar-accuracy-evidence.json)
- [`docs-feature-visibility.json`](../../generated/release-readiness/docs-feature-visibility.json)
- [`target-support.json`](../../generated/release-readiness/target-support.json)

The release checklist records package verification, dry-run dependency
blockers, downstream simulation, crates.io ownership, publication order, and
yank/rollback preparation.

The four API baselines are deliberately not interchangeable: a safe wrapper
path is not evidence that its raw declaration is stable, and an unsafe ABI path
is not evidence of a safe facade. Each baseline records its target and feature
profile so future target-carrier additions cannot silently change the frozen
surface.
