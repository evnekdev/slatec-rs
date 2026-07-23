# DQAWF

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given Fourier integral I=Integral of F(X)*W(X) over (A,INFINITY) where W(X) = COS(OMEGA*X) or W(X) = SIN(OMEGA*X). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.EPSABS.

## Description

Computation of Fourier integrals Standard fortran subroutine Double precision version

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
- GAMS classifications: `H2A3A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_fourier_tail`

## Providers

- Canonical provider: `main-src/src/dqawf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqawf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqawf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqawf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DQAWF](https://www.netlib.org/slatec/src/dqawf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Lower limit of integration. |
| 3 | `OMEGA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameter in the integrand WEIGHT function. |
| 4 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates which of the WEIGHT functions is used 1 W(X) = COS(OMEGA*X) 2 W(X) = SIN(OMEGA*X) IF INTEGR. NE. 1. AND. INTEGR. 2, the routine will end with IER = 6. |
| 5 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Absolute accuracy requested, EPSABS. GT. 0. If EPSABS. LE. 0, the routine will end with IER = 6. |
| 6 | `RESULT` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Approximation to the integral. |
| 7 | `ABSERR` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT). |
| 8 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of integrand evaluations. |
| 9 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine. The estimates for integral and error are less reliable. |
| 10 | `LIMLST` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | gives an upper bound on the number of cycles, LIMLST. GE. 3. If LIMLST. LT. 3, the routine will end with IER = 6. |
| 11 | `LST` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | On return, LST indicates the number of cycles actually needed for the integration. If OMEGA = 0, then LST is set to 1. |
| 12 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. |
| 13 | `MAXP1` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | gives an upper bound on the number of Chebyshev moments which can be stored, i. e. for the intervals of lengths ABS(B-A)*2**(-L), L = 0,1,. , MAXP1-2, MAXP1. GE. 1. |
| 14 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be at least LENIW*2+MAXP1*25. If LENW. LT. (LENIW*2+MAXP1*25), the routine will end with IER = 6. |
| 15 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Vector of dimension at least LENIW On return, IWORK(K) FOR K = 1, 2,. , LST contain the error flags on the cycles. |
| 16 | `WORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension at least. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. If OMEGA.NE.0 |
| `IER` | `1` | 1 Maximum number of cycles allowed has been achieved, i.e. of subintervals (A+(K-1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. |
| `IER` | `4` | 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. |
| `IER` | `1` | 1, it is advised to examine the array IWORK which contains the error flags on the cycles. |
| `IER` | `6` | 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). RESULT, ABSERR, NEVAL, LST are set to zero. |
| `IER` | `7` | 7 Bad integrand behaviour occurs within one or more of the cycles. Location and type of the difficulty involved can be determined from the first LST elements of vector IWORK. Here LST is the number of cycles actually needed (see below). |
| `IER` | `1` | 1 The maximum number of subdivisions (=(LENIW-LIMLST) /2) has been achieved on the K th cycle. |
| `IER` | `2` | 2 Occurrence of roundoff error is detected and prevents the tolerance imposed on the K th cycle, from being achieved on this cycle. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the K th cycle. |
| `IER` | `4` | 4 The integration procedure over the K th cycle does not converge (to within the required accuracy) due to roundoff in the extrapolation procedure invoked on this cycle. It is assumed that the result on this interval is the best which can be obtained. |
| `IER` | `5` | 5 The integral over the K th cycle is probably divergent or slowly convergent. It must be noted that divergence can occur with any other value of IWORK(K). |
| `IER` | `0` | 0 and INTEGR = 1, The integral is calculated by means of DQAGIE, and IER = IWORK(1) (with meaning as described |
| `IER` | `1` | 1). |

### Storage and workspace requirements

`IWORK`: Integer Vector of dimension at least LENIW On return, IWORK(K) FOR K = 1, 2, ..., LST contain the error flags on the cycles.

`WORK`: Double precision Vector of dimension at least

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dqawf`. Native symbol: `dqawf_`. Declaration feature: `quadrature-fourier`. Provider feature: `quadrature-fourier`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqawf`
- Public declaration feature: `quadrature-fourier`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_fourier_tail`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
