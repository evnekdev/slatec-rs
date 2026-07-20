# RJ

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete (X or Y or Z is zero) elliptic integral of the 3rd kind. For X, Y, and Z nonnegative, at most one of them zero, and P positive, RJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt.

## Description

1. RJ Standard FORTRAN function routine Single precision version The routine calculates an approximation result to RJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt, where X, Y, and Z are nonnegative, at most one of them is zero, and P is positive. If X or Y or Z is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence RJ( X, Y, Z, P, IER ) Parameters On Entry Values assigned by the calling routine X - Single precision, nonnegative variable Y - Single precision, nonnegative variable Z - Single precision, nonnegative variable P - Single precision, positive variable On Return (values assigned by the RJ routine) RJ - Single precision approximation to the integral IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X, Y, Z, P are unaltered. 3. Error Messages Value of IER assigned by the RJ routine Value Assigned Error Message Printed IER = 1 MIN(X,Y,Z) .LT. 0.0E0 = 2 MIN(X+Y,X+Z,Y+Z,P) .LT. LOLIM = 3 MAX(X,Y,Z,P) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X Y, Z, and P LOLIM is not less than the cube root of the value of LOLIM used in the routine for RC. UPLIM is not greater than 0.3 times the cube root of the value of UPLIM used in the routine for RC. Acceptable Values For: LOLIM UPLIM IBM 360/370 SERIES : 2.0E-26 3.0E+24 CDC 6000/7000 SERIES : 5.0E-98 3.0E+106 UNIVAC 1100 SERIES : 5.0E-13 6.0E+11 CRAY : 1.32E-822 1.4E+821 VAX 11 SERIES : 2.5E-13 9.0E+11 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". Relative error due to truncation of the series for RJ is less than 3 * ERRTOL ** 6 / (1 - ERRTOL) ** 3/2. The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order Introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the truncation error there will be round-off error, but in practice the total error from both sources is usually less than the amount given in the table. Sample choices: ERRTOL Relative Truncation error less than 1.0E-3 4.0E-18 3.0E-3 3.0E-15 1.0E-2 4.0E-12 3.0E-2 3.0E-9 1.0E-1 4.0E-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. *Long Description: RJ Special Comments Check by addition theorem: RJ(X,X+Z,X+W,X+P) + RJ(Y,Y+Z,Y+W,Y+P) + (A-B) * RJ(A,B,B,A) + 3 / SQRT(A) = RJ(0,Z,W,P), where X,Y,Z,W,P are positive and X * Y = Z * W, A = P * P * (X+Y+Z+W), B = P * (P+X) * (P+Y), and B - A = P * (P-Z) * (P-W). The sum of the third and fourth terms on the left side is 3 * RC(A,B). On Input: X, Y, Z, and P are the variables in the integral RJ(X,Y,Z,P). On Output: X, Y, Z, and P are unaltered. ******************************************************** Warning: Changes in the program may improve speed at the expense of robustness. Special Functions via RJ and RF Legendre form of ELLIPTIC INTEGRAL of 3rd kind PHI 2 -1 P(PHI,K,N) = INT (1+N SIN (THETA) ) * 0 2 2 -1/2 *(1-K SIN (THETA) ) D THETA 2 2 2 = SIN (PHI) RF(COS (PHI), 1-K SIN (PHI),1) 3 2 2 2 -(N/3) SIN (PHI) RJ(COS (PHI),1-K SIN (PHI), 2 1,1+N SIN (PHI)) Bulirsch form of ELLIPTIC INTEGRAL of 3rd kind 2 2 2 EL3(X,KC,P) = X RF(1,1+KC X ,1+X ) + 3 2 2 2 2 +(1/3)(1-P) X RJ(1,1+KC X ,1+X ,1+PX ) 2 CEL(KC,P,A,B) = A RF(0,KC ,1) + 2 +(1/3)(B-PA) RJ(0,KC ,1,P) Heuman's LAMBDA function 2 2 2 1/2 L(A,B,P) = (COS(A)SIN(B)COS(B)/(1-COS (A)SIN (B)) ) 2 2 2 *(SIN(P) RF(COS (P),1-SIN (A) SIN (P),1) 2 3 2 2 +(SIN (A) SIN (P)/(3(1-COS (A) SIN (B)))) 2 2 2 *RJ(COS (P),1-SIN (A) SIN (P),1,1- 2 2 2 2 -SIN (A) SIN (P)/(1-COS (A) SIN (B)))) (PI/2) LAMBDA0(A,B) =L(A,B,PI/2) = 2 2 2 -1/2 = COS (A) SIN(B) COS(B) (1-COS (A) SIN (B)) 2 2 2 *RF(0,COS (A),1) + (1/3) SIN (A) COS (A) 2 2 -3/2 *SIN(B) COS(B) (1-COS (A) SIN (B)) 2 2 2 2 2 *RJ(0,COS (A),1,COS (A) COS (B)/(1-COS (A) SIN (B))) Jacobi ZETA function 2 2 2 1/2 Z(B,K) = (K/3) SIN(B) COS(B) (1-K SIN (B)) 2 2 2 2 *RJ(0,1-K ,1,1-K SIN (B)) / RF (0,1-K ,1)

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

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::special::numerical::rj`
- Current legacy Rust paths: `slatec_sys::special_scalar_expanded::rj`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rj_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
