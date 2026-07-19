# DXLEGF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute normalized Legendre polynomials and associated Legendre functions.

## Description

DXLEGF: Extended-range Double-precision Legendre Functions A feature of the DXLEGF subroutine for Legendre functions is the use of extended-range arithmetic, a software extension of ordinary floating-point arithmetic that greatly increases the exponent range of the representable numbers. This avoids the need for scaling the solutions to lie within the exponent range of the most restrictive manufacturer's hardware. The increased exponent range is achieved by allocating an integer storage location together with each floating-point storage location. The interpretation of the pair (X,I) where X is floating-point and I is integer is X*(IR**I) where IR is the internal radix of the computer arithmetic. This subroutine computes one of the following vectors: 1. Legendre function of the first kind of negative order, either a. P(-MU1,NU,X), P(-MU1-1,NU,X), ..., P(-MU2,NU,X) or b. P(-MU,NU1,X), P(-MU,NU1+1,X), ..., P(-MU,NU2,X) 2. Legendre function of the second kind, either a. Q(MU1,NU,X), Q(MU1+1,NU,X), ..., Q(MU2,NU,X) or b. Q(MU,NU1,X), Q(MU,NU1+1,X), ..., Q(MU,NU2,X) 3. Legendre function of the first kind of positive order, either a. P(MU1,NU,X), P(MU1+1,NU,X), ..., P(MU2,NU,X) or b. P(MU,NU1,X), P(MU,NU1+1,X), ..., P(MU,NU2,X) 4. Normalized Legendre polynomials, either a. PN(MU1,NU,X), PN(MU1+1,NU,X), ..., PN(MU2,NU,X) or b. PN(MU,NU1,X), PN(MU,NU1+1,X), ..., PN(MU,NU2,X) where X = COS(THETA). The input values to DXLEGF are DNU1, NUDIFF, MU1, MU2, THETA, and ID. These must satisfy DNU1 is DOUBLE PRECISION and greater than or equal to -0.5; NUDIFF is INTEGER and non-negative; MU1 is INTEGER and non-negative; MU2 is INTEGER and greater than or equal to MU1; THETA is DOUBLE PRECISION and in the half-open interval (0,PI/2]; ID is INTEGER and equal to 1, 2, 3 or 4; and additionally either NUDIFF = 0 or MU2 = MU1. If ID=1 and NUDIFF=0, a vector of type 1a above is computed with NU=DNU1. If ID=1 and MU1=MU2, a vector of type 1b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. If ID=2 and NUDIFF=0, a vector of type 2a above is computed with NU=DNU1. If ID=2 and MU1=MU2, a vector of type 2b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. If ID=3 and NUDIFF=0, a vector of type 3a above is computed with NU=DNU1. If ID=3 and MU1=MU2, a vector of type 3b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. If ID=4 and NUDIFF=0, a vector of type 4a above is computed with NU=DNU1. If ID=4 and MU1=MU2, a vector of type 4b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. In each case the vector of computed Legendre function values is returned in the extended-range vector (PQA(I),IPQA(I)). The length of this vector is either MU2-MU1+1 or NUDIFF+1. Where possible, DXLEGF returns IPQA(I) as zero. In this case the value of the Legendre function is contained entirely in PQA(I), so it can be used in subsequent computations without further consideration of extended-range arithmetic. If IPQA(I) is nonzero, then the value of the Legendre function is not representable in floating-point because of underflow or overflow. The program that calls DXLEGF must test IPQA(I) to ensure correct usage. IERROR is an error indicator. If no errors are detected, IERROR=0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. If IERROR=210 or 211, invalid input was provided to DXLEGF. If IERROR=201,202,203, or 204, invalid input was provided to DXSET. If IERROR=205 or 206, an internal consistency error occurred in DXSET (probably due to a software malfunction in the library routine I1MACH). If IERROR=207, an overflow or underflow of an extended-range number was detected in DXADJ. If IERROR=208, an overflow or underflow of an extended-range number was detected in DXC210.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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

- Canonical provider: `main-src/src/dxlegf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dxlegf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dxlegf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dxlegf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
