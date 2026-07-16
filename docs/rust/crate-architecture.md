# Proposed Rust crate architecture

## Status

This document is a **proposal for `slatec-rs`**, not a description of historical SLATEC. It synthesizes the collected source, taxonomy, domain and dependency evidence.

## Architectural drivers

The design must satisfy constraints that do not naturally share one boundary:

1. preserve historical routine and package provenance;
2. ensure one selected implementation for every native symbol;
3. include complete native dependency closures;
4. avoid Cargo dependency cycles;
5. expose safe APIs by mathematical workflow;
6. isolate callback and mutable-global-state risks;
7. support an incremental path from a buildable bootstrap to smaller crates.

The dependency specification concludes that GAMS/domain classifications alone are insufficient for native ownership. Source-file co-location, direct/transitive dependencies, strongly connected components, duplicate symbols, common blocks, `BLOCK DATA`, callback infrastructure and provider selection all affect the linkable boundary ([dependency extraction specification](../architecture/dependency-extraction-spec.md); [dependency-informed boundaries](domain-boundaries.md)).

## Preferred architecture: staged hybrid ownership

### Stage A — bootstrap monolith

```text
slatec-sys
├── build.rs
├── generated source manifest
├── one conflict-resolved native library
├── raw declarations
└── logical Rust modules
    ├── runtime
    ├── blas
    ├── linpack
    ├── eispack
    ├── quadpack
    ├── pchip
    ├── fftpack
    ├── slap
    ├── nonlinear
    ├── diffeq
    └── special
```

`slatec-sys` initially owns:

- the Cargo `links` identity;
- source selection and patch manifests;
- the Fortran compiler invocation;
- native symbols and runtime-library linkage;
- generated raw declarations;
- build fingerprints and validation metadata.

The native side is initially one archive or shared library built from an explicitly selected, duplicate-free source set. Logical modules do **not** imply separate native copies.

