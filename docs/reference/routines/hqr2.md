# HQR2

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors of a real upper Hessenberg matrix using QR method.

## Description

This subroutine is a translation of the ALGOL procedure HQR2, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). This subroutine finds the eigenvalues and eigenvectors of a REAL UPPER Hessenberg matrix by the QR method. The eigenvectors of a REAL GENERAL matrix can also be found if ELMHES and ELTRAN or ORTHES and ORTRAN have been used to reduce this general matrix to Hessenberg form and to accumulate the similarity transformations. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, H and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix H. N is an INTEGER variable. N must be less than or equal to NM. LOW and IGH are two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix, N. H contains the upper Hessenberg matrix. H is a two-dimensional REAL array, dimensioned H(NM,N). Z contains the transformation matrix produced by ELTRAN after the reduction by ELMHES, or by ORTRAN after the reduction by ORTHES, if performed. If the eigenvectors of the Hessenberg matrix are desired, Z must contain the identity matrix. Z is a two-dimensional REAL array, dimensioned Z(NM,M). On OUTPUT H has been destroyed. WR and WI contain the real and imaginary parts, respectively, of the eigenvalues. The eigenvalues are unordered except that complex conjugate pairs of values appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N. WR and WI are onedimensional REAL arrays, dimensioned WR(N) and WI(N). Z contains the real and imaginary parts of the eigenvectors. If the J-th eigenvalue is real, the J-th column of Z contains its eigenvector. If the J-th eigenvalue is complex with positive imaginary part, the J-th and (J+1)-th columns of Z contain the real and imaginary parts of its eigenvector. The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been found. IERR is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2, ..., N, but no eigenvectors are computed. Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/hqr2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/hqr2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/hqr2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::hqr2`
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
