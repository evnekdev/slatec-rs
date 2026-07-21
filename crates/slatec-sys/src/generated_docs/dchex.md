# Purpose

DCHEX updates the Cholesky factorization A = TRANS(R)*R of a positive definite matrix A of order P under diagonal permutations of the form TRANS(E)*A*E where E is a permutation matrix. Specifically, given an upper triangular matrix R and a permutation matrix E (which is specified by K, L, and JOB), DCHEX determines an orthogonal matrix U such that U*R*E = RR, where RR is upper triangular. At the users option, the transformation U will be multiplied into the array Z. If A = TRANS(X)*X, so that R is the triangular part of the QR factorization of X, then RR is the triangular part of the QR factorization of X*E, i.e. X with its columns permuted. For a less terse description of what DCHEX does and how it may be applied, see the LINPACK guide. The matrix Q is determined as the product U(L-K)*...*U(1) of plane rotations of the form ( C(I) S(I) ) ( ) , ( -S(I) C(I) ) where C(I) is double precision. The rows these rotations operate on are described below. There are two types of permutations, which are determined by the value of JOB. 1. Right circular shift (JOB = 1). The columns are rearranged in the following order. 1,...,K-1,L,K,K+1,...,L-1,L+1,...,P. U is the product of L-K rotations U(I), where U(I) acts in the (L-I,L-I+1)-plane. 2. Left circular shift (JOB = 2). 1,...,K-1,K+1,K+2,...,L,K,L+1,...,P. acts in the (K+I-1,K+I)-plane. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DCHEX`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCHEX](https://www.netlib.org/slatec/lin/dchex.f).

# Arguments

## `R`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDR, *).

DOUBLE PRECISION(LDR,P), where LDR. GE. P. contains the upper triangular factor that is to be updated. Elements of R below the diagonal are not referenced. contains the updated factor.

## `LDR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array R.

## `P`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the order of the matrix R.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the first column to be permuted.

## `L`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the last column to be permuted. must be strictly greater than K.

## `Z`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDZ, *).

DOUBLE PRECISION(LDZ,N)Z), where LDZ. GE. P. is an array of NZ P-vectors into which the transformation U is multiplied. Z is not referenced if NZ = 0. contains the updated matrix Z.

## `LDZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array Z.

## `NZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of columns of the matrix Z.

## `C`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(P). contains the cosines of the transforming rotations.

## `S`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(P). contains the sines of the transforming rotations.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. JOB determines the type of permutation. 1 right circular shift. 2 left circular shift.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `R`: not a workspace argument
- `LDR`: not a workspace argument
- `Z`: not a workspace argument
- `LDZ`: not a workspace argument
- `C`: not a workspace argument
- `S`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dchex`
- Original SLATEC routine: `DCHEX`
- Native symbol: `dchex_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DCHEX](https://www.netlib.org/slatec/lin/dchex.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
