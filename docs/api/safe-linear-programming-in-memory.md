# Safe in-memory SLATEC linear programming

Feature: `optimization-linear-programming-in-memory` (requires `std` and an
explicit native backend).

The safe API wraps the original real `SPLP` (`f32`) and `DSPLP` (`f64`)
drivers. It solves the native mathematical model

```text
minimize    cᵀx
subject to  A x = w
```

where both decision variables `x` and row activities `w` have independent
typed bounds. `LpBound::Fixed(v)` and a range with equal endpoints encode an
equality. `Free`, `Lower`, `Upper`, and `Range` distinguish free, one-sided,
and two-sided quantities without exposing the native `IND` integers.

This is linear optimization, not constrained least squares or quadratic
programming. There is no objective constant and the initial API exposes only
the native default minimization sense.

## In-memory-only guarantee

> This API supports only LPs that fit completely in native in-memory
> workspace. It never opens a paging file. Problems requiring external paging
> are rejected before native execution.

`PINITM`/`DPINTM` reserve `N + 6` staging words around the coefficient data.
The reviewed no-paging allocation is therefore

```text
LAMAT = max(N + 7, N + nnz(A) + 6)
```

and every validated sparse coefficient is assigned exactly once. The selected
source profile omits `PRWPGE`, `DPRWPG`, `PRWVIR`, `DPRWVR`, `SOPENM`, and
`SCLOSM`; ABI-compatible project-owned symbols only count a forbidden entry
and perform no I/O. Any nonzero counter discards the native result as a
contract violation. Option key 54, save/restore keys, Fortran units, filenames,
and temporary-directory policy are not public.

`LpOptions::maximum_matrix_entries` is a resident coefficient limit. A value
below `nnz(A)` returns `LpError::PagingRequired` before FFI. It never causes a
smaller native staging area or activates paging.

## Sparse callback and storage

`SparseColumns` owns conventional compressed sparse columns:

- `column_offsets.len() == columns + 1`;
- offsets start at zero, end at `values.len()`, and never decrease;
- row indices are zero-based, in range, and strictly increasing per column;
- duplicates, explicit zeros, and non-finite values are rejected.

The private `USRMT`/`DUSRMT` trampoline delivers columns sequentially and
converts each coordinate to one-based Fortran indexing. It uses `INDCAT=0`
(assignment), catches panics, rejects malformed native requests, and clears its
thread-local context after every return. No Rust unwind crosses Fortran. The
matrix is neither densified nor silently reordered.

## Workspaces and results

For `m` rows, `n` columns, `z = nnz(A)`,

```text
LAMAT = max(n + 7, n + z + 6)
LBM   = 8m
LW    = LAMAT + LBM + 4n + 8m
LIW   = LAMAT + 2*LBM + n + 11m
```

Every operation uses checked arithmetic and every length must fit default GNU
Fortran `INTEGER`. `LBM=8m` is the driver’s documented nominal LA05 basis
workspace, not an unproved dense worst-case estimate. If basis fill exhausts
it, the wrapper returns `InsufficientBasisWorkspace`; it does not retry with a
heuristic allocation and does not enable paging.

Optimal variables, `A x`, and `cᵀx` are independently recomputed from the
original Rust problem and checked against native activities and all bounds.
Infeasible, no-finite-solution, combined, and iteration-limit outcomes remain
distinct and carry no fabricated solution.

## Runtime policy

The complete callback registration, XERROR scope, solve, status extraction,
and cleanup hold the process-wide native runtime lock (`SerializedGlobal`).
LA05 uses mutable `LA05DD` COMMON state and initialized saved locals. It also
calls `XSETUN`; the wrapper captures and restores both the XERROR control flag
and the complete output-unit list. Avoiding paging does not make the native
solver reentrant.

The core API uses `Vec`, slices, and exact sparse arrays. It adds no dependency
on `nalgebra`, `ndarray`, `faer`, or a competing matrix container. Optional
ecosystem adapters remain possible as separate future features.

See `examples/linear_programming/basic.rs`, `bounds.rs`, and `infeasible.rs`.
Paging support, unit lifecycle management, warm starts, user bases,
sensitivity analysis, mixed-integer, quadratic, and nonlinear programming all
remain deferred.
