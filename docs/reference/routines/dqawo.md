# DQAWO

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate an approximation to a given definite integral I= Integral of F(X)*W(X) over (A,B), where W(X) = COS(OMEGA*X) or W(X) = SIN(OMEGA*X), hopefully satisfying the following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of oscillatory integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration OMEGA - Double precision Parameter in the integrand weight function INTEGR - Integer Indicates which of the weight functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) If INTEGR.NE.1.AND.INTEGR.NE.2, the routine will end with IER = 6. EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision

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
- Safe Rust paths: `slatec::quadrature::integrate_oscillatory`

## Providers

- Canonical provider: `main-src/src/dqawo.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqawo.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqawo.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqawo.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DQAWO](https://www.netlib.org/slatec/src/dqawo.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `OMEGA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `EPSREL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `RESULT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `ABSERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 12 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `MAXP1` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 14 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 15 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 16 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 17 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

which should equal or exceed ABS(I-RESULT) NEVAL  - Integer Number of integrand evaluations IER    - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 Abnormal termination of the routine. less reliable. It is assumed that the requested accuracy has not been achieved. IER = 1 Maximum number of subdivisions allowed has been achieved (= LENIW/2). One can allow more subdivisions by increasing the value of LENIW (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used which is designed for handling the type of difficulty involved. detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some interior points of the integration interval. = 4 The algorithm does not converge. extrapolation table. It is presumed that the requested tolerance cannot be achieved due to roundoff in the extrapolation table, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or (INTEGR.NE.1 AND INTEGR.NE.2), or LENIW.LT.2 OR MAXP1.LT.1 or LENW.LT.LENIW*2+MAXP1*25. RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LENIW, MAXP1 or LENW are invalid, WORK(LIMIT*2+1), WORK(LIMIT*3+1), IWORK(1), IWORK(LIMIT+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS LENIW  - Integer Dimensioning parameter for IWORK. LENIW/2 equals the maximum number of subintervals allowed in the partition of the given integration interval (A,B), LENIW.GE.2. If LENIW.LT.2, the routine will end with IER = 6. MAXP1  - Integer Gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the intervals of lengths ABS(B-A)*2**(-L), L=0,1, ..., MAXP1-2, MAXP1.GE.1 If MAXP1.LT.1, the routine will end with IER = 6. LENW   - Integer Dimensioning parameter for WORK LENW must be at least LENIW*2+MAXP1*25. If LENW.LT.(LENIW*2+MAXP1*25), the routine will end with IER = 6. LAST   - Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. WORK ARRAYS IWORK  - Integer Vector of dimension at least LENIW on return, the first K elements of which contain subintervals, such that WORK(LIMIT*3+IWORK(1)), .. WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with LIMIT = LENW/2 , and K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise. Furthermore, IWORK(LIMIT+1), ..., IWORK(LIMIT+ LAST) indicate the subdivision levels of the subintervals, such that IWORK(LIMIT+I) = L means that the subinterval numbered I is of length ABS(B-A)*2**(1-L). WORK   - Double precision Vector of dimension at least LENW On return WORK(1), ..., WORK(LAST) contain the left end points of the subintervals in the partition of (A,B), WORK(LIMIT+1), ..., WORK(LIMIT+LAST) contain the right end points, WORK(LIMIT*2+1), ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, WORK(LIMIT*3+1), ..., WORK(LIMIT*3+LAST) WORK(LIMIT*4+1), ..., WORK(LIMIT*4+MAXP1*25) Provide space for storing the Chebyshev moments. Note that LIMIT = LENW/2.

### Storage and workspace requirements

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`WORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dqawo`. Native symbol: `dqawo_`. Declaration feature: `quadrature-oscillatory`. Provider feature: `quadrature-oscillatory`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqawo`
- Public declaration feature: `quadrature-oscillatory`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_oscillatory`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
