# Purpose

CCHDD downdates an augmented Cholesky decomposition or the triangular factor of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector X, a column vector Z, and a scalar Y, CCHDD determines a unitary matrix U and a scalar ZETA such that (R Z ) (RR ZZ) U * ( ) = ( ) , (0 ZETA) ( X Y) where RR is upper triangular. If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) removed. In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the downdated problem is SQRT(RHO**2 - ZETA**2). CCHDD will simultaneously downdate several triplets (Z,Y,RHO) along with R. For a less terse description of what CCHDD does and how it may be applied, see the LINPACK Guide. The matrix U is determined as the product U(1)*...*U(P) where U(I) is a rotation in the (P+1,I)-plane of the form ( C(I) -CONJG(S(I)) ) ( ) . ( S(I) C(I) ) the rotations are chosen so that C(I) is real. The user is warned that a given downdating problem may be impossible to accomplish or may produce inaccurate results. For example, this can happen if X is near a vector whose removal will reduce the rank of R. Beware. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CCHDD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CCHDD](https://www.netlib.org/slatec/lin/cchdd.f).

# Arguments

## `R`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDR, *).

COMPLEX(LDR,P), where LDR. GE. P. contains the upper triangular matrix that is to be downdated. The part of R below the diagonal is not referenced.

## `LDR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array R.

## `P`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the order of the matrix R.

## `X`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(P). contains the row vector that is to be removed from R. X is not altered by CCHDD.

## `Z`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDZ, *).

COMPLEX(LDZ,NZ), where LDZ. GE. P. is an array of NZ P-vectors which are to be downdated along with R. contain the downdated quantities.

## `LDZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array Z.

## `NZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of vectors to be downdated NZ may be zero, in which case Z, Y, and RHO are not referenced.

## `Y`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(NZ). contains the scalars for the downdating of the vectors Z. Y is not altered by CCHDD.

## `RHO`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(NZ). contains the norms of the residual vectors that are to be downdated.

## `C`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(P). contains the cosines of the transforming rotations.

## `S`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(P). contains the sines of the transforming rotations.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is set as follows. INFO = 0 if the entire downdating was successful. INFO =-1 if R could not be downdated. in this case, all quantities are left unaltered. INFO = 1 if some RHO could not be downdated.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 if the entire downdating was successful. |
| `INFO` | `-1` | if R could not be downdated. in this case, all quantities are left unaltered. |
| `INFO` | `1` | 1 if some RHO could not be downdated. The offending RHO's are set to -1. |

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

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchdd`
- Original SLATEC routine: `CCHDD`
- Native symbol: `cchdd_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CCHDD](https://www.netlib.org/slatec/lin/cchdd.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
