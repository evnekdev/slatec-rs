# QAWF

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given Fourier integral I = Integral of F(X)*W(X) over (A,INFINITY) where W(X) = COS(OMEGA*X) or W(X) = SIN(OMEGA*X). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.EPSABS.

## Description

Computation of Fourier integrals Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Real Lower limit of integration OMEGA - Real Parameter in the integrand WEIGHT function INTEGR - Integer Indicates which of the WEIGHT functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine will end with IER = 6. EPSABS - Real Absolute accuracy requested, EPSABS.GT.0. If EPSABS.LE.0, the routine will end with IER = 6. ON RETURN RESULT - Real Approximation to the integral ABSERR - Real

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
- GAMS classifications: `H2A3A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_fourier_tail_f32`

## Providers

- Canonical provider: `main-src/src/qawf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qawf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qawf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qawf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [QAWF](https://www.netlib.org/slatec/src/qawf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `OMEGA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `RESULT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `ABSERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 10 | `LIMLST` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `LST` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 12 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `MAXP1` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 14 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 15 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 16 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

Which should equal or exceed ABS(I-RESULT) NEVAL  - Integer Number of integrand evaluations IER    - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. less reliable. It is assumed that the requested accuracy has not been achieved. If OMEGA.NE.0 IER = 1 Maximum number of cycles allowed has been achieved, i.e. of subintervals (A+(K-1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), FOR K = 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. As in the case of IER = 1, it is advised to examine the array IWORK which contains = 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). RESULT, ABSERR, NEVAL, LST are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and is detected and prevents the tolerance imposed on the K th cycle, from being achieved on this cycle. = 3 Extremely bad integrand behaviour occurs at some points of the K th cycle. = 4 The integration procedure over the K th cycle does not converge (to within the required accuracy) due to roundoff in the extrapolation procedure invoked on this cycle. It is assumed that the result on this interval is the best which can be obtained. = 5 The integral over the K th cycle is probably divergent or slowly convergent. It must be noted that divergence can occur with any other value of IWORK(K). If OMEGA = 0 and INTEGR = 1, The integral is calculated by means of DQAGIE, and IER = IWORK(1) (with meaning as described for IWORK(K),K = 1). DIMENSIONING PARAMETERS LIMLST - Integer LIMLST gives an upper bound on the number of cycles, LIMLST.GE.3. If LIMLST.LT.3, the routine will end with IER = 6. LST    - Integer On return, LST indicates the number of cycles actually needed for the integration. If OMEGA = 0, then LST is set to 1. LENIW  - Integer Dimensioning parameter for IWORK. On entry, (LENIW-LIMLST)/2 equals the maximum number of subintervals allowed in the partition of each cycle, LENIW.GE.(LIMLST+2). If LENIW.LT.(LIMLST+2), the routine will end with IER = 6. MAXP1  - Integer MAXP1 gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the intervals of lengths ABS(B-A)*2**(-L), L = 0,1, ..., MAXP1-2, MAXP1.GE.1. If MAXP1.LT.1, the routine will end with IER = 6. LENW   - Integer Dimensioning parameter for WORK LENW must be at least LENIW*2+MAXP1*25. If LENW.LT.(LENIW*2+MAXP1*25), the routine will end with IER = 6. WORK ARRAYS IWORK  - Integer Vector of dimension at least LENIW On return, IWORK(K) FOR K = 1, 2, ..., LST WORK   - Real Vector of dimension at least On return, WORK(1), ..., WORK(LST) contain the integral approximations over the cycles, WORK(LIMLST+1), ..., WORK(LIMLST+LST) contain further elements of WORK have no specific meaning for the user.

### Storage and workspace requirements

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`WORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::qawf`. Native symbol: `qawf_`. Declaration feature: `quadrature-fourier`. Provider feature: `quadrature-fourier`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::qawf`
- Public declaration feature: `quadrature-fourier`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_fourier_tail_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
