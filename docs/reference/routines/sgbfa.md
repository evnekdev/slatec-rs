# SGBFA

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a band matrix using Gaussian elimination.

## Description

SGBFA factors a real band matrix by elimination. SGBFA is usually called by SBGCO, but it can be called directly with a saving in time if RCOND is not needed. On Entry ABD REAL(LDA, N) contains the matrix in band storage. The columns of the matrix are stored in the columns of ABD and the diagonals of the matrix are stored in rows ML+1 through 2*ML+MU+1 of ABD . See the comments below for details. LDA INTEGER the leading dimension of the array ABD . LDA must be .GE. 2*ML + MU + 1 . N INTEGER the order of the original matrix. ML INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . MU INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if ML .LE. MU . On Return ABD an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written A = L*U , where L is a product of permutation and unit lower triangular matrices and U is upper triangular. IPVT INTEGER(N) an integer vector of pivot indices. INFO INTEGER = 0 normal value. = K if U(K,K) .EQ. 0.0 . This is not an error condition for this subroutine, but it does indicate that SGBSL will divide by zero if called. Use RCOND in SBGCO for a reliable indication of singularity. Band Storage If A is a band matrix, the following program segment will set up the input. ML = (band width below the diagonal) MU = (band width above the diagonal) M = ML + MU + 1 DO 20 J = 1, N I1 = MAX(1, J-MU) I2 = MIN(N, J+ML) DO 10 I = I1, I2 K = I - J + M ABD(K,J) = A(I,J) 10 CONTINUE 20 CONTINUE This uses rows ML+1 through 2*ML+MU+1 of ABD . In addition, the first ML rows in ABD are used for elements generated during the triangularization. The total number of rows needed in ABD is 2*ML+MU+1 . The ML+MU by ML+MU upper left triangle and the ML by ML lower right triangle are not referenced.

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
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/sgbfa.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sgbfa.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sgbfa.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::banded::sgbfa`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
