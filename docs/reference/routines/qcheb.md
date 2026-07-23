# QCHEB

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

This routine computes the CHEBYSHEV series expansion of degrees 12 and 24 of a function using A FAST FOURIER TRANSFORM METHOD F(X) = SUM(K=1,..,13) (CHEB12(K)*T(K-1,X)), F(X) = SUM(K=1,..,25) (CHEB24(K)*T(K-1,X)), Where T(K,X) is the CHEBYSHEV POLYNOMIAL OF DEGREE K.

## Description

Chebyshev Series Expansion Standard Fortran Subroutine Real version PARAMETERS ON ENTRY X - Real Vector of dimension 11 containing the Values COS(K*PI/24), K = 1, ..., 11 FVAL - Real Vector of dimension 25 containing the function values at the points (B+A+(B-A)*COS(K*PI/24))/2, K = 0, ...,24, where (A,B) is the approximation interval. FVAL(1) and FVAL(25) are divided by two (these values are destroyed at output). ON RETURN CHEB12 - Real Vector of dimension 13 containing the CHEBYSHEV coefficients for degree 12 CHEB24 - Real Vector of dimension 25 containing the CHEBYSHEV Coefficients for degree 24

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `quadpack`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qcheb.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qcheb.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qcheb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (11) | Chebyshev Series Expansion Standard Fortran Subroutine Real version PARAMETERS ON ENTRY X - Real Vector of dimension 11 containing the Values COS(K*PI/24), K = 1, ..., 11 FVAL - Real Vector of dimension 25 containing the function values at the points (B+A+(B-A)*COS(K*PI/24))/2, K = 0, ...,24, where (A,B) is the approximation interval. | Chebyshev Series Expansion Standard Fortran Subroutine Real version PARAMETERS ON ENTRY X - Real Vector of dimension 11 containing the Values COS(K*PI/24), K = 1, ..., 11 FVAL - Real Vector of dimension 25 containing the function values at the points (B+A+(B-A)*COS(K*PI/24))/2, K = 0, ...,24, where (A,B) is the approximation interval. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FVAL` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (25) | Chebyshev Series Expansion Standard Fortran Subroutine Real version PARAMETERS ON ENTRY X - Real Vector of dimension 11 containing the Values COS(K*PI/24), K = 1, ..., 11 FVAL - Real Vector of dimension 25 containing the function values at the points (B+A+(B-A)*COS(K*PI/24))/2, K = 0, ...,24, where (A,B) is the approximation interval. | Chebyshev Series Expansion Standard Fortran Subroutine Real version PARAMETERS ON ENTRY X - Real Vector of dimension 11 containing the Values COS(K*PI/24), K = 1, ..., 11 FVAL - Real Vector of dimension 25 containing the function values at the points (B+A+(B-A)*COS(K*PI/24))/2, K = 0, ...,24, where (A,B) is the approximation interval. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CHEB12` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (13) | ON RETURN CHEB12 - Real Vector of dimension 13 containing the CHEBYSHEV coefficients for degree 12 CHEB24 - Real Vector of dimension 25 containing the CHEBYSHEV Coefficients for degree 24 | ON RETURN CHEB12 - Real Vector of dimension 13 containing the CHEBYSHEV coefficients for degree 12 CHEB24 - Real Vector of dimension 25 containing the CHEBYSHEV Coefficients for degree 24 Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CHEB24` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (25) | ON RETURN CHEB12 - Real Vector of dimension 13 containing the CHEBYSHEV coefficients for degree 12 CHEB24 - Real Vector of dimension 25 containing the CHEBYSHEV Coefficients for degree 24 | ON RETURN CHEB12 - Real Vector of dimension 13 containing the CHEBYSHEV coefficients for degree 12 CHEB24 - Real Vector of dimension 25 containing the CHEBYSHEV Coefficients for degree 24 Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
