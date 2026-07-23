# PCHIC

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Set derivatives needed to determine a piecewise monotone piecewise cubic Hermite interpolant to given data. User control is available over boundary conditions and/or treatment of points where monotonicity switches direction.

## Description

PCHIC: Piecewise Cubic Hermite Interpolation Coefficients. Sets derivatives needed to determine a piecewise monotone piece- wise cubic interpolant to the data given in X and F satisfying the boundary conditions specified by IC and VC. The treatment of points where monotonicity switches direction is controlled by argument SWITCH. To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by PCHFE or PCHFD. Calling sequence: PARAMETER (INCFD = ...) INTEGER IC(2), N, NWK, IERR REAL VC(2), SWITCH, X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL PCHIC (IC, VC, SWITCH, N, X, F, D, INCFD, WK, NWK, IERR)

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [PCHIC](https://www.netlib.org/slatec/pchip/pchic.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IC` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. IBEG = 0 for the default boundary condition (the same as used by PCHIM). If IBEG. NE. 0, then its sign indicates whether the boundary derivative is to be adjusted, if necessary, to be compatible with monotonicity: IBEG. |
| 2 | `VC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (2) | (input) real array of length 2 specifying desired boundary values, as indicated above. VC(1) need be set only if IC(1) = 1 or 2. VC(2) need be set only if IC(2) = 1 or 2. |
| 3 | `SWITCH` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) indicates desired treatment of points where direction of monotonicity switches: Set SWITCH to zero if interpolant is required to be mono- tonic in each interval, regardless of monotonicity of data. NOTES: 1. This will cause D to be set to zero at all switch points, thus forcing extrema there. 2. The result of using this option with the default boun- dary conditions will be identical to using PCHIM, but will generally cost more compute time. This option is provided only to facilitate comparison of different switch and/or boundary conditions. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) number of data points. (Error return if N. LT. 2. ). |
| 5 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (input) real array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ). |
| 6 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | (input) real array of dependent variable values to be inter- polated. F(1+(I-1)*INCFD) is value corresponding to X(I). |
| 7 | `D` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | (output) real array of derivative values at the data points. These values will determine a monotone cubic Hermite func- tion on each subinterval on which the data are monotonic, except possibly adjacent to switches in monotonicity. The value corresponding to X(I) is stored in I=1(1)N. No other entries in D are changed. |
| 8 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. (Error return if INCFD. LT. 1. ). |
| 9 | `WK` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NWK) | (scratch) real array of working storage. The user may wish to know that the returned values are: H(I) = X(I+1) - X(I) ; SLOPE(I) = (F(1,I+1) - F(1,I)) / H(I) for I = 1(1)N-1. |
| 10 | `NWK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) length of work array. (Error return if NWK. LT. 2*(N-1). ). |
| 11 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (output) error flag. Normal return: 0 (no errors). Warning errors: 1 if IBEG. LT. 0 and D(1) had to be adjusted for monotonicity. 2 if IEND. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

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
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
