# Purpose

SCHUD updates an augmented Cholesky decomposition of the triangular part of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector X, a column vector Z, and a scalar Y, SCHUD determines a unitary matrix U and a scalar ZETA such that (R Z) (RR ZZ ) U * ( ) = ( ) , (X Y) ( 0 ZETA) where RR is upper triangular. If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) appended. In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the updated problem is SQRT(RHO**2 + ZETA**2). SCHUD will simultaneously update several triplets (Z,Y,RHO). For a less terse description of what SCHUD does and how it may be applied, see the LINPACK guide. The matrix U is determined as the product U(P)*...*U(1), where U(I) is a rotation in the (I,P+1) plane of the form ( C(I) S(I) ) ( ) . ( -S(I) C(I) ) The rotations are chosen so that C(I) is real. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SCHUD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SCHUD](https://www.netlib.org/slatec/lin/schud.f).

# Arguments

## `R`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDR, *).

REAL(LDR,P), where LDR. GE. P. contains the upper triangular matrix that is to be updated. The part of R below the diagonal is not referenced.

## `LDR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array R.

## `P`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the order of the matrix R.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(P). contains the row to be added to R. X is not altered by SCHUD.

## `Z`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDZ, *).

REAL(LDZ,NZ), where LDZ. GE. P. is an array containing NZ P-vectors to be updated with R. not altered by SCHUD.

## `LDZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array Z.

## `NZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of vectors to be updated. NZ may be zero, in which case Z, Y, and RHO are not referenced.

## `Y`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(NZ). contains the scalars for updating the vectors not altered by SCHUD.

## `RHO`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(NZ). contains the norms of the residual vectors that are to be updated. If RHO(J) is negative, it is left unaltered. contain the updated quantities.

## `C`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(P). contains the cosines of the transforming rotations.

## `S`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(P). contains the sines of the transforming rotations.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `R`: not a workspace argument
- `LDR`: not a workspace argument
- `X`: not a workspace argument
- `Z`: not a workspace argument
- `LDZ`: not a workspace argument
- `Y`: not a workspace argument
- `RHO`: not a workspace argument
- `C`: not a workspace argument
- `S`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::schud`
- Original SLATEC routine: `SCHUD`
- Native symbol: `schud_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SCHUD](https://www.netlib.org/slatec/lin/schud.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
