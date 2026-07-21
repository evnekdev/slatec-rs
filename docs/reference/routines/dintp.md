# DINTP

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Approximate the solution at XOUT by evaluating the polynomial computed in DSTEPS at XOUT. Must be used in conjunction with DSTEPS.

## Description

The methods in subroutine DSTEPS approximate the solution near X by a polynomial. Subroutine DINTP approximates the solution at

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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

- Canonical provider: `main-src/src/dintp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dintp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dintp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DINTP](https://www.netlib.org/slatec/src/dintp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input current integration abscissa from `DSTEPS`. It and the history arguments must be from the same unmodified `DSTEPS` state. |
| 2 | `Y` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Readable current solution vector from `DSTEPS`, with at least `NEQN` elements. |
| 3 | `XOUT` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | by evaluating the polynomial there.  Information defining this polynomial is passed from  DSTEPS  so  DINTP  cannot be used alone. Subroutine DSTEPS is completely explained and documented in the text "Computer Solution of Ordinary Differential Equations, the Initial Value Problem"  by L. F. Shampine and M. K. Gordon. Input to DINTP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines point at which solution is desired. The remaining parameters are defined in  DSTEPS  and passed to DINTP  from that subroutine Output from  DINTP -- |
| 4 | `YOUT` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | solution at  XOUT |
| 5 | `YPOUT` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | derivative of solution at  XOUT The remaining parameters are returned unaltered from their input values.  Integration with  DSTEPS  may be continued. |
| 6 | `NEQN` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input number of differential equations. It is the required length of `Y`, `YOUT`, and `YPOUT` and the first dimension of `PHI`. |
| 7 | `KOLD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input interpolation order saved by `DSTEPS`. It controls how many columns of the `PHI` history are used and must be passed unchanged from that integrator state. |
| 8 | `PHI` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (NEQN, 16) | Readable `DSTEPS` history matrix with Fortran shape `(NEQN, 16)`. It defines the local interpolation polynomial and must not be synthesized independently. |
| 9 | `IVC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Readable interpolation-cache control index supplied by `DSTEPS`; it selects cached data in `IV` and `OW` when the order changes. |
| 10 | `IV` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (10) | Readable integer interpolation cache of length 10 supplied by `DSTEPS`. It is part of the persistent integrator state and must be passed unchanged. |
| 11 | `KGI` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Readable `DSTEPS` interpolation-history order marker used to decide whether cached `GI` values apply. |
| 12 | `GI` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (11) | Readable interpolation cache of length 11 supplied by `DSTEPS`. It stores precomputed integral factors and must remain consistent with `KGI` and `KOLD`. |
| 13 | `ALPHA` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (12) | Readable `DSTEPS` coefficient array of length 12 used to reconstruct the interpolation factors. |
| 14 | `OG` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (13) | Readable `DSTEPS` interpolation-history array of length 13 used when evaluating the local polynomial. |
| 15 | `OW` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (12) | Readable `DSTEPS` interpolation-cache array of length 12. It is indexed through `IVC` and `IV` and is not independent workspace. |
| 16 | `OX` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Readable previous integration abscissa from `DSTEPS`; together with `X` it defines the interpolation interval. |
| 17 | `OY` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Readable previous solution vector from `DSTEPS`, with at least `NEQN` elements. It supplies the endpoint data for the smooth interpolant. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::dintp`. Native symbol: `dintp_`. Declaration feature: `ode`. Provider feature: `ode-integration`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::ode::dintp`
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
