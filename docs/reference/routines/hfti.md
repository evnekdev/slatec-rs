# HFTI

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linear least squares problems by performing a QR factorization of the matrix using Householder transformations.

## Description

DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. The problem data consists of an M by N matrix A, an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below. The NB column vectors of B represent right-side vectors for NB distinct linear least squares problems. This set of problems can also be written as the matrix least squares problem AX = B, where X is the N by NB solution matrix. Note that if B is the M by M identity matrix, then X will be the pseudo-inverse of A. This subroutine first transforms the augmented matrix (A B) to a matrix (R C) using premultiplying Householder transformations with column interchanges. All subdiagonal elements in the matrix R are zero and its diagonal elements satisfy ABS(R(I,I)).GE.ABS(R(I+1,I+1)), I = 1,...,L-1, where L = MIN(M,N). The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. Then a solution of minimum Euclidean length is computed using the first KRANK rows of (R C). To be specific we suggest that the user consider an easily computable matrix norm, such as, the maximum of all column sums of magnitudes. Now if the relative uncertainty of B is EPS, (norm of uncertainty/ norm of B), it is suggested that TAU be set approximately equal to EPS*(norm of A). The user must dimension all arrays appearing in the call list.. A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N). This permits the solution of a range of problems in the same array space. The entire set of parameters for HFTI are

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/hfti.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/hfti.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/hfti.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/hfti.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (MDA, *) | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (MDB, *) | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDB` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NB` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TAU` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | The problem data consists of an M by N matrix A, an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KRANK` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `G` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. | DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::hfti`. Native symbol: `hfti_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::hfti`
- Compatibility aliases: `none`
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
