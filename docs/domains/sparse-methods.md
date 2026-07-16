# Sparse linear algebra and iterative methods

## Scope

This survey covers SLATEC sparse matrix storage, matrix-vector operations, iterative methods, preconditioning and sparse linear-programming overlap where it affects infrastructure. The central named package is the SLATEC Linear Algebra Package (SLAP), which Netlib places in the relocated `slatec/lin` subset alongside BLAS, LINPACK and EISPACK ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-lin`](https://www.netlib.org/slatec/lin/)).

## Problem classes

- nonsymmetric sparse linear systems;
- normal-equation formulations;
- sparse positive-definite systems where applicable;
- stationary iterations;
- Krylov methods;
- left/right or split preconditioning;
- diagonal scaling;
- incomplete-LU preconditioning;
- sparse matrix storage conversion;
- sparse matrix-vector and transpose-matrix-vector operations;
- user-supplied matrix and preconditioner actions.

## SLAP routine families

| Algorithm/family | Representative routines | Preconditioning/configuration | Precision |
|---|---|---|---|
| BiConjugate Gradient | `SBCG/DBCG` and scaled/ILU drivers | generic, diagonal, incomplete LU variants | S/D |
| Conjugate Gradient on normal equations | `SCGN/DCGN` and variants | generic, diagonal, incomplete LU | S/D |
| BiCG Squared | `SCGS/DCGS` and variants | generic, diagonal, incomplete LU | S/D |
| GMRES | `SGMRES/DGMRES`, `SSDGMR/DSDGMR`, `SSLUGM/DSLUGM` | generic, diagonal or ILU | S/D |
| Orthomin | `SOMN/DOMN` and variants | generic, diagonal or ILU | S/D |
| stationary methods | `SSJAC/DSJAC`, `SSGS/DSGS`, iterative refinement families | Jacobi, Gauss–Seidel, splitting/ILU | S/D |
| sparse matvec | `SSMV/DSMV`, `SSMTV/DSMTV` | SLAP column format | S/D |
| storage conversion | `SS2Y/DS2Y` and associated sorting/format helpers | triad to SLAP column format | S/D |
| triangular solve/preconditioner support | `SSLI/DSLI`, `SSLI2/DSLI2` and related routines | lower/upper factor application | S/D |
| stopping tests/internal Krylov machinery | `ISDBCG`, `SPIGMR/DPIGMR`, `SORTH/DORTH`, `SHELS/DHELS` | solver-specific | mostly subsidiary |

Sources: [`slatec-toc`](https://www.netlib.org/slatec/toc), [`netlib-slap`](https://www.netlib.org/slap/), [`slatec-lin`](https://www.netlib.org/slatec/lin/).

The SLATEC table of contents identifies `SLPDOC/DLPDOC` as SLAP Version 2.0.2 documentation and describes the package as solving large sparse symmetric and nonsymmetric positive-definite systems with preconditioned iterative methods ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Storage models

The table of contents identifies at least:

- **SLAP Triad format**, a coordinate-like list used as an input/conversion form;
- **SLAP Column format**, used by sparse matrix-vector products;
- internal incomplete-factor and preconditioner representations.

`SS2Y/DS2Y` convert triad storage to the SLAP column form. A safe wrapper must verify:

- array lengths;
- index base;
- sortedness and duplicate-entry rules;
- valid row/column ranges;
- whether conversion mutates or reorders inputs;
- whether diagonal entries receive special placement.

These details require routine-prologue extraction before implementation.

## User-facing versus internal routines

High-level drivers such as `DGMRES` and `DBCG` are user-callable. Their source prologues expose iteration controls, stopping criteria, work arrays and user-supplied matrix/preconditioner actions. Internal routines implement Arnoldi/orthogonalization, triangular least-squares solves, stopping tests and workspace checks ([`DGMRES source`](https://www.netlib.org/slatec/lin/dgmres.f); [`DBCG source`](https://www.netlib.org/slatec/lin/dbcg.f)).

The presence of a public routine does not guarantee a simple assembled-matrix API. SLAP supports abstract operation callbacks through conventions such as `MATVEC` and `MSOLVE`, allowing matrix-free or custom-preconditioned use.

## Callback and reverse-control characteristics

Representative iterative drivers accept external routines for:

- multiplication by \(A\);
- multiplication by \(A^T\), where needed;
- application of a preconditioner or approximate solve;
- optional iteration monitoring or stopping logic in some families.

These callbacks may receive workspace, integer parameters and solver state. They are more complex than a single Rust closure `Fn(&[f64], &mut [f64])`.

**Project interpretation:** the safe layer should define traits such as `LinearOperator` and `Preconditioner`, but the raw adapter must preserve every callback argument and documented control flag.

## Algorithm assumptions

### Krylov methods

BiCG and CGS target nonsymmetric problems but can be sensitive to breakdown and rounding. GMRES builds an orthogonal basis and requires a restart/subspace dimension, giving a memory-versus-convergence trade-off. CG on normal equations changes conditioning and may be less attractive than methods acting on the original problem.

### Preconditioning

SLAP offers generic callback preconditioning plus packaged diagonal and incomplete-LU variants. Preconditioner application is part of the mathematical algorithm, not merely a performance option.

### Stopping tests

SLAP routines expose tolerance and stopping-test modes. A reported convergence code depends on the selected residual/error estimate and preconditioner conventions. Safe wrappers should report the selected criterion and final measured quantity.

## Dependencies

Sparse drivers depend on:

- BLAS-like vector operations;
- sparse matrix-vector kernels;
- sorting/index utilities;
- error handling and machine constants;
- preconditioner-specific triangular solves;
- Krylov internal routines;
- real and integer workspaces.

Strongly connected groups may make very fine-grained raw crates impractical.

## Limitations

- storage conventions are historical and not automatically compatible with CSR/CSC used by modern Rust libraries;
- one-based integer indices may be embedded in data structures;
- duplicate-entry and sorting behavior must be verified;
- workspace formulas can be large and depend on restart parameters;
- breakdown and nonconvergence statuses are algorithm-specific;
- implicit callback state may inhibit thread safety;
- no complex sparse family is evident as a regular S/D/C set;
- iterative convergence depends on scaling, preconditioning and stopping choices;
- historical “large sparse” claims should not be interpreted as modern scalability guarantees.

## Relation to modern sparse libraries

Modern APIs commonly separate sparse storage from iterative solvers and support abstract linear operators. SciPy’s sparse linear algebra module and PETSc’s KSP interface both organize multiple direct/iterative methods and operator/preconditioner abstractions, although their capabilities and scale are far broader than SLAP ([SciPy sparse linear algebra](https://docs.scipy.org/doc/scipy/reference/sparse.linalg.html); [PETSc KSP](https://petsc.org/release/manual/ksp/)).

**Modern interpretation:** SLAP can offer historically important, self-contained iterative methods, but `slatec-rs` should interoperate with common Rust sparse formats through explicit conversion rather than defining SLAP storage as the universal public format.

## Preliminary FFI risks

| Risk | Why it matters | Proposed mitigation |
|---|---|---|
| index width/base | Fortran integer and one-based indices differ from Rust conventions | validate and checked-convert into owned FFI buffers |
| callback ABI | operator/preconditioner callbacks have package-specific signatures | dedicated trampolines generated per signature |
| callback aliasing | input/output arrays may share work storage | model documented mutability conservatively |
| workspace sizing | GMRES and ILU sizes depend on problem/control parameters | checked size calculators and owned workspace objects |
| integer workspaces | indices and offsets may overflow 32-bit Fortran integer | reject oversized problems before call |
| error/global state | workspace checker can call SLATEC error handling | capture raw status and serialize until proven safe |
| matrix conversion | sorting/reordering may mutate arrays | conversion consumes owned buffers |
| cancellation/monitoring | package flags may control iteration | map only verified controls to observer/cancellation APIs |
| concurrency | callbacks and error handlers may use global registries | thread-local scoped context plus stress testing |

## Project proposals for safe Rust APIs

### Operator traits

```text
trait LinearOperator<T> {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn apply(&mut self, x: &[T], y: &mut [T]) -> Result<()>;
    fn apply_transpose(&mut self, x: &[T], y: &mut [T]) -> Result<()>;
}

