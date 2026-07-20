# MINFIT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the singular value decomposition of a rectangular matrix and solve the related linear least squares problem.

## Description

This subroutine is a translation of the ALGOL procedure MINFIT, NUM. MATH. 14, 403-420(1970) by Golub and Reinsch. HANDBOOK FOR AUTO. COMP., VOL II-LINEAR ALGEBRA, 134-151(1971). This subroutine determines, towards the solution of the linear T system AX=B, the singular value decomposition A=USV of a real T M by N rectangular matrix, forming U B rather than U. Householder bidiagonalization and a variant of the QR algorithm are used. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, A and B, as declared in the calling program dimension statement. Note that NM must be at least as large as the maximum of M and N. NM is an INTEGER variable. M is the number of rows of A and B. M is an INTEGER variable. N is the number of columns of A and the order of V. N is an INTEGER variable. A contains the rectangular coefficient matrix of the system. A is a two-dimensional REAL array, dimensioned A(NM,N). IP is the number of columns of B. IP can be zero. B contains the constant column matrix of the system if IP is not zero. Otherwise, B is not referenced. B is a twodimensional REAL array, dimensioned B(NM,IP). On OUTPUT A has been overwritten by the matrix V (orthogonal) of the decomposition in its first N rows and columns. If an error exit is made, the columns of V corresponding to indices of correct singular values should be correct. W contains the N (non-negative) singular values of A (the diagonal elements of S). They are unordered. If an error exit is made, the singular values should be correct for indices IERR+1, IERR+2, ..., N. W is a one-dimensional REAL array, dimensioned W(N). T B has been overwritten by U B. If an error exit is made, T the rows of U B corresponding to indices of correct singular values should be correct. IERR is an INTEGER flag set to Zero for normal return, K if the K-th singular value has not been determined after 30 iterations. The singular values should be correct for indices IERR+1, IERR+2, ..., N. RV1 is a one-dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/minfit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/minfit.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/minfit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::minfit`
- Current legacy Rust paths: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
