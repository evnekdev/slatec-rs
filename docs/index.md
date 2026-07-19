# SLATEC knowledge-base index

## Collection status

This repository contains the documentation and metadata foundation for a future `slatec-rs` ecosystem. The collection stage covers the historical library, authoritative sources, architecture, mathematical taxonomy, representative routines, dependency extraction and proposed Rust organization.

The documents deliberately distinguish:

- sourced historical or technical facts;
- modern interpretation;
- proposals for `slatec-rs`;
- unresolved or conflicting evidence.

The collection is ready to advance to reproducible bulk source ingestion and metadata extraction, but it is **not yet evidence that SLATEC has been fully catalogued, built or safely bound to Rust**. See [Collection status](collection-status.md) and [Audit report](audit-report.md).

## Start here

1. [Collection status](collection-status.md) — completed work, readiness and blockers.
2. [Audit report](audit-report.md) — quality review and prioritized issues.
3. [Open research questions](open-research-questions.md) — unresolved historical, technical and Rust questions.
4. [Source register](sources/source-register.md) — authoritative source catalogue.
5. [Domain overview](domains/overview.md) — mathematical-domain map.
6. [Routine catalogue](routines/README.md) — pilot metadata and future extraction process.
7. [Rust crate architecture](rust/crate-architecture.md) — proposed staged Rust organization.

## Complete catalogue

[Complete SLATEC Routine Index](reference/slatec-routine-index.md) provides the generated historical inventory and `slatec-rs` coverage map, with [family browsing](reference/routines-by-family.md), [alphabetical lookup](reference/routines-alphabetical.md), and [coverage and reconciliation](reference/routine-coverage.md).

## Sources and evidence policy

- [Source register](sources/source-register.md)
- [Citation policy](sources/citation-policy.md)
- [Terminology](sources/terminology.md)
- [`metadata/sources.toml`](../metadata/sources.toml)

These define source precedence, evidence status and project terminology. Primary SLATEC source and routine prologues take precedence over later summaries.

## History

- [Historical overview](history/overview.md)
- [Chronology](history/chronology.md)
- [Participating institutions](history/participating-institutions.md)
- [Releases and changes](history/releases-and-changes.md)

These pages summarize the cooperative origin of the library and the evidence available for releases and change history. Dates that are not supported by a primary source remain marked for verification.

## Architecture and conventions

- [Library architecture](architecture/library-architecture.md)
- [Source organization](architecture/source-organization.md)
- [Routine prologues](architecture/routine-prologues.md)
- [Naming and precision](architecture/naming-and-precision.md)
- [Error handling](architecture/error-handling.md)
- [Machine constants](architecture/machine-constants.md)
- [Dependency model](architecture/dependency-model.md)
- [Dependency extraction specification](architecture/dependency-extraction-spec.md)

These pages explain fixed-form Fortran organization, public/subsidiary status, precision families, error infrastructure, machine-dependent support, dependency evidence and static-linking consequences.

## Mathematical taxonomy and package provenance

- [GAMS overview](taxonomy/gams-overview.md)
- [SLATEC subpackages](taxonomy/slatec-subpackages.md)
- [GAMS-to-domain map](taxonomy/gams-to-domain-map.md)
- [Package overview](packages/overview.md)
- [Imported packages](packages/imported-packages.md)
- [`metadata/domains.toml`](../metadata/domains.toml)
- [`metadata/packages.toml`](../metadata/packages.toml)

GAMS categories describe mathematical purpose; package provenance describes historical lineage; proposed Rust domains describe future API organization. They are intentionally separate many-to-many classifications.

## Domain surveys

- [Domain overview](domains/overview.md)
- [Dense and structured linear algebra](domains/linear-algebra.md)
- [Sparse methods](domains/sparse-methods.md)
- [Nonlinear equations](domains/nonlinear-equations.md)
- [Optimization and nonlinear least squares](domains/optimization.md)
- [Quadrature](domains/quadrature.md)
- [Differential equations](domains/differential-equations.md)
- [Interpolation and approximation](domains/interpolation.md)
- [Transforms](domains/transforms.md)
- [Special functions](domains/special-functions.md)
- [Probability and statistics](domains/statistics.md)
- [Utilities and runtime support](domains/utilities.md)

The surveys are representative, not exhaustive routine catalogues. Routine membership and dependencies require source-derived validation.

## Routine metadata

- [Routine metadata guide](routines/README.md)
- [Pilot index](routines/index.md)
- [Routine schema](../metadata/routine-schema.md)
- [Schema example](../metadata/routine-schema.example.toml)
- [Pilot catalogue](../metadata/routines-pilot.toml)
- [Pilot pages](routines/pilot/)

The pilot covers twenty representative routines. It tests evidence-aware metadata conventions but does not establish complete SLATEC coverage.

## Dependency metadata and native build planning

- [Dependency schema](../metadata/dependency-schema.md)
- [Dependency example](../metadata/dependency-schema.example.toml)
- [Dependency extraction specification](architecture/dependency-extraction-spec.md)
- [Domain boundaries](rust/domain-boundaries.md)
- [Native build strategy](rust/native-build-strategy.md)

The complete graph must be generated from source, prologues, Netlib dependency resources, object symbols and linker experiments. No single evidence source is sufficient by itself.

## Proposed Rust architecture

All pages in this section are proposals:

- [Crate architecture](rust/crate-architecture.md)
- [FFI design](rust/ffi-design.md)
- [Callback safety](rust/callback-safety.md)
- [Migration roadmap](rust/migration-roadmap.md)
- [Open Rust questions](rust/open-questions.md)
- [Domain boundaries](rust/domain-boundaries.md)
- [Native build strategy](rust/native-build-strategy.md)

The preferred path begins with one native `slatec-sys` owner, adds logical modules and validated components, then introduces domain-safe crates and a facade. Independent native `*-sys` crates are deferred until the dependency graph proves clean ownership.

## Metadata inventory

| File | Purpose |
|---|---|
| `metadata/sources.toml` | authoritative source register |
| `metadata/domains.toml` | proposed stable mathematical-domain IDs |
| `metadata/packages.toml` | package and provenance candidates |
| `metadata/routine-schema.md` | routine-record conventions |
| `metadata/routine-schema.example.toml` | parseable schema example |
| `metadata/routines-pilot.toml` | twenty-routine pilot |
| `metadata/dependency-schema.md` | dependency graph conventions |
| `metadata/dependency-schema.example.toml` | parseable dependency example |
| `metadata/coverage.toml` | collection coverage and audit status |

## Next operational step

Use Codex High and deterministic scripts to pin the complete source snapshot, parse every program unit and prologue, reconcile package copies, generate the routine/dependency catalogues, compile objects, inspect symbols and validate minimal links. The [migration roadmap](rust/migration-roadmap.md) defines the staged acceptance criteria.
