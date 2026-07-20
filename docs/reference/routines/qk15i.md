# QK15I

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The original (infinite integration range is mapped onto the interval (0,1) and (A,B) is a part of (0,1). it is the purpose to compute I = Integral of transformed integrand over (A,B), J = Integral of ABS(Transformed Integrand) over (A,B).

## Description

Integration Rule Standard Fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand FUNCTION F(X). The actual name for F needs to be Declared E X T E R N A L in the calling program. BOUN - Real Finite bound of original integration Range (SET TO ZERO IF INF = +2) INF - Integer If INF = -1, the original interval is (-INFINITY,BOUND), If INF = +1, the original interval is (BOUND,+INFINITY), If INF = +2, the original interval is (-INFINITY,+INFINITY) AND The integral is computed as the sum of two integrals, one over (-INFINITY,0) and one over (0,+INFINITY). A - Real Lower limit for integration over subrange of (0,1) B - Real Upper limit for integration over subrange of (0,1) ON RETURN RESULT - Real Approximation to the integral I Result is computed by applying the 15-POINT KRONROD RULE(RESK) obtained by optimal addition of abscissae to the 7-POINT GAUSS RULE(RESG). ABSERR - Real Estimate of the modulus of the absolute error, WHICH SHOULD EQUAL or EXCEED ABS(I-RESULT) RESABS - Real Approximation to the integral J RESASC - Real Approximation to the integral of ABS((TRANSFORMED INTEGRAND)-I/(B-A)) over (A,B)

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
- GAMS classifications: `H2A3A2`
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

- Canonical provider: `main-src/src/qk15i.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qk15i.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qk15i.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qk15i.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `mangled_source_prologue`
- Description provenance: `source_prologue`
- Assessment: mechanical source-prologue checks found text that requires a documented repair or review
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F` | callback | `REAL` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Integration Rule Standard Fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand FUNCTION F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BOUN` | input | `REAL` (`explicit`) | `*mut f32` | scalar | BOUN - Real Finite bound of original integration Range (SET TO ZERO IF INF = +2) INF - Integer If INF = -1, the original interval is (-INFINITY,BOUND), If INF = +1, the original interval is (BOUND,+INFINITY), If INF = +2, the original interval is (-INFINITY,+INFINITY) AND The integral is computed as the sum of two integrals, one over (-INFINITY,0) and one over (0,+INFINITY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INF` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | BOUN - Real Finite bound of original integration Range (SET TO ZERO IF INF = +2) INF - Integer If INF = -1, the original interval is (-INFINITY,BOUND), If INF = +1, the original interval is (BOUND,+INFINITY), If INF = +2, the original interval is (-INFINITY,+INFINITY) AND The integral is computed as the sum of two integrals, one over (-INFINITY,0) and one over (0,+INFINITY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | The actual name for F needs to be Declared E X T E R N A L in the calling program. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Lower limit for integration over subrange of (0,1) B - Real Upper limit for integration over subrange of (0,1) ON RETURN RESULT - Real Approximation to the integral I Result is computed by applying the 15-POINT KRONROD RULE(RESK) obtained by optimal addition of abscissae to the 7-POINT GAUSS RULE(RESG). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | output | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Lower limit for integration over subrange of (0,1) B - Real Upper limit for integration over subrange of (0,1) ON RETURN RESULT - Real Approximation to the integral I Result is computed by applying the 15-POINT KRONROD RULE(RESK) obtained by optimal addition of abscissae to the 7-POINT GAUSS RULE(RESG). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | output | `REAL` (`explicit`) | `*mut f32` | scalar | ABSERR - Real Estimate of the modulus of the absolute error, WHICH SHOULD EQUAL or EXCEED ABS(I-RESULT) RESABS - Real Approximation to the integral J RESASC - Real Approximation to the integral of ABS((TRANSFORMED INTEGRAND)-I/(B-A)) over (A,B) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESABS` | output | `REAL` (`explicit`) | `*mut f32` | scalar | ABSERR - Real Estimate of the modulus of the absolute error, WHICH SHOULD EQUAL or EXCEED ABS(I-RESULT) RESABS - Real Approximation to the integral J RESASC - Real Approximation to the integral of ABS((TRANSFORMED INTEGRAND)-I/(B-A)) over (A,B) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESASC` | output | `REAL` (`explicit`) | `*mut f32` | scalar | ABSERR - Real Estimate of the modulus of the absolute error, WHICH SHOULD EQUAL or EXCEED ABS(I-RESULT) RESABS - Real Approximation to the integral J RESASC - Real Approximation to the integral of ABS((TRANSFORMED INTEGRAND)-I/(B-A)) over (A,B) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::callbacks::qk15i`. Native symbol: `qk15i_`. Feature: `quadrature-callbacks`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_i32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qk15i`
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
