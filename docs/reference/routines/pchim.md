# PCHIM

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Set derivatives needed to determine a monotone piecewise cubic Hermite interpolant to given data. Boundary values are provided which are compatible with monotonicity. The interpolant will have an extremum at each point where monotonicity switches direction. (See PCHIC if user control is desired over boundary or switch conditions.)

## Description

PCHIM: Piecewise Cubic Hermite Interpolation to Monotone data. Sets derivatives needed to determine a monotone piecewise cubic Hermite interpolant to the data given in X and F. Default boundary conditions are provided which are compatible with monotonicity. (See PCHIC if user control of boundary con- ditions is desired.) If the data are only piecewise monotonic, the interpolant will have an extremum at each point where monotonicity switches direc- tion. (See PCHIC if user control is desired in such cases.) To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by PCHFE or PCHFD. Calling sequence: PARAMETER (INCFD = ...) INTEGER N, IERR REAL X(N), F(INCFD,N), D(INCFD,N) CALL PCHIM (N, X, F, D, INCFD, IERR)

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
- Safe Rust paths: `slatec::pchip::PiecewiseCubicHermite::monotone`

## Providers

- Canonical provider: `pchip/pchim.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchim.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/pchim.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [PCHIM](https://www.netlib.org/slatec/pchip/pchim.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) number of data points. (Error return if N. LT. 2. ) If N=2, simply does linear interpolation. |
| 2 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (input) real array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ). |
| 3 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | (input) real array of dependent variable values to be inter- polated. F(1+(I-1)*INCFD) is value corresponding to X(I). PCHIM is designed for monotonic data, but it will work for any F-array. It will force extrema at points where mono- tonicity switches direction. If some other treatment of switch points is desired, PCHIC should be used instead. |
| 4 | `D` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | (output) real array of derivative values at the data points. If the data are monotonic, these values will determine a a monotone cubic Hermite function. The value corresponding to X(I) is stored in I=1(1)N. No other entries in D are changed. |
| 5 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. (Error return if INCFD. LT. 1. ). |
| 6 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (output) error flag. Normal return: 0 (no errors). Warning error: IERR. GT. 0 means that IERR switches in the direction of monotonicity were detected. "Recoverable" errors: -1 if N. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::pchim`. Native symbol: `pchim_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::pchim`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::pchip::PiecewiseCubicHermite::monotone`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
