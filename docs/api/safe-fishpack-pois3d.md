# Safe structured FISHPACK `POIS3D`

`fishpack-pois3d` exposes one checked, owned single-precision wrapper over
the selected SLATEC/FISHPACK `POIS3D` driver. It is a structured discrete
linear-system solver, not a general three-dimensional PDE or six-face
boundary-condition API.

## Selected source and closure

The ABI authority is the selected `fishfft/pois3d.f` source with SHA-256
`9daf0cd2c9eab106f9b508f426003a98719d21ed29b4fde0f224300d8e88da78`.
At acquisition, the direct current Netlib `fishfft/pois3d.f` provider had the
same verified body; the cached selected source remains authoritative.

The exact source closure has 32 source units:

- `POIS3D`, `POS3D1`, `TRIDQ`, and `PIMACH`;
- 28 canonical units shared with the existing real-FFTPACK closure.

The feature reuses those canonical FFTPACK identities rather than compiling a
second FFTPACK copy. `POIS3D` has native symbol `pois3d_`, uses GNU Fortran
default 32-bit `INTEGER`, and receives single-precision scalars by reference.
It mutates `F` and workspace; in cyclic mode it also temporarily modifies and
restores its private owned copies of `A`, `B`, and `C`.

## Mathematical contract

Unknowns are `X(i,j,k)` for `i=1..L`, `j=1..M`, and `k=1..N`. The native
routine solves exactly

```text
C1 * (X(i-1,j,k) - 2 X(i,j,k) + X(i+1,j,k))
+ C2 * (X(i,j-1,k) - 2 X(i,j,k) + X(i,j+1,k))
+ A(k) * X(i,j,k-1) + B(k) * X(i,j,k) + C(k) * X(i,j,k+1)
= F(i,j,k).
```

`C1` and `C2` multiply unscaled second differences. Grid spacing is not a
native argument; callers incorporating a uniform physical grid normally put
`1/dx²` and `1/dy²` into those coefficients and construct the third-axis
operator themselves. This surface does not claim to solve arbitrary continuous
elliptic equations.

`Grid3` dimensions are counts of unknowns and every dimension must be at least
three. It stores `values[z * nx * ny + y * nx + x]`, so the first index varies
fastest and its contiguous buffer matches `F(LDIMF, MDIMF, N)` with private
leading dimensions `LDIMF=L` and `MDIMF=M`.

## Transverse ghost rules

`TransverseBoundary` hides `LPEROD` and `MPEROD`; the same rule applies to the
first and second indices. With a generic sequence `X(1)..X(Q)`, the variants
are:

| Variant | Lower ghost | Upper ghost |
| --- | --- | --- |
| `Periodic` | `X(0)=X(Q)` | `X(Q+1)=X(1)` |
| `ZeroBoth` | `X(0)=0` | `X(Q+1)=0` |
| `ZeroLowerReflectUpper` | `X(0)=0` | `X(Q+1)=X(Q-1)` |
| `ReflectBoth` | `X(0)=X(2)` | `X(Q+1)=X(Q-1)` |
| `ReflectLowerZeroUpper` | `X(0)=X(2)` | `X(Q+1)=0` |

These are discrete ghost-node relations. The API deliberately does not label
all zero or reflection rules as continuous Dirichlet or Neumann conditions,
and accepts no nonzero face arrays.

## Third axis

`ThirdAxisOperator` is one of two safe models:

- `Cyclic(CyclicAxisCoefficients)`: `k-1` and `k+1` wrap modulo `N`; the
  lower and upper coupling are one finite nonzero `off_diagonal`, while the
  diagonal is constant. This exactly makes `A(K)=C(K)=C(1)` and `B(K)=B(1)`,
  the condition checked by native `IERROR=9`.
- `Tridiagonal(TridiagonalAxisCoefficients)`: owns finite equal-length lower,
  diagonal, and upper vectors; `lower[0]` and `upper[N-1]` must be exactly
  zero. This corresponds to native `NPEROD=1` and prevents `IERROR=10`.

The safe constructor also checks coefficient-vector length against `Grid3::nz`,
finite RHS values and transverse coefficients, checked volume/allocation, and
32-bit native integer conversion.

## Workspace and statuses

Workspace is private and allocated with checked arithmetic using the native
documented lower bound:

```text
30 + L + M + 2*N + max(L,M,N)
+ 7 * (floor((L+1)/2) + floor((M+1)/2)).
```

The safe API prevents normal reachability of native errors 1–10. If a
prevalidated call returns a nonzero code, `Pois3dError::NativeFailure` retains
the raw value. The codes are: invalid first mode (1), `L<3` (2), invalid second
mode (3), `M<3` (4), invalid third mode (5), `N<3` (6), private leading-
dimension failures (7–8), cyclic coefficient inconsistency (9), and noncyclic
endpoint inconsistency (10). `POIS3D` documents no HWSCRT-style perturbation
or uniqueness result, so `solve` returns `Grid3` directly.

## Runtime and limitations

The complete closure has no reviewed `COMMON`, `SAVE` statement, `DATA`,
XERROR, Fortran-I/O, or callback path. `POIS3D`'s local array named `SAVE` is
a temporary restoration buffer, not a persistent state declaration. Calls
remain process-globally serialized through the same native lock as M1 and
FFTPACK because backend and Fortran runtime concurrent-entry guarantees remain
unqualified.

Deferred scope includes arbitrary six-face boundary data, general Cartesian
3D Helmholtz facades, public `GENBUN`/`BLKTRI`, non-Cartesian FISHPACK drivers,
complex PDEs, generic PDE traits, unstructured meshes, and variable-coefficient
general elliptic PDEs.

See [`fishpack_pois3d.rs`](../../examples/fishpack_pois3d.rs) for a small
asymmetric manufactured discrete system that derives its RHS, solves it, and
reports maximum error.
