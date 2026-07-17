# slatec-rs

Safe APIs are selected by coherent family features such as `blas-level1`,
`special-gamma`, `quadrature-basic`, and `roots-scalar`. Hosted builds use the
`bundled` provider by default: Cargo retrieves only checksum-pinned reviewed
sources, compiles the enabled families' dependency closure as separate objects,
and links a static archive without requiring `SLATEC_NATIVE_LIB_DIR` or a
SLATEC DLL. Historical source rights remain unresolved, so source and native
bytes stay in Cargo's cache/build directories and are not redistributed in the
crate.

The safe Rust layer is `no_std`. `alloc` is an independent capability and does
not require `std`; `std` enables `alloc`. The current GNU MinGW native backend
is hosted and runtime-validated, not a bare-metal backend. See
[`docs/api/family-features-and-backends.md`](docs/api/family-features-and-backends.md).

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

The callback-bearing safe API covers focused QUADPACK integration through
`QAG`, `QAGS`, `QAGI`, `QAWC`, `QAGP`, `QAWS`, `QAWO`, `QAWF`, `QNG`, and
`QNC79` in both precisions. See
[`docs/api/safe-quadrature.md`](docs/api/safe-quadrature.md) for workspace,
panic-containment, concurrency, tolerance, and native-profile rules.

The opt-in `roots` feature provides bracketed scalar root finding through the
original `FZERO` and `DFZERO` routines. It shares the contained callback
runtime with quadrature; polynomial and nonlinear-system routines remain
deferred. See [`docs/api/safe-roots.md`](docs/api/safe-roots.md).

With the complete selected evidence and GNU MinGW compiler available, run:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-native-ffi --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-runtime-profile --offline
```
## Safe API capability layers

The safe crates are `no_std` by architecture. Allocation and hosted runtime services are explicit Cargo capabilities: `alloc` uses Rust's standalone allocation crate without enabling `std`, while `std` implies `alloc`. Slice-based BLAS wrappers remain allocation-free; the current special-function runtime and callback-bearing quadrature/root APIs require `std` because the validated Fortran profile uses process-global state, TLS, and panic containment.

This does **not** claim bare-metal support. The only validated native backend is GNU Fortran for `x86_64-w64-mingw32`. See [safe API capability and native support](docs/api/no-std-and-native-support.md) and the [complete safe function index](docs/api/function-index.md).
