# SRLCAL

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Internal routine for SGMRES.

## Description

This routine calculates the scaled residual RL from the V(I)'s. *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. KMP :IN Integer The number of previous V vectors the new vector VNEW must be made orthogonal to. (KMP .le. MAXL) LL :IN Integer The current dimension of the Krylov subspace. MAXL :IN Integer The maximum dimension of the Krylov subspace. V :IN Real V(N,LL) The N x LL array containing the orthogonal vectors V(*,1) to V(*,LL). Q :IN Real Q(2*MAXL) A real array of length 2*MAXL containing the components of the Givens rotations used in the QR decomposition of HES. It is loaded in SHEQR and used in SHELS. RL :OUT Real RL(N) The residual vector RL. This is either SB*(B-A*XL) if not preconditioning or preconditioning on the right, or SB*(M-inverse)*(B-A*XL) if preconditioning on the left. SNORMW :IN Real Scale factor. PROD :IN Real The product s1*s2*...*sl = the product of the sines of the Givens rotations used in the QR factorization of the Hessenberg matrix HES. R0NRM :IN Real The scaled norm of initial residual R0.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
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

- Canonical provider: `lin/srlcal.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/srlcal.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/srlcal.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KMP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MAXL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (N, *) | This routine calculates the scaled residual RL from the V(I)'s. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Q` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RL` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | This routine calculates the scaled residual RL from the V(I)'s. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SNORMW` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PROD` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R0NRM` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. | *Usage: INTEGER N, KMP, LL, MAXL REAL V(N,LL), Q(2*MAXL), RL(N), SNORMW, PROD, R0NORM CALL SRLCAL(N, KMP, LL, MAXL, V, Q, RL, SNORMW, PROD, R0NRM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
