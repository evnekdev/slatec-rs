# Purpose

CGBFA factors a complex band matrix by elimination. CGBFA is usually called by CGBCO, but it can be called directly with a saving in time if RCOND is not needed. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CGBFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGBFA](https://www.netlib.org/slatec/lin/cgbfa.f).

# Arguments

## 1. `ABD`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). COMPLEX(LDA, N) contains the matrix in band storage.  The columns of the matrix are stored in the columns of  ABD  and the diagonals of the matrix are stored in rows an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written  A = L*U  where L  is a product of permutation and unit lower triangular matrices and  U  is upper triangular. A(I,J) 10    CONTINUE 20 CONTINUE This uses rows  ML+1  through  2*ML+MU+1  of  ABD . are used for elements generated during the triangularization. The total number of rows needed in  ABD  is  2*ML+MU+1 . The  ML+MU by ML+MU  upper left triangle and the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABD . must be .GE. 2*ML + MU + 1 . INTEGER the leading dimension of the array  ABD . must be .GE. 2*ML + MU + 1 . INTEGER the leading dimension of the array  ABD . must be .GE. 2*ML + MU + 1 . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the original matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ML`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. through 2*ML+MU+1 of  ABD . See the comments below for details. INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . (band width below the diagonal) are used for elements generated during the triangularization. The total number of rows needed in  ABD  is  2*ML+MU+1 . The  ML+MU by ML+MU  upper left triangle and the are not referenced. are not referenced. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if  ML .LE. MU . On Return (band width above the diagonal) M = ML + MU + 1 DO 20 J = 1, N I1 = MAX(1, J-MU) I2 = MIN(N, J+ML) DO 10 I = I1, I2 K = I - J + M not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IPVT`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0  normal value. = K  if  U(K,K) .EQ. 0.0 .  This is not an error condition for this subroutine, but it does indicate that CGBSL will divide by zero if called.  Use  RCOND  in CGBCO for a reliable indication of singularity. Band Storage If  A  is a band matrix, the following program segment will set up the input. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `ML`: not a workspace argument
- `MU`: not a workspace argument
- `IPVT`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cgbfa`
- Original SLATEC routine: `CGBFA`
- Native symbol: `cgbfa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_array_rank1,mut_i32)`
- Exact Netlib source file: [CGBFA](https://www.netlib.org/slatec/lin/cgbfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
