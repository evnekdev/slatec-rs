# Purpose

CCHDD downdates an augmented Cholesky decomposition or the triangular factor of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector X, a column vector Z, and a scalar Y, CCHDD determines a unitary matrix U and a scalar ZETA such that

# Description

This canonical unsafe binding exposes original SLATEC routine `CCHDD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CCHDD](https://www.netlib.org/slatec/lin/cchdd.f).

# Arguments

## 1. `R`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDR, *). Z )     (RR  ZZ) U * (      )  =  (      ) , (0 ZETA)     ( X   Y) where RR is upper triangular.  If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) removed.  In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the downdated problem is COMPLEX(LDR,P), where LDR .GE. P. contains the upper triangular matrix that is to be downdated.  The part of R below the diagonal is not referenced. is not altered by CCHDD. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array R. INTEGER. is the leading dimension of the array R. INTEGER. is the leading dimension of the array R. not a workspace argument

## 3. `P`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. plane of the form INTEGER. is the order of the matrix R. vectors which are to be downdated along with R. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `X`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(P). contains the row vector that is to is not altered by CCHDD. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `Z`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDZ, *). COMPLEX(LDZ,NZ), where LDZ .GE. P. vectors which are to be downdated along with R. is not altered by CCHDD. contain the downdated quantities. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `LDZ`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array Z. INTEGER. is the leading dimension of the array Z. INTEGER. is the leading dimension of the array Z. not a workspace argument

## 7. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. vectors which are to be downdated along with R. INTEGER. is the number of vectors to be downdated may be zero, in which case Z, Y, and RHO are not referenced. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `Y`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(NZ). contains the scalars for the downdating is not altered by CCHDD. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RHO`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). ZETA**2).  CCHDD will simultaneously downdate several triplets (Z,Y,RHO) along with R. For a less terse description of what CCHDD does and how it may be applied, see the LINPACK Guide. The matrix U is determined as the product U(1)*...*U(P) REAL(NZ). contains the norms of the residual vectors that are to be downdated. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `C`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). CONJG(S(I)) ) (                    ) . ( S(I)       C(I)    ) the rotations are chosen so that C(I) is real. The user is warned that a given downdating problem may be impossible to accomplish or may produce inaccurate results.  For example, this can happen if X is near a vector whose removal will reduce the rank of R.  Beware. On Entry REAL(P). contains the cosines of the transforming rotations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `S`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(P). contains the sines of the transforming rotations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is set as follows. 0  if the entire downdating was successful. 1  if R could not be downdated. in this case, all quantities are left unaltered. 1  if some RHO could not be downdated.  The offending RHO's are set to -1. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `R`: not a workspace argument
- `LDR`: not a workspace argument
- `P`: not a workspace argument
- `X`: not a workspace argument
- `Z`: not a workspace argument
- `LDZ`: not a workspace argument
- `NZ`: not a workspace argument
- `Y`: not a workspace argument
- `RHO`: not a workspace argument
- `C`: not a workspace argument
- `S`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchdd`
- Original SLATEC routine: `CCHDD`
- Native symbol: `cchdd_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CCHDD](https://www.netlib.org/slatec/lin/cchdd.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
