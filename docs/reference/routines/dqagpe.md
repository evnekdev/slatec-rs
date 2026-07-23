# DQAGPE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Approximate a given definite integral I = Integral of F over (A,B), hopefully satisfying the accuracy claim: ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)). Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user.

## Description

Computation of a definite integral Standard fortran subroutine Double precision version

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
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dqagpe.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqagpe.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqagpe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqagpe.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DQAGPE](https://www.netlib.org/slatec/src/dqagpe.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Lower limit of integration. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Upper limit of integration. |
| 4 | `NPTS2` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number equal to two more than the number of user-supplied break points within the integration range, NPTS2. GE. 2. If NPTS2. LT. 2, the routine will end with IER = 6. |
| 5 | `POINTS` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension NPTS2, the first (NPTS2-2) elements of which are the user provided break POINTS. If these POINTS do not constitute an ascending sequence there will be an automatic sorting. |
| 6 | `EPSABS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Absolute accuracy requested. |
| 7 | `EPSREL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Relative accuracy requested If EPSABS. LE. 0 and EPSREL. LT. MAX(50*REL. MACH. |
| 8 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT. GE. NPTS2 If LIMIT. LT. NPTS2, the routine will end with IER = 6. |
| 9 | `RESULT` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Approximation to the integral. |
| 10 | `ABSERR` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT). |
| 11 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of integrand evaluations. |
| 12 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine. The estimates for integral and error are less reliable. |
| 13 | `ALIST` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 14 | `BLIST` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 15 | `RLIST` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 16 | `ELIST` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 17 | `PTS` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. |
| 18 | `IORD` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, such that ELIST(IORD(1)),. , ELIST(IORD(K)) form a decreasing sequence, with K = LAST If LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise. |
| 19 | `LEVEL` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i. e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration limit, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L). |
| 20 | `NDIN` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), I = 0,1,. , NPTS2-2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. If this happens for the subinterval numbered K, NDIN(K) is put to 1, otherwise NDIN(K) = 0. |
| 21 | `LAST` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals Integer Number of subintervals actually produced in the subdivisions process. |

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
| `IER` | `>0` | Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. |
| `IER` | `2` | 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs At some points of the integration interval. |
| `IER` | `4` | 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. |
| `IER` | `5` | 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value |
| `IER` | `>0` | . |
| `IER` | `6` | 6 The input is invalid because NPTS2.LT.2 or Break points are specified outside the integration range or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or LIMIT.LT.NPTS2. RESULT, ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::dqagpe`. Native symbol: `dqagpe_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64,mut_f64,mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqagpe`
- Public declaration feature: `quadrature-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
