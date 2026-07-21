# QAWS

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = Integral of F*W over (A,B), (where W shows a singular behaviour at the end points see parameter INTEGR). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Integration of functions having algebraico-logarithmic end point singularities Standard fortran subroutine Real version PARAMETERS ON ENTRY

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
- Safe Rust paths: `slatec::quadrature::integrate_weighted_endpoints_f32`

## Providers

- Canonical provider: `main-src/src/qaws.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qaws.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qaws.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qaws.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [QAWS](https://www.netlib.org/slatec/src/qaws.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Lower limit of integration 6. 1) or BETA.LE.(-1) or or INTEGR.LT.1 or INTEGR.GT.4 or |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Upper limit of integration, B.GT.A 6. 1) or BETA.LE.(-1) or or INTEGR.LT.1 or INTEGR.GT.4 or |
| 4 | `ALFA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real 1) 1), the routine will end with 1) or BETA.LE.(-1) or or INTEGR.LT.1 or INTEGR.GT.4 or |
| 5 | `BETA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real 1) 1), the routine will end with |
| 6 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Indicates which WEIGHT function is to be used = 1  (X-A)**ALFA*(B-X)**BETA = 2  (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3  (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4  (X-A)**ALFA*(B-X)**BETA*LOG(X-A)*LOG(B-X) If INTEGR.LT.1 or INTEGR.GT.4, the routine |
| 7 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Absolute accuracy requested and |
| 8 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.2 or LENW.LT.LIMIT*4. |
| 9 | `RESULT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Approximation to the integral are set to |
| 10 | `ABSERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) are set to |
| 11 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations are set to |
| 12 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 6. 6. 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine The estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of 6. 6. |
| 13 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand, in order to determine the integration difficulties which prevent the requested tolerance from being achieved. In case of a jump discontinuity or a local singularity of algebraico-logarithmic type at one or more interior points of the integration range, one should proceed by splitting up the interval at these points and calling the integrator on the subranges. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because is invalid are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.2. 6. form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), LAST otherwise ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. |
| 14 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is invalid Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end |
| 15 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are set to Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the significant number of elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B), |
| 16 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | WORK(LIMIT*2+1) and Integer Vector of dimension LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ..., form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), |
| 17 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), Real Vector of dimension LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`IWORK`: WORK(LIMIT*2+1) and Integer Vector of dimension LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ..., form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2),

`WORK`: are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), Real Vector of dimension LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::qaws`. Native symbol: `qaws_`. Declaration feature: `quadrature-weighted`. Provider feature: `quadrature-weighted`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::qaws`
- Public declaration feature: `quadrature-weighted`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_weighted_endpoints_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
