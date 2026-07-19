# QZIT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The second step of the QZ algorithm for generalized eigenproblems. Accepts an upper Hessenberg and an upper triangular matrix and reduces the former to quasi-triangular form while preserving the form of the latter. Usually preceded by QZHES and followed by QZVAL and QZVEC.

## Description

This subroutine is the second step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART, as modified in technical note NASA TN D-7305(1973) by WARD. This subroutine accepts a pair of REAL matrices, one of them in upper Hessenberg form and the other in upper triangular form. It reduces the Hessenberg matrix to quasi-triangular form using orthogonal transformations while maintaining the triangular form of the other matrix. It is usually preceded by QZHES and followed by QZVAL and, possibly, QZVEC. On Input NM must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM. A contains a real upper Hessenberg matrix. A is a twodimensional REAL array, dimensioned A(NM,N). B contains a real upper triangular matrix. B is a twodimensional REAL array, dimensioned B(NM,N). EPS1 is a tolerance used to determine negligible elements. EPS1 = 0.0 (or negative) may be input, in which case an element will be neglected only if it is less than roundoff error times the norm of its matrix. If the input EPS1 is positive, then an element will be considered negligible if it is less than EPS1 times the norm of its matrix. A positive value of EPS1 may result in faster execution, but less accurate results. EPS1 is a REAL variable. MATZ should be set to .TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to .FALSE. otherwise. MATZ is a LOGICAL variable. Z contains, if MATZ has been set to .TRUE., the transformation matrix produced in the reduction by QZHES, if performed, or else the identity matrix. If MATZ has been set to .FALSE., Z is not referenced. Z is a two-dimensional REAL array, dimensioned Z(NM,N). On Output A has been reduced to quasi-triangular form. The elements below the first subdiagonal are still zero, and no two consecutive subdiagonal elements are nonzero. B is still in upper triangular form, although its elements have been altered. The location B(N,1) is used to store EPS1 times the norm of B for later use by QZVAL and QZVEC. Z contains the product of the right hand transformations (for both steps) if MATZ has been set to .TRUE. IERR is an INTEGER flag set to Zero for normal return, J if neither A(J,J-1) nor A(J-1,J-2) has become zero after a total of 30*N iterations. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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
- GAMS classifications: `D4C1B3`
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

- Canonical provider: `lin/qzit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/qzit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
