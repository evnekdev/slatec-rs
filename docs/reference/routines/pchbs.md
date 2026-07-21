# PCHBS

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Piecewise Cubic Hermite to B-Spline converter.

## Description

Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PCHBS computes the B-spline representation of the PCH function determined by N,X,F,D. To be compatible with the rest of PCHIP, PCHBS includes INCFD, the increment between successive values of

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

- Canonical provider: `pchip/pchbs.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchbs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/pchbs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [PCHBS](https://www.netlib.org/slatec/pchip/pchbs.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN  is the number of data points, N.ge.2 .  (not checked) X(N-1)). X(N-1)). X(1))  . Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used in a parametric setting. are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) |
| 2 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | IN  is the real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N.   (not checked) nmax, the dimension of X, must be .ge.N. (X(2)-X(1))  ; X(N-1)). X(N-1)). (X(N)-X(N-1)); X(1))  . X(1))  . Here M=NDIM=2*N. Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used This option is provided for improved efficiency when used in a parametric setting. in a parametric setting. values and the boundary knots set as indicated above. If KNOTYP.LT.0, it is assumed that T was set by a previous call to PCHBS.  (This routine does **not** verify that T forms a legitimate knot sequence.) are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) 1,...,N .  (not checked) 1,...,N .  (not checked) 3. INCFD.GT.0 .  (not checked) 3. INCFD.GT.0 .  (not checked) 4. KNOTYP.LE.2 .  (error return if not) 4. KNOTYP.LE.2 .  (error return if not) |
| 3 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | IN  is the real array of dependent variable values. 1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of F, must be .ge.N. subscripted arrays. and D-arrays. The output is the B-representation for the function:  NKNOTS, T, Fortran 77, in the strict sense, but it works on all systems on which PCHBS has been tested. |
| 4 | `D` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | IN  is the real array of derivative values at the data points. 1)*INCFD) is the value corresponding to X(I). nmax, the second dimension of D, must be .ge.N. subscripted arrays. Fortran 77, in the strict sense, but it works on all systems on which PCHBS has been tested. |
| 5 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ...) REAL  X(nmax), F(INCFD,nmax), D(INCFD,nmax), T(2*nmax+4), IN  is the increment between successive values in F and D. This argument is provided primarily for 2-D applications. It may have the value 1 for one-dimensional applications, are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 .  (not checked) patible with the rest of PCHIP. 900410  Converted prologue to SLATEC 4.0 format. 900410  Added calls to XERMSG and changed constant 3. to 3 to reduce single/double differences. 900411  Added reference. 900501  Corrected declarations. 930317  Minor cosmetic changes.  (FNF) 930514  Corrected problems with dimensioning of arguments and clarified DESCRIPTION.  (FNF) 930604  Removed  NKNOTS from PCHKT call list.  (FNF) |
| 6 | `KNOTYP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN  is a flag to control the knot sequence. The knot sequence T is normally computed from X by putting a double knot at each X and setting the end knot pairs Quadruple knots at X(1) and X(N).  (default) Replicate lengths of extreme subintervals: Periodic placement of boundary knots: is an input variable, and an |
| 7 | `NKNOTS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INOUT  is the number of knots. If KNOTYP.GE.0, then NKNOTS will be set to NDIM+4. is an input variable, and an NDIM+4 = 2*N+4 .  (error return if not) |
| 8 | `T` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (X(2)-X(1))  ; (X(2)-X(1))  ; X(N-1)). X(N-1)). (X(N)-X(N-1)); (X(N)-X(N-1)); X(1))  . X(1))  . Here M=NDIM=2*N. Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used This option is provided for improved efficiency when used in a parametric setting. in a parametric setting. INOUT  is the array of 2*N+4 knots for the B-representation. If KNOTYP.GE.0, T will be returned by PCHBS with the T(2*k) = X(k), k=1,...,N .  (not checked) Indicates this applies only if KNOTYP.LT.0 . Portability: Argument INCFD is used only to cause the compiler to generate efficient code for the subscript expressions (1+(I-1)*INCFD) . The normal usage, in which PCHBS is called with one-dimensional |
| 9 | `BCOEF` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | CALL  PCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, OUT  is the array of 2*N B-spline coefficients. NDIM, KORD. Caution: Since it is assumed that the input PCH function has been computed by one of the other routines in the package PCHIP, |
| 10 | `NDIM` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | KORD, IERR) OUT  is the dimension of the B-spline space.  (Set to 2*N.) |
| 11 | `KORD` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT  is the order of the B-spline.  (Set to 4.) |
| 12 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT  is an error flag. Normal return: 0  (no errors). "Recoverable" errors: 4  if KNOTYP.GT.2 . 5  if KNOTYP.LT.0 and NKNOTS.NE.(2*N+4). |

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

Canonical Rust path: `slatec_sys::interpolation::pchbs`. Native symbol: `pchbs_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::pchbs`
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
