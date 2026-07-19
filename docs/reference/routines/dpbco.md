# DPBCO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a real symmetric positive definite matrix stored in band form and estimate the condition number of the matrix.

## Description

DPBCO factors a double precision symmetric positive definite matrix stored in band form and estimates the condition of the matrix. If RCOND is not needed, DPBFA is slightly faster. To solve A*X = B , follow DPBCO by DPBSL. To compute INVERSE(A)*C , follow DPBCO by DPBSL. To compute DETERMINANT(A) , follow DPBCO by DPBDI. On Entry ABD DOUBLE PRECISION(LDA, N) the matrix to be factored. The columns of the upper triangle are stored in the columns of ABD and the diagonals of the upper triangle are stored in the rows of ABD . See the comments below for details. LDA INTEGER the leading dimension of the array ABD . LDA must be .GE. M + 1 . N INTEGER the order of the matrix A . M INTEGER the number of diagonals above the main diagonal. 0 .LE. M .LT. N . On Return ABD an upper triangular matrix R , stored in band form, so that A = TRANS(R)*R . If INFO .NE. 0 , the factorization is not complete. RCOND DOUBLE PRECISION an estimate of the reciprocal condition of A . For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND . If RCOND is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then A may be singular to working precision. In particular, RCOND is zero if exact singularity is detected or the estimate underflows. If INFO .NE. 0 , RCOND is unchanged. Z DOUBLE PRECISION(N) a work vector whose contents are usually unimportant. If A is singular to working precision, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z) . If INFO .NE. 0 , Z is unchanged. INFO INTEGER = 0 for normal return. = K signals an error condition. The leading minor of order K is not positive definite. Band Storage If A is a symmetric positive definite band matrix, the following program segment will set up the input. M = (band width above diagonal) DO 20 J = 1, N I1 = MAX(1, J-M) DO 10 I = I1, J K = I-J+M+1 ABD(K,J) = A(I,J) 10 CONTINUE 20 CONTINUE This uses M + 1 rows of A , except for the M by M upper left triangle, which is ignored. Example: If the original matrix is 11 12 13 0 0 0 12 22 23 24 0 0 13 23 33 34 35 0 0 24 34 44 45 46 0 0 35 45 55 56 0 0 0 46 56 66 then N = 6 , M = 2 and ABD should contain * * 13 24 35 46 * 12 23 34 45 56 11 22 33 44 55 66

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2B2`
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

- Canonical provider: `lin/dpbco.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dpbco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dpbco.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
