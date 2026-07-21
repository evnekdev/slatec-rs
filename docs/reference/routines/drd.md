# DRD

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete elliptic integral of the 2nd kind. For X and Y nonnegative, X+Y and Z positive, DRD(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -3/2 (3/2)(t+X) (t+Y) (t+Z) dt. If X or Y is zero, the integral is complete.

## Description

1. DRD Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the second kind Standard FORTRAN function routine Double precision version The routine calculates an approximation result to

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
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rd`

## Providers

- Canonical provider: `main-src/src/drd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/drd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/drd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/drd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DRD](https://www.netlib.org/slatec/src/drd.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Integral from zero to infinity of -1/2     -1/2     -3/2 (t+Y)    (t+Z)    dt. is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -3/2 (t+Y)    (t+Z)    dt, is positive, and Z is is positive, and Z is is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling Sequence DRD( X, Y, Z, IER ) Parameters On Entry Values assigned by the calling routine Double precision, nonnegative variable is positive are unaltered. 3.    Error Messages Value of IER assigned by the DRD routine Value assigned         Error message printed are positive. are positive. are the variables in the integral DRD(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. AX DRF(1,1+KC X ,1+X ) + 3          2 2    2 +(1/3)(B-A) X DRD(1,1+KC X ,1+X ) Legendre form of alternative ELLIPTIC INTEGRAL of 2nd kind |
| 2 | `Y` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Integral from zero to infinity of -1/2     -1/2     -3/2 is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -3/2 is positive, and Z is is positive, and Z is is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling Sequence DRD( X, Y, Z, IER ) Parameters On Entry Values assigned by the calling routine Double precision, nonnegative variable is positive are unaltered. 3.    Error Messages Value of IER assigned by the DRD routine Value assigned         Error message printed are positive. are positive. are the variables in the integral DRD(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. |
| 3 | `Z` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Integral from zero to infinity of -1/2     -1/2     -3/2 Integral from zero to infinity of -1/2     -1/2     -3/2 Double precision, positive variable On Return    (values assigned by the DRD routine) DRD     - Double precision approximation to the integral are unaltered. 3.    Error Messages Value of IER assigned by the DRD routine Value assigned         Error message printed are positive. are positive. are the variables in the integral DRD(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. K SIN (B),1) 2             2 DRD(0,1-K ,1)/DRF(0,1-K ,1) 2       3           2       2   2 -(K /3) SIN (B) DRD(COS (B),1-K SIN (B),1) |
| 4 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. >  0 Abnormal termination of the routine 1                MIN(X,Y) .LT. 0.0D0 = 2                MIN(X + Y, Z ) .LT. LOLIM = 3                MAX(X,Y,Z) .GT. UPLIM 4.     Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y, and Z LOLIM  - Lower limit of valid arguments Not less  than 2 / (machine maximum) ** (2/3). UPLIM  - Upper limit of valid arguments Not greater than (0.1D0 * ERRTOL / machine minimum) ** (2/3), where ERRTOL is described below. In the following table it is assumed that ERRTOL will never be chosen smaller than 1.0D-5. Acceptable values for:   LOLIM      UPLIM IBM 360/370 SERIES   :   6.0D-51     1.0D+48 CDC 6000/7000 SERIES :   5.0D-215    2.0D+191 UNIVAC 1100 SERIES   :   1.0D-205    2.0D+201 CRAY                 :   3.0D-1644   1.69D+1640 VAX 11 SERIES        :   1.0D-25     4.5D+21 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL    Relative error due to truncation is less than 3 * ERRTOL ** 6 / (1-ERRTOL) ** 3/2. The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column.  In addition to the truncation |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

total error from both sources is usually less than the amount given in the table. Sample choices:  ERRTOL   Relative truncation 1.0D-3    4.0D-18 3.0D-3    3.0D-15 1.0D-2    4.0D-12 3.0D-2    3.0D-9 1.0D-1    4.0D-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: DRD Special Comments Check: DRD(X,Y,Z) + DRD(Y,Z,X) + DRD(Z,X,Y)

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::drd`. Native symbol: `drd_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::drd`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rd`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
