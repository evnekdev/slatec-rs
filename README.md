# slatec-rs

Workspace skeleton for Rust bindings and safe wrappers around selected SLATEC numerical routines.

## Intended layers

- `slatec-sys`: raw, unsafe FFI declarations. It never implicitly downloads or
  compiles Fortran during a Cargo build.
- `slatec-core`: shared safe types, errors, and workspace abstractions.
- `slatec`: safe public API grouped by numerical domain.
- `slatec-tools`: development utilities such as metadata and dependency-graph generation.

The first safe Rust facade is an opt-in BLAS Level 1 subset. It calls the
original selected SLATEC Fortran through validated raw FFI; no numerical
implementation is translated into Rust. See
[`docs/api/safe-blas-level1.md`](docs/api/safe-blas-level1.md) for the
supported GNU MinGW profile, explicit native-link setup, and API boundary.
Selected real BLAS Level 2 and 3 slice APIs build on those conventions; see
[`docs/api/safe-blas-level2-level3.md`](docs/api/safe-blas-level2-level3.md).

## Canonical corpus preparation

`slatec-tools` provides the `slatec-corpus` command for acquiring and inventorying the policy-v1 `main-src` archive subset without adding its source bytes to Git. The 735 selected `src/*.f` files are reproducible evidence for that subset, not a claim to be the complete SLATEC library. See [`docs/extraction/corpus-tooling.md`](docs/extraction/corpus-tooling.md) for the offline workflow, evidence layout, and rights boundary; the [full-corpus audit policy](docs/source-corpus/corpus-completeness-audit-policy.md) defines the separate relocated-subset audit.

With cached evidence, `slatec-corpus select-full-corpus --offline` derives the
separate complete SLATEC-hosted provider profile. It does not alter the
`main-src` snapshot or make a redistribution conclusion.

Follow-on extraction stages currently include the fixed-form program-unit scanner and the SLATEC prologue parser. Both consume verified local evidence, write compact generated indexes, and keep detailed source-derived text in ignored evidence directories.

The FFI preparation stage is a conservative executable-interface inventory and
an explicit corpus-wide GNU Fortran compilation and raw-binding generator. It
uses the original selected Fortran without translating numerical algorithms;
ABI-sensitive and unresolved interfaces remain gated for review. See
[`docs/extraction/ffi-interface-inventory.md`](docs/extraction/ffi-interface-inventory.md).
Native archive construction and raw-binding validation are explicit, local
operations; ordinary Cargo builds and CI never compile or download Fortran.
Historical machine templates are not treated as validated representations of a
modern host. The separate
[`GNU MinGW runtime-profile validation`](docs/extraction/runtime-profile-validation.md)
configures machine constants, characterizes legacy error levels in child
processes, and verifies representative FNLIB initialization paths before any
broad safe special-function API is attempted.

The resulting scalar special-function facade is separately opt-in, remains
limited to that validated GNU MinGW profile, and serializes its FNLIB calls
where legacy saved/error state is process-global. See
[`docs/api/safe-special-functions.md`](docs/api/safe-special-functions.md) for
the supported families, native setup, and domain/error boundary.

The first callback-bearing safe API covers focused adaptive quadrature through
`QAG`, `QAGS`, `QAGI`, and `QAWC` in both precisions. See
[`docs/api/safe-quadrature.md`](docs/api/safe-quadrature.md) for workspace,
panic-containment, concurrency, tolerance, and native-profile rules.

With the complete selected evidence and GNU MinGW compiler available, run:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-native-ffi --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-runtime-profile --offline
```
