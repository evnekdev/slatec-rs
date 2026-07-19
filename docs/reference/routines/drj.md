# DRJ

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the incomplete or complete (X or Y or Z is zero) elliptic integral of the 3rd kind. For X, Y, and Z nonnegative, at most one of them zero, and P positive, RJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt.

## Description

1. DRJ Standard FORTRAN function routine Double precision version The routine calculates an approximation result to DRJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt, where X, Y, and Z are nonnegative, at most one of them is zero, and P is positive. If X or Y or Z is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2. Calling Sequence DRJ( X, Y, Z, P, IER ) Parameters on Entry Values assigned by the calling routine X - Double precision, nonnegative variable Y - Double precision, nonnegative variable Z - Double precision, nonnegative variable P - Double precision, positive variable On Return (values assigned by the DRJ routine) DRJ - Double precision approximation to the integral IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X, Y, Z, P are unaltered. 3. Error Messages Value of IER assigned by the DRJ routine Value assigned Error Message printed IER = 1 MIN(X,Y,Z) .LT. 0.0D0 = 2 MIN(X+Y,X+Z,Y+Z,P) .LT. LOLIM = 3 MAX(X,Y,Z,P) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y, Z, and P LOLIM is not less than the cube root of the value of LOLIM used in the routine for DRC. UPLIM is not greater than 0.3 times the cube root of the value of UPLIM used in the routine for DRC. Acceptable values for: LOLIM UPLIM IBM 360/370 SERIES : 2.0D-26 3.0D+24 CDC 6000/7000 SERIES : 5.0D-98 3.0D+106 UNIVAC 1100 SERIES : 5.0D-103 6.0D+101 CRAY : 1.32D-822 1.4D+821 VAX 11 SERIES : 2.5D-13 9.0D+11 ERRTOL determines the accuracy of the answer the value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". Relative error due to truncation of the series for DRJ is less than 3 * ERRTOL ** 6 / (1 - ERRTOL) ** 3/2. The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the truncation error there will be round-off error, but in practice the total error from both sources is usually less than the amount given in the table. Sample choices: ERRTOL Relative truncation error less than 1.0D-3 4.0D-18 3.0D-3 3.0D-15 1.0D-2 4.0D-12 3.0D-2 3.0D-9 1.0D-1 4.0D-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. *Long Description: DRJ Special Comments Check by addition theorem: DRJ(X,X+Z,X+W,X+P) + DRJ(Y,Y+Z,Y+W,Y+P) + (A-B) * DRJ(A,B,B,A) + 3.0D0 / SQRT(A) = DRJ(0,Z,W,P), where X,Y,Z,W,P are positive and X * Y = Z * W, A = P * P * (X+Y+Z+W), B = P * (P+X) * (P+Y), and B - A = P * (P-Z) * (P-W). The sum of the third and fourth terms on the left side is 3.0D0 * DRC(A,B). On Input: X, Y, Z, and P are the variables in the integral DRJ(X,Y,Z,P). On Output: X, Y, Z, P are unaltered. ******************************************************** WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRJ and DRF Legendre form of ELLIPTIC INTEGRAL of 3rd kind PHI 2 -1 P(PHI,K,N) = INT (1+N SIN (THETA) ) * 0 2 2 -1/2 *(1-K SIN (THETA) ) D THETA 2 2 2 = SIN (PHI) DRF(COS (PHI), 1-K SIN (PHI),1) 3 2 2 2 -(N/3) SIN (PHI) DRJ(COS (PHI),1-K SIN (PHI), 2 1,1+N SIN (PHI)) Bulirsch form of ELLIPTIC INTEGRAL of 3rd kind 2 2 2 EL3(X,KC,P) = X DRF(1,1+KC X ,1+X ) + 3 2 2 2 2 +(1/3)(1-P) X DRJ(1,1+KC X ,1+X ,1+PX ) 2 CEL(KC,P,A,B) = A RF(0,KC ,1) + 2 +(1/3)(B-PA) DRJ(0,KC ,1,P) Heuman's LAMBDA function 2 2 2 1/2 L(A,B,P) =(COS (A)SIN(B)COS(B)/(1-COS (A)SIN (B)) ) 2 2 2 *(SIN(P) DRF(COS (P),1-SIN (A) SIN (P),1) 2 3 2 2 +(SIN (A) SIN (P)/(3(1-COS (A) SIN (B)))) 2 2 2 *DRJ(COS (P),1-SIN (A) SIN (P),1,1- 2 2 2 2 -SIN (A) SIN (P)/(1-COS (A) SIN (B)))) (PI/2) LAMBDA0(A,B) =L(A,B,PI/2) = 2 2 2 -1/2 = COS (A) SIN(B) COS(B) (1-COS (A) SIN (B)) 2 2 2 *DRF(0,COS (A),1) + (1/3) SIN (A) COS (A) 2 2 -3/2 *SIN(B) COS(B) (1-COS (A) SIN (B)) 2 2 2 2 2 *DRJ(0,COS (A),1,COS (A) COS (B)/(1-COS (A) SIN (B))) Jacobi ZETA function 2 2 2 1/2 Z(B,K) = (K/3) SIN(B) COS(B) (1-K SIN (B)) 2 2 2 2 *DRJ(0,1-K ,1,1-K SIN (B)) / DRF (0,1-K ,1)

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
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rj`

## Providers

- Canonical provider: `main-src/src/drj.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/drj.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/drj.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/drj.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::special_scalar_expanded::drj`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rj`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
