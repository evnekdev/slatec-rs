# Incremental migration roadmap

## Status

This roadmap proposes a sequence from documentation-only research to a publishable Rust ecosystem. Passing one phase does not imply that all SLATEC routines are ready.

## Phase 0 — evidence baseline

### Inputs

- authoritative source register;
- history and architecture references;
- domain/package taxonomy;
- routine metadata schema and pilot;
- dependency extraction specification.

### Exit criteria

- source snapshot and rights are pinned;
- documentation distinguishes verified, inferred and unresolved facts;
- required metadata schemas parse and are versioned;
- no build or API claim exceeds the evidence.

## Phase 1 — reproducible source corpus

### Work

- acquire the complete official source distribution and relocated subsets;
- record checksums and provenance;
- reconcile duplicate files and versions;
- identify machine-specific files;
- create minimal reproducible patch series;
- inventory notices and redistribution obligations.

### Exit criteria

- every selected source has one canonical repository path;
- original and patched checksums are recorded;
- duplicate candidates are unresolved explicitly or one provider is selected;
- ordinary Cargo builds require no network access.

## Phase 2 — complete metadata extraction

Use Codex High and deterministic scripts to:

- parse all program units and prologue dialects;
- populate routine metadata;
- extract calls, external references, common blocks, `ENTRY`, `SAVE` and `BLOCK DATA`;
- ingest Netlib dependency products;
- generate direct/transitive graphs, SCCs and unresolved-reference reports;
- compare packages against upstream archives.

### Exit criteria

- every routine/source/symbol has a stable ID;
- all parser outputs validate against schemas;
- manual exceptions are recorded rather than hidden;
- generated files are reproducible.

## Phase 3 — monolithic native prototype

Create private/unpublished `slatec-sys`:

- one conflict-resolved source set;
- one native archive and optionally one shared-library comparison;
- one Cargo `links` owner;
- compiler detection;
- generated symbol declarations;
- minimal ABI probes;
- selected official quick checks.

### Initial supported scope

Prefer GNU Fortran on one primary platform before expanding. Build success alone is insufficient.

### Exit criteria

- no unexpected duplicate selected symbols;
- undefined symbols are classified;
- minimal executables link without accidental host libraries;
- machine constants and error paths are verified;
- build fingerprint is emitted.

## Phase 4 — logical raw modules

Organize declarations and source manifests into internal modules:

```text
runtime
blas
linpack
eispack
quadpack
pchip
fftpack
slap
nonlinear
diffeq
special
```

No additional native copies are introduced.

### Exit criteria

- module ownership matches generated metadata;
- every raw declaration points to one selected symbol;
- feature roots include complete validated closures;
- docs identify unsupported ABI classes.

## Phase 5 — first safe pilot

Recommended first pilot: PCHIP or a small value-only special-function subset.

Reasons:

- no application callbacks for basic operations;
- clear construction/evaluation workflow;
- manageable workspace and status handling;
- useful standalone API;
- tests can compare known interpolation/function values.

### Exit criteria

- all public functions are safe under documented preconditions;
- raw status and method identity are preserved;
- no unchecked length or integer conversions;
- cross-platform CI passes for the declared support matrix;
- documentation distinguishes historical and Rust behavior.

## Phase 6 — callback infrastructure

Implement and validate:

- generated per-signature trampolines;
- scoped callback contexts;
- panic containment;
- callback error/cancellation transport;
- global error capture/serialization;
- nested and concurrent tests.

Pilot with one bracketed root or QUADPACK routine before stateful nonlinear/DAE solvers.

## Phase 7 — domain-safe crates

Suggested sequence:

1. `slatec-interpolation`;
2. `slatec-special` subset;
3. `slatec-roots`;
4. `slatec-quadrature`;
5. `slatec-transforms`;
6. `slatec-nonlinear`;
7. `slatec-optimization`;
8. `slatec-linalg`;
9. `slatec-sparse`;
10. `slatec-diffeq`.

The ordering is risk-based, not a judgment of importance.

## Phase 8 — internal native components

