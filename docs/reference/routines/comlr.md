# COMLR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues of a complex upper Hessenberg matrix using the modified LR method.

## Description

This subroutine is a translation of the ALGOL procedure COMLR, NUM. MATH. 12, 369-376(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 396-403(1971). This subroutine finds the eigenvalues of a COMPLEX UPPER Hessenberg matrix by the modified LR method. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, HR and HI, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix H=(HR,HI). N is an INTEGER variable. N must be less than or equal to NM. LOW and IGH are two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N. HR and HI contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. Their lower triangles below the subdiagonal contain the multipliers which were used in the reduction by COMHES, if performed. HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and HI(NM,N). On OUTPUT The upper Hessenberg portions of HR and HI have been destroyed. Therefore, they must be saved before calling COMLR if subsequent calculation of eigenvectors is to be performed. WR and WI contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N. WR and WI are onedimensional REAL arrays, dimensioned WR(N) and WI(N). IERR is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2, ..., N. Calls CSROOT for complex square root. Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C2B`
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

- Canonical provider: `lin/comlr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/comlr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/comlr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::comlr`
- Current legacy Rust paths: `none`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
