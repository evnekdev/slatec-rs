# PCHSP

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Set derivatives needed to determine the Hermite representation of the cubic spline interpolant to given data, with specified boundary conditions.

## Description

PCHSP: Piecewise Cubic Hermite Spline Computes the Hermite representation of the cubic spline inter- polant to the data given in X and F satisfying the boundary conditions specified by IC and VC. To facilitate two-dimensional applications, includes an increment

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
- GAMS classifications: `E1A`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::pchip::PiecewiseCubicHermite::spline`

## Providers

- Canonical provider: `pchip/pchsp.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchsp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/pchsp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [PCHSP](https://www.netlib.org/slatec/pchip/pchsp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IC` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. 1 or 2 . 1 or 2 . |
| 2 | `VC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (2) | 0. IEND may take on the same values as IBEG, but applied to 0. (input) real array of length 2 specifying desired boundary values, as indicated above. 1 or 2 . 1 or 2 . |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 or 2, the value is given in VC(2). NOTES: 1. An error return is taken if IEND is out of range. 2. For the "natural" boundary condition, use IEND=2 and (input) number of data points.  (Error return if N.LT.2 .) |
| 4 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1 or 2, the value is given in VC(2). NOTES: 1. An error return is taken if IEND is out of range. 2. For the "natural" boundary condition, use IEND=2 and (input) real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. |
| 5 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by PCHFE or PCHFD. NOTE:  This is a modified version of C. de Boor's cubic spline routine CUBSPL. (input) real array of dependent variable values to be inter- 1)*INCFD) is value corresponding to X(I). |
| 6 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | tinuous at X(2).  This is the "not a knot" condition provided by de Boor's cubic spline routine CUBSPL. < This is the default boundary condition. > IBEG = 1  if first derivative at X(1) is given in VC(1). IBEG = 2  if second derivative at X(1) is given in VC(1). IBEG = 3  to use the 3-point difference formula for D(1). (Reverts to the default b.c. if N.LT.3 .) IBEG = 4  to use the 4-point difference formula for D(1). (Reverts to the default b.c. if N.LT.4 .) NOTES: 1. An error return is taken if IBEG is out of range. 2. For the "natural" boundary condition, use IBEG=2 and (output) real array of derivative values at the data points. These values will determine the cubic spline interpolant with the requested boundary conditions. The value corresponding to X(I) is stored in 1)*INCFD),  I=1(1)N. No other entries in D are changed. array has not been changed in any of these cases.) array may have been changed in this case.) (             Do **NOT** use it!                ) |
| 7 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ...) INTEGER  IC(2), N, NWK, IERR REAL  VC(2), X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL  PCHSP (IC, VC, N, X, F, D, INCFD, WK, NWK, IERR) Parameters: (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. |
| 8 | `WK` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (2, *) | (scratch) real array of working storage. |
| 9 | `NWK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) length of work array. |
| 10 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (output) error flag. Normal return: 0  (no errors). "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. 4  if IBEG.LT.0 or IBEG.GT.4 . 5  if IEND.LT.0 of IEND.GT.4 . 6  if both of the above are true. 7  if NWK is too small. NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. 8  in case of trouble solving the linear system for the interior derivative values. |

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

Canonical Rust path: `slatec_sys::interpolation::pchsp`. Native symbol: `pchsp_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::pchsp`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::pchip::PiecewiseCubicHermite::spline`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
