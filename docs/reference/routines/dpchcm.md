# DPCHCM

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Check a cubic Hermite function for monotonicity.

## Description

Usage: PARAMETER (INCFD = ...) INTEGER N, ISMON(N), IERR DOUBLE PRECISION X(N), F(INCFD,N), D(INCFD,N) LOGICAL SKIP CALL DPCHCM (N, X, F, D, INCFD, SKIP, ISMON, IERR) DPCHCM: Piecewise Cubic Hermite -- Check Monotonicity. Checks the piecewise cubic Hermite function defined by N,X,F,D for monotonicity. To provide compatibility with DPCHIM and DPCHIC, includes an increment between successive values of the F- and D-arrays. Cautions: This provides the same capability as old DPCHMC, except that a new output value, -3, was added February 1989. (Formerly, -3 and +3 were lumped together in the single value 3.) Codes that flag nonmonotonicity by "IF (ISMON.EQ.2)" need not be changed. Codes that check via "IF (ISMON.GE.3)" should change the test to "IF (IABS(ISMON).GE.3)". Codes that declare monotonicity via "IF (ISMON.LE.1)" should change to "IF (IABS(ISMON).LE.1)".

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

- Canonical provider: `pchip/dpchcm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dpchcm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dpchcm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DPCHCM](https://www.netlib.org/slatec/pchip/dpchcm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `F` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, N) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `D` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, N) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `SKIP` | `input-output` | `scalar` | `LOGICAL` | `*mut crate::FortranLogical` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `ISMON` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

X:IN  is a real*8 array of independent variable values.  The elements of X must be strictly increasing: X(I-1) .LT. X(I),  I = 2(1)N. F:IN  is a real*8 array of function values.  F(1+(I-1)*INCFD) is the value corresponding to X(I). D:IN  is a real*8 array of derivative values.  D(1+(I-1)*INCFD) is is the value corresponding to X(I). INCFD:IN  is the increment between successive values in F and D. SKIP:INOUT  is a logical variable which should be set to .TRUE. if the user wishes to skip checks for validity of preceding parameters, or to .FALSE. otherwise. This will save time in case these checks have already been performed. SKIP will be set to .TRUE. on normal return. ISMON:OUT  is an integer array indicating on which intervals the PCH function defined by  N, X, F, D  is monotonic. For data interval [X(I),X(I+1)], ISMON(I) = -3  if function is probably decreasing; ISMON(I) = -1  if function is strictly decreasing; ISMON(I) =  0  if function is constant; ISMON(I) =  1  if function is strictly increasing; ISMON(I) =  2  if function is non-monotonic; ISMON(I) =  3  if function is probably increasing. If ABS(ISMON)=3, this means that the D-values are near the boundary of the monotonicity region.  A small increase produces non-monotonicity; decrease, strict monotonicity. The above applies to I=1(1)N-1.  ISMON(N) indicates whether the entire function is monotonic on [X(1),X(N)]. Normal return: IERR = -1  if N.LT.2 . IERR = -2  if INCFD.LT.1 . IERR = -3  if the X-array is not strictly increasing. (The ISMON-array has not been changed in any of these cases.) and following arguments have **NOT** been validated.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dpchcm`. Native symbol: `dpchcm_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dpchcm`
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
