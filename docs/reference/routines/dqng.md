# DQNG

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = integral of F over (A,B), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

NON-ADAPTIVE INTEGRATION STANDARD FORTRAN SUBROUTINE DOUBLE PRECISION VERSION F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral I Result is obtained by applying the 21-POINT GAUSS-KRONROD RULE (RES21) obtained by optimal addition of abscissae to the 10-POINT GAUSS RULE (RES10), or by applying the 43-POINT RULE (RES43) obtained by optimal addition of abscissae to the 21-POINT GAUSS-KRONROD RULE, or by applying the 87-POINT RULE (RES87) obtained by optimal addition of abscissae to the 43-POINT RULE. ABSERR - Double precision

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
- GAMS classifications: `H2A1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_non_adaptive`

## Providers

- Canonical provider: `main-src/src/dqng.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqng.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqng.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqng.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DQNG](https://www.netlib.org/slatec/src/dqng.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `EPSREL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `RESULT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `ABSERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

which should EQUAL or EXCEED ABS(I-RESULT) NEVAL  - Integer Number of integrand evaluations IER    - IER = 0 normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. It is assumed that the requested accuracy has not been achieved. IER = 1 The maximum number of steps has been executed. The integral is probably too difficult to be calculated by DQNG. = 6 The input is invalid, because EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28). RESULT, ABSERR and NEVAL are set to zero.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dqng`. Native symbol: `dqng_`. Declaration feature: `quadrature-nonadaptive`. Provider feature: `quadrature-nonadaptive`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqng`
- Public declaration feature: `quadrature-nonadaptive`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_non_adaptive`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
