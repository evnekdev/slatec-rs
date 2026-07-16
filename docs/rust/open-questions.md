# Open architectural questions

## Purpose

This register collects decisions that must remain open until source extraction, build prototypes or runtime experiments provide evidence. It should be updated with links to issues, experiments and accepted decisions.

## Native ownership

### N1 — permanent native owner

**Question:** Should one `slatec-sys` package permanently own all native libraries, or should validated components eventually receive independent `links` owners?

**Current recommendation:** one owner through the bootstrap and component-validation stages.

**Evidence required:** Cargo prototype, multi-crate link tests, component DAG and publication ergonomics.

### N2 — archive versus shared library

**Question:** Should the default bundled implementation be one static archive, several archives or one shared library?

**Evidence required:** binary size, link reliability, deployment complexity, global-state behavior and platform support.

### N3 — component granularity

**Question:** How large are the source-level SCCs and common-state groups?

**Evidence required:** complete dependency extraction, object symbols and common-block inventory.

### N4 — source transformation

**Question:** May files containing several program units or `ENTRY` points be mechanically split?

**Current recommendation:** no, unless a reviewed transformation is necessary and numerical equivalence is tested.

## Providers

### P1 — BLAS default

**Question:** Should the default be bundled historical BLAS or a tuned system BLAS?

**Tradeoff:** reproducibility versus performance and application interoperability.

**Evidence required:** benchmarks, symbol-conflict tests, ABI probes and numerical comparison.

### P2 — BLAS provider configuration

**Question:** Should mutually exclusive providers use separate crates, environment/config files or conflict-checked features?

**Constraint:** Cargo feature unification makes provider selection through ordinary additive features hazardous.

### P3 — LAPACK integration

**Question:** Should safe linear-algebra APIs offer optional LAPACK-backed implementations?

**Current recommendation:** only as explicitly named alternative backends; never silently replace historical LINPACK/EISPACK routines.

### P4 — external FFT or special-function providers

**Question:** Is provider abstraction useful outside BLAS?

**Current recommendation:** defer until routine semantics and demand justify it.

## ABI and build

### A1 — compiler dependency

**Question:** Is a Fortran compiler an acceptable build-time requirement?

**Alternatives:** source build, prebuilt native artifacts, system SLATEC, or hybrid.

### A2 — supported compilers

**Question:** Which compiler/version matrix is promised?

**Evidence required:** ABI probes for symbol mangling, integer/logical, complex, character lengths, callbacks, common blocks and runtime linkage.

### A3 — stable wrappers

**Question:** Should all routines receive generated `BIND(C)`/C wrappers, or only ABI-sensitive interfaces?

**Tradeoff:** stable ABI and easier binding versus build complexity and another generated layer.

### A4 — 32-bit targets

**Question:** Are 32-bit Windows/Linux targets in the initial support matrix?

**Evidence required:** compiler availability, integer/address constraints, test capacity and user demand.

### A5 — prebuilt artifacts

**Question:** Should common platforms receive precompiled native libraries?

**Evidence required:** redistribution rights, CI security, compiler-runtime packaging and reproducibility policy.

## Callbacks and state

### C1 — callback context mechanism

**Question:** Can generated wrappers add explicit context pointers, or is a thread-local stack required?

**Evidence required:** prototypes for QUADPACK, MINPACK-derived and SLAP signatures.

### C2 — callback termination

**Question:** How should Rust callback errors stop routines lacking a documented cancellation flag?

**Possible outcomes:** conservative placeholder return, error flag shim, unsupported safe wrapper.

### C3 — nested callbacks

**Question:** Can a callback safely invoke another SLATEC routine?

**Evidence required:** nested-call tests and proof that callback pointers are not retained.

### C4 — concurrent calls

**Question:** Which routines are thread-safe?

**Evidence required:** common/`SAVE` inventory, error-subsystem behavior, callback-thread behavior and stress tests.

### C5 — global error handling

**Question:** Can `XERROR`/`XERMSG` state be isolated, captured or made thread-local?

**Fallback:** serialize native calls through the narrowest possible lock.

### C6 — fatal paths

**Question:** Can every `STOP`, `XERHLT` or site hook be intercepted so a Rust process is not terminated unexpectedly?

