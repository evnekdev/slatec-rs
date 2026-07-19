# COMQR2

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors of a complex upper Hessenberg matrix.

## Description

This subroutine is a translation of a unitary analogue of the ALGOL procedure COMLR2, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). The unitary analogue substitutes the QR algorithm of Francis (COMP. JOUR. 4, 332-345(1962)) for the LR algorithm. This subroutine finds the eigenvalues and eigenvectors of a COMPLEX UPPER Hessenberg matrix by the QR method. The eigenvectors of a COMPLEX GENERAL matrix can also be found if CORTH has been used to reduce this general matrix to Hessenberg form. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, HR, HI, ZR, and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix H=(HR,HI). N is an INTEGER variable. N must be less than or equal to NM. LOW and IGH are two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N. ORTR and ORTI contain information about the unitary transformations used in the reduction by CORTH, if performed. Only elements LOW through IGH are used. If the eigenvectors of the Hessenberg matrix are desired, set ORTR(J) and ORTI(J) to 0.0E0 for these elements. ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and ORTI(IGH). HR and HI contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. Their lower triangles below the subdiagonal contain information about the unitary transformations used in the reduction by CORTH, if performed. If the eigenvectors of the Hessenberg matrix are desired, these elements may be arbitrary. HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and HI(NM,N). On OUTPUT ORTR, ORTI, and the upper Hessenberg portions of HR and HI have been destroyed. WR and WI contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N. WR and WI are onedimensional REAL arrays, dimensioned WR(N) and WI(N). ZR and ZI contain the real and imaginary parts, respectively, of the eigenvectors. The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been found. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). IERR is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2, ..., N, but no eigenvectors are computed. Calls CSROOT for complex square root. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/comqr2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/comqr2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/comqr2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
