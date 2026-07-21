# SQRSL

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Apply the output of SQRDC to compute coordinate transformations, projections, and least squares solutions.

## Description

SQRSL applies the output of SQRDC to compute coordinate transformations, projections, and least squares solutions. For K .LE. MIN(N,P), let XK be the matrix XK = (X(JPVT(1)),X(JPVT(2)), ... ,X(JPVT(K))) formed from columns JPVT(1), ... ,JPVT(K) of the original N x P matrix X that was input to SQRDC (if no pivoting was done, XK consists of the first K columns of X in their original order). SQRDC produces a factored orthogonal matrix Q and an upper triangular matrix R such that XK = Q * (R) (0) This information is contained in coded form in the arrays X and QRAUX. On Entry

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

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SQRSL](https://www.netlib.org/slatec/lin/sqrsl.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDX, *) | REAL(LDX,P) contains the output of SQRDC. |
| 2 | `LDX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER is the leading dimension of the array X. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER is the number of rows of the matrix XK. It must have the same value as N in SQRDC. |
| 4 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER is the number of columns of the matrix XK. K must not be greater than MIN(N,P), where P is the same as in the calling sequence to SQRDC. |
| 5 | `QRAUX` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(P) contains the auxiliary output from SQRDC. |
| 6 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N) contains an N-vector that is to be manipulated by SQRSL. |
| 7 | `QY` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N). contains Q*Y, if its computation has been requested. |
| 8 | `QTY` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N). contains TRANS(Q)*Y, if its computation has been requested. Here TRANS(Q) is the transpose of the matrix Q. |
| 9 | `B` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(K) contains the solution of the least squares problem minimize norm2(Y - XK*B), if its computation has been requested. (Note that if pivoting was requested in SQRDC, the J-th component of B will be associated with column JPVT(J) of the original matrix X that was input into SQRDC. ). |
| 10 | `RSD` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N). contains the least squares residual Y - XK*B, if its computation has been requested. RSD is also the orthogonal projection of Y onto the orthogonal complement of the column space of XK. |
| 11 | `XB` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N). contains the least squares approximation XK*B, if its computation has been requested. XB is also the orthogonal projection of Y onto the column space of X. |
| 12 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER specifies what is to be computed. JOB has the decimal expansion ABCDE, with the following meaning. If A. NE. 0, compute QY. If B,C,D, or E. |
| 13 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is zero unless the computation of B has been requested and R is exactly singular. In this case, INFO is the index of the first zero diagonal element of R and B is left unaltered. The parameters QY, QTY, B, RSD, and XB are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence. A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::sqrsl`. Native symbol: `sqrsl_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sqrsl`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
