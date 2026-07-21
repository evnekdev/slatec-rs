# Purpose

SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. If M.GE.N, the least squares solution is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factor- ization). If the matrix A is determined to be rank deficient, that is the rank of A is less than MIN(M,N), then the minimal length least squares solution is computed. SGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes LLSIA and ULSIA are recommended. SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. WARNING - All input arrays are changed on exit. * SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO)

# Description

This canonical unsafe binding exposes original SLATEC routine `SGLSS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGLSS](https://www.netlib.org/slatec/src/sglss.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (MDA, *).

Linear coefficient matrix of AX=B, with MDA the Contains the triangular part of the reduced matrix and the transformation information. It together with the first 2*MIN(M,N) elements of WORK (see below) completely specify the factorization of A.

## `MDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

actual first dimension of A in the calling program.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS).

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

actual first dimension of A in the calling program.

## `B`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (MDB, *).

Right hand side(s), with MDB the actual first Contains the N by NB solution matrix X.

## `MDB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimension of B in the calling program. NB is the number of M by 1 right hand sides. Must have MDB. GE. MAX(M,N). If NB = 0, B is never accessed.

## `NB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimension of B in the calling program. NB is the number of M by 1 right hand sides. Must have MDB. GE. MAX(M,N). If NB = 0, B is never accessed.

## `RNORM`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Vector of length at least NB. On input the contents of RNORM are unused. Contains the Euclidean length of the NB residual vectors B(I)-AX(I), I=1,NB.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A real work array dimensioned 5*MIN(M,N). The first 2*MIN(M,N) locations of WORK contain value necessary to reproduce the factorization of A.

## `LW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Actual dimension of WORK.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

work array dimensioned at least N+M. The first M+N locations contain the order in which the rows and columns of A were used. If M. GE. N columns then rows. LT.

## `LIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Actual dimension of IWORK.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

A flag which provides for the efficient solution of subsequent problems involving the same A but different B. If INFO = 0 original call INFO = 1 subsequent calls On subsequent calls, the user must supply A, INFO, LW, IWORK, LIW, and the first 2*MIN(M,N) locations of WORK as output by the original call to SGLSS. Flag to indicate status of computation on completion -1 Parameter error(s) 0 - Full rank N. GT. 0 - Reduced rank rank=MIN(M,N)-INFO.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 original call |
| `INFO` | `1` | 1 subsequent calls On subsequent calls, the user must supply A, INFO, LW, IWORK, LIW, and the first 2*MIN(M,N) locations of WORK as output by the original call to SGLSS. -1 Parameter error(s) 0 - Full rank N.GT.0 - Reduced rank rank=MIN(M,N)-INFO |

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `RNORM`: not a workspace argument
- `WORK`: A real work array dimensioned 5*MIN(M,N). The first 2*MIN(M,N) locations of WORK contain value necessary to reproduce the factorization of A.
- `IWORK`: work array dimensioned at least N+M. The first M+N locations contain the order in which the rows and columns of A were used. If M. GE. N columns then rows. LT.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::sglss`
- Original SLATEC routine: `SGLSS`
- Native symbol: `sglss_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SGLSS](https://www.netlib.org/slatec/src/sglss.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
