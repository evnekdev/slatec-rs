# SINTRP

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Approximate the solution at XOUT by evaluating the polynomial computed in STEPS at XOUT. Must be used in conjunction with STEPS.

## Description

The methods in subroutine STEPS approximate the solution near X by a polynomial. Subroutine SINTRP approximates the solution at

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A1B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sintrp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sintrp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sintrp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [SINTRP](https://www.netlib.org/slatec/src/sintrp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input current integration abscissa from `DSTEPS`. It and the history arguments must be from the same unmodified `DSTEPS` state. |
| 2 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Readable current solution vector from `DSTEPS`, with at least `NEQN` elements. |
| 3 | `XOUT` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | by evaluating the polynomial there. Information defining this polynomial is passed from STEPS so SINTRP cannot be used alone. Subroutine STEPS is completely explained and documented in the text, "Computer Solution of Ordinary Differential Equations, the Initial Value Problem" by L. F. Shampine and M. K. |
| 4 | `YOUT` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | solution at XOUT. |
| 5 | `YPOUT` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | derivative of solution at XOUT The remaining parameters are returned unaltered from their input values. Integration with STEPS may be continued. |
| 6 | `NEQN` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input number of differential equations. It is the required length of `Y`, `YOUT`, and `YPOUT` and the first dimension of `PHI`. |
| 7 | `KOLD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input interpolation order saved by `DSTEPS`. It controls how many columns of the `PHI` history are used and must be passed unchanged from that integrator state. |
| 8 | `PHI` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NEQN, 16) | Readable `DSTEPS` history matrix with Fortran shape `(NEQN, 16)`. It defines the local interpolation polynomial and must not be synthesized independently. |
| 9 | `IVC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Readable interpolation-cache control index supplied by `DSTEPS`; it selects cached data in `IV` and `OW` when the order changes. |
| 10 | `IV` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (10) | Readable integer interpolation cache of length 10 supplied by `DSTEPS`. It is part of the persistent integrator state and must be passed unchanged. |
| 11 | `KGI` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Readable `DSTEPS` interpolation-history order marker used to decide whether cached `GI` values apply. |
| 12 | `GI` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (11) | Readable interpolation cache of length 11 supplied by `DSTEPS`. It stores precomputed integral factors and must remain consistent with `KGI` and `KOLD`. |
| 13 | `ALPHA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (12) | Readable `DSTEPS` coefficient array of length 12 used to reconstruct the interpolation factors. |
| 14 | `OG` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (13) | Readable `DSTEPS` interpolation-history array of length 13 used when evaluating the local polynomial. |
| 15 | `OW` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (12) | Readable `DSTEPS` interpolation-cache array of length 12. It is indexed through `IVC` and `IV` and is not independent workspace. |
| 16 | `OX` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Readable previous integration abscissa from `DSTEPS`; together with `X` it defines the interpolation interval. |
| 17 | `OY` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Readable previous solution vector from `DSTEPS`, with at least `NEQN` elements. It supplies the endpoint data for the smooth interpolant. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::sintrp`. Native symbol: `sintrp_`. Declaration feature: `ode`. Provider feature: `ode-integration`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::ode::sintrp`
- Public declaration feature: `ode`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
