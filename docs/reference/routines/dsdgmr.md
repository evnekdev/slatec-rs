# DSDGMR

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Diagonally scaled GMRES iterative sparse Ax=b solver. This routine uses the generalized minimum residual (GMRES) method with diagonal scaling to solve possibly non-symmetric linear systems of the form: Ax = b.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, NSAVE, ITOL INTEGER ITMAX, ITER, IERR, IUNIT, LENW, IWORK(LENIW), LENIW DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, RWORK(LENW) CALL DSDGMR(N, B, X, NELT, IA, JA, A, ISYM, NSAVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, $ RWORK, LENW, IWORK, LENIW) DSDGMR solves a linear system A*X = B rewritten in the form: (SB*A*(M-inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or (SB*(M-inverse)*A*(SX-inverse))*(SX*X) = SB*(M-inverse)*B, with left preconditioning, where A is an n-by-n double precision matrix, X and B are N-vectors, SB and SX are diagonal scaling matrices, and M is the diagonal of A. It uses preconditioned Krylov subpace methods based on the generalized minimum residual method (GMRES). This routine is a driver routine which assumes a SLAP matrix data structure and sets up the necessary information to do diagonal preconditioning and calls the main GMRES routine DGMRES for the solution of the linear system. DGMRES optionally performs either the full orthogonalization version of the GMRES algorithm or an incomplete variant of it. Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES algorithm generates a sequence of approximations X(L) to the true solution of the above linear system. The convergence criteria for stopping the iteration is based on the size of the scaled norm of the residual R(L) = B -

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2A4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/dsdgmr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsdgmr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dsdgmr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DSDGMR](https://www.netlib.org/slatec/lin/dsdgmr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Order of the Matrix. |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision B(N). Right-hand side vector. |
| 3 | `X` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. The actual stopping test is either: norm(SB*(B-A*X(L))). le. TOL*norm(SB*B), for right preconditioning, or norm(SB*(M-inverse)*(B-A*X(L))). |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Number of Non-Zeros stored in A. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IA(NELT). A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '\|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 \|11 12 0 0 15\| A: 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 \|21 22 0 0 0\| IA: 1 2 5 \| 2 1 \| 3 5 \| 4 \| 5 1 3 \| 0 0 33 0 35\| JA: 1 4 6 8 9 12 \| 0 0 0 44 0\| \|51 0 53 0 55\| Side Effects: The SLAP Triad format (IA, JA, A) is modified internally to be the SLAP Column format. |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | JA(NELT). |
| 7 | `A` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | Double Precision A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format. See "Description", below. If the SLAP Triad format is chosen it is changed internally to the SLAP Column format. The actual stopping test is either: norm(SB*(B-A*X(L))). le. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. |
| 9 | `NSAVE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Number of direction vectors to save and orthogonalize against. Must be greater than 1. |
| 10 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Flag to indicate the type of convergence criterion used. 0 Means the iteration stops when the test described below on the residual RL is satisfied. This is the "Natural Stopping Criteria" for this routine. Other values of ITOL cause extra, otherwise unnecessary, computation per iteration and are therefore much less efficient. See ISDGMR (the stop test routine) for more information. |
| 11 | `TOL` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double Precision. Convergence criterion, as described below. If TOL is set to zero on input, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. |
| 12 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Maximum number of iterations. This routine uses the default of NRMAX = ITMAX/NSAVE to determine when each restart should occur. See the description of NRMAX and MAXL in DGMRES for a full and frightfully interesting discussion of this topic. |
| 13 | `ITER` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. |
| 14 | `ERR` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double Precision. Error estimate of error in final approximate solution, as defined by ITOL. Letting norm() denote the Euclidean norm, ERR is defined as follows. If ITOL=0, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), for right or no preconditioning, and norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), for left preconditioning. If ITOL=1, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), since right or no preconditioning being used. If ITOL=2, then ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ since left preconditioning is being If ITOL=3, then ERR = Max \|(Minv*(B-A*X(L)))(i)/x(i)\| i=1,n If ITOL=11, then ERR = norm(SB*(X(L)-SOLN))/norm(SB*SOLN). |
| 15 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Return error flag. 0 => All went well. 1 => Insufficient storage allocated for RGWK or IGWK. 2 => Routine DPIGMR failed to reduce the norm of the current residual on its last call, and so the iteration has stalled. In this case, X equals the last computed approximation. |
| 16 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur. |
| 17 | `RWORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (LENW) | Double Precision RWORK(LENW). Double Precision array of size LENW. |
| 18 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Length of the double precision workspace, RWORK. 1 + N*(NSAVE+7) + NSAVE*(NSAVE+3). For the recommended values of NSAVE (10), RWORK has size at least 131 + 17*N. |
| 19 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (LENIW) | IWORK(USER DEFINED >= 30). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. IWORK(10) Amount of Double Precision workspace actually used. |
| 20 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Length of the integer workspace IWORK. LENIW >= 30. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`RWORK`: Double Precision RWORK(LENW). Double Precision array of size LENW.

`IWORK`: IWORK(USER DEFINED >= 30). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. IWORK(10) Amount of Double Precision workspace actually used.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::dsdgmr`. Native symbol: `dsdgmr_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::dsdgmr`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
