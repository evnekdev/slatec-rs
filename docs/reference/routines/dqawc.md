# DQAWC

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a Cauchy principal value I = INTEGRAL of F*W over (A,B) (W(X) = 1/((X-C), C.NE.A, C.NE.B), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABE,EPSREL*ABS(I)).

## Description

Computation of a Cauchy principal value Standard fortran subroutine Double precision version PARAMETERS ON ENTRY

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
- GAMS classifications: `H2A2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_principal_value`

## Providers

- Canonical provider: `main-src/src/dqawc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqawc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqawc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqawc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DQAWC](https://www.netlib.org/slatec/src/dqawc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Double precision Function subprogram defining the integrand Function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Under limit of integration LIMIT.GE.1. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Upper limit of integration LIMIT.GE.1. |
| 4 | `C` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameter in the weight function, C.NE.A, C.NE.B. A or C = B, the routine will end with A or C = B or |
| 5 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Absolute accuracy requested and |
| 6 | `EPSREL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.1 or LENW.LT.LIMIT*4. |
| 7 | `RESULT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Approximation to the integral are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and |
| 8 | `ABSERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Estimate or the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and |
| 9 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and |
| 10 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 6 . 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine the estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 2 The occurrence of roundoff error is detec- ted, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because 6. 6. |
| 11 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval 6. form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), LAST otherwise ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. |
| 12 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end with |
| 13 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B), |
| 14 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), |
| 15 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), Double precision Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`IWORK`: Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2),

`WORK`: are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), Double precision Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dqawc`. Native symbol: `dqawc_`. Declaration feature: `quadrature-basic`. Provider feature: `quadrature-basic`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqawc`
- Public declaration feature: `quadrature-basic`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_principal_value`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
