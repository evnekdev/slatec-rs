# DRF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete elliptic integral of the 1st kind. For X, Y, and Z non-negative and at most one of them zero, RF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt. If X, Y or Z is zero, the integral is complete.

## Description

1. DRF Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the first kind Standard FORTRAN function routine Double precision version The routine calculates an approximation result to DRF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt, where X, Y, and Z are nonnegative and at most one of them is zero. If one of them is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling sequence DRF( X, Y, Z, IER ) Parameters On entry Values assigned by the calling routine X - Double precision, nonnegative variable Y - Double precision, nonnegative variable Z - Double precision, nonnegative variable On Return (values assigned by the DRF routine) DRF - Double precision approximation to the integral IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X, Y, Z are unaltered. 3. Error Messages Value of IER assigned by the DRF routine Value assigned Error Message Printed IER = 1 MIN(X,Y,Z) .LT. 0.0D0 = 2 MIN(X+Y,X+Z,Y+Z) .LT. LOLIM = 3 MAX(X,Y,Z) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y and Z LOLIM - Lower limit of valid arguments Not less than 5 * (machine minimum). UPLIM - Upper limit of valid arguments Not greater than (machine maximum) / 5. Acceptable values for: LOLIM UPLIM IBM 360/370 SERIES : 3.0D-78 1.0D+75 CDC 6000/7000 SERIES : 1.0D-292 1.0D+321 UNIVAC 1100 SERIES : 1.0D-307 1.0D+307 CRAY : 2.3D-2466 1.09D+2465 VAX 11 SERIES : 1.5D-38 3.0D+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - Relative error due to truncation is less than ERRTOL ** 6 / (4 * (1-ERRTOL) . The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the truncation error there will be round-off error, but in practice the total error from both sources is usually less than the amount given in the table. Sample choices: ERRTOL Relative Truncation error less than 1.0D-3 3.0D-19 3.0D-3 2.0D-16 1.0D-2 3.0D-13 3.0D-2 2.0D-10 1.0D-1 3.0D-7 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. *Long Description: DRF Special Comments Check by addition theorem: DRF(X,X+Z,X+W) + DRF(Y,Y+Z,Y+W) = DRF(0,Z,W), where X,Y,Z,W are positive and X * Y = Z * W. On Input: X, Y, and Z are the variables in the integral DRF(X,Y,Z). On Output: X, Y, Z are unaltered. ******************************************************** WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRF Legendre form of ELLIPTIC INTEGRAL of 1st kind 2 2 2 F(PHI,K) = SIN(PHI) DRF(COS (PHI),1-K SIN (PHI),1) 2 K(K) = DRF(0,1-K ,1) PI/2 2 2 -1/2 = INT (1-K SIN (PHI) ) D PHI 0 Bulirsch form of ELLIPTIC INTEGRAL of 1st kind 2 2 2 EL1(X,KC) = X DRF(1,1+KC X ,1+X ) Lemniscate constant A 1 4 -1/2 A = INT (1-S ) DS = DRF(0,1,2) = DRF(0,2,1) 0

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

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::special_scalar_expanded::drf`
- Public declaration feature: `raw-ffi-scalar-functions`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rf`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
