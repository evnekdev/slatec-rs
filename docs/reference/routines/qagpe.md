# QAGPE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Approximate a given definite integral I = Integral of F over (A,B), hopefully satisfying the accuracy claim: ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)). Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user.

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
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qagpe.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qagpe.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qagpe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qagpe.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [QAGPE](https://www.netlib.org/slatec/src/qagpe.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Integral of F Real Lower limit of integration provided break point or integration limit, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L). LAST |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Upper limit of integration |
| 4 | `NPTS2` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. 6. 2) 2) elements of which are the user provided break elements of which are the user provided break or Break points are specified outside the integration range or 2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. If this happens for the subinterval numbered K, NDIN(K) is put to 1, otherwise |
| 5 | `POINTS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real If these POINTS do not constitute an ascending sequence there will be an automatic sorting. |
| 6 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user. Real Absolute accuracy requested and |
| 7 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user. Real Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.NPTS2. |
| 8 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.NPTS2 If LIMIT.LT.NPTS2, the routine will end with (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs At some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because LAST LAST otherwise otherwise |
| 9 | `RESULT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user. Real Approximation to the integral ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. ALIST(1) and |
| 10 | `ABSERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) |
| 11 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations |
| 12 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of |
| 13 | `ALIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first |
| 14 | `BLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are set to A and B respectively. Real Vector of dimension at least LIMIT, the first |
| 15 | `RLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first |
| 16 | `ELIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first |
| 17 | `PTS` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. |
| 18 | `IORD` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the |
| 19 | `LEVEL` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Integer Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as |
| 20 | `NDIN` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Integer Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), 0. |
| 21 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals LAST otherwise Integer Number of subintervals actually produced in the subdivisions process |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

such that ELIST(IORD(1)), ..., ELIST(IORD(K))

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::qagpe`. Native symbol: `qagpe_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qagpe`
- Public declaration feature: `quadrature-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
