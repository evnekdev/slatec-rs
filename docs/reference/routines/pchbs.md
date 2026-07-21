# PCHBS

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Piecewise Cubic Hermite to B-Spline converter.

## Description

Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) REAL X(nmax), F(INCFD,nmax), D(INCFD,nmax), T(2*nmax+4), PCHBS computes the B-spline representation of the PCH function determined by N,X,F,D. To be compatible with the rest of PCHIP, PCHBS includes INCFD, the increment between successive values of the F- and D-arrays. The output is the B-representation for the function: NKNOTS, T, BCOEF, NDIM, KORD. Caution: Since it is assumed that the input PCH function has been computed by one of the other routines in the package PCHIP, input arguments N, X, INCFD are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 . (not checked) 2. X(i).LT.X(i+1), i=1,...,N . (not checked) 3. INCFD.GT.0 . (not checked) 4. KNOTYP.LE.2 . (error return if not)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `pchip/pchbs.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchbs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/pchbs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [PCHBS](https://www.netlib.org/slatec/pchip/pchbs.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of data points, N. ge. 2. (not checked). |
| 2 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is the real array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (not checked) nmax, the dimension of X, must be. ge. |
| 3 | `F` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | is the real array of dependent variable values. is the value corresponding to X(I). nmax, the second dimension of F, must be. ge. N. |
| 4 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | is the real array of derivative values at the data points. is the value corresponding to X(I). nmax, the second dimension of D, must be. ge. N. |
| 5 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the increment between successive values in F and D. This argument is provided primarily for 2-D applications. It may have the value 1 for one-dimensional applications, in which case F and D may be singly-subscripted arrays. |
| 6 | `KNOTYP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a flag to control the knot sequence. The knot sequence T is normally computed from X by putting a double knot at each X and setting the end knot pairs according to the value of KNOTYP: 0: Quadruple knots at X(1) and X(N). (default) 1: Replicate lengths of extreme subintervals: 2: Periodic placement of boundary knots:. |
| 7 | `NKNOTS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of knots. If KNOTYP. GE. 0, then NKNOTS will be set to NDIM+4. LT. 0, then NKNOTS is an input variable, and an error return will be taken if it is not equal to NDIM+4. |
| 8 | `T` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (X(2)-X(1)) ; T(M+3) = X(N) + (X(N)-X(N-1)). (X(N)-X(N-1)); T(M+3) = X(N) + (X(2)-X(1)). Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used in a parametric setting. is the array of 2*N+4 knots for the B-representation. |
| 9 | `BCOEF` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | CALL PCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, NDIM, KORD, IERR) is the array of 2*N B-spline coefficients. |
| 10 | `NDIM` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the dimension of the B-spline space. (Set to 2*N. ). |
| 11 | `KORD` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the B-spline. (Set to 4. ). |
| 12 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an error flag. Normal return: 0 (no errors). "Recoverable" errors: -4 if KNOTYP. GT. 2. -5 if KNOTYP. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::pchbs`. Native symbol: `pchbs_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::pchbs`
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
