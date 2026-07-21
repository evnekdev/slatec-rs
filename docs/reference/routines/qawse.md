# QAWSE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = Integral of F*W over (A,B), (where W shows a singular behaviour at the end points, see parameter INTEGR). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Integration of functions having algebraico-logarithmic end point singularities Standard fortran subroutine Real version PARAMETERS ON ENTRY

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

- Canonical provider: `main-src/src/qawse.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qawse.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qawse.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qawse.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [QAWSE](https://www.netlib.org/slatec/src/qawse.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Lower limit of integration 6. 1) or BETA.LE.(-1), or |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Upper limit of integration, B.GT.A 6. 1) or BETA.LE.(-1), or |
| 4 | `ALFA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real 1) 1), the routine will end with 1) or BETA.LE.(-1), or |
| 5 | `BETA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real 1) 1), the routine will end with |
| 6 | `INTEGR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Indicates which WEIGHT function is to be used = 1  (X-A)**ALFA*(B-X)**BETA = 2  (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3  (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4  (X-A)**ALFA*(B-X)**BETA*LOG(X-A)*LOG(B-X) If INTEGR.LT.1 or INTEGR.GT.4, the routine or INTEGR.GT.4, or |
| 7 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Absolute accuracy requested and |
| 8 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Real Relative accuracy requested If  EPSABS.LE.0 28), 28), or LIMIT.LT.2. |
| 9 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.2 6. ON RETURN However, if this yields no improvement, it is advised to analyze the integrand in order to determine the integration difficulties which prevent the requested tolerance from being achieved. In case of a jump DISCONTINUITY or a local SINGULARITY of algebraico-logarithmic type at one or more interior points of the integration range, one should proceed by splitting up the interval at these points and calling the integrator on the subranges. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because LAST LAST otherwise form a decreasing sequence otherwise form a decreasing sequence |
| 10 | `RESULT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Approximation to the integral ABSERR, NEVAL, RLIST(1), ELIST(1), |
| 11 | `ABSERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) |
| 12 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer Number of integrand evaluations |
| 13 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 6. 6. 6. 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine the estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| 14 | `ALIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first |
| 15 | `BLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first |
| 16 | `RLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first |
| 17 | `ELIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Real Vector of dimension at least LIMIT, the first LAST LAST |
| 18 | `IORD` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. Integer Vector of dimension at least LIMIT, the first K of which are pointers to the error estimates over the subintervals, so that LAST LAST |
| 19 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals LAST otherwise form a decreasing sequence Integer Number of subintervals actually produced in the subdivision process |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

= 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::qawse`. Native symbol: `qawse_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qawse`
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
