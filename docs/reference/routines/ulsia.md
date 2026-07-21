# ULSIA

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve an underdetermined linear system of equations by performing an LQ factorization of the matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. User input bounds on the uncertainty in the elements of A are used to detect numerical rank deficiency. The algorithm employs a row and column pivot strategy to minimize the growth of uncertainty and round-off errors. ULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space * WARNING - All input arrays are changed on exit. * *

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/ulsia.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ulsia.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ulsia.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ulsia.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [ULSIA](https://www.netlib.org/slatec/src/ulsia.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDA, *) | B, with MDA the must be between 0 and 1. A minimum of 10*machine precision will be enforced. must be greater than or equal to 0. rithm will use that value for each column of A. The parameter KEY indicates whether scalars or vectors are being input. Contains the lower triangular part of the reduced matrix and the transformation information. It togeth with the first M elements of WORK (see below) completely specify the LQ factorization of A. |
| 2 | `MDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | actual first dimension of A in the calling program. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). Must have MDA.GE.M and M.LE.N. locations contain the order in which the rows of A were used. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | actual first dimension of A in the calling program. contain the order in which the columns of A were used. The next |
| 5 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDB, *) | Right hand side(s), with MDB the actual first is the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N. If Contains the N by NB solution matrix for X. AX(I), I=1,NB. If the matrix A is of |
| 6 | `MDB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N. If |
| 7 | `NB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the is the number of M by 1 right hand sides.  Since the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N. If solution is returned in B, must have MDB.GE.N. If 0, B is never accessed. * |
| 8 | `RE` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are what make this             * code significantly different from               * other linear least squares solvers.             * However, the inexperienced user is              * 0.,AE=0.,KEY=0.               * * is a vector of length N such that RE(I) is is a vector of length N such that RE(I) is the maximum relative uncertainty in row I of the maximum relative uncertainty in row I of must be between 0 and 1. A minimum of 10*machine precision will be enforced. or AE have been specified as vectors, dimension WORK 4*M. If both RE and AE have been specified as vectors, dimension WORK 3*M. are not accessed. |
| 9 | `AE` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are what make this             * code significantly different from               * other linear least squares solvers.             * However, the inexperienced user is              * is a vector of length N such that AE(I) is is a vector of length N such that AE(I) is the maximum absolute uncertainty in row I of the maximum absolute uncertainty in row I of must be greater than or equal to 0. are not accessed. |
| 10 | `KEY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | For ease of use, RE and AE may be input as either 0     RE scalar  AE scalar 1     RE vector  AE scalar 2     RE scalar  AE vector 3     RE vector  AE vector are not accessed. |
| 11 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The integer MODE indicates how the routine is to react if rank deficiency is detected. 0 return immediately, no solution 1 compute truncated solution 2 compute minimal length least squares sol 0 |
| 12 | `NP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. 0. WORK()        A real work array dimensioned 5*M.  However, if are not accessed. |
| 13 | `KRANK` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The numerical rank of A,  based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. The algorithm returns a solution based on KRANK. KSURE provides an indication of the precision of the rank. |
| 14 | `KSURE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The numerical rank of A,  based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. The algorithm returns a solution based on KRANK. KSURE provides an indication of the precision of the rank. |
| 15 | `RNORM` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Contains the Euclidean length of the NB residual 0.0. WORK()        The first M locations of WORK contain values necessary to reproduce the Householder transformation. |
| 16 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Writable real work array for the rank-revealing LQ solve. It requires `5*M` elements when `RE` and `AE` are scalar, `4*M` when either is vector-valued, and `3*M` when both are vector-valued. Its leading entries are persistent factorization state for an `INFO=1` continuation call. |
| 17 | `LW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Actual dimension of WORK IWORK, LIW, and the first 2*M locations of WORK as output by the original call to ULSIA. MODE must be equal to the value of MODE in the original call. If MODE.LT.2, only the first N locations of WORK |
| 18 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Integer work array dimensioned at least N+M. contain the order in which the columns of A were used. The next |
| 19 | `LIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Actual dimension of IWORK. |
| 20 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Is a flag which provides for the efficient solution of subsequent problems involving the same A but different B. 0 original call 1 subsequent calls On subsequent calls, the user must supply A, KRANK, Flag to indicate status of computation on completion -1   Parameter error(s) 0 - Rank deficient, no solution 1 - Rank deficient, truncated solution 2 - Rank deficient, minimal length least squares sol 3 - Numerical rank 0, zero solution 4 - Rank .LT. NP 5 - Full rank |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: Writable real work array for the rank-revealing LQ solve. It requires `5*M` elements when `RE` and `AE` are scalar, `4*M` when either is vector-valued, and `3*M` when both are vector-valued. Its leading entries are persistent factorization state for an `INFO=1` continuation call.

`IWORK`: Integer work array dimensioned at least N+M. contain the order in which the columns of A were used. The next

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::ulsia`. Native symbol: `ulsia_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ulsia`
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
