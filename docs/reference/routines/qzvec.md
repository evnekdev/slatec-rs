# QZVEC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The optional fourth step of the QZ algorithm for generalized eigenproblems. Accepts a matrix in quasi-triangular form and another in upper triangular and computes the eigenvectors of the triangular problem and transforms them back to the original coordinates Usually preceded by QZHES, QZIT, and QZVAL.

## Description

This subroutine is the optional fourth step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL matrices, one of them in quasi-triangular form (in which each 2-by-2 block corresponds to a pair of complex eigenvalues) and the other in upper triangular form. It computes the eigenvectors of the triangular problem and transforms the results back to the original coordinate system. It is usually preceded by QZHES, QZIT, and QZVAL. On Input NM must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM. A contains a real upper quasi-triangular matrix. A is a twodimensional REAL array, dimensioned A(NM,N). B contains a real upper triangular matrix. In addition, location B(N,1) contains the tolerance quantity (EPSB) computed and saved in QZIT. B is a two-dimensional REAL array, dimensioned B(NM,N). ALFR, ALFI, and BETA are one-dimensional REAL arrays with components whose ratios ((ALFR+I*ALFI)/BETA) are the generalized eigenvalues. They are usually obtained from QZVAL. They are dimensioned ALFR(N), ALFI(N), and BETA(N). Z contains the transformation matrix produced in the reductions by QZHES, QZIT, and QZVAL, if performed. If the eigenvectors of the triangular problem are desired, Z must contain the identity matrix. Z is a two-dimensional REAL array, dimensioned Z(NM,N). On Output A is unaltered. Its subdiagonal elements provide information about the storage of the complex eigenvectors. B has been destroyed. ALFR, ALFI, and BETA are unaltered. Z contains the real and imaginary parts of the eigenvectors. If ALFI(J) .EQ. 0.0, the J-th eigenvalue is real and the J-th column of Z contains its eigenvector. If ALFI(J) .NE. 0.0, the J-th eigenvalue is complex. If ALFI(J) .GT. 0.0, the eigenvalue is the first of a complex pair and the J-th and (J+1)-th columns of Z contain its eigenvector. If ALFI(J) .LT. 0.0, the eigenvalue is the second of a complex pair and the (J-1)-th and J-th columns of Z contain the conjugate of its eigenvector. Each eigenvector is normalized so that the modulus of its largest component is 1.0 . Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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
- GAMS classifications: `D4C3`
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

- Canonical provider: `lin/qzvec.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/qzvec.f) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
