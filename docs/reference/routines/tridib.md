# TRIDIB

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.

## Description

This subroutine is a translation of the ALGOL procedure BISECT, NUM. MATH. 9, 386-393(1967) by Barth, Martin, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 249-256(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix between specified boundary indices, using bisection. On Input N is the order of the matrix. N is an INTEGER variable.

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

- Canonical provider: `lin/tridib.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/tridib.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/tridib.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [TRIDIB](https://www.netlib.org/slatec/lin/tridib.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `EPS1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `D` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `E` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `E2` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `LB` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `UB` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `M11` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 11 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Status argument classified by fixed-form executable read/write analysis. |
| 12 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `RV4` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 14 | `RV5` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

values.  If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix.  EPS1 is a REAL variable. D contains the diagonal elements of the symmetric tridiagonal matrix.  D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions.  E(1) is arbitrary.  E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E. E2(1) is arbitrary.  E2 is a one-dimensional REAL array, dimensioned E2(N). M11 specifies the lower boundary index for the set of desired eigenvalues.  M11 is an INTEGER variable. M specifies the number of eigenvalues desired.  The upper boundary index M22 is then obtained as M22=M11+M-1. M is an INTEGER variable. On Output EPS1 is unaltered unless it has been reset to its (last) default value. D and E are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero. LB and UB define an interval containing exactly the desired eigenvalues.  LB and UB are REAL variables. W contains, in its first M positions, the eigenvalues between indices M11 and M22 in ascending order. W is a one-dimensional REAL array, dimensioned W(M). IND contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. IND is an one-dimensional INTEGER array, dimensioned IND(M). IERR is an INTEGER flag set to Zero       for normal return, 3*N+1      if multiple eigenvalues at index M11 make unique selection of LB impossible, 3*N+2      if multiple eigenvalues at index M22 make unique selection of UB impossible. RV4 and RV5 are one-dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in the bisection process.  RV4 and RV5 are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY ------------------------------------------------------------------

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::tridib`. Native symbol: `tridib_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tridib`
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
