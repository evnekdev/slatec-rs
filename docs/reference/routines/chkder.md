# CHKDER

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Check the gradients of M nonlinear functions in N variables, evaluated at a point X, for consistency with the functions themselves.

## Description

This subroutine is a companion routine to SNLS1,SNLS1E,SNSQ,and SNSQE which may be used to check the calculation of the Jacobian. SUBROUTINE CHKDER This subroutine checks the gradients of M nonlinear functions in N variables, evaluated at a point X, for consistency with the functions themselves. The user must call CKDER twice, first with MODE = 1 and then with MODE = 2. MODE = 1. On input, X must contain the point of evaluation. On output, XP is set to a neighboring point. MODE = 2. On input, FVEC must contain the functions and the rows of FJAC must contain the gradients of the respective functions each evaluated at X, and FVECP must contain the functions evaluated at XP. On output, ERR contains measures of correctness of the respective gradients. The subroutine does not perform reliably if cancellation or rounding errors cause a severe loss of significance in the evaluation of a function. Therefore, none of the components of X should be unusually small (in particular, zero) or any other value which may cause loss of significance. The SUBROUTINE statement is SUBROUTINE CHKDER(M,N,X,FVEC,FJAC,LDFJAC,XP,FVECP,MODE,ERR) where M is a positive integer input variable set to the number of functions. N is a positive integer input variable set to the number of variables. X is an input array of length N. FVEC is an array of length M. On input when MODE = 2, FVEC must contain the functions evaluated at X. FJAC is an M by N array. On input when MODE = 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. LDFJAC is a positive integer input parameter not less than M which specifies the leading dimension of the array FJAC. XP is an array of length N. On output when MODE = 1, XP is set to a neighboring point of X. FVECP is an array of length M. On input when MODE = 2, FVECP must contain the functions evaluated at XP. MODE is an integer input variable set to 1 on the first call and 2 on the second. Other values of MODE are equivalent to MODE = 1. ERR is an array of length M. On output when MODE = 2, ERR contains measures of correctness of the respective gradients. If there is no severe loss of significance, then if ERR(I) is 1.0 the I-th gradient is correct, while if ERR(I) is 0.0 the I-th gradient is incorrect. For values of ERR between 0.0 and 1.0, the categorization is less certain. In general, a value of ERR(I) greater than 0.5 indicates that the I-th gradient is probably correct, while a value of ERR(I) less than 0.5 indicates that the I-th gradient is probably incorrect.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::nonlinear::check_jacobian_f32`

## Providers

- Canonical provider: `main-src/src/chkder.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/chkder.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/chkder.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/chkder.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::nonlinear::chkder`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::nonlinear::check_jacobian_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
