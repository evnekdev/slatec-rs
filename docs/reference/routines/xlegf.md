# XLEGF

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute normalized Legendre polynomials and associated Legendre functions.

## Description

XLEGF: Extended-range Single-precision Legendre Functions A feature of the XLEGF subroutine for Legendre functions is the use of extended-range arithmetic, a software extension of ordinary floating-point arithmetic that greatly increases the exponent range of the representable numbers. This avoids the need for scaling the solutions to lie within the exponent range of the most restrictive manufacturer's hardware. The increased exponent range is achieved by allocating an integer storage location together with each floating-point storage location. The interpretation of the pair (X,I) where X is floating-point and I is integer is X*(IR**I) where IR is the internal radix of the computer arithmetic. This subroutine computes one of the following vectors: 1. Legendre function of the first kind of negative order, either a. P(-MU1,NU,X), P(-MU1-1,NU,X), ..., P(-MU2,NU,X) or b. P(-MU,NU1,X), P(-MU,NU1+1,X), ..., P(-MU,NU2,X) 2. Legendre function of the second kind, either a. Q(MU1,NU,X), Q(MU1+1,NU,X), ..., Q(MU2,NU,X) or b. Q(MU,NU1,X), Q(MU,NU1+1,X), ..., Q(MU,NU2,X) 3. Legendre function of the first kind of positive order, either a. P(MU1,NU,X), P(MU1+1,NU,X), ..., P(MU2,NU,X) or b. P(MU,NU1,X), P(MU,NU1+1,X), ..., P(MU,NU2,X) 4. Normalized Legendre polynomials, either a. PN(MU1,NU,X), PN(MU1+1,NU,X), ..., PN(MU2,NU,X) or b. PN(MU,NU1,X), PN(MU,NU1+1,X), ..., PN(MU,NU2,X) where X = COS(THETA). The input values to XLEGF are DNU1, NUDIFF, MU1, MU2, THETA, and ID. These must satisfy DNU1 is REAL and greater than or equal to -0.5; NUDIFF is INTEGER and non-negative; MU1 is INTEGER and non-negative; MU2 is INTEGER and greater than or equal to MU1; THETA is REAL and in the half-open interval (0,PI/2]; ID is INTEGER and equal to 1, 2, 3 or 4; and additionally either NUDIFF = 0 or MU2 = MU1. If ID=1 and NUDIFF=0, a vector of type 1a above is computed with NU=DNU1. If ID=1 and MU1=MU2, a vector of type 1b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. If ID=2 and NUDIFF=0, a vector of type 2a above is computed with NU=DNU1. If ID=2 and MU1=MU2, a vector of type 2b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. If ID=3 and NUDIFF=0, a vector of type 3a above is computed with NU=DNU1. If ID=3 and MU1=MU2, a vector of type 3b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. If ID=4 and NUDIFF=0, a vector of type 4a above is computed with NU=DNU1. If ID=4 and MU1=MU2, a vector of type 4b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. In each case the vector of computed Legendre function values is returned in the extended-range vector (PQA(I),IPQA(I)). The length of this vector is either MU2-MU1+1 or NUDIFF+1. Where possible, XLEGF returns IPQA(I) as zero. In this case the value of the Legendre function is contained entirely in PQA(I), so it can be used in subsequent computations without further consideration of extended-range arithmetic. If IPQA(I) is nonzero, then the value of the Legendre function is not representable in floating-point because of underflow or overflow. The program that calls XLEGF must test IPQA(I) to ensure correct usage. IERROR is an error indicator. If no errors are detected, IERROR=0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. If IERROR=110 or 111, invalid input was provided to XLEGF. If IERROR=101,102,103, or 104, invalid input was provided to XSET. If IERROR=105 or 106, an internal consistency error occurred in XSET (probably due to a software malfunction in the library routine I1MACH). If IERROR=107, an overflow or underflow of an extended-range number was detected in XADJ. If IERROR=108, an overflow or underflow of an extended-range number was detected in XC210.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C3A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/xlegf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xlegf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xlegf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xlegf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `DNU1` | input | `REAL` (`explicit`) | `*mut f32` | scalar | The input values to XLEGF are DNU1, NUDIFF, MU1, MU2, THETA, and ID. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NUDIFF` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The input values to XLEGF are DNU1, NUDIFF, MU1, MU2, THETA, and ID. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MU1` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | P(-MU1,NU,X), P(-MU1-1,NU,X), ..., P(-MU2,NU,X) or b. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MU2` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | P(-MU1,NU,X), P(-MU1-1,NU,X), ..., P(-MU2,NU,X) or b. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `THETA` | input | `REAL` (`explicit`) | `*mut f32` | scalar | PN(MU,NU1,X), PN(MU,NU1+1,X), ..., PN(MU,NU2,X) where X = COS(THETA). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ID` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The input values to XLEGF are DNU1, NUDIFF, MU1, MU2, THETA, and ID. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PQA` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | In each case the vector of computed Legendre function values is returned in the extended-range vector (PQA(I),IPQA(I)). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPQA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | In each case the vector of computed Legendre function values is returned in the extended-range vector (PQA(I),IPQA(I)). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IERROR is an error indicator. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::special::xlegf`. Native symbol: `xlegf_`. Feature: `special`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::xlegf`
- Compatibility aliases: `slatec_sys::special::numerical::xlegf`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
