# QAGSE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = Integral of F over (A,B), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of a definite integral Standard fortran subroutine Real version

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
- GAMS classifications: `H2A1A1`
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

- Canonical provider: `main-src/src/qagse.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qagse.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qagse.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qagse.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [QAGSE](https://www.netlib.org/slatec/src/qagse.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Lower limit of integration. |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Upper limit of integration. |
| 4 | `EPSABS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Absolute accuracy requested. |
| 5 | `EPSREL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Relative accuracy requested If EPSABS. LE. 0 and EPSREL. LT. MAX(50*REL. MACH. |
| 6 | `LIMIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Gives an upper bound on the number of subintervals in the partition of (A,B). |
| 7 | `RESULT` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Approximation to the integral. |
| 8 | `ABSERR` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT). |
| 9 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of integrand evaluations. |
| 10 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER. GT. 0 Abnormal termination of the routine the estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| 11 | `ALIST` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 12 | `BLIST` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 13 | `RLIST` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 14 | `ELIST` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first. |
| 15 | `IORD` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, such that ELIST(IORD(1)),. , ELIST(IORD(K)) form a decreasing sequence, with K = LAST If LAST. LE. (LIMIT/2+2), and K = LIMIT+1-LAST otherwise. |
| 16 | `LAST` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals Integer Number of subintervals actually produced in the subdivision process. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. |
| `IER` | `>0` | Abnormal termination of the routine the estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. |
| `IER` | `1` | 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. |
| `IER` | `2` | 2 The occurrence of roundoff error is detec- ted, which prevents the requested tolerance from being achieved. The error may be under-estimated. |
| `IER` | `3` | 3 Extremely bad integrand behaviour occurs at some points of the integration interval. |
| `IER` | `4` | 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. |
| `IER` | `5` | 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. |
| `IER` | `6` | 6 The input is invalid, because EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28). RESULT, ABSERR, NEVAL, LAST, RLIST(1), IORD(1) and ELIST(1) are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::qagse`. Native symbol: `qagse_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qagse`
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