Generate component manifests and compile several archives under the same `slatec-sys` owner.

### Exit criteria

- SCCs remain atomic;
- archive order follows a component DAG;
- each component closure links in isolation with declared dependencies;
- `BLOCK DATA` and common storage are retained correctly;
- enabled feature sets select no duplicate provider.

## Phase 9 — BLAS provider options

Add bundled historical BLAS first. Prototype a system BLAS provider separately.

### Exit criteria

- provider conflict is impossible or rejected at build time;
- required symbols and integer ABI are probed;
- numerical differences are documented;
- build metadata reports provider identity;
- applications using other BLAS consumers are tested for duplicate conflicts.

Do not introduce implicit LAPACK substitution.

## Phase 10 — focused raw crates

Begin with namespace/re-export crates depending on `slatec-sys`. Independent native ownership is deferred until component validation demonstrates value.

Potential crates:

```text
slatec-quadpack-sys
slatec-pchip-sys
slatec-fftpack-sys
slatec-slap-sys
```

### Exit criteria

- no crate compiles duplicate native symbols;
- documentation clearly identifies the actual `links` owner;
- semver and feature interactions are tested in multi-crate applications.

## Phase 11 — facade and ecosystem release

Publish `slatec` after several safe crates have stable result/error conventions.

Facade policy:

- small defaults;
- explicit domain features;
- no raw APIs by default;
- consistent documentation and examples;
- version compatibility table.

## Versioning strategy

### Native/raw layer

Breaking changes include:

- symbol/provider changes;
- ABI type changes;
- source snapshot changes that alter behavior;
- feature/source selection changes;
- removal or renaming of declarations.

A source snapshot update should produce a release note with routine-level impact where known.

### Safe layer

Use ordinary semver for public Rust APIs. Safe wrappers may update internal raw dependencies without a major version only when behavior and documented compatibility remain within contract.

### Metadata schemas

Version schemas independently. Generators record the schema version and reject unsupported major versions.

## Publication ordering

Acyclic order:

```text
metadata/support utilities
    -> slatec-sys
        -> safe domain crates
            -> slatec facade
```

Focused raw re-export crates either follow `slatec-sys` or remain workspace-only until needed. Independent native raw crates cannot be published until their lower native dependencies already exist.

## CI progression

1. one compiler/OS;
2. Linux and Windows GNU Fortran;
3. static and shared experiments;
4. callback matrix;
5. external BLAS;
6. additional compilers;
7. 32-bit targets where practical;
8. concurrency/state tests.

Unsupported combinations are explicit.

## Stop/go checkpoints

Pause a split when:

- an SCC crosses the boundary;
- symbols duplicate;
- a source file contains units assigned to both sides;
- common/block-data ownership is unclear;
- a minimal link needs undeclared host libraries;
- safe API requires inferred argument intent;
- callback termination cannot be made safe.

Keep the routine in the shared owner until the issue is resolved.

## ChatGPT versus Codex

### ChatGPT

Best suited to:

- architecture review;
- interpreting manuals and individual prologues;
- evaluating API tradeoffs;
- reviewing generated discrepancy reports;
- drafting user documentation;
- analyzing platform failures after logs are available.

### Codex High

Required for:

- full source parsing;
- metadata generation;
- source normalization/diffs;
- graph and SCC computation;
- build-script implementation;
- compiler/symbol/link probing;
- generated wrappers and declarations;
- matrix CI and regression tests;
- bulk routine-page generation.

## Milestone definition of done

The ecosystem is not “complete” when every symbol has a declaration. Completion requires:

- verified source ownership;
- reproducible native builds;
- ABI validation;
- complete dependencies;
- safe wrappers for advertised domains;
- status/error fidelity;
- callback/global-state safety;
- tests and limitations documented.

## Sources

- [Crate architecture](crate-architecture.md)
- [FFI design](ffi-design.md)
- [Callback safety](callback-safety.md)
- [Native build strategy](native-build-strategy.md)
- [Dependency extraction specification](../architecture/dependency-extraction-spec.md)
- [Routine catalogue overview](../routines/README.md)
