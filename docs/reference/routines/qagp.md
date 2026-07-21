# QAGP

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = Integral of F over (A,B), hopefully satisfying following claim for accuracy break points of the integration interval, where local difficulties of the integrand may occur(e.g. SINGULARITIES, DISCONTINUITIES), are provided by the user.

## Description

Computation of a definite integral Standard fortran subroutine Real version PARAMETERS ON ENTRY

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

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [QAGP](https://www.netlib.org/slatec/src/qagp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Real Function subprogram defining the integrand Function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Lower limit of integration provided break point or integration LIMIT, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L), |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Upper limit of integration NPTS2)/2). DIMENSIONING PARAMETERS |
| 4 | `NPTS2` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS.GE.2. 6. 2) 2) elements of which are the user provided break elements of which are the user provided break or break points are specified outside the integration range or 2). 2), the routine will end with |
| 5 | `POINTS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real If these points do not constitute an ascending sequence there will be an automatic sorting. |
| 6 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Absolute accuracy requested and |
| 7 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Relative accuracy requested If  EPSABS.LE.0 28), 28)) |
| 8 | `RESULT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Approximation to the integral are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), |
| 9 | `ABSERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), |
| 10 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), |
| 11 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. The estimates for integral and error are less reliable. it is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. one can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned RESULT is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. it must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because 6. 6. |
| 12 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | NPTS2)/2). DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK NPTS2)/2, NPTS2)/2, which is the maximum number of subintervals in the which is the maximum number of subintervals in the partition of the given integration interval (A,B), partition of the given integration interval (A,B), 2). 2), the routine will end with NPTS2. NPTS2, the routine will end NPTS2)/2. NPTS2)/2. |
| 13 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Dimensioning parameter for WORK NPTS2. NPTS2, the routine will end |
| 14 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. Contain the subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) contain the left end points of the subintervals in the partition of (A,B), |
| 15 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Contain the Contain the subdivision levels of the subintervals, i.e. subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) if (AA,BB) is a subinterval of (P1,P2) ..., IWORK(LIMIT*2+NPTS2) have no significance for the user, |
| 16 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | and WORK(LIMIT*3+1) are set to zero. is set to A and WORK(LIMIT+1) ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Real Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the corresponding error estimates, ..., WORK(LIMIT*4+NPTS2) contain the integration limits and the break points sorted in an ascending sequence. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`IWORK`: Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Contain the Contain the subdivision levels of the subintervals, i.e. subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) if (AA,BB) is a subinterval of (P1,P2) ..., IWORK(LIMIT*2+NPTS2) have no significance for the user,

`WORK`: and WORK(LIMIT*3+1) are set to zero. is set to A and WORK(LIMIT+1) ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Real Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the corresponding error estimates, ..., WORK(LIMIT*4+NPTS2) contain the integration limits and the break points sorted in an ascending sequence.

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
