# RJ

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete (X or Y or Z is zero) elliptic integral of the 3rd kind. For X, Y, and Z nonnegative, at most one of them zero, and P positive, RJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt.

## Description

1. RJ Standard FORTRAN function routine Single precision version The routine calculates an approximation result to RJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt, where X, Y, and Z are nonnegative, at most one of them is zero, and P is positive. If X or Y or Z is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence RJ( X, Y, Z, P, IER ) Parameters On Entry Values assigned by the calling routine X - Single precision, nonnegative variable Y - Single precision, nonnegative variable Z - Single precision, nonnegative variable P - Single precision, positive variable On Return (values assigned by the RJ routine) RJ - Single precision approximation to the integral IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X, Y, Z, P are unaltered.

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
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rj_f32`

## Providers

- Canonical provider: `main-src/src/rj.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/rj.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/rj.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/rj.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [RJ](https://www.netlib.org/slatec/src/rj.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `Y` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `Z` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `P` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

Value of IER assigned by the RJ routine IER = 1               MIN(X,Y,Z) .LT. 0.0E0 = 2               MIN(X+Y,X+Z,Y+Z,P) .LT. LOLIM = 3               MAX(X,Y,Z,P) .GT. UPLIM 4.     Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X Y, Z, and P LOLIM is not less than the cube root of the value of LOLIM used in the routine for RC. UPLIM is not greater than 0.3 times the cube root of the value of UPLIM used in the routine for RC. Acceptable Values For:   LOLIM      UPLIM IBM 360/370 SERIES   :   2.0E-26     3.0E+24 CDC 6000/7000 SERIES :   5.0E-98     3.0E+106 UNIVAC 1100 SERIES   :   5.0E-13     6.0E+11 CRAY                 :   1.32E-822   1.4E+821 VAX 11 SERIES        :   2.5E-13     9.0E+11 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". is less than 3 * ERRTOL ** 6 / (1 - ERRTOL) ** 3/2. The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order second column of the following table for each value of ERRTOL in the first column.  In addition to the trunca- than the amount given in the table. Sample choices:  ERRTOL   Relative Truncation 1.0E-3    4.0E-18 3.0E-3    3.0E-15 1.0E-2    4.0E-12 3.0E-2    3.0E-9 1.0E-1    4.0E-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: RJ Special Comments Check by addition theorem: RJ(X,X+Z,X+W,X+P) + RJ(Y,Y+Z,Y+W,Y+P) + (A-B) * RJ(A,B,B,A) + 3 / SQRT(A) = RJ(0,Z,W,P), where X,Y,Z,W,P are positive and X * Y = Z * W,  A = P * P * (X+Y+Z+W),  B = P * (P+X) * (P+Y), and B - A = P * (P-Z) * (P-W).  The sum of the third and fourth terms on the left side is 3 * RC(A,B). On Input: X, Y, Z, and P are the variables in the integral RJ(X,Y,Z,P). On Output: X, Y, Z, and P are unaltered. Warning: Changes in the program may improve speed at the expense of robustness. ------------------------------------------------------------ Special Functions via RJ and RF Legendre form of ELLIPTIC INTEGRAL of 3rd kind ---------------------------------------------- PHI         2         -1 P(PHI,K,N) = INT (1+N SIN (THETA) )   *

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::rj`. Native symbol: `rj_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::rj`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rj_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