## Safe API shape

### S1 — domain crate count

**Question:** Should roots and nonlinear systems be separate crates? Should ODE, DAE and BVP share `slatec-diffeq`?

**Evidence required:** shared public types, dependency weight and release cadence.

### S2 — statistics crate

**Question:** Is there enough coherent SLATEC statistics functionality for a separate safe crate?

**Current concern:** many routines are better owned by special functions, interpolation or optimization.

### S3 — workspace visibility

**Question:** Should expert APIs expose reusable workspaces for allocation control?

**Recommendation:** hide by default; offer typed expert workspace objects only where repeated-call benefit is substantial.

### S4 — raw status preservation

**Question:** Should every safe result expose the original integer status?

**Recommendation:** yes, at least through a stable method/field, while also providing typed interpretation.

### S5 — matrix ecosystem

**Question:** Which Rust matrix/sparse traits should wrappers accept?

**Alternatives:** crate-local views, `nalgebra`, `ndarray`, `faer`, generic traits, or conversion adapters.

**Constraint:** core wrappers should avoid forcing a heavy ecosystem dependency without evidence.

### S6 — generic precision APIs

**Question:** Should S/D/C families share generic Rust traits?

**Recommendation:** only where routine coverage and semantics align; irregular historical families should remain explicit.

## Metadata and generation

### M1 — authoritative source snapshot

**Question:** Which exact complete source artifact and relocated subsets define the project baseline?

### M2 — Netlib dependency products

**Question:** What snapshots and formats generated `tree1`, `tree` and plus-dependency outputs?

### M3 — generated files in Git

**Question:** Which generated declarations, metadata and documentation should be committed?

**Recommendation:** commit deterministic user-facing artifacts and schemas; keep transient object/symbol logs as CI artifacts unless needed for reproducibility.

### M4 — inferred intent

**Question:** What evidence threshold permits a safe wrapper when prologue intent is missing?

**Recommendation:** source review plus tests; parser inference alone is insufficient.

### M5 — package revisions

**Question:** Which BLAS, LINPACK, EISPACK, QUADPACK, FNLIB, FFTPACK, SLAP, MINPACK-derived and DASSL revisions are represented?

## Versioning and release

### V1 — source snapshot versioning

**Question:** How should a change in bundled SLATEC source snapshot affect crate semver?

**Proposal:** routine-semantic or symbol changes may require a major raw release; patch-only source corrections require documented compatibility review.

### V2 — synchronized versions

**Question:** Should workspace crates share one version?

**Tradeoff:** simple compatibility story versus unnecessary synchronized releases.

**Preferred direction:** independent safe-crate versions with a tested compatibility matrix; raw/native owner updates remain deliberately coordinated.

### V3 — MSRV

**Question:** What minimum supported Rust version should be promised?

**Evidence required:** build dependency requirements, edition choice and maintenance resources.

### V4 — experimental API labeling

**Question:** How long should callback-heavy and stateful wrappers remain explicitly experimental?

## Decision-record template

When resolving an item, record:

```text
Decision ID:
Date:
Status:
Chosen option:
Evidence:
Rejected alternatives:
Compatibility impact:
Required follow-up:
```

Do not delete the original question; move it to a resolved section with the decision record.

## Priority order

Highest-priority blockers before publishing `slatec-sys`:

1. source snapshot and redistribution;
2. duplicate/provider selection;
3. compiler ABI baseline;
4. error and machine support;
5. deterministic build manifest.

Highest-priority blockers before callback-safe crates:

1. panic containment;
2. context and cancellation strategy;
3. global-state classification;
4. nested/concurrent tests.

Highest-priority blockers before native crate splitting:

1. complete graph and SCCs;
2. source/symbol ownership;
3. isolated component links;
4. Cargo/provider prototype;
5. multi-crate duplicate-symbol tests.

## Sources

- [Crate architecture](crate-architecture.md)
- [FFI design](ffi-design.md)
- [Callback safety](callback-safety.md)
- [Migration roadmap](migration-roadmap.md)
- [Dependency-informed boundaries](domain-boundaries.md)
- [Native build strategy](native-build-strategy.md)
- [Dependency schema](../../metadata/dependency-schema.md)
