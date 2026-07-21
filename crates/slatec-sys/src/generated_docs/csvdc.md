# Purpose

CSVDC is a subroutine to reduce a complex NxP matrix X by unitary transformations U and V to diagonal form. The diagonal elements S(I) are the singular values of X. The columns of U are the corresponding left singular vectors, and the columns of V the right singular vectors. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CSVDC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSVDC](https://www.netlib.org/slatec/lin/csvdc.f).

# Arguments

## 1. `X`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDX, *). COMPLEX(LDX,P), where LDX .GE. N. contains the matrix whose singular value decomposition is to be computed.  X is destroyed by CSVDC. is the bidiagonal matrix with the elements of S on its diagonal and the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array X. INTEGER. is the leading dimension of the array X. INTEGER. is the leading dimension of the array X. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the number of rows of the matrix X. contain the singular values of X arranged in descending order of magnitude. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `P`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the number of columns of the matrix X. contain the singular values of X arranged in descending order of magnitude. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `S`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). MIN(N+1,P). contain the singular values of X arranged in descending order of magnitude. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `E`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(P). ordinarily contains zeros.  However see the discussion of INFO for exceptions. diagonal (CTRANS(U) is the conjugate-transpose of U).  Thus the singular values of X and B are the same. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `U`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDU, *). COMPLEX(LDU,K), where LDU .GE. N.  If JOBA .EQ. 1 then K .EQ. N.  If JOBA .GE. 2 then K .EQ. MIN(N,P). contains the matrix of right singular vectors. is not referenced if JOBA .EQ. 0.  If N .LE. P or if JOBA .GT. 2, then U may be identified with X in the subroutine call. is the bidiagonal matrix with the elements of S on its diagonal and the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `LDU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array U (see below). INTEGER. is the leading dimension of the array U (see below). INTEGER. is the leading dimension of the array U (see below). not a workspace argument

## 9. `V`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDV, *). COMPLEX(LDV,P), where LDV .GE. P. contains the matrix of right singular vectors. is not referenced if JOB .EQ. 0.  If P .LE. N, then V may be identified with X in the subroutine call. is the bidiagonal matrix with the elements of S on its diagonal and the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `LDV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array V (see below). INTEGER. is the leading dimension of the array V (see below). INTEGER. is the leading dimension of the array V (see below). not a workspace argument

## 11. `WORK`

workspace `workspace` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N). is a scratch array. not stated by selected source not applicable or not stated by selected source

## 12. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. controls the computation of the singular vectors.  It has the decimal expansion AB with the following meaning A .EQ. 0    Do not compute the left singular vectors. A .EQ. 1    Return the N left singular vectors in U. A .GE. 2    Return the first MIN(N,P) left singular vectors in U. B .EQ. 0    Do not compute the right singular vectors. B .EQ. 1    Return the right singular vectors in V. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. The singular values (and their corresponding singular vectors) S(INFO+1),S(INFO+2),...,S(M) are correct (here M=MIN(N,P)).  Thus if 0, all the singular values and their vectors are correct.  In any event, the matrix not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `LDX`: not a workspace argument
- `N`: not a workspace argument
- `P`: not a workspace argument
- `S`: not a workspace argument
- `E`: not a workspace argument
- `U`: not a workspace argument
- `LDU`: not a workspace argument
- `V`: not a workspace argument
- `LDV`: not a workspace argument
- `WORK`: COMPLEX(N). is a scratch array.
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::csvdc`
- Original SLATEC routine: `CSVDC`
- Native symbol: `csvdc_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank2,mut_i32,mut_complex32_array_rank2,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CSVDC](https://www.netlib.org/slatec/lin/csvdc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
