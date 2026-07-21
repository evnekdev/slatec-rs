# QAWCE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a CAUCHY PRINCIPAL VALUE I = Integral of F*W over (A,B) (W(X) = 1/(X-C), (C.NE.A, C.NE.B), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I))

## Description

Computation of a CAUCHY PRINCIPAL VALUE Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Real Lower limit of integration B - Real Upper limit of integration C - Real Parameter in the WEIGHT function, C.NE.A, C.NE.B If C = A OR C = B, the routine will end with IER = 6. EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN RESULT - Real Approximation to the integral ABSERR - Real

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

- Canonical provider: `main-src/src/qawce.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qawce.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qawce.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qawce.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [QAWCE](https://www.netlib.org/slatec/src/qawce.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `RESULT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `ABSERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `NEVAL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 12 | `ALIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 13 | `BLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 14 | `RLIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 15 | `ELIST` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 16 | `IORD` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 17 | `LAST` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

which should equal or exceed ABS(I-RESULT) NEVAL  - Integer Number of integrand evaluations IER    - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine less reliable. It is assumed that the requested accuracy has not been achieved. IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of LIMIT. However, if this yields no improvement it is advised to analyze the the integrand, in order to determine the the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. ted, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some interior points of the integration interval. = 6 The input is invalid, because C = A or C = B or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or LIMIT.LT.1. RESULT, ABSERR, NEVAL, RLIST(1), ELIST(1), IORD(1) and LAST are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. ALIST   - Real Vector of dimension at least LIMIT, the first LAST  elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) BLIST   - Real Vector of dimension at least LIMIT, the first LAST  elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) RLIST   - Real Vector of dimension at least LIMIT, the first LAST  elements of which are the integral approximations on the subintervals ELIST   - Real Vector of dimension LIMIT, the first  LAST elements of which are the moduli of the absolute IORD    - Integer Vector of dimension at least LIMIT, the first K estimates over the subintervals, so that ELIST(IORD(1)), ..., ELIST(IORD(K)) with K = LAST If LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise, form a decreasing sequence LAST    - Integer Number of subintervals actually produced in the subdivision process

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::qawce`. Native symbol: `qawce_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qawce`
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
