# slatec-rs

Safe APIs are selected by coherent family features such as `blas-level1`,
`special-gamma`, `quadrature-basic`, `roots-scalar`, `nonlinear-easy`,
`nonlinear-expert`, `least-squares-nonlinear-easy`, and
`least-squares-nonlinear-expert`, `least-squares-covariance`, and
`least-squares-linear-nonnegative`, `least-squares-linear-bounded`, and
`least-squares-linear-bounded-constrained`, `ode-sdrive-expert`, `dassl`, and
`fishpack-cartesian-2d`, `optimization-linear-programming-in-memory`, and
`piecewise-polynomial`.
Numerical families
require one explicit backend: `prebuilt`, `source-build`, `system`, or
`external-backend`. Prebuilt publication is currently blocked because the
historical source rights remain unresolved. `source-build` is offline-only and
consumes a separately acquired, SHA-256-verified cache; ordinary Cargo builds
never download SLATEC source from `build.rs`.

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
The separate `special-scalar-expanded` feature adds reviewed real scalar
logarithmic and Spence integrals plus Carlson `RC`/`RF`/`RD`/`RJ` elliptic
integrals in both precisions. It remains process-serialized because the native
closure uses saved initialization and XERROR; complex, sequence, workspace,
and translated implementations remain excluded.

The callback-bearing safe API covers focused QUADPACK integration through
`QAG`, `QAGS`, `QAGI`, `QAWC`, `QAGP`, `QAWS`, `QAWO`, `QAWF`, `QNG`, and
`QNC79` in both precisions. See
[`docs/api/safe-quadrature.md`](docs/api/safe-quadrature.md) for workspace,
panic-containment, concurrency, tolerance, and native-profile rules.

The opt-in `roots` feature provides bracketed scalar root finding through the
original `FZERO` and `DFZERO` routines. It shares the contained callback
runtime with quadrature; polynomial roots remain deferred. The opt-in
`nonlinear-easy` feature adds finite-difference easy drivers over original
`SNSQE` and `DNSQE`. `nonlinear-expert` exposes reviewed `SNSQ`/`DNSQ` controls,
including banded finite differences, scaling, and contained dense user
Jacobians. `nonlinear-jacobian-check` provides alloc-only `CHKDER`/`DCKDER`
helpers. `least-squares-nonlinear-easy` adds residual-only, finite-difference
Levenberg--Marquardt wrappers over `SNLS1E` and `DNLS1E` for problems with
`M >= N`. `least-squares-nonlinear-expert` adds reviewed `SNLS1`/`DNLS1`
finite-difference and dense analytic-Jacobian modes with checked expert
controls, scaling, exact workspace, and native evaluation counts; covariance
estimation is a separate `least-squares-covariance` feature over reviewed
`SCOV`/`DCOV`. See the [easy-driver guide](docs/api/safe-nonlinear-easy-drivers.md),
[expert nonlinear guide](docs/api/safe-nonlinear-expert.md), and
[least-squares guides](docs/api/safe-least-squares-easy-drivers.md) and
[expert least-squares guide](docs/api/safe-least-squares-expert.md), plus the
[covariance guide](docs/api/safe-least-squares-covariance.md). The separate
`least-squares-linear-nonnegative` feature wraps `WNNLS`/`DWNNLS` for linear
least squares with an optional equality block and per-variable free or
nonnegative constraints; it is neither nonlinear fitting nor linear
programming. The separate `least-squares-linear-bounded` feature wraps
`SBOLS`/`DBOLS` for dense residual minimization with closed per-variable
lower/upper bounds. The `least-squares-linear-constrained` family additionally
adds dense equality and lower-sided inequality constraints through `LSEI` and
`DLSEI`; it does not add linear programming. See [the WNNLS guide](docs/api/safe-weighted-nonnegative-least-squares.md),
[bounded-driver guide](docs/api/safe-bounded-linear-least-squares.md), and
[general-constraint guide](docs/api/safe-linear-constraint-least-squares.md).
`least-squares-linear-bounded-constrained` separately wraps `SBOCLS`/`DBOCLS`
for bounds on both variables and linear constraint expressions; see the
[bounded constrained guide](docs/api/safe-bounded-constrained-linear-least-squares.md).
The opt-in `ode-sdrive-expert` feature adds owned, panic-contained real
explicit-IVP sessions over `SDRIV3`/`DDRIV3`; it is deliberately limited to an
RHS callback and caller-controlled continuation. See the
[SDRIVE session guide](docs/api/safe-ode-sdrive-expert.md).
The separate hosted `dassl` feature adds real residual-only index-1 DAE
sessions over `SDASSL`/`DDASSL` for `G(t, y, y') = 0`. It owns all continuation
workspace, requires caller-supplied sufficiently consistent initial `y` and
`y'`, contains callback errors and panics, and uses only DASSL's internally
differenced dense iteration matrix. User Jacobians, banded storage, event
handling, and automatic consistency calculation remain deferred. See the
[DASSL guide](docs/api/safe-dassl.md).
The opt-in `optimization-linear-programming-in-memory` feature wraps original
`SPLP`/`DSPLP` for sparse linear programs that fit entirely in native resident
workspace. Paging, Fortran units, save/restore, and legacy printing are never
enabled; insufficient resident capacity is rejected before FFI. Optimal
results expose checked typed basis data, source-audited row multipliers and
reduced costs, and independently computed primal-dual KKT diagnostics. The
reviewed controls cover iteration limit, feasibility tolerances, and pricing
only. See the [in-memory LP guide](docs/api/safe-linear-programming-in-memory.md).
The opt-in hosted `fftpack-real` feature adds reusable single-precision real
FFTPACK plans for periodic, easy Fourier, full sine/cosine, and quarter-wave
sine/cosine transforms. Plans own their initialized native workspaces and
preserve FFTPACK's native packed layouts and normalization; the authoritative
snapshot has no reviewed double-precision counterparts. See the
[real FFTPACK guide](docs/api/safe-real-fftpack.md).
The opt-in hosted `fftpack-complex` feature adds one-dimensional in-place
single-precision complex FFTPACK plans over `num_complex::Complex32`. It uses
the reviewed standard real-array `CFFTI1/CFFTF1/CFFTB1` interface, preserving
the negative/positive exponent directions and native `N` round-trip scale.
The selected snapshot has no reviewed complex `f64` roots. See the
[complex FFTPACK guide](docs/api/safe-complex-fftpack.md).
The opt-in hosted `fishpack-cartesian-2d` feature adds an owned `f32`
Cartesian Poisson/Helmholtz finite-difference solve over selected `HWSCRT`.
Typed periodic, value, derivative, and mixed edges are validated before FFI;
the result preserves perturbation and non-uniqueness information, while
workspace and leading dimensions remain private. Calls are process-serialized.
See the [Cartesian FISHPACK guide](docs/api/safe-fishpack-cartesian-2d.md).
The opt-in hosted `pchip` feature adds owned `f32` and `f64` piecewise-cubic
Hermite curves over original PCHIP routines. It supports monotone and typed
controlled derivative construction, PCHSP's typed endpoint conditions, value
and first-derivative evaluation, and definite integration. It never sorts or
merges knots; endpoint-cubic extrapolation is rejected by default and may be
enabled explicitly with a report. See the [PCHIP guide](docs/api/safe-pchip.md).

