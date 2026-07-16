# Dependency-informed Rust domain boundaries

## Status

This page contains project proposals. It does not describe historical SLATEC crate structure.

## Boundary objectives

A future split should satisfy four different constraints:

1. preserve exact routine and package provenance;
2. avoid duplicate native definitions;
3. expose coherent safe APIs by mathematical domain;
4. permit incremental compilation and optional features without incomplete native closures.

These goals do not imply the same boundary.

## Three layers of ownership

### Source ownership

Every physical Fortran source file has exactly one selected owner in a build configuration. If one file contains several program units or `ENTRY` points, they travel together unless the source is transformed through a separately reviewed process.

### Native implementation ownership

Every exported native symbol has exactly one implementation provider:

- bundled historical source;
- compatibility shim;
- external BLAS or other system provider;
- generated wrapper.

The provider is selected explicitly, never by archive order.

### Rust API ownership

A safe Rust crate may expose wrappers for routines implemented in another shared native component. Public API ownership therefore need not match source-file ownership.

## One-routine/one-owning-crate rule

**Project proposal:** each canonical routine ID has exactly one owning raw crate or shared native component in a resolved feature graph. Other crates may depend on and re-export it, but must not compile another copy.

The rule applies to implementation ownership, not documentation cross-listing. A routine can belong to multiple GAMS classes and be relevant to several safe domains while retaining one native owner.

## Candidate boundary inputs

A crate-boundary decision must consider:

- package provenance;
- source-file co-location;
- direct/transitive dependencies;
- SCCs;
- duplicate symbols;
- common blocks and mutable saved state;
- block-data initialization;
- callback infrastructure;
- licensing/notice boundaries;
- external-provider policy;
- expected safe API cohesion.

GAMS and domain classifications are useful for user-facing grouping but insufficient for raw ownership.

## Proposed raw architecture

### Preferred baseline: one native core, several raw Rust modules/crates

For the first functional release, the lowest-risk design is likely:

```text
slatec-native-sys
  builds one explicitly selected source set
  exports generated raw declarations by module/domain

slatec-blas-sys       -> depends/re-exports selected symbols from slatec-native-sys
slatec-quadpack-sys   -> depends/re-exports selected symbols from slatec-native-sys
slatec-slap-sys       -> depends/re-exports selected symbols from slatec-native-sys
...
```

This prevents duplicate support symbols and cross-archive cycles while preserving logical raw namespaces. Whether Cargo permits the exact packaging ergonomics desired must be prototyped.

### Later option: multiple native components

Split native archives only after the dependency graph demonstrates clean closures. Candidate components may include:

- runtime/error/machine support;
- BLAS kernels;
- LINPACK;
- EISPACK;
- QUADPACK;
- PCHIP;
- FFTPACK;
- FISHPACK;
- SLAP;
- FNLIB and other special-function families;
- MINPACK-derived nonlinear routines;
- DASSL/ODE/BVP families.

SCCs and shared symbols can force several candidates into one native component even if safe crates remain separate.

## Safe crate proposals

Safe crates should follow user workflows:

- `slatec-linalg`
- `slatec-sparse`
- `slatec-roots`
- `slatec-nonlinear`
- `slatec-optimization`
- `slatec-quadrature`
- `slatec-diffeq`
- `slatec-interpolation`
- `slatec-transforms`
- `slatec-special`
- optional `slatec-statistics`
- facade `slatec`

A safe crate can depend on several raw owners. For example, a DAE wrapper may use a DASSL raw owner plus runtime support and linear algebra.

## Ownership algorithm

1. Select one implementation for every duplicate symbol group.
2. Collapse call-graph SCCs into atomic native units.
3. Merge units sharing inseparable source files or alternate entries.
4. Merge units sharing incompatible or jointly initialized common blocks.
5. Add required block-data units.
6. Apply package/provenance preference where it does not violate closure.
7. Assign shared support to a single component.
8. Calculate component dependency DAG.
9. Map each component to one raw owner.
10. Map safe APIs independently to mathematical domains.

## Boundary conflict examples

### QUADPACK and error support

`DQAG` belongs functionally and historically to quadrature, but calls `XERROR`, and selected local rules call `D1MACH`. Compiling private copies of those support routines into a QUADPACK crate and other domain crates risks duplicate symbols and divergent error state. Shared native ownership is preferable.

### FISHPACK and FFTPACK

FISHPACK belongs to PDE solvers but can depend on FFTPACK. The shared historical Netlib directory does not imply one safe crate; the dependency direction should instead be represented explicitly.

### MINPACK-derived routines

Nonlinear systems and nonlinear least squares share QR, norm and trust-region support. Provenance may favor one raw MINPACK-derived component, while safe APIs split between nonlinear equations and optimization.

### Alternate entries

Two entry-point symbols from one Fortran program unit cannot be assigned to separate native owners without source transformation.

## Feature policy

Cargo features selecting native implementations must be additive only when their source sets are disjoint. Mutually exclusive providers such as bundled versus system BLAS should use explicit conflict checks.

Suggested feature concepts:

```text
bundled-blas
system-blas
quadpack
slap
pchip
fftpack
fishpack
special-functions
full
```

The final names are provisional. A feature must include a complete validated native closure, not merely root source files.

## Generated ownership products

The dependency pipeline should generate:

- routine-to-source map;
- source-to-native-component map;
- symbol-provider map;
- native-component dependency DAG;
- raw-crate ownership map;
- safe-crate wrapper map;
- duplicate/conflict report;
- feature-to-source manifest.

## Acceptance criteria for a split

A proposed raw split is acceptable only if:

- every routine and source has one owner;
- all undefined symbols are classified;
- every component links in isolation with declared dependencies;
- duplicate symbols are resolved explicitly;
- block data and common storage are validated;
- representative quick checks pass;
- enabling multiple raw crates does not compile duplicate native implementations;
- safe crates can use the components without cyclic Cargo dependencies.

## Recommendation

Start with one native build owner and logical domain modules. Use Codex High to derive and validate smaller components. Split native ownership only when measured build/link benefits outweigh the complexity of shared support, provider selection and cross-package cycles.

## Open questions

- Can one `links`-owning Cargo package expose all native symbols cleanly to several raw crates?
- Should raw subcrates be actual crates or modules/features of one `slatec-sys` crate?
- Which external BLAS policies preserve historical reproducibility?
- How large are the SCCs after full extraction?
- Which domains share mutable global error state?
