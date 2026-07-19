# Safe Cartesian FISHPACK 2D solver

The opt-in `fishpack-cartesian-2d` feature exposes one checked, owned
single-precision Cartesian elliptic solver:
[`CartesianHelmholtz2d`](../../crates/slatec/src/differential_equations/pde.rs)
over selected SLATEC/FISHPACK `HWSCRT`. It solves the standard five-point
finite-difference approximation to

\[
u_{xx} + u_{yy} + \lambda u = f(x,y)
\]

on `[a,b] × [c,d]`. It is deliberately not a general PDE framework.

## Selected native contract

The ABI authority is verified cached SLATEC relocated source
[`fishfft/hwscrt.f`](https://www.netlib.org/slatec/fishfft/hwscrt.f), SHA-256
`9bcd5a3be9e6d63e7dcc33637eb37ef07ba10b727b74859d08cb4daa7f813202`.
It is a single-precision (`f32`) Fortran subroutine exported as `hwscrt_` on
the reviewed GNU MinGW profile, whose `INTEGER` arguments are 32-bit. The
safe facade calls no other public FISHPACK entry point.

The standalone Netlib FISHPACK `hwscrt.f` is not silently substituted. It is a
materially different later revision, including a workspace/GENBUN preparation
expression: the selected source uses `3*MUNK`, while standalone uses
`3.*FLOAT(MUNK)`. The selected cache body fixes the argument contract and
numerical behavior used here.

The exact source closure has 11 units: `HWSCRT`, `GENBUN`, `POISD2`, `POISN2`,
`POISP2`, `COSGEN`, `S1MERG`, `TRIX`, `TRI3`, `PIMACH`, and existing canonical
BLAS `SCOPY`. `GENBUN` is an internal subsidiary. No FFTPACK transform root,
XERROR/XERMSG path, callback, or Fortran I/O is reachable. FISHPACK, cyclic
reduction, 3D, polar, cylindrical, spherical, complex, `GENBUN` public,
`BLKTRI` public, and generic-PDE APIs remain deferred.

## Grid and discrete equation

`UniformAxis::new(lower, upper, intervals)` stores a finite, strictly
increasing interval with at least four panels. It has `intervals + 1` nodes,
including both endpoints:

\[
x_i=a+i\Delta x,\quad \Delta x=(b-a)/M,\qquad
y_j=c+j\Delta y,\quad \Delta y=(d-c)/N.
\]

`Grid2` owns every node, including boundary nodes. Its public indexing is
`grid[(x, y)]`; x varies fastest, so storage is `values[y * nx + x]`. With
`nx=M+1`, that is exactly contiguous Fortran memory for `F(IDIMF, N+1)` when
the private `IDIMF` is `M+1`. There is no public leading dimension, hidden
transpose, or borrowed native workspace.

At an interior node the documented equation is

\[
\frac{u_{i-1,j}-2u_{i,j}+u_{i+1,j}}{\Delta x^2}+\frac{u_{i,j-1}-2u_{i,j}+u_{i,j+1}}{\Delta y^2}+\lambda u_{i,j}=f_{i,j}.
\]

The RHS is sampled at grid nodes. Dirichlet values replace boundary entries
before native entry; other boundary entries remain RHS samples. The owned RHS
storage becomes the returned solution.

## Typed boundaries

`AxisBoundary` is used for x and y independently; raw codes are private. An
x-edge vector has `y.nodes()` values and a y-edge vector has `x.nodes()`
values. Every supplied vector includes both corners.

| Variant | Native code | Lower endpoint | Upper endpoint |
| --- | ---: | --- | --- |
| `Periodic` | 0 | periodic identification | periodic identification |
| `Dirichlet` | 1 | value | value |
| `DirichletNeumann` | 2 | value | derivative |
| `Neumann` | 3 | derivative | derivative |
| `NeumannDirichlet` | 4 | derivative | value |

Derivatives are with respect to the increasing coordinate (`dU/dx` on x and
`dU/dy` on y), not outward normals. The facade does not negate them. HWSCRT
reads derivative arrays only for the applicable code; other native arrays are
private dummy buffers.

When both axes prescribe a Dirichlet value for a corner, values must be finite
and exactly equal as `f32`, else construction returns
`PdeError::InconsistentCornerValues`. Periodic duplicate RHS edges must also
match exactly. The wrapper never silently chooses an incompatible corner.

## Results, singularity, and status

`CartesianPdeSolution` returns owned `Grid2`, optional `PERTRB`,
`SolutionUniqueness`, and `NativePdeStatus`.

- `lambda == 0` with each axis periodic or code-3 Neumann is singular. The
  result reports `DefinedUpToAdditiveConstant`.
- HWSCRT may calculate and subtract `PERTRB` from the RHS to make that system
  compatible. The solution is least-squares for the original approximation,
  exact for the perturbed system, and is defined up to a constant.
  `perturbation()` preserves the native value, including zero.
- `lambda > 0` is not rejected: HWSCRT attempts the solve and `IERROR=6`
  becomes `NativePdeStatus::PositiveCoefficientMayNotHaveSolution`.
- Invalid inputs are rejected before FFI. An unexpected native code remains in
  `PdeError::NativeFailure`.

## Workspace and serialization

The wrapper privately allocates the reviewed upper bound

```text
4*(N+1) + (13 + floor(log2(N+1)))*(M+1)
```

with checked arithmetic and verifies native `W(1)`. Callers never pass an
`IDIMF` or work array. Dimensions are checked before conversion to 32-bit
Fortran integers and allocations use fallible reserves.

Focused source and object inspection found no `COMMON`, `SAVE`, `DATA`, block
data, or I/O in this closure. The complete FFI call nevertheless holds the
repository-wide process-global native lock. It is `SerializedGlobal`, not a
claim that FISHPACK calls can execute in parallel.

## Feature, example, and validation

The API requires `std`, one explicit backend, and the narrow family feature:

```text
cargo run -p slatec --example fishpack_cartesian_2d \
  --no-default-features --features std,source-build,fishpack-cartesian-2d \
  --target x86_64-pc-windows-gnu
```

The source backend consumes a separately acquired, hash-verified cache.
`external-backend` verifies Rust compilation against the same `hwscrt_` ABI,
but its runtime behavior is not claimed without a compatible library. See the
complete [example](../../examples/fishpack_cartesian_2d.rs) and
[family/backend policy](family-features-and-backends.md).

The native suite covers asymmetric exact-stencil Dirichlet Poisson,
negative-coefficient Helmholtz, mixed codes 2/4 and derivative orientation,
periodic code 0, singular code-3 Neumann perturbation, positive-coefficient
status, x-fast layout, and multi-threaded global-lock serialization. Exact
quadratics are compared tightly because the centered stencil is exact; other
manufactured solutions use finite-difference-appropriate tolerances.

Machine-readable audit records are generated as
`generated/safe-api/fishpack-cartesian-2d-*.json`.
