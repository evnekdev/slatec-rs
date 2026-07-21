# SGLSS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linear least squares problems by performing a QR factorization of the matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

SGLSS solves both underdetermined and overdetermined

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

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sglss.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sglss.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sglss.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sglss.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SGLSS](https://www.netlib.org/slatec/src/sglss.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDA, *) | is an M by N matrix and B is an M by NB matrix of right hand sides. If B, with MDA the Contains the triangular part of the reduced matrix and the transformation information. It together with the first 2*MIN(M,N) elements of WORK (see below) completely specify the factorization of A. |
| 2 | `MDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | actual first dimension of A in the calling program. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factor- ization). If the matrix A is determined to be rank deficient, that is the rank of A is less than then the minimal length least squares solution is computed. SGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes LLSIA and ULSIA are recommended. SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. * WARNING - All input arrays are changed on exit.        * * SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). 0, B is never accessed. contain value necessary to reproduce the factorization of A. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factor- ization). If the matrix A is determined to be rank deficient, that is the rank of A is less than then the minimal length least squares solution is computed. SGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes LLSIA and ULSIA are recommended. SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. * WARNING - All input arrays are changed on exit.        * * SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) actual first dimension of A in the calling program. 0, B is never accessed. contain value necessary to reproduce the factorization of A. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. Reduced rank  rank=MIN(M,N)-INFO |
| 5 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDB, *) | is an M by N matrix and B is an M by NB matrix of right hand sides. If Right hand side(s), with MDB the actual first is the number of M by 1 right hand sides. Must have Contains the N by NB solution matrix X. AX(I), I=1,NB. |
| 6 | `MDB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of M by 1 right hand sides. Must have 0, B is never accessed. |
| 7 | `NB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the is the number of M by 1 right hand sides. Must have number of M by 1 right hand sides. Must have 0, B is never accessed. |
| 8 | `RNORM` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Vector of length at least NB.  On input the contents of RNORM are unused. Contains the Euclidean length of the NB residual |
| 9 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A real work array dimensioned 5*MIN(M,N). contain value contain value necessary to reproduce the factorization of A. necessary to reproduce the factorization of A. |
| 10 | `LW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Actual dimension of WORK. IWORK, LIW, and the first 2*MIN(M,N) locations of WORK as output by the original call to SGLSS. |
| 11 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Integer work array dimensioned at least N+M. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. |
| 12 | `LIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Actual dimension of IWORK. |
| 13 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | A flag which provides for the efficient solution of subsequent problems involving the same A but different B. 0 original call 1 subsequent calls On subsequent calls, the user must supply A, INFO, Flag to indicate status of computation on completion -1   Parameter error(s) 0 - Full rank |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: A real work array dimensioned 5*MIN(M,N). contain value contain value necessary to reproduce the factorization of A. necessary to reproduce the factorization of A.

`IWORK`: Integer work array dimensioned at least N+M. contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::sglss`. Native symbol: `sglss_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sglss`
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
