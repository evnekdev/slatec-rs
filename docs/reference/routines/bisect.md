# BISECT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.

## Description

This subroutine is a translation of the bisection technique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval, using bisection.

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [BISECT](https://www.netlib.org/slatec/lin/bisect.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix. N is an INTEGER variable. |
| 2 | `EPS1` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | is an absolute error tolerance for the computed eigenvalues. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. is a REAL variable. is unaltered unless it has been reset to its (last) default value. |
| 3 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the diagonal elements of the input matrix. is a one-dimensional REAL array, dimensioned D(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. |
| 4 | `E` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the subdiagonal elements of the input matrix in its last N-1 positions. E(1) is arbitrary. is a one-dimensional REAL array, dimensioned E(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. |
| 5 | `E2` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the squares of the corresponding elements of E. is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). is also set to zero. |
| 6 | `LB` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables. |
| 7 | `UB` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables. |
| 8 | `MM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | should be set to an upper bound for the number of eigenvalues in the interval. WARNING - If more than MM eigenvalues are determined to lie in the interval, an error return is made with no eigenvalues found. is an INTEGER variable. |
| 9 | `M` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of eigenvalues determined to lie in (LB,UB). is an INTEGER variable. |
| 10 | `W` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the M eigenvalues in ascending order. is a one-dimensional REAL array, dimensioned W(MM). |
| 11 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. is an one-dimensional INTEGER array, dimensioned IND(MM). |
| 12 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, 3*N+1 if M exceeds MM. In this case, M contains the number of eigenvalues determined to lie in (LB,UB). |
| 13 | `RV4` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY. |
| 14 | `RV5` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::bisect`. Native symbol: `bisect_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bisect`
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
