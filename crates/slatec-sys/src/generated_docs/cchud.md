# Purpose

CCHUD updates an augmented Cholesky decomposition of the triangular part of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector

# Description

This canonical unsafe binding exposes original SLATEC routine `CCHUD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CCHUD](https://www.netlib.org/slatec/lin/cchud.f).

# Arguments

## 1. `R`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDR, *). Z)     (RR   ZZ ) U  * (    )  =  (        ) , COMPLEX(LDR,P), where LDR .GE. P. contains the upper triangular matrix that is to be updated.  The part of R below the diagonal is not referenced. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array R. INTEGER. is the leading dimension of the array R. INTEGER. is the leading dimension of the array R. not a workspace argument

## 3. `P`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the order of the matrix R. vectors to be updated with R. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `X`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). a column vector Z, and a scalar Y, CCHUD determines a unitary matrix U and a scalar ZETA such that Y)     ( 0  ZETA) where RR is upper triangular.  If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) appended.  In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the updated problem is COMPLEX(P). contains the row to be added to R.  X is not altered by CCHUD. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `Z`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDZ, *). COMPLEX(LDZ,NZ), where LDZ .GE. P. vectors to be updated with R. is not altered by CCHUD. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `LDZ`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array Z. INTEGER. is the leading dimension of the array Z. INTEGER. is the leading dimension of the array Z. not a workspace argument

## 7. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. vectors to be updated with R. INTEGER. is the number of vectors to be updated may be zero, in which case Z, Y, and RHO are not referenced. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `Y`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(NZ). contains the scalars for updating the vectors is not altered by CCHUD. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RHO`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). + ZETA**2).  CCHUD will simultaneously update several triplets (Z,Y,RHO). For a less terse description of what CCHUD does and how it may be applied see the LINPACK Guide. The matrix U is determined as the product U(P)*...*U(1), where U(I) is a rotation in the (I,P+1) plane of the form (     (CI)      S(I) ) (                    ) . ( -CONJG(S(I))  (CI) ) The rotations are chosen so that C(I) is real. On Entry REAL(NZ). contains the norms of the residual vectors that are to be updated.  If RHO(J) is negative, it is left unaltered. On Return RC contain the updated quantities. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `C`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(P). contains the cosines of the transforming rotations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `S`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(P). contains the sines of the transforming rotations. not stated by selected source not applicable or not stated by selected source not a workspace argument

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

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchud`
- Original SLATEC routine: `CCHUD`
- Native symbol: `cchud_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_complex32_array_rank1)`
- Exact Netlib source file: [CCHUD](https://www.netlib.org/slatec/lin/cchud.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
