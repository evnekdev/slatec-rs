# DBINT4

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the B-representation of a cubic spline which interpolates given data.

## Description

Abstract **** a double precision routine **** DBINT4 computes the B representation (T,BCOEF,N,K) of a

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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

- Canonical provider: `main-src/src/dbint4.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbint4.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbint4.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbint4.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DBINT4](https://www.netlib.org/slatec/src/dbint4.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is not specified is not specified by the problem, it is common practice to use a natural by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) spline by setting second derivatives at X(1) and X(NDATA) NDATA+2.  The knots T(1),T(2),T(3) lie to is anticipated, the is anticipated, the is unique for given boundary conditions. are double precision X vector of abscissae of length NDATA, distinct and in increasing order to FBCL = 2 constrain the second derivative at to FBCL to FBCR to FBCR are knot values to the right of X(NDATA) in increasing order to be supplied by the user |
| 2 | `Y` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are double precision Y vector of ordinates of length NDATA |
| 3 | `NDATA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Parameters IBCL, IBCR, FBCL, FBCR allow the specification of the spline first or second derivative at is not specified by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) is anticipated, the number of data points, NDATA .GE. 2 to FBCR to FBCR |
| 4 | `IBCL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IBCR=2,FBCL=FBCR=0.0).  The spline is defined on T(4) .LE. X .LE. T(N+1) with (ordered) interior knots at selection parameter for left boundary condition 1 constrain the first derivative at |
| 5 | `IBCR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | selection parameter for right boundary condition 1 constrain first derivative at 2 constrain second derivative at |
| 6 | `FBCL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are double precision left boundary values governed by IBCL |
| 7 | `FBCR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are double precision right boundary values governed by IBCR |
| 8 | `KNTOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | knot selection parameter 1 sets knot multiplicity at T(4) and 3, then W(1),W(2),W(3) are knot values to |
| 9 | `T` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | X(1) and the knots T(N+2), T(N+3), T(N+4) X(NDATA) in increasing order.  If T(2)=T(3)=T(4)=X(1) and T(N+2)=T(N+3)=T(N+4)= X(NDATA) can be specified by KNTOPT=1.  KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the X(1) and similarly for X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+4) to the right of X(NDATA) in the work array is unique is unique for given boundary conditions. for given boundary conditions. to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets T(I)=W(I) and T(N+1+I)=W(3+I),I=1,3 are double precision knot array of length N+4 |
| 10 | `BCOEF` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are double precision B spline coefficient array of length N |
| 11 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | NDATA+2.  The knots T(1),T(2),T(3) lie to X(NDATA) in increasing order.  If X(NDATA) can be specified by KNTOPT=1.  KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 X(NDATA).  KNTOPT=3 allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+4) to the right of X(NDATA) in the work array is unique for given boundary conditions. to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets T(I)=W(I) and T(N+1+I)=W(3+I),I=1,3 number of coefficients, N=NDATA+2 |
| 12 | `K` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 4) which interpolates data (X(I),Y(I)), order of spline, K=4 |
| 13 | `W` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (5, *) | through W(6).  In any case, the interpolation on are double precision 1,6 is supplied by the user work array of dimension at least 5*(NDATA+2) are knot are knot are knot values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing order to be supplied by the user order to be supplied by the user order to be supplied by the user |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

Improper  input is a fatal error Singular system of equations is a fatal error

### Storage and workspace requirements

`W`: through W(6).  In any case, the interpolation on are double precision 1,6 is supplied by the user work array of dimension at least 5*(NDATA+2) are knot are knot are knot values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing values to the right of X(NDATA) in increasing order to be supplied by the user order to be supplied by the user order to be supplied by the user

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dbint4`. Native symbol: `dbint4_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank2)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbint4`
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
