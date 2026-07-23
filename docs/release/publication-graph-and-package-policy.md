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
- [`final-independent-audit.json`](../../generated/release-readiness/final-independent-audit.json)
- [`package-size-audit.json`](../../generated/release-readiness/package-size-audit.json)
- [`release-check-trust-boundaries.json`](../../generated/release-readiness/release-check-trust-boundaries.json)
- [`release-blockers.json`](../../generated/release-readiness/release-blockers.json)
- [`generator-drift-analysis.json`](../../generated/release-readiness/generator-drift-analysis.json) â€” transactional freshness partition, output-to-input ownership, and the resolved semantic-owner regression

The release checklist records package verification, dry-run dependency
blockers, downstream simulation, crates.io ownership, publication order, and
yank/rollback preparation.

Cargo writes the current Git revision to `.cargo_vcs_info.json` in a package.
That receipt remains in every actual `.crate` and the independent audit still
enforces its real compressed-size limit. Committed package evidence instead
records canonical size and hash metrics that omit only this volatile receipt,
so creating the evidence commit cannot itself make the next regeneration
stale. The audit labels that normalization explicitly; all other packaged
files remain part of the canonical metrics.

The four API baselines are deliberately not interchangeable: a safe wrapper
path is not evidence that its raw declaration is stable, and an unsafe ABI path
is not evidence of a safe facade. Each baseline records its target and feature
profile so future target-carrier additions cannot silently change the frozen
surface.

The independent audit is intentionally a separate package-first recomputation:
it recreates a directory registry, repackages every publishable crate in the
release order, decompresses each `.crate`, recomputes hashes and carrier
receipt checks, and compares its counts with the release-candidate report.
Its trust-boundary document labels stored generator output as self-referential
rather than mistaking it for independent proof.

## Generated-evidence ownership and freshness

`public_api_semantic_review` is the sole writer for canonical-public routine
pages and the documentation-quality and argument-coverage mirrors. The
release-readiness generator owns secondary routine pages, family navigation,
and reconciliation evidence only. `validate-release-readiness` recomputes its
own outputs transactionally and restores the working tree before reporting a
failure. Its diagnostic report classifies every difference by owner, category,
risk, line count, EOL/ordering status, schema change, and canonical/safe-path
impact; it never treats a classification as permission to accept the change.

Generate and validate the durable report with:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-release-readiness-drift-report --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-release-readiness-drift-report --offline
```
