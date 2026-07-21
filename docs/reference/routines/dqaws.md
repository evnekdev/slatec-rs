# DQAWS

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = Integral of F*W over (A,B), (where W shows a singular behaviour at the end points see parameter INTEGR). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Integration of functions having algebraico-logarithmic end point singularities Standard fortran subroutine Double precision version

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
- Safe Rust paths: `slatec::quadrature::integrate_weighted_endpoints`

## Providers

- Canonical provider: `main-src/src/dqaws.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqaws.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqaws.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqaws.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DQAWS](https://www.netlib.org/slatec/src/dqaws.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Lower limit of integration. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Upper limit of integration, B. GT. A If B. LE. A, the routine will end with IER = 6. |
| 4 | `ALFA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameter in the integrand function, ALFA. GT. (-1) If ALFA. LE. (-1), the routine will end with IER = 6. |
| 5 | `BETA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameter in the integrand function, BETA. GT. (-1) If BETA. LE. (-1), the routine will end with IER = 6. |
| 6 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates which WEIGHT function is to be used = 1 (X-A)**ALFA*(B-X)**BETA = 2 (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3 (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4 (X-A)**ALFA*(B-X)**BETA*LOG(X-A)*LOG(B-X) If INTEGR. LT. 1 or INTEGR. GT. 4, the routine will end with IER = 6. |
| 7 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Absolute accuracy requested. |
| 8 | `EPSREL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Relative accuracy requested If EPSABS. LE. 0 and EPSREL. LT. MAX(50*REL. MACH. |
| 9 | `RESULT` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Approximation to the integral. |
| 10 | `ABSERR` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT). |
| 11 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of integrand evaluations. |
| 12 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine The estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| 13 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. |
| 14 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be at least LIMIT*4. If LENW. LT. LIMIT*4, the routine will end with IER = 6. |
| 15 | `LAST` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | On return, LAST equals the number of subintervals produced in the subdivision process, which determines the significant number of elements actually in the WORK ARRAYS. |
| 16 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Vector of dimension LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),. , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence with K = LAST if LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise. |
| 17 | `WORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension LENW. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `6` | 6. |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine The estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand, in order to determine the integration difficulties which prevent the requested tolerance from being achieved. In case of a jump discontinuity or a local singularity of algebraico-logarithmic type at one or more interior points of the integration range, one should proceed by splitting up the interval at these points and calling the integrator on the subranges. |
| `IER` | `2` | 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the integration interval. |
| `IER` | `6` | 6 The input is invalid, because B.LE.A or ALFA.LE.(-1) or BETA.LE.(-1) or |
| `IER` | `>0` | or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or LIMIT.LT.2 or LENW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LENW or LIMIT is invalid IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. |

### Storage and workspace requirements

`IWORK`: Integer Vector of dimension LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ..., WORK(LIMIT*3+IWORK(K)) form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise

`WORK`: Double precision Vector of dimension LENW

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dqaws`. Native symbol: `dqaws_`. Declaration feature: `quadrature-weighted`. Provider feature: `quadrature-weighted`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqaws`
- Public declaration feature: `quadrature-weighted`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_weighted_endpoints`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
