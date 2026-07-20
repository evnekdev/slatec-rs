# QZVAL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The third step of the QZ algorithm for generalized eigenproblems. Accepts a pair of real matrices, one in quasi-triangular form and the other in upper triangular form and computes the eigenvalues of the associated eigenproblem. Usually preceded by QZHES, QZIT, and followed by QZVEC.

## Description

This subroutine is the third step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL matrices, one of them in quasi-triangular form and the other in upper triangular form. It reduces the quasi-triangular matrix further, so that any remaining 2-by-2 blocks correspond to pairs of complex eigenvalues, and returns quantities whose ratios give the generalized eigenvalues. It is usually preceded by QZHES and QZIT and may be followed by QZVEC. On Input NM must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM. A contains a real upper quasi-triangular matrix. A is a twodimensional REAL array, dimensioned A(NM,N). B contains a real upper triangular matrix. In addition, location B(N,1) contains the tolerance quantity (EPSB) computed and saved in QZIT. B is a two-dimensional REAL array, dimensioned B(NM,N). MATZ should be set to .TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to .FALSE. otherwise. MATZ is a LOGICAL variable. Z contains, if MATZ has been set to .TRUE., the transformation matrix produced in the reductions by QZHES and QZIT, if performed, or else the identity matrix. If MATZ has been set to .FALSE., Z is not referenced. Z is a two-dimensional REAL array, dimensioned Z(NM,N). On Output A has been reduced further to a quasi-triangular matrix in which all nonzero subdiagonal elements correspond to pairs of complex eigenvalues. B is still in upper triangular form, although its elements have been altered. B(N,1) is unaltered. ALFR and ALFI contain the real and imaginary parts of the diagonal elements of the triangular matrix that would be obtained if A were reduced completely to triangular form by unitary transformations. Non-zero values of ALFI occur in pairs, the first member positive and the second negative. ALFR and ALFI are one-dimensional REAL arrays, dimensioned ALFR(N) and ALFI(N). BETA contains the diagonal elements of the corresponding B, normalized to be real and non-negative. The generalized eigenvalues are then the ratios ((ALFR+I*ALFI)/BETA). BETA is a one-dimensional REAL array, dimensioned BETA(N). Z contains the product of the right hand transformations (for all three steps) if MATZ has been set to .TRUE. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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
- GAMS classifications: `D4C2C`
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

- Canonical provider: `lin/qzval.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/qzval.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-logical`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
