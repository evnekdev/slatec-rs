# Purpose

SBGCO factors a real band matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, SGBFA is slightly faster. To solve A*X = B , follow SBGCO by SGBSL. To compute INVERSE(A)*C , follow SBGCO by SGBSL. To compute DETERMINANT(A) , follow SBGCO by SGBDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SGBCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGBCO](https://www.netlib.org/slatec/lin/sgbco.f).

# Arguments

## 1. `ABD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) contains the matrix in band storage.  The columns of the matrix are stored in the columns of  ABD  and the diagonals of the matrix are stored in rows an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written  A = L*U  where L  is a product of permutation and unit lower triangular matrices and  U  is upper triangular. A(I,J) 10    CONTINUE 20 CONTINUE This uses rows  ML+1  through  2*ML+MU+1  of  ABD . are used for elements generated during the triangularization. The total number of rows needed in  ABD  is  2*ML+MU+1 . The  ML+MU by ML+MU  upper left triangle and the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABD . must be .GE. 2*ML + MU + 1 . INTEGER the leading dimension of the array  ABD . must be .GE. 2*ML + MU + 1 . INTEGER the leading dimension of the array  ABD . must be .GE. 2*ML + MU + 1 . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the original matrix. 6, ML = 1, MU = 2, LDA .GE. 5  and ABD should contain *  *  +  +  +  , * = not used * 13 24 35 46  , + = used for pivoting 12 23 34 45 56 11 22 33 44 55 66 21 32 43 54 65  * not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ML`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. through 2*ML+MU+1 of  ABD . See the comments below for details. INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . (band width below the diagonal) are used for elements generated during the triangularization. The total number of rows needed in  ABD  is  2*ML+MU+1 . The  ML+MU by ML+MU  upper left triangle and the are not referenced. are not referenced. Example:  If the original matrix is Example:  If the original matrix is 11 12 13  0  0  0 11 12 13  0  0  0 21 22 23 24  0  0 21 22 23 24  0  0 0 32 33 34 35  0 0 32 33 34 35  0 0  0 43 44 45 46 0  0 43 44 45 46 0  0  0 54 55 56 0  0  0 54 55 56 0  0  0  0 65 66 0  0  0  0 65 66 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if  ML .LE. MU . On Return (band width above the diagonal) M = ML + MU + 1 DO 20 J = 1, N I1 = MAX(1, J-MU) I2 = MIN(N, J+ML) DO 10 I = I1, I2 K = I - J + M not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `RCOND`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL an estimate of the reciprocal condition of  A . For the system  A*X = B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate underflows. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `Z`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) a work vector whose contents are usually unimportant. If  A  is close to a singular matrix, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . Band Storage If  A  is a band matrix, the following program segment will set up the input. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::sgbco`
- Original SLATEC routine: `SGBCO`
- Native symbol: `sgbco_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SGBCO](https://www.netlib.org/slatec/lin/sgbco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
