# DRF

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete elliptic integral of the 1st kind. For X, Y, and Z non-negative and at most one of them zero, RF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt. If X, Y or Z is zero, the integral is complete.

## Description

1. DRF Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the first kind Standard FORTRAN function routine Double precision version The routine calculates an approximation result to

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C14`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rf`

## Providers

- Canonical provider: `main-src/src/drf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/drf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/drf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/drf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DRF](https://www.netlib.org/slatec/src/drf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | negative and at most one of Integral from zero to infinity of -1/2     -1/2     -1/2 (t+Y)    (t+Z)    dt. is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -1/2 (t+Y)    (t+Z)    dt, are nonnegative and at most one of them is zero.  If one of them  is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling sequence DRF( X, Y, Z, IER ) Parameters On entry Values assigned by the calling routine Double precision, nonnegative variable are unaltered. 3.    Error Messages Value of IER assigned by the DRF routine Value assigned         Error Message Printed are positive and X * Y = Z * W. are the variables in the integral DRF(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRF Legendre form of ELLIPTIC INTEGRAL of 1st kind X DRF(1,1+KC X ,1+X ) Lemniscate constant A |
| 2 | `Y` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | negative and at most one of Integral from zero to infinity of -1/2     -1/2     -1/2 is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -1/2 are nonnegative and at most one of them is zero.  If one of them  is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling sequence DRF( X, Y, Z, IER ) Parameters On entry Values assigned by the calling routine Double precision, nonnegative variable are unaltered. 3.    Error Messages Value of IER assigned by the DRF routine Value assigned         Error Message Printed are positive and X * Y = Z * W. are the variables in the integral DRF(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRF Legendre form of ELLIPTIC INTEGRAL of 1st kind |
| 3 | `Z` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | negative and at most one of Integral from zero to infinity of -1/2     -1/2     -1/2 is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -1/2 are nonnegative and at most one of them is zero.  If one of them  is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling sequence DRF( X, Y, Z, IER ) Parameters On entry Values assigned by the calling routine Double precision, nonnegative variable On Return    (values assigned by the DRF routine) DRF     - Double precision approximation to the integral are unaltered. 3.    Error Messages Value of IER assigned by the DRF routine Value assigned         Error Message Printed are positive and X * Y = Z * W. are positive and X * Y = Z * W. are the variables in the integral DRF(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRF Legendre form of ELLIPTIC INTEGRAL of 1st kind |
| 4 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. >  0 Abnormal termination of the routine 1                MIN(X,Y,Z) .LT. 0.0D0 = 2                MIN(X+Y,X+Z,Y+Z) .LT. LOLIM = 3                MAX(X,Y,Z) .GT. UPLIM 4.     Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y and Z LOLIM  - Lower limit of valid arguments Not less than 5 * (machine minimum). UPLIM  - Upper limit of valid arguments Not greater than (machine maximum) / 5. Acceptable values for:   LOLIM      UPLIM IBM 360/370 SERIES   :   3.0D-78     1.0D+75 CDC 6000/7000 SERIES :   1.0D-292    1.0D+321 UNIVAC 1100 SERIES   :   1.0D-307    1.0D+307 CRAY                 :   2.3D-2466   1.09D+2465 VAX 11 SERIES        :   1.5D-38     3.0D+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - Relative error due to truncation is less than ERRTOL ** 6 / (4 * (1-ERRTOL)  . The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column.  In addition to the truncation |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

total error from both sources is usually less than the amount given in the table. Sample choices:  ERRTOL   Relative Truncation 1.0D-3    3.0D-19 3.0D-3    2.0D-16 1.0D-2    3.0D-13 3.0D-2    2.0D-10 1.0D-1    3.0D-7 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: DRF Special Comments Check by addition theorem: DRF(X,X+Z,X+W) + DRF(Y,Y+Z,Y+W)

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::drf`. Native symbol: `drf_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::drf`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rf`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
