# Purpose

CNBCO factors a complex band matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, CNBFA is slightly faster. To solve A*X = B , follow CNBCO by CNBSL. To compute INVERSE(A)*C , follow CNBCO by CNBSL. To compute DETERMINANT(A) , follow CNBCO by CNBDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CNBCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CNBCO](https://www.netlib.org/slatec/src/cnbco.f).

# Arguments

## 1. `ABE`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). COMPLEX(LDA, NC) contains the matrix in band storage.  The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE. NC must be .GE. 2*ML+MU+1 . See the comments below for details. an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written  A = L*U  where L is a product of permutation and unit lower triangular matrices and  U  is upper triangular. A(I,J) 10    CONTINUE 20 CONTINUE This uses columns  1  through  ML+MU+1  of ABE . Furthermore,  ML  additional columns are needed in starting with column  ML+MU+2  for elements generated during the triangularization.  The total number of columns needed in  ABE  is  2*ML+MU+1 . Example:  If the original matrix is 11 12 13  0  0  0 21 22 23 24  0  0 0 32 33 34 35  0 0  0 43 44 45 46 0  0  0 54 55 56 0  0  0  0 65 66 COMPLEX(LDA, NC) contains the matrix in band storage.  The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE. NC must be .GE. 2*ML+MU+1 . See the comments below for details. an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written  A = L*U  where L is a product of permutation and unit lower triangular matrices and  U  is upper triangular. A(I,J) 10    CONTINUE 20 CONTINUE This uses columns  1  through  ML+MU+1  of ABE . Furthermore,  ML  additional columns are needed in starting with column  ML+MU+2  for elements generated during the triangularization.  The total number of columns needed in  ABE  is  2*ML+MU+1 . Example:  If the original matrix is 11 12 13  0  0  0 21 22 23 24  0  0 0 32 33 34 35  0 0  0 43 44 45 46 0  0  0 54 55 56 0  0  0  0 65 66 not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array ABE. must be .GE. N . INTEGER the leading dimension of the array ABE. must be .GE. N . INTEGER the leading dimension of the array ABE. must be .GE. N . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the original matrix. 6, ML = 1, MU = 2, LDA .GE. 5  and ABE should contain 11 12 13  +     , * = not used 21 22 23 24  +     , + = used for pivoting 32 33 34 35  + 43 44 45 46  + 54 55 56  *  + 65 66  *  *  + not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ML`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . (band width below the diagonal) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if ML .LE. MU . On Return (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `RCOND`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL an estimate of the reciprocal condition of  A . For the system  A*X = B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate underflows. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `Z`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) a work vector whose contents are usually unimportant. If  A  is close to a singular matrix, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . Band Storage If  A  is a band matrix, the following program segment will set up the input. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `ABE`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `ML`: not a workspace argument
- `MU`: not a workspace argument
- `IPVT`: not a workspace argument
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cnbco`
- Original SLATEC routine: `CNBCO`
- Native symbol: `cnbco_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_array_rank1,mut_f32,mut_complex32_array_rank1)`
- Exact Netlib source file: [CNBCO](https://www.netlib.org/slatec/src/cnbco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
