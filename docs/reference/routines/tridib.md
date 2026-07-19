# TRIDIB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.

## Description

This subroutine is a translation of the ALGOL procedure BISECT, NUM. MATH. 9, 386-393(1967) by Barth, Martin, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 249-256(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix between specified boundary indices, using bisection. On Input N is the order of the matrix. N is an INTEGER variable. EPS1 is an absolute error tolerance for the computed eigenvalues. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. EPS1 is a REAL variable. D contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E. E2(1) is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). M11 specifies the lower boundary index for the set of desired eigenvalues. M11 is an INTEGER variable. M specifies the number of eigenvalues desired. The upper boundary index M22 is then obtained as M22=M11+M-1. M is an INTEGER variable. On Output EPS1 is unaltered unless it has been reset to its (last) default value. D and E are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero. LB and UB define an interval containing exactly the desired eigenvalues. LB and UB are REAL variables. W contains, in its first M positions, the eigenvalues between indices M11 and M22 in ascending order. W is a one-dimensional REAL array, dimensioned W(M). IND contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. IND is an one-dimensional INTEGER array, dimensioned IND(M). IERR is an INTEGER flag set to Zero for normal return, 3*N+1 if multiple eigenvalues at index M11 make unique selection of LB impossible, 3*N+2 if multiple eigenvalues at index M22 make unique selection of UB impossible. RV4 and RV5 are one-dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in the bisection process. RV4 and RV5 are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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
