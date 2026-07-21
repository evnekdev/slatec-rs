# RF

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete elliptic integral of the 1st kind. For X, Y, and Z non-negative and at most one of them zero, RF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt. If X, Y or Z is zero, the integral is complete.

## Description

1. RF Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the first kind Standard FORTRAN function routine Single precision version The routine calculates an approximation result to RF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt, where X, Y, and Z are nonnegative and at most one of them is zero. If one of them is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence RF( X, Y, Z, IER ) Parameters on Entry Values assigned by the calling routine X - Single precision, nonnegative variable Y - Single precision, nonnegative variable Z - Single precision, nonnegative variable On Return (values assigned by the RF routine) RF - Single precision approximation to the integral IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X, Y, Z are unaltered.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
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
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rf_f32`

## Providers

- Canonical provider: `main-src/src/rf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/rf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/rf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/rf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [RF](https://www.netlib.org/slatec/src/rf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `Y` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `Z` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

Value of IER assigned by the RF routine IER = 1                MIN(X,Y,Z) .LT. 0.0E0 = 2                MIN(X+Y,X+Z,Y+Z) .LT. LOLIM = 3                MAX(X,Y,Z) .GT. UPLIM 4.     Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y and Z LOLIM  - Lower limit of valid arguments Not less than 5 * (machine minimum). UPLIM  - Upper limit of valid arguments Not greater than (machine maximum) / 5. Acceptable Values For:   LOLIM      UPLIM IBM 360/370 SERIES   :   3.0E-78     1.0E+75 CDC 6000/7000 SERIES :   1.0E-292    1.0E+321 UNIVAC 1100 SERIES   :   1.0E-37     1.0E+37 CRAY                 :   2.3E-2466   1.09E+2465 VAX 11 SERIES        :   1.5E-38     3.0E+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL ** 6 / (4 * (1-ERRTOL)  . The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order second column of the following table for each value of ERRTOL in the first column.  In addition to the trunca- than the amount given in the table. Sample Choices:  ERRTOL   Relative Truncation 1.0E-3    3.0E-19 3.0E-3    2.0E-16 1.0E-2    3.0E-13 3.0E-2    2.0E-10 1.0E-1    3.0E-7 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: RF Special Comments Check by addition theorem: RF(X,X+Z,X+W) + RF(Y,Y+Z,Y+W) = RF(0,Z,W), where X,Y,Z,W are positive and X * Y = Z * W. On Input: X, Y, and Z are the variables in the integral RF(X,Y,Z). On Output: X, Y, and Z are unaltered. Warning: Changes in the program may improve speed at the expense of robustness. Special Functions via RF Legendre form of ELLIPTIC INTEGRAL of 1st kind ---------------------------------------------- 2         2   2 F(PHI,K) = SIN(PHI) RF(COS (PHI),1-K SIN (PHI),1)

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::rf`. Native symbol: `rf_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::rf`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rf_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
