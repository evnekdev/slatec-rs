# DPCHSP

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Set derivatives needed to determine the Hermite representation of the cubic spline interpolant to given data, with specified boundary conditions.

## Description

DPCHSP: Piecewise Cubic Hermite Spline Computes the Hermite representation of the cubic spline inter- polant to the data given in X and F satisfying the boundary conditions specified by IC and VC. To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by DPCHFE or DPCHFD. NOTE: This is a modified version of C. de Boor's cubic spline routine CUBSPL. Calling sequence: PARAMETER (INCFD = ...) INTEGER IC(2), N, NWK, IERR DOUBLE PRECISION VC(2), X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL DPCHSP (IC, VC, N, X, F, D, INCFD, WK, NWK, IERR)

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DPCHSP](https://www.netlib.org/slatec/pchip/dpchsp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IC` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. IBEG = 0 to set D(1) so that the third derivative is con- tinuous at X(2). This is the "not a knot" condition provided by de Boor's cubic spline routine CUBSPL. < This is the default boundary condition. > IBEG = 1 if first derivative at X(1) is given in VC(1). |
| 2 | `VC` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (2) | (input) real*8 array of length 2 specifying desired boundary values, as indicated above. VC(1) need be set only if IC(1) = 1 or 2. VC(2) need be set only if IC(2) = 1 or 2. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) number of data points. (Error return if N. LT. 2. ). |
| 4 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | (input) real*8 array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ). |
| 5 | `F` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | (input) real*8 array of dependent variable values to be interpolated. F(1+(I-1)*INCFD) is value corresponding to. |
| 6 | `D` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | (output) real*8 array of derivative values at the data points. These values will determine the cubic spline interpolant with the requested boundary conditions. The value corresponding to X(I) is stored in I=1(1)N. No other entries in D are changed. |
| 7 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. (Error return if INCFD. LT. 1. ). |
| 8 | `WK` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (2, *) | (scratch) real*8 array of working storage. |
| 9 | `NWK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) length of work array. (Error return if NWK. LT. 2*N. ). |
| 10 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (output) error flag. Normal return: 0 (no errors). "Recoverable" errors: -1 if N. LT. 2. -2 if INCFD. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

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
