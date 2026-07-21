# DPCHSP

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Set derivatives needed to determine the Hermite representation of the cubic spline interpolant to given data, with specified boundary conditions.

## Description

DPCHSP: Piecewise Cubic Hermite Spline Computes the Hermite representation of the cubic spline inter- polant to the data given in X and F satisfying the boundary conditions specified by IC and VC. To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by DPCHFE or DPCHFD. NOTE: This is a modified version of C. de Boor's cubic spline routine CUBSPL. ---------------------------------------------------------------------- Calling sequence: PARAMETER (INCFD = ...) INTEGER IC(2), N, NWK, IERR DOUBLE PRECISION VC(2), X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL DPCHSP (IC, VC, N, X, F, D, INCFD, WK, NWK, IERR) Parameters: IC -- (input) integer array of length 2 specifying desired boundary conditions: IC(1) = IBEG, desired condition at beginning of data. IC(2) = IEND, desired condition at end of data. IBEG = 0 to set D(1) so that the third derivative is con- tinuous at X(2). This is the "not a knot" condition provided by de Boor's cubic spline routine CUBSPL. < This is the default boundary condition. > IBEG = 1 if first derivative at X(1) is given in VC(1). IBEG = 2 if second derivative at X(1) is given in VC(1). IBEG = 3 to use the 3-point difference formula for D(1). (Reverts to the default b.c. if N.LT.3 .) IBEG = 4 to use the 4-point difference formula for D(1). (Reverts to the default b.c. if N.LT.4 .) NOTES:

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E1A`
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

- Canonical provider: `pchip/dpchsp.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dpchsp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dpchsp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DPCHSP](https://www.netlib.org/slatec/pchip/dpchsp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IC` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | Array argument classified by fixed-form executable read/write analysis. |
| 2 | `VC` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (2) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `F` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `D` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `WK` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (2, *) | Array argument classified by fixed-form executable read/write analysis. |
| 9 | `NWK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

2. For the "natural" boundary condition, use IBEG=2 and VC(1)=0. IEND may take on the same values as IBEG, but applied to derivative at X(N).  In case IEND = 1 or 2, the value is given in VC(2). NOTES: 2. For the "natural" boundary condition, use IEND=2 and VC(2)=0. VC -- (input) real*8 array of length 2 specifying desired boundary values, as indicated above. VC(1) need be set only if IC(1) = 1 or 2 . VC(2) need be set only if IC(2) = 1 or 2 . X -- (input) real*8 array of independent variable values.  The elements of X must be strictly increasing: X(I-1) .LT. X(I),  I = 2(1)N. F -- (input) real*8 array of dependent variable values to be interpolated.  F(1+(I-1)*INCFD) is value corresponding to X(I). D -- (output) real*8 array of derivative values at the data points.  These values will determine the cubic spline interpolant with the requested boundary conditions. The value corresponding to X(I) is stored in D(1+(I-1)*INCFD),  I=1(1)N. No other entries in D are changed. INCFD -- (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. WK -- (scratch) real*8 array of working storage. NWK -- (input) length of work array. Normal return: IERR = -1  if N.LT.2 . IERR = -2  if INCFD.LT.1 . IERR = -3  if the X-array is not strictly increasing. IERR = -4  if IBEG.LT.0 or IBEG.GT.4 . IERR = -5  if IEND.LT.0 of IEND.GT.4 . IERR = -6  if both of the above are true. IERR = -7  if NWK is too small. and following arguments have **NOT** been validated. (The D-array has not been changed in any of these cases.) IERR = -8  in case of trouble solving the linear system for the interior derivative values. (The D-array may have been changed in this case.) (             Do **NOT** use it!                )

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dpchsp`. Native symbol: `dpchsp_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dpchsp`
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
