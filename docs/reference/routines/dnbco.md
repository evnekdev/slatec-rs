# DNBCO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a band matrix using Gaussian elimination and estimate the condition number.

## Description

DNBCO factors a double precision band matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, DNBFA is slightly faster. To solve A*X = B , follow DNBCO by DNBSL. To compute INVERSE(A)*C , follow DNBCO by DNBSL. To compute DETERMINANT(A) , follow DNBCO by DNBDI. On Entry ABE DOUBLE PRECISION(LDA, NC) contains the matrix in band storage. The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE. NC must be .GE. 2*ML+MU+1 . See the comments below for details. LDA INTEGER the leading dimension of the array ABE. LDA must be .GE. N . N INTEGER the order of the original matrix. ML INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . MU INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if ML .LE. MU . On Return ABE an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written A = L*U where L is a product of permutation and unit lower triangular matrices and U is upper triangular. IPVT INTEGER(N) an integer vector of pivot indices. RCOND DOUBLE PRECISION an estimate of the reciprocal condition of A . For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND . If RCOND is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then A may be singular to working precision. In particular, RCOND is zero if exact singularity is detected or the estimate underflows. Z DOUBLE PRECISION(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z) . Band Storage If A is a band matrix, the following program segment will set up the input. ML = (band width below the diagonal) MU = (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 ABE(I,K) = A(I,J) 10 CONTINUE 20 CONTINUE This uses columns 1 through ML+MU+1 of ABE . Furthermore, ML additional columns are needed in ABE starting with column ML+MU+2 for elements generated during the triangularization. The total number of columns needed in ABE is 2*ML+MU+1 . Example: If the original matrix is 11 12 13 0 0 0 21 22 23 24 0 0 0 32 33 34 35 0 0 0 43 44 45 46 0 0 0 54 55 56 0 0 0 0 65 66 then N = 6, ML = 1, MU = 2, LDA .GE. 5 and ABE should contain * 11 12 13 + , * = not used 21 22 23 24 + , + = used for pivoting 32 33 34 35 + 43 44 45 46 + 54 55 56 * + 65 66 * * +

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
- GAMS classifications: `D2A2`
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

- Canonical provider: `main-src/src/dnbco.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnbco.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnbco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnbco.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
