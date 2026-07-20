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
layer 0: slatec-src    slatec-sys
                         |
layer 1:             slatec-core
                      /   |   \
layer 2:                 slatec
```

`slatec-src` and `slatec-sys` are independent and may be published in either
order. `slatec-core` depends on `slatec-sys`. The safe `slatec` facade depends
on all three lower crates. `slatec-tools` is workspace-only and has
`publish = false`.

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
evidence and source caches, downloaded Fortran corpus bytes, native objects and
archives, DLLs and executables, link maps, logs, credentials, and local Cargo
configuration.

`slatec-src` packages deterministic provider metadata and reviewed machine
constant overrides. It does not package the separately acquired SLATEC source
cache. The unavailable prebuilt mode remains blocked pending rights review.
These are technical redistribution boundaries, not new legal conclusions.

Generated evidence is committed at:

- [`workspace-publication-graph.json`](../../generated/release-readiness/workspace-publication-graph.json)
- [`package-content-audit.json`](../../generated/release-readiness/package-content-audit.json)

The release checklist records package verification, dry-run dependency
blockers, downstream simulation, crates.io ownership, publication order, and
yank/rollback preparation.
