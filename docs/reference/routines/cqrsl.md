# CQRSL

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Apply the output of CQRDC to compute coordinate transformations, projections, and least squares solutions.

## Description

CQRSL applies the output of CQRDC to compute coordinate transformations, projections, and least squares solutions. For K .LE. MIN(N,P), let XK be the matrix XK = (X(JVPT(1)),X(JVPT(2)), ... ,X(JVPT(K))) formed from columns JVPT(1), ... ,JVPT(K) of the original N x P matrix X that was input to CQRDC (if no pivoting was done, XK consists of the first K columns of X in their original order). CQRDC produces a factored unitary matrix Q and an upper triangular matrix R such that XK = Q * (R) (0) This information is contained in coded form in the arrays X and QRAUX. On Entry X COMPLEX(LDX,P). X contains the output of CQRDC. LDX INTEGER. LDX is the leading dimension of the array X. N INTEGER. N is the number of rows of the matrix XK. It must have the same value as N in CQRDC. K INTEGER. K is the number of columns of the matrix XK. K must not be greater than (N,P), where P is the same as in the calling sequence to CQRDC. QRAUX COMPLEX(P). QRAUX contains the auxiliary output from CQRDC. Y COMPLEX(N) Y contains an N-vector that is to be manipulated by CQRSL. JOB INTEGER. JOB specifies what is to be computed. JOB has the decimal expansion ABCDE, with the following meaning. If A .NE. 0, compute QY. If B,C,D, or E .NE. 0, compute QTY. If C .NE. 0, compute B. If D .NE. 0, compute RSD . If E .NE. 0, compute XB. Note that a request to compute B, RSD, or XB automatically triggers the computation of QTY, for which an array must be provided in the calling sequence. On Return QY COMPLEX(N). QY contains Q*Y, if its computation has been requested. QTY COMPLEX(N). QTY contains CTRANS(Q)*Y, if its computation has been requested. Here CTRANS(Q) is the conjugate transpose of the matrix Q. B COMPLEX(K) B contains the solution of the least squares problem minimize NORM2(Y - XK*B), if its computation has been requested. (Note that if pivoting was requested in CQRDC, the J-th component of B will be associated with column JVPT(J) of the original matrix X that was input into CQRDC.) RSD COMPLEX(N). RSD contains the least squares residual Y - XK*B, if its computation has been requested. RSD is also the orthogonal projection of Y onto the orthogonal complement of the column space of XK. XB COMPLEX(N). XB contains the least squares approximation XK*B, if its computation has been requested. XB is also the orthogonal projection of Y onto the column space of X. INFO INTEGER. INFO is zero unless the computation of B has been requested and R is exactly singular. In this case, INFO is the index of the first zero diagonal element of R and B is left unaltered. The parameters QY, QTY, B, RSD, and XB are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence. A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY. In this case one may identify Y, QTY, and one of B, RSD, or XB, while providing separate arrays for anything else that is to be computed. Thus the calling sequence CALL CQRSL(X,LDX,N,K,QRAUX,Y,DUM,Y,B,Y,DUM,110,INFO) will result in the computation of B and RSD, with RSD overwriting Y. More generally, each item in the following list contains groups of permissible identifications for a single calling sequence. 1. (Y,QTY,B) (RSD) (XB) (QY) 2. (Y,QTY,RSD) (B) (XB) (QY) 3. (Y,QTY,XB) (B) (RSD) (QY) 4. (Y,QY) (QTY,B) (RSD) (XB) 5. (Y,QY) (QTY,RSD) (B) (XB) 6. (Y,QY) (QTY,XB) (B) (RSD) In any group the value returned in the array allocated to the group corresponds to the last member of the group.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
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

- Canonical provider: `lin/cqrsl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cqrsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cqrsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 2; dimensions (LDX, *) | MIN(N,P), let XK be the matrix XK = (X(JVPT(1)),X(JVPT(2)), ... | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDX` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Entry X COMPLEX(LDX,P). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | MIN(N,P), let XK be the matrix XK = (X(JVPT(1)),X(JVPT(2)), ... | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | For K .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `QRAUX` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | CQRDC produces a factored unitary matrix Q and an upper triangular matrix R such that XK = Q * (R) (0) This information is contained in coded form in the arrays X and QRAUX. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | Y COMPLEX(N) Y contains an N-vector that is to be manipulated by CQRSL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `QY` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | 0, compute QY. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `QTY` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | 0, compute QTY. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | If B,C,D, or E .NE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RSD` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | 0, compute RSD . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XB` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | 0, compute XB. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | JOB INTEGER. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | INFO INTEGER. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::complex::cqrsl`. Native symbol: `cqrsl_`. Feature: `linear-algebra-complex`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cqrsl`
- Compatibility aliases: `none`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
