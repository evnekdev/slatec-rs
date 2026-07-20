# BISECT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.

## Description

This subroutine is a translation of the bisection technique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval, using bisection. On INPUT N is the order of the matrix. N is an INTEGER variable. EPS1 is an absolute error tolerance for the computed eigenvalues. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. EPS1 is a REAL variable. D contains the diagonal elements of the input matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the input matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E. E2(1) is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). LB and UB define the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. LB and UB are REAL variables. MM should be set to an upper bound for the number of eigenvalues in the interval. WARNING - If more than MM eigenvalues are determined to lie in the interval, an error return is made with no eigenvalues found. MM is an INTEGER variable. On OUTPUT EPS1 is unaltered unless it has been reset to its (last) default value. D and E are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero. M is the number of eigenvalues determined to lie in (LB,UB). M is an INTEGER variable. W contains the M eigenvalues in ascending order. W is a one-dimensional REAL array, dimensioned W(MM). IND contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. IND is an one-dimensional INTEGER array, dimensioned IND(MM). IERR is an INTEGER flag set to Zero for normal return, 3*N+1 if M exceeds MM. In this case, M contains the number of eigenvalues determined to lie in (LB,UB). RV4 and RV5 are one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4A5`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/bisect.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/bisect.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/bisect.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Eigenvalue problems](../families/eigenvalue-problems.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On INPUT N is the order of the matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPS1` | input | `REAL` (`explicit`) | `*mut f32` | scalar | EPS1 is an absolute error tolerance for the computed eigenvalues. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | D contains the diagonal elements of the input matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | E contains the subdiagonal elements of the input matrix in its last N-1 positions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E2` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | E2 contains the squares of the corresponding elements of E. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LB` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | LB and UB define the interval to be searched for eigenvalues. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `UB` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | LB and UB define the interval to be searched for eigenvalues. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MM` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | MM should be set to an upper bound for the number of eigenvalues in the interval. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | M is the number of eigenvalues determined to lie in (LB,UB). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | W contains the M eigenvalues in ascending order. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IND` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IND contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IERR is an INTEGER flag set to Zero for normal return, 3*N+1 if M exceeds MM. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RV4` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | RV4 and RV5 are one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). | RV4 and RV5 are one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RV5` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | RV4 and RV5 are one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). | RV4 and RV5 are one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::bisect`. Native symbol: `bisect_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bisect`
- Compatibility aliases: `slatec_sys::eigen::numerical::bisect`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
