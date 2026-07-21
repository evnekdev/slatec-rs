# BINT4

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the B-representation of a cubic spline which interpolates given data.

## Description

BINT4 computes the B representation (T,BCOEF,N,K) of a cubic spline (K=4) which interpolates data (X(I)),Y(I))), I=1,NDATA. Parameters IBCL, IBCR, FBCL, FBCR allow the specification of the spline first or second derivative at both X(1) and X(NDATA). When this data is not specified by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) to zero (IBCL=IBCR=2,FBCL=FBCR=0.0). The spline is defined on T(4) .LE. X .LE. T(N+1) with (ordered) interior knots at X(I)) values where N=NDATA+2. The knots T(1), T(2), T(3) lie to the left of T(4)=X(1) and the knots T(N+2), T(N+3), T(N+4) lie to the right of T(N+1)=X(NDATA) in increasing order. If no extrapolation outside (X(1),X(NDATA)) is anticipated, the knots T(1)=T(2)=T(3)=T(4)=X(1) and T(N+2)=T(N+3)=T(N+4)= T(N+1)=X(NDATA) can be specified by KNTOPT=1. KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the first 7 knots symmetric about T(4)=X(1) and similarly for T(N+2), T(N+3), T(N+4) about T(N+1)=X(NDATA). KNTOPT=3 allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+3), T(N+4) to the right of X(NDATA) in the work array W(1) through W(6). In any case, the interpolation on T(4) .LE. X .LE. T(N+1) by using function BVALU is unique for given boundary conditions.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E1A`
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

- Canonical provider: `main-src/src/bint4.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bint4.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bint4.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bint4.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [BINT4](https://www.netlib.org/slatec/src/bint4.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | X vector of abscissae of length NDATA, distinct and in increasing order |
| 2 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Y vector of ordinates of length NDATA |
| 3 | `NDATA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of data points, NDATA .GE. 2 |
| 4 | `IBCL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | selection parameter for left boundary condition 1 constrain the first derivative at X(1) to FBCL = 2 constrain the second derivative at X(1) to FBCL |
| 5 | `IBCR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | selection parameter for right boundary condition 1 constrain first derivative at X(NDATA) to FBCR 2 constrain second derivative at X(NDATA) to FBCR |
| 6 | `FBCL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | left boundary values governed by IBCL |
| 7 | `FBCR` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | right boundary values governed by IBCR |
| 8 | `KNTOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | knot selection parameter 1 sets knot multiplicity at T(4) and T(N+1) to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets TNP)=WNP) and T(N+1+I)=w(3+I),I=1,3 where WNP),I=1,6 is supplied by the user 3, then W(1),W(2),W(3) are knot values to the left of X(1) and W(4),W(5),W(6) are knot values to the right of X(NDATA) in increasing order to be supplied by the user |
| 9 | `T` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | knot array of length N+4 |
| 10 | `BCOEF` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | B-spline coefficient array of length N |
| 11 | `N` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of coefficients, N=NDATA+2 |
| 12 | `K` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order of spline, K=4 |
| 13 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 2; dimensions (5, *) | work array of dimension at least 5*(NDATA+2) |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: work array of dimension at least 5*(NDATA+2)

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::bint4`. Native symbol: `bint4_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::bint4`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
