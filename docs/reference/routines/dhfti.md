# DHFTI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a least squares problem for banded matrices using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted.

## Description

DIMENSION A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N) This subroutine solves a linear least squares problem or a set of linear least squares problems having the same matrix but different right-side vectors. The problem data consists of an M by N matrix A, an M by NB matrix B, and an absolute tolerance parameter TAU whose usage is described below. The NB column vectors of B represent right-side vectors for NB distinct linear least squares problems. This set of problems can also be written as the matrix least squares problem AX = B, where X is the N by NB solution matrix. Note that if B is the M by M identity matrix, then X will be the pseudo-inverse of A. This subroutine first transforms the augmented matrix (A B) to a matrix (R C) using premultiplying Householder transformations with column interchanges. All subdiagonal elements in the matrix R are zero and its diagonal elements satisfy ABS(R(I,I)).GE.ABS(R(I+1,I+1)), I = 1,...,L-1, where L = MIN(M,N). The subroutine will compute an integer, KRANK, equal to the number of diagonal terms of R that exceed TAU in magnitude. Then a solution of minimum Euclidean length is computed using the first KRANK rows of (R C). To be specific we suggest that the user consider an easily computable matrix norm, such as, the maximum of all column sums of magnitudes. Now if the relative uncertainty of B is EPS, (norm of uncertainty/ norm of B), it is suggested that TAU be set approximately equal to EPS*(norm of A). The user must dimension all arrays appearing in the call list.. A(MDA,N),(B(MDB,NB) or B(M)),RNORM(NB),H(N),G(N),IP(N). This permits the solution of a range of problems in the same array space. The entire set of parameters for DHFTI are INPUT.. All TYPE REAL variables are DOUBLE PRECISION A(*,*),MDA,M,N The array A(*,*) initially contains the M by N matrix A of the least squares problem AX = B. The first dimensioning parameter of the array A(*,*) is MDA, which must satisfy MDA.GE.M Either M.GE.N or M.LT.N is permitted. There is no restriction on the rank of A. The condition MDA.LT.M is considered an error. B(*),MDB,NB If NB = 0 the subroutine will perform the orthogonal decomposition but will make no

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
- GAMS classifications: `D9`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dhfti.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dhfti.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dhfti.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dhfti.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
