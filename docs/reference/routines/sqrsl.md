# SQRSL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Apply the output of SQRDC to compute coordinate transformations, projections, and least squares solutions.

## Description

SQRSL applies the output of SQRDC to compute coordinate transformations, projections, and least squares solutions. For K .LE. MIN(N,P), let XK be the matrix XK = (X(JPVT(1)),X(JPVT(2)), ... ,X(JPVT(K))) formed from columns JPVT(1), ... ,JPVT(K) of the original N x P matrix X that was input to SQRDC (if no pivoting was done, XK consists of the first K columns of X in their original order). SQRDC produces a factored orthogonal matrix Q and an upper triangular matrix R such that XK = Q * (R) (0) This information is contained in coded form in the arrays X and QRAUX. On Entry X REAL(LDX,P) X contains the output of SQRDC. LDX INTEGER LDX is the leading dimension of the array X. N INTEGER N is the number of rows of the matrix XK. It must have the same value as N in SQRDC. K INTEGER K is the number of columns of the matrix XK. K must not be greater than MIN(N,P), where P is the same as in the calling sequence to SQRDC. QRAUX REAL(P) QRAUX contains the auxiliary output from SQRDC. Y REAL(N) Y contains an N-vector that is to be manipulated by SQRSL. JOB INTEGER JOB specifies what is to be computed. JOB has the decimal expansion ABCDE, with the following meaning. If A .NE. 0, compute QY. If B,C,D, or E .NE. 0, compute QTY. If C .NE. 0, compute B. If D .NE. 0, compute RSD. If E .NE. 0, compute XB. Note that a request to compute B, RSD, or XB automatically triggers the computation of QTY, for which an array must be provided in the calling sequence. On Return QY REAL(N). QY contains Q*Y, if its computation has been requested. QTY REAL(N). QTY contains TRANS(Q)*Y, if its computation has been requested. Here TRANS(Q) is the transpose of the matrix Q. B REAL(K) B contains the solution of the least squares problem minimize norm2(Y - XK*B), if its computation has been requested. (Note that if pivoting was requested in SQRDC, the J-th component of B will be associated with column JPVT(J) of the original matrix X that was input into SQRDC.) RSD REAL(N). RSD contains the least squares residual Y - XK*B, if its computation has been requested. RSD is also the orthogonal projection of Y onto the orthogonal complement of the column space of XK. XB REAL(N). XB contains the least squares approximation XK*B, if its computation has been requested. XB is also the orthogonal projection of Y onto the column space of X. INFO INTEGER. INFO is zero unless the computation of B has been requested and R is exactly singular. In this case, INFO is the index of the first zero diagonal element of R and B is left unaltered. The parameters QY, QTY, B, RSD, and XB are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence. A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY. In this case one may identify Y, QTY, and one of B, RSD, or XB, while providing separate arrays for anything else that is to be computed. Thus the calling sequence CALL SQRSL(X,LDX,N,K,QRAUX,Y,DUM,Y,B,Y,DUM,110,INFO) will result in the computation of B and RSD, with RSD overwriting Y. More generally, each item in the following list contains groups of permissible identifications for a single calling sequence. 1. (Y,QTY,B) (RSD) (XB) (QY) 2. (Y,QTY,RSD) (B) (XB) (QY) 3. (Y,QTY,XB) (B) (RSD) (QY) 4. (Y,QY) (QTY,B) (RSD) (XB) 5. (Y,QY) (QTY,RSD) (B) (XB) 6. (Y,QY) (QTY,XB) (B) (RSD) In any group the value returned in the array allocated to the group corresponds to the last member of the group.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/sqrsl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sqrsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sqrsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
