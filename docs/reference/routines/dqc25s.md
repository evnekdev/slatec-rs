# DQC25S

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F*W over (BL,BR), with error estimate, where the weight function W has a singular behaviour of ALGEBRAICO-LOGARITHMIC type at the points A and/or B. (BL,BR) is a part of (A,B).

## Description

Integration rules for integrands having ALGEBRAICO-LOGARITHMIC end point singularities Standard fortran subroutine Double precision version PARAMETERS

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dqc25s.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqc25s.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqc25s.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqc25s.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DQC25S](https://www.netlib.org/slatec/src/dqc25s.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Double precision Function subprogram defining the integrand The actual name for F needs to be declared E X T E R N A L  in the driver program. I/(B-A)) |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a part of (A,B). Double precision Left end point of the original interval |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a part of (A,B). Double precision Right end point of the original interval, B.GT.A X) |
| 4 | `BL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a part of (A,B). Double precision Lower limit of integration, BL.GE.A |
| 5 | `BR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a part of (A,B). Double precision Upper limit of integration, BR.LE.B |
| 6 | `ALFA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision PARAMETER IN THE WEIGHT FUNCTION |
| 7 | `BETA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Parameter in the weight function |
| 8 | `RI` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (25) | Double precision Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) |
| 9 | `RJ` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (25) | Double precision Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) |
| 10 | `RG` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (25) | Double precision Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) |
| 11 | `RH` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (25) | Double precision Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO) |
| 12 | `RESULT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Approximation to the integral is computed by using a generalized CLENSHAW-CURTIS method if B1 = A or BR = B. in all other cases the 15-POINT KRONROD RULE is applied, obtained by optimal addition of Abscissae to the 7-POINT GAUSS RULE. |
| 13 | `ABSERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) |
| 14 | `RESASC` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision |
| 15 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Which determines the weight function = 1   W(X) = (X-A)**ALFA*(B-X)**BETA = 2   W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3   W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4   W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(X-A)* |
| 16 | `NEV` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::dqc25s`. Native symbol: `dqc25s_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64,mut_f64,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqc25s`
- Public declaration feature: `quadrature-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
