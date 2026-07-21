# QAGIE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given integral I = Integral of F over (BOUND,+INFINITY) or I = Integral of F over (-INFINITY,BOUND) or I = Integral of F over (-INFINITY,+INFINITY), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I))

## Description

Integration over infinite intervals Standard fortran subroutine

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
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qagie.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qagie.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qagie.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qagie.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [QAGIE](https://www.netlib.org/slatec/src/qagie.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | INFINITY,BOUND) INFINITY,+INFINITY), hopefully satisfying following claim for accuracy Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `BOUND` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Finite bound of integration range (has no meaning if interval is doubly-infinite) |
| 3 | `INF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Real Indicating the kind of integration range involved 1 corresponds to  (BOUND,+INFINITY), 1            to  (-INFINITY,BOUND), INFINITY,+INFINITY). |
| 4 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Absolute accuracy requested and |
| 5 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Relative accuracy requested If  EPSABS.LE.0 28), 28), |
| 6 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN (and taking the according dimension adjustments into account).  However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is assumed that the requested tolerance cannot be achieved, and that the returned LAST LAST otherwise otherwise |
| 7 | `RESULT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Approximation to the integral is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because ABSERR, NEVAL, LAST, RLIST(1), |
| 8 | `ABSERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) |
| 9 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations |
| 10 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 6. Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 Abnormal termination of the routine. The estimates for result and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of |
| 11 | `ALIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are set to 0 and 1 respectively. Real Vector of dimension at least LIMIT, the first |
| 12 | `BLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are set to 0 and 1 respectively. Real Vector of dimension at least LIMIT, the first |
| 13 | `RLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first |
| 14 | `ELIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are set to zero. Real Vector of dimension at least LIMIT,  the first |
| 15 | `IORD` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | are set to zero. Integer Vector of dimension LIMIT, the first K elements of which are pointers to the |
| 16 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | elements of which are the left end points of the subintervals in the partition of the transformed integration range (0,1). elements of which are the right end points of the subintervals in the partition of the transformed integration range (0,1). elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals LAST otherwise Integer Number of subintervals actually produced in the subdivision process |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

such that ELIST(IORD(1)), ..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::qagie`. Native symbol: `qagie_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qagie`
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
