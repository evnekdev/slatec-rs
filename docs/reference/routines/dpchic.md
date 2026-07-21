# DPCHIC

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Set derivatives needed to determine a piecewise monotone piecewise cubic Hermite interpolant to given data. User control is available over boundary conditions and/or treatment of points where monotonicity switches direction.

## Description

DPCHIC: Piecewise Cubic Hermite Interpolation Coefficients. Sets derivatives needed to determine a piecewise monotone piece- wise cubic interpolant to the data given in X and F satisfying the boundary conditions specified by IC and VC. The treatment of points where monotonicity switches direction is controlled by argument SWITCH. To facilitate two-dimensional applications, includes an increment

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

- Canonical provider: `pchip/dpchic.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dpchic.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dpchic.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DPCHIC](https://www.netlib.org/slatec/pchip/dpchic.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IC` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. IBEG = 0  for the default boundary condition (the same as used by DPCHIM). If IBEG.NE.0, then its sign indicates whether the boundary derivative is to be adjusted, if necessary, to be compatible with monotonicity: IBEG.GT.0  if no adjustment is to be performed. IBEG.LT.0  if the derivative is to be adjusted for monotonicity. Allowable values for the magnitude of IBEG are: IBEG = 1  if first derivative at X(1) is given in VC(1). IBEG = 2  if second derivative at X(1) is given in VC(1). IBEG = 3  to use the 3-point difference formula for D(1). (Reverts to the default b.c. if N.LT.3 .) IBEG = 4  to use the 4-point difference formula for D(1). (Reverts to the default b.c. if N.LT.4 .) 1 or 2 . 1 or 2 . |
| 2 | `VC` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (2) | (input) real*8 array of length 2 specifying desired boundary values, as indicated above. 1 or 2 . 1 or 2 . |
| 3 | `SWITCH` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | (input) indicates desired treatment of points where direction of monotonicity switches: tonic in each interval, regardless of monotonicity of data. NOTES: 1. This will cause D to be set to zero at all switch points, thus forcing extrema there. 2. The result of using this option with the default boun- dary conditions will be identical to using DPCHIM, but will generally cost more compute time. This option is provided only to facilitate comparison of different switch and/or boundary conditions. point difference formula in the vicinity of switch points. If SWITCH is positive, the interpolant on each interval containing an extremum is controlled to not deviate from the data by more than SWITCH*DFLOC, where DFLOC is the maximum of the change of F on this interval and its two immediate neighbors. If SWITCH is negative, no such control is to be imposed. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 or 2, the value is given in VC(2). NOTES (IEND): 1. An error return is taken if ABS(IEND).GT.5 . 2. Only in case  IEND.LE.0  is it guaranteed that the interpolant will be monotonic in the last interval. 1)*INCFD) lies between 1), the interpolant will be monotonic. This is **NOT** checked if IEND.GT.0 . 1)*INCFD) had to be changed to achieve monotonicity, a warning error is returned. (input) number of data points.  (Error return if N.LT.2 .) 1+I) = SLOPE(I) = (F(1,I+1) - F(1,I)) / H(I) 1. 1)*INCFD) had to be adjusted for monotonicity. ting local monotone piecewise cubic interpolants, SIAM Journal on Scientific and Statistical Computing 5, 2 (June 1984), pp. 300-304. 3. F. N. Fritsch and R. E. Carlson, Monotone piecewise cubic interpolation, SIAM Journal on Numerical Ana- lysis 17, 2 (April 1980), pp. 238-246. |
| 5 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 1 or 2, the value is given in VC(2). NOTES (IEND): 1. An error return is taken if ABS(IEND).GT.5 . 2. Only in case  IEND.LE.0  is it guaranteed that the interpolant will be monotonic in the last interval. (input) real*8 array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. X(I) ; |
| 6 | `F` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by DPCHFE or DPCHFD. (input) real*8 array of dependent variable values to be 1)*INCFD) is value corresponding to ting local monotone piecewise cubic interpolants, SIAM Journal on Scientific and Statistical Computing 5, 2 (June 1984), pp. 300-304. 3. F. N. Fritsch and R. E. Carlson, Monotone piecewise cubic interpolation, SIAM Journal on Numerical Ana- lysis 17, 2 (April 1980), pp. 238-246. |
| 7 | `D` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | tinuous at X(2). (Reverts to the default b.c. if N.LT.4.) This option is somewhat analogous to the "not a knot" boundary condition provided by DPCHSP. NOTES (IBEG): 1. An error return is taken if ABS(IBEG).GT.5 . 2. Only in case  IBEG.LE.0  is it guaranteed that the interpolant will be monotonic in the first interval. If the returned value of D(1) lies between zero and 3*SLOPE(1), the interpolant will be monotonic.  This is **NOT** checked if IBEG.GT.0 . tonicity, a warning error is returned. IEND may take on the same values as IBEG, but applied to 1)*INCFD) lies between 1)*INCFD) had to be changed to achieve monotonicity, a warning error is returned. (output) real*8 array of derivative values at the data points.  These values will determine a monotone cubic Hermite function on each subinterval on which the data are monotonic, except possibly adjacent to switches in monotonicity. The value corresponding to X(I) is stored in 1)*INCFD),  I=1(1)N. No other entries in D are changed. 1)*INCFD) had to be adjusted for monotonicity. array has not been changed in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. |
| 8 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ...) INTEGER  IC(2), N, NWK, IERR DOUBLE PRECISION  VC(2), SWITCH, X(N), F(INCFD,N), D(INCFD,N), (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. |
| 9 | `WK` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NWK) | CALL DPCHIC (IC, VC, SWITCH, N, X, F, D, INCFD, WK, NWK, IERR) Parameters: (scratch) real*8 array of working storage.  The user may wish to know that the returned values are: X(I) ; 1+I) = SLOPE(I) = (F(1,I+1) - F(1,I)) / H(I) |
| 10 | `NWK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | CALL DPCHIC (IC, VC, SWITCH, N, X, F, D, INCFD, WK, NWK, IERR) Parameters: (input) length of work array. |
| 11 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (output) error flag. Normal return: 0  (no errors). Warning errors: 1  if IBEG.LT.0 and D(1) had to be adjusted for monotonicity. 1)*INCFD) had to be adjusted for monotonicity. 3  if both of the above are true. "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. 4  if ABS(IBEG).GT.5 . 5  if ABS(IEND).GT.5 . 6  if both of the above are true. 7  if NWK.LT.2*(N-1) . |

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

Canonical Rust path: `slatec_sys::interpolation::dpchic`. Native symbol: `dpchic_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dpchic`
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
