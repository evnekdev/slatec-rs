# Purpose

ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. User input bounds on the uncertainty in the elements of A are used to detect numerical rank deficiency. The algorithm employs a row and column pivot strategy to minimize the growth of uncertainty and round-off errors. ULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space WARNING - All input arrays are changed on exit. *

# Description

This canonical unsafe binding exposes original SLATEC routine `ULSIA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ULSIA](https://www.netlib.org/slatec/src/ulsia.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (MDA, *).

Linear coefficient matrix of AX=B, with MDA the Contains the lower triangular part of the reduced matrix and the transformation information. It togeth with the first M elements of WORK (see below) completely specify the LQ factorization of A.

## `MDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

actual first dimension of A in the calling program.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). Must have MDA. GE.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

actual first dimension of A in the calling program.

## `B`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (MDB, *).

Right hand side(s), with MDB the actual first Contains the N by NB solution matrix for X.

## `MDB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimension of B in the calling program. NB is the number of M by 1 right hand sides. Since the solution is returned in B, must have MDB. GE. N. If.

## `NB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimension of B in the calling program. NB is the number of M by 1 right hand sides. Since the solution is returned in B, must have MDB. GE. N. If 0, B is never accessed.

## `RE`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

RE() is a vector of length N such that RE(I) is the maximum relative uncertainty in row I of the matrix A. The values of RE() must be between 0 and 1. A minimum of 10*machine precision will be enforced.

## `AE`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

AE() is a vector of length N such that AE(I) is the maximum absolute uncertainty in row I of the matrix A. The values of AE() must be greater than or equal to 0.

## `KEY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

For ease of use, RE and AE may be input as either vectors or scalars. If a scalar is input, the algo- rithm will use that value for each column of A. The parameter KEY indicates whether scalars or vectors are being input. 0 RE scalar AE scalar 1 RE vector AE scalar 2 RE scalar AE vector 3 RE vector AE vector.

## `MODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The integer MODE indicates how the routine is to react if rank deficiency is detected. If MODE = 0 return immediately, no solution 1 compute truncated solution 2 compute minimal length least squares sol The inexperienced user is advised to set MODE=0.

## `NP`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. The inexperienced user is advised to set NP=0. WORK() A real work array dimensioned 5*M. However, if RE or AE have been specified as vectors, dimension WORK 4*M. If both RE and AE have been specified as vectors, dimension WORK 3*M.

## `KRANK`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The numerical rank of A, based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. The algorithm returns a solution based on KRANK. KSURE provides an indication of the precision of the rank.

## `KSURE`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The numerical rank of A, based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. The algorithm returns a solution based on KRANK. KSURE provides an indication of the precision of the rank.

## `RNORM`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Contains the Euclidean length of the NB residual vectors B(I)-AX(I), I=1,NB. If the matrix A is of full rank, then RNORM=0. 0. WORK() The first M locations of WORK contain values necessary to reproduce the Householder transformation.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Writable real work array for the rank-revealing LQ solve. It requires `5*M` elements when `RE` and `AE` are scalar, `4*M` when either is vector-valued, and `3*M` when both are vector-valued. Its leading entries are persistent factorization state for an `INFO=1` continuation call.

## `LW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Actual dimension of WORK.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

work array dimensioned at least N+M. The first N locations contain the order in which the columns of A were used. The next M locations contain the order in which the rows of A were used.

## `LIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Actual dimension of IWORK.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Is a flag which provides for the efficient solution of subsequent problems involving the same A but different B. If INFO = 0 original call INFO = 1 subsequent calls On subsequent calls, the user must supply A, KRANK, LW, IWORK, LIW, and the first 2*M locations of WORK as output by the original call to ULSIA. MODE must be equal to the value of MODE in the original call. If MODE. LT. 2, only the first N locations of WORK are accessed.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 original call |
| `INFO` | `1` | 1 subsequent calls On subsequent calls, the user must supply A, KRANK, LW, IWORK, LIW, and the first 2*M locations of WORK as output by the original call to ULSIA. MODE must be equal to the value of MODE in the original call. If MODE.LT.2, only the first N locations of WORK are accessed. AE, RE, KEY, and NP are not accessed. -1 Parameter error(s) 0 - Rank deficient, no solution 1 - Rank deficient, truncated solution 2 - Rank deficient, minimal length least squares sol 3 - Numerical rank 0, zero solution 4 - Rank .LT. NP 5 - Full rank |

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `RE`: not a workspace argument
- `AE`: not a workspace argument
- `RNORM`: not a workspace argument
- `W`: Writable real work array for the rank-revealing LQ solve. It requires `5*M` elements when `RE` and `AE` are scalar, `4*M` when either is vector-valued, and `3*M` when both are vector-valued. Its leading entries are persistent factorization state for an `INFO=1` continuation call.
- `IWORK`: work array dimensioned at least N+M. The first N locations contain the order in which the columns of A were used. The next M locations contain the order in which the rows of A were used.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::ulsia`
- Original SLATEC routine: `ULSIA`
- Native symbol: `ulsia_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [ULSIA](https://www.netlib.org/slatec/src/ulsia.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