The opt-in hosted `bspline` feature adds owned `f32` and `f64` scalar B-spline
curves. `BINTK`/`DBINTK` construct exact interpolants from strictly increasing
nodes, values, an explicit complete knot sequence, and order; `BVALU`/`DBVALU`
evaluate and `BSQAD`/`DBSQAD` integrate them. It has no hidden knot-generation
policy, sorting, coefficient conversion, or caller workspace. See the
[B-spline guide](docs/api/safe-bspline.md).

The opt-in hosted `piecewise-polynomial` feature adds owned real univariate
PP curves using exact native right-Taylor storage. It evaluates values and
derivatives through `PPVAL`/`DPPVAL`, integrates exactly through
`PPQAD`/`DPPQAD`, and converts a reviewed `BSpline` through
`BSPPP`/`DBSPPP` when both features are enabled. It rejects native
extrapolation by default, does not sort inputs, and leaves PCHIP and
PP-to-B-spline conversion deferred. See the
[piecewise-polynomial guide](docs/api/safe-piecewise-polynomial.md).

The opt-in hosted `banded-linear-systems` feature provides compact general
real `f32`/`f64` LINPACK LU factors and direct/transpose solves together with
`SGBCO`/`DGBCO` reciprocal 1-norm condition estimates and `SGBDI`/`DGBDI`
base-ten scaled determinant metadata. It does not create a dense matrix,
packing, or ecosystem-adapter API. See the
[banded-system guide](docs/api/safe-banded-linear-systems.md).

All native-call concurrency and storage claims are deliberately conservative:
hosted wrappers are process-serialized, while existing `no_std`/`alloc` BLAS
and Jacobian-checking APIs are backend-dependent rather than advertised as
thread-safe. Matrix layout is explicit; no hidden transpose, packing, sparse
conversion, or arbitrary-stride materialization is performed. See the
[runtime concurrency and storage policy](docs/architecture/runtime-concurrency-and-storage-policy.md)
and its generated per-function records.

With the complete selected evidence and GNU MinGW compiler available, run:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-native-ffi --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-runtime-profile --offline
```
## Safe API capability layers

The safe crates are `no_std` by architecture. Allocation and hosted runtime services are explicit Cargo capabilities: `alloc` uses Rust's standalone allocation crate without enabling `std`, while `std` implies `alloc`. Slice-based BLAS wrappers remain allocation-free; Jacobian checking requires only `alloc`; the current special-function runtime and callback-bearing quadrature, root, easy nonlinear, and expert nonlinear APIs require `std` because the validated Fortran profile uses process-global state, TLS, panic containment, and—in nonlinear solving—internally allocated workspace.

This does **not** claim bare-metal support. The only validated native backend is GNU Fortran for `x86_64-w64-mingw32`. See [safe API capability and native support](docs/api/no-std-and-native-support.md) and the [complete safe function index](docs/api/function-index.md).