This follows the existing recommendation to begin with one native owner because QUADPACK, SLAP, differential-equation families and other packages share machine constants, error infrastructure and low-level numerical routines. A premature package-per-crate split could compile duplicate `XERROR`/`XERMSG`, `D1MACH` or BLAS symbols and could fragment mutable error state ([domain boundaries](domain-boundaries.md#boundary-conflict-examples); [native build strategy](native-build-strategy.md#native-component-strategy)).

### Stage B — internal component manifests

Before publishing additional `-sys` crates, split the monolithic source selection internally:

```text
components/
├── runtime.toml
├── blas.toml
├── linpack.toml
├── eispack.toml
├── quadpack.toml
├── pchip.toml
├── fftpack.toml
├── fishpack.toml
├── slap.toml
├── minpack-derived.toml
├── diffeq.toml
└── special.toml
```

Each component manifest records:

- owning source files;
- exported and internal symbols;
- required components;
- selected providers;
- common blocks and block-data units;
- feature membership;
- checksums and patches;
- graph/SCC identifiers;
- validation results.

The build may still emit one native library. This stage validates conceptual boundaries without imposing public Cargo compatibility commitments.

### Stage C — separate native archives under one owner

Once dependency extraction proves a component DAG, `slatec-sys` may compile multiple archives while retaining one `links` owner:

```text
libslatec_runtime.a
libslatec_blas.a
libslatec_quadpack.a
libslatec_pchip.a
...
```

This yields incremental compilation and feature-based source reduction while keeping provider selection and link order centralized. Components containing call-graph cycles, shared common storage or inseparable source files remain merged.

### Stage D — domain/package-specific raw crates

Only after practical Cargo prototypes and isolated-link validation should public raw crates appear. Two forms are possible:

#### Re-export crates

```text
slatec-quadpack-sys -> slatec-sys
slatec-slap-sys     -> slatec-sys
slatec-pchip-sys    -> slatec-sys
```

These provide focused namespaces but do not compile native code. `slatec-sys` remains the sole native owner.

#### Independent native owners

```text
slatec-runtime-sys
slatec-blas-sys
slatec-quadpack-sys
...
```

This is allowed only where:

- each source and symbol has one owner;
- components form an acyclic Cargo/native dependency graph;
- each component links and tests independently;
- enabling multiple crates does not select duplicate implementations;
- common state is intentionally shared or proven independent.

The re-export form is the safer default.

## Safe wrapper layer

Safe crates should follow user problems rather than historical source packages:

```text
slatec-linalg
slatec-sparse
slatec-roots
slatec-nonlinear
slatec-optimization
slatec-quadrature
slatec-diffeq
slatec-interpolation
slatec-transforms
slatec-special
slatec-statistics        # optional, only if coherent
```

Examples of mismatched raw and safe boundaries:

- MINPACK-derived routines may share one raw component, while nonlinear systems and nonlinear least squares belong to different safe crates.
- FISHPACK belongs to PDE APIs but may depend on FFTPACK.
- spline quadrature routines may be exposed through interpolation even when classified partly under integration.
- probability functions may remain implemented in special-function components but be re-exported by a statistics facade.

Safe wrappers should depend downward on raw owners and shared internal support; safe crates should not depend on one another merely to reuse private FFI machinery. Reusable Rust-only infrastructure belongs in non-domain internal crates.

## Facade layer

A facade crate named `slatec` may re-export safe crates:

```text
slatec
├── linalg
├── sparse
├── roots
├── nonlinear
├── optimization
├── quadrature
├── diffeq
├── interpolation
├── transforms
└── special
```

Recommended policies:

- default features expose a deliberately small stable subset;
- `full` enables all safe domains but not every native provider option;
- expert/raw APIs are not re-exported by default;
- safe domain crates remain independently usable;
- the facade version can advance independently while respecting compatible lower-layer ranges.

## One-owner rule

For a resolved build configuration:

1. every physical source file has one selected source owner;
2. every exported implementation symbol has one provider;
3. every canonical routine ID has one raw/native owner;
4. every common block and block-data initializer has one compatible definition;
5. all re-exports point to that owner rather than compiling another copy.

The provider can be bundled source, a compatibility shim, generated wrapper or an external library. Selection is explicit and emitted in build metadata; archive order is not a provider-selection mechanism.

## Shared support routines

Machine constants, error handling, low-level sorting, callback shims and compiler-runtime glue are cross-cutting. Three options exist:

1. keep them in the monolithic native owner;
2. place them in one `slatec-runtime-sys` native component;
3. replace selected support routines with reviewed shims.

The preferred initial choice is option 1. A separate runtime component becomes appropriate only when its symbol closure, global state and initialization behavior are verified.

No domain crate should privately compile its own copy of shared error or machine support.

## BLAS and LAPACK strategy

### Bundled historical BLAS

Purpose:

- preserve reproducibility;
- match historical routine dependencies;
- provide a self-contained baseline.

### System/tuned BLAS

Purpose:

- improve performance;
- reduce duplicate low-level implementations in applications already using BLAS.

Requirements:

- bundled conflicting symbols are omitted;
- required BLAS level/routine inventory is validated;
- integer, complex and character ABI assumptions are tested;
- provider identity is recorded;
- documentation states that numerical behavior may differ.

### LAPACK

LAPACK should not initially replace LINPACK/EISPACK routines transparently. It is a modern alternative, not the historical implementation. Safe APIs may later offer explicitly named backends where semantics can be aligned, but a wrapper called “SLATEC DGEFA” should not silently execute LAPACK.

## Feature flags versus separate crates

Use a feature when it selects an optional, complete component within one native ownership system:

```text
quadpack
pchip
fftpack
fishpack
slap
special-functions
full
```

Use a separate safe crate when the public API, release cadence or dependency weight is independently meaningful.

Avoid using additive Cargo features to choose mutually exclusive native providers. Cargo features are unified across a dependency graph, so `bundled-blas` and `system-blas` can accidentally become enabled together. Provider selection should use one of:

- separate provider crates;
- explicit build configuration outside ordinary additive features;
- conflict detection that fails immediately when incompatible features unify.

Cargo’s feature documentation explains that features are unified for a package in a dependency graph; provider choices must therefore not rely on a “last one wins” model ([Cargo features](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification)).

## Avoiding cyclic dependencies

Native cycles and Cargo cycles are different:

- collapse native routine SCCs into one component;
- permit archive/component dependencies only along the resulting DAG;
- keep safe crates dependent on raw components, never the reverse;
- put shared Rust callback/result/storage types in lower internal crates;
- use facade re-exports rather than safe-crate cross-dependencies where possible;
- do not split two `ENTRY` symbols from one source program unit without reviewed source transformation.

## Metadata-driven source selection

The build resolver should consume generated metadata, not scan directories heuristically.

Inputs:

- routine catalogue;
- source-to-symbol map;
- dependency graph;
- SCC list;
- package/domain metadata;
- duplicate-symbol groups;
- provider policies;
- feature-to-root mapping;
- common/block-data records.

Outputs:

- selected source manifest;
- component DAG;
- expected undefined-symbol inventory;
- compiler/link instructions;
- generated raw declarations;
- build fingerprint.

Ordinary `cargo build` should not download mutable upstream files or regenerate the entire graph.

## Alternative architecture 1: permanent monolith

```text
slatec-sys -> one native library forever
slatec     -> one safe crate
```

### Advantages

- simplest duplicate-symbol and global-state model;
- easiest initial linking;
- smallest public package graph;
- lowest migration cost.

### Disadvantages

- large builds and binary/link surface;
- all domains share one publication cadence;
- difficult optionality;
- safe API becomes too broad;
- provenance and domain boundaries remain documentation-only.

This is an acceptable bootstrap but not the preferred permanent architecture.

## Alternative architecture 2: immediate package-per-crate split

```text
slatec-blas-sys
slatec-linpack-sys
slatec-quadpack-sys
slatec-pchip-sys
...
```

Each crate independently compiles its package and support closure.

### Advantages

- intuitive provenance;
- small focused packages;
- familiar `*-sys` ecosystem pattern;
- independent publication.

### Disadvantages

- duplicated shared symbols and error state;
- cycles and incomplete closures;
- source files or symbols may belong to multiple apparent packages;
- external BLAS provider conflicts;
- Cargo cannot tolerate cyclic package dependencies;
- one application may compile incompatible historical variants.

This is rejected as the initial architecture. It becomes viable only for graph-validated independent components.

## Why the staged hybrid is preferred

The staged hybrid gives the project a buildable first milestone without freezing uncertain package boundaries. It preserves one native owner while collecting the data needed to split safely. Logical modules and safe crates can mature before native archives are physically separated.

It also permits retreat: a component that proves inseparable can remain inside the shared native owner without disrupting a domain-safe API.

## Publication order

Recommended order:

1. metadata/schema crates or repository tools, if published;
2. `slatec-sys` bootstrap;
3. one small safe pilot crate, preferably a low-callback family such as PCHIP or selected special functions;
4. `slatec-quadrature` or `slatec-roots` after callback infrastructure;
5. `slatec-linalg` and `slatec-sparse` after storage/provider validation;
6. `slatec-diffeq` after stateful-session and callback validation;
7. facade `slatec`;
8. focused raw re-export crates;
9. independent native-owner crates only after link-graph proof.

## Sources

- [Dependency-informed Rust domain boundaries](domain-boundaries.md)
- [Native build and link strategy](native-build-strategy.md)
- [Dependency model](../architecture/dependency-model.md)
- [Dependency extraction specification](../architecture/dependency-extraction-spec.md)
- [Domain overview](../domains/overview.md)
- [Package provenance overview](../packages/overview.md)
- [Routine metadata schema](../../metadata/routine-schema.md)
- [Cargo build scripts and `links`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key)
- [Cargo feature unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification)
