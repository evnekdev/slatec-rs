# PCHIC

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Set derivatives needed to determine a piecewise monotone piecewise cubic Hermite interpolant to given data. User control is available over boundary conditions and/or treatment of points where monotonicity switches direction.

## Description

PCHIC: Piecewise Cubic Hermite Interpolation Coefficients. Sets derivatives needed to determine a piecewise monotone piece- wise cubic interpolant to the data given in X and F satisfying the boundary conditions specified by IC and VC. The treatment of points where monotonicity switches direction is controlled by argument SWITCH. To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by PCHFE or PCHFD. ---------------------------------------------------------------------- Calling sequence: PARAMETER (INCFD = ...) INTEGER IC(2), N, NWK, IERR REAL VC(2), SWITCH, X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL PCHIC (IC, VC, SWITCH, N, X, F, D, INCFD, WK, NWK, IERR) Parameters: IC -- (input) integer array of length 2 specifying desired boundary conditions: IC(1) = IBEG, desired condition at beginning of data. IC(2) = IEND, desired condition at end of data. IBEG = 0 for the default boundary condition (the same as used by PCHIM). If IBEG.NE.0, then its sign indicates whether the boundary derivative is to be adjusted, if necessary, to be compatible with monotonicity: IBEG.GT.0 if no adjustment is to be performed. IBEG.LT.0 if the derivative is to be adjusted for monotonicity. Allowable values for the magnitude of IBEG are: IBEG = 1 if first derivative at X(1) is given in VC(1). IBEG = 2 if second derivative at X(1) is given in VC(1). IBEG = 3 to use the 3-point difference formula for D(1). (Reverts to the default b.c. if N.LT.3 .) IBEG = 4 to use the 4-point difference formula for D(1). (Reverts to the default b.c. if N.LT.4 .) IBEG = 5 to set D(1) so that the second derivative is con- tinuous at X(2). (Reverts to the default b.c. if N.LT.4.) This option is somewhat analogous to the "not a knot" boundary condition provided by PCHSP. NOTES (IBEG):

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
- Safe Rust paths: `slatec::pchip::PiecewiseCubicHermite::monotone_with_conditions`

## Providers

- Canonical provider: `pchip/pchic.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchic.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/pchic.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [PCHIC](https://www.netlib.org/slatec/pchip/pchic.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IC` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | Array argument classified by fixed-form executable read/write analysis. |
| 2 | `VC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (2) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `SWITCH` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `WK` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NWK) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `NWK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

2. Only in case  IBEG.LE.0  is it guaranteed that the interpolant will be monotonic in the first interval. If the returned value of D(1) lies between zero and 3*SLOPE(1), the interpolant will be monotonic.  This is **NOT** checked if IBEG.GT.0 . 3. If IBEG.LT.0 and D(1) had to be changed to achieve mono- IEND may take on the same values as IBEG, but applied to derivative at X(N).  In case IEND = 1 or 2, the value is given in VC(2). NOTES (IEND): 2. Only in case  IEND.LE.0  is it guaranteed that the interpolant will be monotonic in the last interval. If the returned value of D(1+(N-1)*INCFD) lies between zero and 3*SLOPE(N-1), the interpolant will be monotonic. This is **NOT** checked if IEND.GT.0 . 3. If IEND.LT.0 and D(1+(N-1)*INCFD) had to be changed to VC -- (input) real array of length 2 specifying desired boundary values, as indicated above. VC(1) need be set only if IC(1) = 1 or 2 . VC(2) need be set only if IC(2) = 1 or 2 . SWITCH -- (input) indicates desired treatment of points where direction of monotonicity switches: Set SWITCH to zero if interpolant is required to be mono- tonic in each interval, regardless of monotonicity of data. NOTES: 1. This will cause D to be set to zero at all switch points, thus forcing extrema there. 2. The result of using this option with the default boun- dary conditions will be identical to using PCHIM, but will generally cost more compute time. This option is provided only to facilitate comparison of different switch and/or boundary conditions. Set SWITCH nonzero to use a formula based on the 3-point difference formula in the vicinity of switch points. If SWITCH is positive, the interpolant on each interval containing an extremum is controlled to not deviate from the data by more than SWITCH*DFLOC, where DFLOC is the maximum of the change of F on this interval and its two immediate neighbors. If SWITCH is negative, no such control is to be imposed. X -- (input) real array of independent variable values.  The elements of X must be strictly increasing: X(I-1) .LT. X(I),  I = 2(1)N. F -- (input) real array of dependent variable values to be inter- polated.  F(1+(I-1)*INCFD) is value corresponding to X(I). D -- (output) real array of derivative values at the data points. These values will determine a monotone cubic Hermite func- tion on each subinterval on which the data are monotonic, except possibly adjacent to switches in monotonicity. The value corresponding to X(I) is stored in D(1+(I-1)*INCFD),  I=1(1)N. No other entries in D are changed. INCFD -- (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. WK -- (scratch) real array of working storage.  The user may wish to know that the returned values are: WK(I)     = H(I)     = X(I+1) - X(I) ; WK(N-1+I) = SLOPE(I) = (F(1,I+1) - F(1,I)) / H(I) for  I = 1(1)N-1. NWK -- (input) length of work array. Normal return: IERR = 1  if IBEG.LT.0 and D(1) had to be adjusted for monotonicity. IERR = 2  if IEND.LT.0 and D(1+(N-1)*INCFD) had to be adjusted for monotonicity. IERR = 3  if both of the above are true. IERR = -1  if N.LT.2 . IERR = -2  if INCFD.LT.1 . IERR = -3  if the X-array is not strictly increasing. IERR = -4  if ABS(IBEG).GT.5 . IERR = -5  if ABS(IEND).GT.5 . IERR = -6  if both of the above are true. IERR = -7  if NWK.LT.2*(N-1) . (The D-array has not been changed in any of these cases.) and following arguments have **NOT** been validated.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::pchic`. Native symbol: `pchic_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::pchic`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::pchip::PiecewiseCubicHermite::monotone_with_conditions`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
