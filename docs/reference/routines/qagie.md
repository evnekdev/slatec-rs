# QAGIE

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given integral I = Integral of F over (BOUND,+INFINITY) or I = Integral of F over (-INFINITY,BOUND) or I = Integral of F over (-INFINITY,+INFINITY), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I))

## Description

Integration over infinite intervals Standard fortran subroutine F - Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. BOUND - Real Finite bound of integration range (has no meaning if interval is doubly-infinite) INF - Real Indicating the kind of integration range involved INF = 1 corresponds to (BOUND,+INFINITY), INF = -1 to (-INFINITY,BOUND), INF = 2 to (-INFINITY,+INFINITY). EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 Abnormal termination of the routine. The estimates for result and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is assumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), RESULT, ABSERR, NEVAL, LAST, RLIST(1), ELIST(1) and IORD(1) are set to zero. ALIST(1) and BLIST(1) are set to 0 and 1 respectively. ALIST - Real Vector of dimension at least LIMIT, the first LAST elements of which are the left

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

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F` | callback | `REAL` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Integration over infinite intervals Standard fortran subroutine F - Real Function subprogram defining the integrand function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BOUND` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | BOUND - Real Finite bound of integration range (has no meaning if interval is doubly-infinite) INF - Real Indicating the kind of integration range involved INF = 1 corresponds to (BOUND,+INFINITY), INF = -1 to (-INFINITY,BOUND), INF = 2 to (-INFINITY,+INFINITY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INF` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | BOUND - Real Finite bound of integration range (has no meaning if interval is doubly-infinite) INF - Real Indicating the kind of integration range involved INF = 1 corresponds to (BOUND,+INFINITY), INF = -1 to (-INFINITY,BOUND), INF = 2 to (-INFINITY,+INFINITY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSABS` | output | `REAL` (`explicit`) | `*mut f32` | scalar | EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSREL` | output | `REAL` (`explicit`) | `*mut f32` | scalar | EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIMIT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | output | `REAL` (`explicit`) | `*mut f32` | scalar | LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | output | `REAL` (`explicit`) | `*mut f32` | scalar | LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEVAL` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.1 ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ALIST(1) and BLIST(1) are set to 0 and 1 respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BLIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ALIST(1) and BLIST(1) are set to 0 and 1 respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RLIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), RESULT, ABSERR, NEVAL, LAST, RLIST(1), ELIST(1) and IORD(1) are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), RESULT, ABSERR, NEVAL, LAST, RLIST(1), ELIST(1) and IORD(1) are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IORD` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), RESULT, ABSERR, NEVAL, LAST, RLIST(1), ELIST(1) and IORD(1) are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LAST` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), RESULT, ABSERR, NEVAL, LAST, RLIST(1), ELIST(1) and IORD(1) are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::callbacks::qagie`. Native symbol: `qagie_`. Feature: `quadrature-callbacks`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qagie`
- Compatibility aliases: `none`
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
