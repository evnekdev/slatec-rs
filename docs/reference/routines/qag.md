# QAG

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = integral of F over (A,B), hopefully satisfying following claim for accuracy ABS(I-RESULT)LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of a definite integral Standard fortran subroutine Real version

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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
- Safe Rust paths: `slatec::quadrature::integrate_f32`

## Providers

- Canonical provider: `main-src/src/qag.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qag.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qag.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qag.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [QAG](https://www.netlib.org/slatec/src/qag.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Synchronous integrand callback. It receives one integration abscissa and returns the function value; it must remain valid for the call, must not retain the native pointer, and must not unwind through Fortran. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input lower limit of integration. |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input upper limit of integration. |
| 4 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input requested absolute integration accuracy. |
| 5 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input requested relative integration accuracy. When EPSABS is nonpositive, the source minimum relative tolerance applies. |
| 6 | `KEY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input selector for the local Gauss-Kronrod integration rule. |
| 7 | `RESULT` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Writable output approximation to the integral. |
| 8 | `ABSERR` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Writable output estimate of the absolute integration error; it is intended to bound abs(I-RESULT). |
| 9 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable output number of integrand evaluations. |
| 10 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable completion and error indicator. Its documented values are rendered in the status table. |
| 11 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input maximum number of subintervals and minimum required IWORK length. LIMIT must be at least 1. |
| 12 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared WORK length. LENW must be at least 4*LIMIT. |
| 13 | `LAST` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable output number of subintervals produced; it determines the significant entries in IWORK and WORK. |
| 14 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Writable integer workspace of at least LIMIT elements. Its leading entries order subinterval error estimates as specified by the selected source. |
| 15 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Writable real workspace of at least LENW elements. Its four LIMIT-strided segments hold left endpoints, right endpoints, subintegral estimates, and error estimates. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine The estimates for RESULT and ERROR are Less reliable. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). HOWEVER, If this yield no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (I.E. SINGULARITY, DISCONTINUITY WITHIN THE INTERVAL) One will probably gain from splitting up the interval at this point and calling the INTEGRATOR on the SUBRANGES. If possible, AN APPROPRIATE SPECIAL-PURPOSE INTEGRATOR should be used which is designed for handling the type of difficulty involved. |
| `IER` | `2` | 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the integration interval. |
| `IER` | `6` | 6 The input is invalid, because (EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero. EXCEPT when LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. 6. |

### Storage and workspace requirements

`IWORK`: Writable integer workspace of at least LIMIT elements. Its leading entries order subinterval error estimates as specified by the selected source.

`WORK`: Writable real workspace of at least LENW elements. Its four LIMIT-strided segments hold left endpoints, right endpoints, subintegral estimates, and error estimates.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::qag`. Native symbol: `qag_`. Declaration feature: `quadrature-basic`. Provider feature: `quadrature-basic`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::qag`
- Public declaration feature: `quadrature-basic`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
