# SGBCO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a band matrix by Gaussian elimination and estimate the condition number of the matrix.

## Description

SBGCO factors a real band matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, SGBFA is slightly faster. To solve A*X = B , follow SBGCO by SGBSL. To compute INVERSE(A)*C , follow SBGCO by SGBSL. To compute DETERMINANT(A) , follow SBGCO by SGBDI. On Entry ABD REAL(LDA, N) contains the matrix in band storage. The columns of the matrix are stored in the columns of ABD and the diagonals of the matrix are stored in rows ML+1 through 2*ML+MU+1 of ABD . See the comments below for details. LDA INTEGER the leading dimension of the array ABD . LDA must be .GE. 2*ML + MU + 1 . N INTEGER the order of the original matrix. ML INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . MU INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if ML .LE. MU . On Return ABD an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written A = L*U where L is a product of permutation and unit lower triangular matrices and U is upper triangular. IPVT INTEGER(N) an integer vector of pivot indices. RCOND REAL an estimate of the reciprocal condition of A . For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND . If RCOND is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then A may be singular to working precision. In particular, RCOND is zero if exact singularity is detected or the estimate underflows. Z REAL(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z) . Band Storage If A is a band matrix, the following program segment will set up the input. ML = (band width below the diagonal) MU = (band width above the diagonal) M = ML + MU + 1 DO 20 J = 1, N I1 = MAX(1, J-MU) I2 = MIN(N, J+ML) DO 10 I = I1, I2 K = I - J + M ABD(K,J) = A(I,J) 10 CONTINUE 20 CONTINUE This uses rows ML+1 through 2*ML+MU+1 of ABD . In addition, the first ML rows in ABD are used for elements generated during the triangularization. The total number of rows needed in ABD is 2*ML+MU+1 . The ML+MU by ML+MU upper left triangle and the ML by ML lower right triangle are not referenced. Example: If the original matrix is 11 12 13 0 0 0 21 22 23 24 0 0 0 32 33 34 35 0 0 0 43 44 45 46 0 0 0 54 55 56 0 0 0 0 65 66 then N = 6, ML = 1, MU = 2, LDA .GE. 5 and ABD should contain * * * + + + , * = not used * * 13 24 35 46 , + = used for pivoting * 12 23 34 45 56 11 22 33 44 55 66 21 32 43 54 65 *

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::linear_algebra::banded::BandMatrixRef::factorize_with_condition_estimate`

## Providers

- Canonical provider: `lin/sgbco.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sgbco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sgbco.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sgbco`
- Current legacy Rust paths: `slatec_sys::banded::sgbco`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::linear_algebra::banded::BandMatrixRef::factorize_with_condition_estimate`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