trait Preconditioner<T> {
    fn solve(&mut self, x: &[T], y: &mut [T]) -> Result<()>;
}
```

The exact trait signatures remain proposals. Some solvers do not require transpose application, so capability-specific traits may be preferable.

### Sparse storage adapters

Provide checked conversion from ecosystem CSR/CSC/COO representations to owned SLAP triad/column buffers. Expose a lower-level native SLAP format only for advanced compatibility use.

### Solver results

```text
IterativeResult {
    x,
    status,
    iterations,
    residual_measure,
    stopping_test,
}
```

Preserve the original status integer and package-specific diagnostics.

### Solver configuration

Use method-specific option structs for BiCG, CGS, GMRES and Orthomin. A single generic option bag would permit invalid combinations.

## Candidate crate boundaries

**Raw, tentative:**

- `slatec-slap-sys` containing the verified SLAP closure;
- possibly a separate `slatec-sparse-support-sys` only if sources are shared without circularity.

**Safe, tentative:**

- `slatec-sparse`;
- modules `storage`, `operator`, `preconditioner`, `bicg`, `cgs`, `gmres`, `orthomin`, `stationary`.

BLAS dependencies should preferably be linked once and feature-controlled to avoid duplicate symbols.

## Open questions and experiments

- What are the exact SLAP triad and column invariants?
- Which routines require transpose multiplication?
- Are duplicate entries summed, rejected or retained?
- Which preconditioner callbacks may mutate persistent state?
- Can two iterative solves run concurrently in one process?
- How do iteration/status results compare with modern reference implementations?
- What are the largest dimensions representable by the target Fortran integer ABI?
- Which SLAP source files overlap the standalone `/slap` collection, and are they identical?
- Can operator callbacks safely borrow Rust sparse matrices without copying?
- Should safe APIs expose the historic stopping-test choices verbatim or normalize them?

## Sources

- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-lin`](https://www.netlib.org/slatec/lin/)
- [`netlib-slap`](https://www.netlib.org/slap/)
- [`DGMRES source`](https://www.netlib.org/slatec/lin/dgmres.f)
- [`DBCG source`](https://www.netlib.org/slatec/lin/dbcg.f)
- [SciPy sparse linear algebra](https://docs.scipy.org/doc/scipy/reference/sparse.linalg.html)
- [PETSc KSP](https://petsc.org/release/manual/ksp/)
