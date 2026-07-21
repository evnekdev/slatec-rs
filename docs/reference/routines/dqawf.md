# DQAWF

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given Fourier integral I=Integral of F(X)*W(X) over (A,INFINITY) where W(X) = COS(OMEGA*X) or W(X) = SIN(OMEGA*X). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.EPSABS.

## Description

Computation of Fourier integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY

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

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DQAWF](https://www.netlib.org/slatec/src/dqawf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Lower limit of integration 1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), FOR K = 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. |
| 3 | `OMEGA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Parameter in the integrand WEIGHT function 0 and INTEGR = 1, The integral is calculated by means of DQAGIE, 0, then LST is set to 1. |
| 4 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Indicates which of the WEIGHT functions is used 1      W(X) = COS(OMEGA*X) 2      W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine AND INTEGR.NE.2) or |
| 5 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Absolute accuracy requested, EPSABS.GT.0. 6. ON RETURN or LIMLST.LT.1 or |
| 6 | `RESULT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Approximation to the integral are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and on this interval is the best which can be obtained. = 5 The integral over the K th cycle is probably divergent or slowly convergent. It must be noted that divergence can occur with any other value of |
| 7 | `ABSERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and |
| 8 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and |
| 9 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of cycles allowed has been achieved, i.e. of subintervals 1, it is advised to examine the array IWORK which contains the error flags on the cycles. = 6 The input is invalid because IWORK(1) (with meaning as described 6. 6. 6. 6. |
| 10 | `LIMLST` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | or MAXP1.LT.1 or Integer gives an upper bound on the number of cycles, LIMLST.GE.3. 6. ..., WORK(LIMLST+LST) contain the error estimates over the cycles. further elements of WORK have no specific meaning for the user. |
| 11 | `LST` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and is the number of cycles actually needed (see below). Integer On return, LST indicates the number of cycles actually needed for the integration. contain the integral approximations over the cycles, |
| 12 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | or MAXP1.LT.1 or LIMLST) /2) has been achieved on the K th cycle. = 2 Occurrence of roundoff error is detected and prevents the tolerance imposed on the K th cycle, from being achieved on this cycle. = 3 Extremely bad integrand behaviour occurs at some points of the K th cycle. = 4 The integration procedure over the K th cycle does not converge (to within the required accuracy) due to roundoff in the extrapolation procedure invoked on this cycle. It is assumed that the Integer Dimensioning parameter for IWORK. On entry, LIMLST)/2 equals the maximum number of subintervals allowed in the partition of each cycle, LENIW.GE.(LIMLST+2). If LENIW.LT.(LIMLST+2), the routine will end with |
| 13 | `MAXP1` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the intervals of lengths ABS(B-A)*2**(-L), 2, MAXP1.GE.1. 6. |
| 14 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Dimensioning parameter for WORK must be at least LENIW*2+MAXP1*25. If LENW.LT.(LENIW*2+MAXP1*25), the routine will |
| 15 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | is the number of cycles actually needed (see below). 1 The maximum number of 1). DIMENSIONING PARAMETERS Integer Vector of dimension at least LENIW 1, 2, ..., LST contain the error flags on the cycles. |
| 16 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | ARRAYS Double precision Vector of dimension at least On return, contain the integral contain the integral approximations over the cycles, approximations over the cycles, ..., WORK(LIMLST+LST) contain the error estimates over the cycles. further elements of WORK have no specific meaning for the user. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

If OMEGA.NE.0

### Storage and workspace requirements

`IWORK`: is the number of cycles actually needed (see below). 1 The maximum number of 1). DIMENSIONING PARAMETERS Integer Vector of dimension at least LENIW 1, 2, ..., LST contain the error flags on the cycles.

`WORK`: ARRAYS Double precision Vector of dimension at least On return, contain the integral contain the integral approximations over the cycles, approximations over the cycles, ..., WORK(LIMLST+LST) contain the error estimates over the cycles. further elements of WORK have no specific meaning for the user.

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
