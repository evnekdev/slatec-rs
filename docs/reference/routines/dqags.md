# DQAGS

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given Definite integral I = Integral of F over (A,B), Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of a definite integral Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand Function F(X). The actual name for F needs to be Declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision

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
- Safe Rust paths: `slatec::quadrature::integrate_singular`

## Providers

- Canonical provider: `main-src/src/dqags.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqags.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqags.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqags.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DQAGS](https://www.netlib.org/slatec/src/dqags.f)

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
| 8 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 10 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 12 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 14 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

which should equal or exceed ABS(I-RESULT) NEVAL  - Integer Number of integrand evaluations IER    - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine less reliable. It is assumed that the requested accuracy has not been achieved. IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of LIMIT (and taking the according dimension adjustments into account. However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (E.G. SINGULARITY, DISCONTINUITY WITHIN THE INTERVAL) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. ted, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because (EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero.  Except when LIMIT or LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) TO B. DIMENSIONING PARAMETERS LIMIT - Integer DIMENSIONING PARAMETER FOR IWORK LIMIT determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.1. IF LIMIT.LT.1, the routine will end with IER = 6. LENW  - Integer DIMENSIONING PARAMETER FOR WORK LENW must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end with IER = 6. LAST  - Integer On return, LAST equals the number of subintervals produced in the subdivision process, determines the number of significant elements actually in the WORK Arrays. WORK ARRAYS IWORK - Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers such that WORK(LIMIT*3+IWORK(1)),... , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST IF LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise WORK  - Double precision Vector of dimension at least LENW on return WORK(1), ..., WORK(LAST) contain the left end-points of the subintervals in the partition of (A,B), WORK(LIMIT+1), ..., WORK(LIMIT+LAST) contain the right end-points, WORK(LIMIT*2+1), ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, WORK(LIMIT*3+1), ..., WORK(LIMIT*3+LAST)

### Storage and workspace requirements

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`WORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dqags`. Native symbol: `dqags_`. Declaration feature: `quadrature-basic`. Provider feature: `quadrature-basic`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqags`
- Public declaration feature: `quadrature-basic`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_singular`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
