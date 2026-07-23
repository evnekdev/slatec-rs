# QAGP

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = Integral of F over (A,B), hopefully satisfying following claim for accuracy break points of the integration interval, where local difficulties of the integrand may occur(e.g. SINGULARITIES, DISCONTINUITIES), are provided by the user.

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
- GAMS classifications: `H2A2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_with_breakpoints_f32`

## Providers

- Canonical provider: `main-src/src/qagp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qagp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qagp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qagp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [QAGP](https://www.netlib.org/slatec/src/qagp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand Function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Lower limit of integration. |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Upper limit of integration. |
| 4 | `NPTS2` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number equal to two more than the number of user-supplied break points within the integration range, NPTS. GE. 2. If NPTS2. LT. 2, The routine will end with IER = 6. |
| 5 | `POINTS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Vector of dimension NPTS2, the first (NPTS2-2) elements of which are the user provided break points. If these points do not constitute an ascending sequence there will be an automatic sorting. |
| 6 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Absolute accuracy requested. |
| 7 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Relative accuracy requested If EPSABS. LE. 0 And EPSREL. LT. MAX(50*REL. MACH. |
| 8 | `RESULT` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Approximation to the integral. |
| 9 | `ABSERR` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT). |
| 10 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of integrand evaluations. |
| 11 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine. The estimates for integral and error are less reliable. |
| 12 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. |
| 13 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be at least LENIW*2-NPTS2. If LENW. LT. LENIW*2-NPTS2, the routine will end with IER = 6. |
| 14 | `LAST` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. |
| 15 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),. , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST if LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise IWORK(LIMIT+1),. ,IWORK(LIMIT+LAST) Contain the subdivision levels of the subintervals, i. |
| 16 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Vector of dimension at least LENW. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine. The estimates for integral and error are less reliable. it is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. one can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. |
| `IER` | `2` | 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the integration interval. |
| `IER` | `4` | 4 The algorithm does not converge. roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned RESULT is the best which can be obtained. |
| `IER` | `5` | 5 The integral is probably divergent, or slowly convergent. it must be noted that divergence can occur with any other value |
| `IER` | `>0` | . |
| `IER` | `6` | 6 The input is invalid because NPTS2.LT.2 or break points are specified outside the integration range or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) WORK(1) is set to A and WORK(LIMIT+1) to B (where LIMIT = (LENIW-NPTS2)/2). |

### Storage and workspace requirements

`IWORK`: Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise IWORK(LIMIT+1), ...,IWORK(LIMIT+LAST) Contain the subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration LIMIT, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L), IWORK(LIMIT*2+1), ..., IWORK(LIMIT*2+NPTS2) have no significance for the user, note that LIMIT = (LENIW-NPTS2)/2.

`WORK`: Real Vector of dimension at least LENW

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::qagp`. Native symbol: `qagp_`. Declaration feature: `quadrature-breakpoints`. Provider feature: `quadrature-breakpoints`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::qagp`
- Public declaration feature: `quadrature-breakpoints`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_with_breakpoints_f32`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
