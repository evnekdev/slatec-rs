# RD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete elliptic integral of the 2nd kind. For X and Y nonnegative, X+Y and Z positive, RD(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -3/2 (3/2)(t+X) (t+Y) (t+Z) dt. If X or Y is zero, the integral is complete.

## Description

1. RD Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the second kind Standard FORTRAN function routine Single precision version The routine calculates an approximation result to RD(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -3/2 (3/2)(t+X) (t+Y) (t+Z) dt, where X and Y are nonnegative, X + Y is positive, and Z is positive. If X or Y is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence RD( X, Y, Z, IER ) Parameters on Entry Values assigned by the calling routine X - Single precision, nonnegative variable Y - Single precision, nonnegative variable X + Y is positive Z - Real, positive variable On Return (values assigned by the RD routine) RD - Real approximation to the integral IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X, Y, Z are unaltered. 3. Error Messages Value of IER assigned by the RD routine Value Assigned Error Message Printed IER = 1 MIN(X,Y) .LT. 0.0E0 = 2 MIN(X + Y, Z ) .LT. LOLIM = 3 MAX(X,Y,Z) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y, and Z LOLIM - Lower limit of valid arguments Not less than 2 / (machine maximum) ** (2/3). UPLIM - Upper limit of valid arguments Not greater than (0.1E0 * ERRTOL / machine minimum) ** (2/3), where ERRTOL is described below. In the following table it is assumed that ERRTOL will never be chosen smaller than 1.0E-5. Acceptable Values For: LOLIM UPLIM IBM 360/370 SERIES : 6.0E-51 1.0E+48 CDC 6000/7000 SERIES : 5.0E-215 2.0E+191 UNIVAC 1100 SERIES : 1.0E-25 2.0E+21 CRAY : 3.0E-1644 1.69E+1640 VAX 11 SERIES : 1.0E-25 4.5E+21 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL Relative error due to truncation is less than 3 * ERRTOL ** 6 / (1-ERRTOL) ** 3/2. The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the truncation error there will be round-off error, but in practice the total error from both sources is usually less than the amount given in the table. Sample Choices: ERRTOL Relative Truncation error less than 1.0E-3 4.0E-18 3.0E-3 3.0E-15 1.0E-2 4.0E-12 3.0E-2 3.0E-9 1.0E-1 4.0E-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. *Long Description: RD Special Comments Check: RD(X,Y,Z) + RD(Y,Z,X) + RD(Z,X,Y) = 3 / SQRT(X * Y * Z), where X, Y, and Z are positive. On Input: X, Y, and Z are the variables in the integral RD(X,Y,Z). On Output: X, Y, and Z are unaltered. ******************************************************** WARNING: Changes in the program may improve speed at the expense of robustness. Special Functions via RD and RF Legendre form of ELLIPTIC INTEGRAL of 2nd kind 2 2 2 E(PHI,K) = SIN(PHI) RF(COS (PHI),1-K SIN (PHI),1) - 2 3 2 2 2 -(K/3) SIN (PHI) RD(COS (PHI),1-K SIN (PHI),1) 2 2 2 E(K) = RF(0,1-K ,1) - (K/3) RD(0,1-K ,1) PI/2 2 2 1/2 = INT (1-K SIN (PHI) ) D PHI 0 Bulirsch form of ELLIPTIC INTEGRAL of 2nd kind 2 2 2 EL2(X,KC,A,B) = AX RF(1,1+KC X ,1+X ) + 3 2 2 2 +(1/3)(B-A) X RD(1,1+KC X ,1+X ) Legendre form of alternative ELLIPTIC INTEGRAL of 2nd kind Q 2 2 2 -1/2 D(Q,K) = INT SIN P (1-K SIN P) DP 0 3 2 2 2 D(Q,K) =(1/3)(SIN Q) RD(COS Q,1-K SIN Q,1) Lemniscate constant B 1 2 4 -1/2 B = INT S (1-S ) DS 0 B =(1/3)RD (0,2,1) Heuman's LAMBDA function (PI/2) LAMBDA0(A,B) = 2 2 = SIN(B) (RF(0,COS (A),1)-(1/3) SIN (A) * 2 2 2 2 *RD(0,COS (A),1)) RF(COS (B),1-COS (A) SIN (B),1) 2 3 2 -(1/3) COS (A) SIN (B) RF(0,COS (A),1) * 2 2 2 *RD(COS (B),1-COS (A) SIN (B),1) Jacobi ZETA function 2 2 2 2 Z(B,K) = (K/3) SIN(B) RF(COS (B),1-K SIN (B),1) 2 2 *RD(0,1-K ,1)/RF(0,1-K ,1) 2 3 2 2 2 -(K /3) SIN (B) RD(COS (B),1-K SIN (B),1)

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
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rd_f32`

## Providers

- Canonical provider: `main-src/src/rd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/rd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/rd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/rd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::special::numerical::rd`
- Current legacy Rust paths: `slatec_sys::special_scalar_expanded::rd`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rd_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
