# DQAG

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = integral of F over (A,B), hopefully satisfying following claim for accuracy ABS(I-RESULT)LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of a definite integral Standard fortran subroutine Double precision version F - Double precision Function subprogram defining the integrand Function F(X). The actual name for F needs to be Declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. KEY - Integer Key for choice of local integration rule A GAUSS-KRONROD PAIR is used with 7 - 15 POINTS If KEY.LT.2, 10 - 21 POINTS If KEY = 2, 15 - 31 POINTS If KEY = 3, 20 - 41 POINTS If KEY = 4, 25 - 51 POINTS If KEY = 5, 30 - 61 POINTS If KEY.GT.5. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision

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
- Safe Rust paths: `slatec::quadrature::integrate`

## Providers

- Canonical provider: `main-src/src/dqag.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqag.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqag.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqag.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DQAG](https://www.netlib.org/slatec/src/dqag.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `EPSREL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `KEY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `RESULT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `ABSERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 11 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 12 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 14 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 15 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

Which should EQUAL or EXCEED ABS(I-RESULT) NEVAL  - Integer Number of integrand evaluations IER    - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine Less reliable. It is assumed that the requested accuracy has not been achieved. IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). HOWEVER, If this yield no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (I.E. SINGULARITY, DISCONTINUITY WITHIN THE INTERVAL) One will probably gain from splitting up the interval at this point and calling the INTEGRATOR on the SUBRANGES. If possible, AN APPROPRIATE SPECIAL-PURPOSE INTEGRATOR should be used which is designed for handling the type of difficulty involved. detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because (EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero. EXCEPT when LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS LIMIT - Integer Dimensioning parameter for IWORK Limit determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.1. If LIMIT.LT.1, the routine will end with IER = 6. LENW  - Integer Dimensioning parameter for work LENW must be at least LIMIT*4. IF LENW.LT.LIMIT*4, the routine will end with IER = 6. LAST  - Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. WORK ARRAYS IWORK - Integer Vector of dimension at least limit, the first K estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),... , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence with K = LAST If LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise WORK  - Double precision Vector of dimension at least LENW on return WORK(1), ..., WORK(LAST) contain the left end points of the subintervals in the partition of (A,B), WORK(LIMIT+1), ..., WORK(LIMIT+LAST) contain the right end points, WORK(LIMIT*2+1), ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, WORK(LIMIT*3+1), ..., WORK(LIMIT*3+LAST) contain

### Storage and workspace requirements

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`WORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dqag`. Native symbol: `dqag_`. Declaration feature: `quadrature-basic`. Provider feature: `quadrature-basic`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqag`
- Public declaration feature: `quadrature-basic`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
