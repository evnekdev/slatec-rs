# DPCHFE

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate a piecewise cubic Hermite function at an array of points. May be used by itself for Hermite interpolation, or as an evaluator for DPCHIM or DPCHIC.

## Description

DPCHFE: Piecewise Cubic Hermite Function Evaluator Evaluates the cubic Hermite function defined by N, X, F, D at the points XE(J), J=1(1)NE. To provide compatibility with DPCHIM and DPCHIC, includes an increment between successive values of the F- and D-arrays. ---------------------------------------------------------------------- Calling sequence: PARAMETER (INCFD = ...) INTEGER N, NE, IERR DOUBLE PRECISION X(N), F(INCFD,N), D(INCFD,N), XE(NE), FE(NE) LOGICAL SKIP CALL DPCHFE (N, X, F, D, INCFD, SKIP, NE, XE, FE, IERR) Parameters:

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

- Canonical provider: `pchip/dpchfe.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dpchfe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dpchfe.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DPCHFE](https://www.netlib.org/slatec/pchip/dpchfe.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `F` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `D` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (INCFD, *) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `SKIP` | `input-output` | `scalar` | `LOGICAL` | `*mut crate::FortranLogical` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `NE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `XE` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 9 | `FE` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

X -- (input) real*8 array of independent variable values.  The elements of X must be strictly increasing: X(I-1) .LT. X(I),  I = 2(1)N. F -- (input) real*8 array of function values.  F(1+(I-1)*INCFD) is the value corresponding to X(I). D -- (input) real*8 array of derivative values.  D(1+(I-1)*INCFD) is the value corresponding to X(I). INCFD -- (input) increment between successive values in F and D. SKIP -- (input/output) logical variable which should be set to .TRUE. if the user wishes to skip checks for validity of preceding parameters, or to .FALSE. otherwise. This will save time in case these checks have already been performed (say, in DPCHIM or DPCHIC). SKIP will be set to .TRUE. on normal return. NE.LT.1 .) XE -- (input) real*8 array of points at which the function is to be evaluated. NOTES: 1. The evaluation will be most efficient if the elements of XE are increasing relative to X; that is,   XE(J) .GE. X(I) implies    XE(K) .GE. X(I),  all K.GE.J . 2. If any of the XE are outside the interval [X(1),X(N)], values are extrapolated from the nearest extreme cubic, FE -- (output) real*8 array of values of the cubic Hermite function defined by  N, X, F, D  at the points  XE. Normal return: IERR.GT.0  means that extrapolation was performed at IERR points. IERR = -1  if N.LT.2 . IERR = -2  if INCFD.LT.1 . IERR = -3  if the X-array is not strictly increasing. IERR = -4  if NE.LT.1 . (The FE-array has not been changed in any of these cases.) and following arguments have **NOT** been validated.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dpchfe`. Native symbol: `dpchfe_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dpchfe`
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
