# DPCHCM

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Check a cubic Hermite function for monotonicity.

## Description

Usage: DPCHCM: Piecewise Cubic Hermite -- Check Monotonicity. Checks the piecewise cubic Hermite function defined by N,X,F,D for monotonicity. To provide compatibility with DPCHIM and DPCHIC, includes an

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

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DPCHCM](https://www.netlib.org/slatec/pchip/dpchcm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN  is the number of data points.  (Error return if N.LT.2 .) is monotonic. For data interval [X(I),X(I+1)], 1.  ISMON(N) indicates whether the entire function is monotonic on [X(1),X(N)]. |
| 2 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | IN  is a real*8 array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. is monotonic. For data interval [X(I),X(I+1)], |
| 3 | `F` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, N) | IN  is a real*8 array of function values.  F(1+(I-1)*INCFD) is the value corresponding to X(I). is monotonic. For data interval [X(I),X(I+1)], and D-arrays. Cautions: This provides the same capability as old DPCHMC, except that a new output value, -3, was added February 1989.  (Formerly, -3 and +3 were lumped together in the single value 3.)  Codes that flag nonmonotonicity by "IF (ISMON.EQ.2)" need not be changed. Codes that check via "IF (ISMON.GE.3)" should change the test to "IF (IABS(ISMON).GE.3)".  Codes that declare monotonicity via "IF (ISMON.LE.1)" should change to "IF (IABS(ISMON).LE.1)". |
| 4 | `D` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, N) | IN  is a real*8 array of derivative values.  D(1+(I-1)*INCFD) is is the value corresponding to X(I). is monotonic. For data interval [X(I),X(I+1)], values are near the boundary of the monotonicity region.  A small increase produces non-monotonicity; decrease, strict monotonicity. |
| 5 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ...) INTEGER  N, ISMON(N), IERR DOUBLE PRECISION  X(N), F(INCFD,N), D(INCFD,N) LOGICAL  SKIP CALL  DPCHCM (N, X, F, D, INCFD, SKIP, ISMON, IERR) IN  is the increment between successive values in F and D. |
| 6 | `SKIP` | `input-output` | `scalar` | `LOGICAL` | `*mut crate::FortranLogical` | scalar | INOUT  is a logical variable which should be set to .TRUE. if the user wishes to skip checks for validity of preceding parameters, or to .FALSE. otherwise. This will save time in case these checks have already been performed. will be set to .TRUE. on normal return. |
| 7 | `ISMON` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (N) | OUT  is an integer array indicating on which intervals the 3  if function is probably decreasing; 1  if function is strictly decreasing; 0  if function is constant; 1  if function is strictly increasing; monotonic; 3  if function is probably increasing. values are near the boundary of the monotonicity region.  A small increase produces non-monotonicity; decrease, strict monotonicity. array has not been changed in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. 3 and modified code so that 1,3,-1 produces ISMON(N)=2, rather than 3. 890306  Added caution about changed output. 890407  Changed name from DPCHMC to DPCHCM, as requested at the March 1989 SLATEC CML meeting, and made a few other minor modifications necessitated by this change. 890407  Converted to new SLATEC format. 890407  Modified DESCRIPTION to LDOC format. 900315  CALLs to XERROR changed to CALLs to XERMSG.  (THJ) 920429  Revised format and order of references.  (WRB,FNF) |
| 8 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT  is an error flag. Normal return: 0  (no errors). "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. |

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
