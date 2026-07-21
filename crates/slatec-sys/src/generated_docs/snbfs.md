# Purpose

Subroutine SNBFS solves a general nonsymmetric banded NxN system of single precision real linear equations using SLATEC subroutines SNBCO and SNBSL. These are adaptations of the LINPACK subroutines SBGCO and SGBSL, which require a different format for storing the matrix elements. If A is an NxN real matrix and if X and B are real

# Description

This canonical unsafe binding exposes original SLATEC routine `SNBFS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SNBFS](https://www.netlib.org/slatec/src/snbfs.f).

# Arguments

## 1. `ABE`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). A(I,J) 10    CONTINUE 20 CONTINUE This uses columns  1  through  ML+MU+1  of ABE . Furthermore,  ML  additional columns are needed in starting with column  ML+MU+2  for elements generated during the triangularization.  The total number of columns needed in  ABE  is  2*ML+MU+1 . Example:  If the original matrix is 11 12 13  0  0  0 21 22 23 24  0  0 0 32 33 34 35  0 0  0 43 44 45 46 0  0  0 54 55 56 0  0  0  0 65 66 REAL(LDA,NC) on entry, contains the matrix in band storage as described above.  NC  must not be less than A(I,J) 10    CONTINUE 20 CONTINUE This uses columns  1  through  ML+MU+1  of ABE . Furthermore,  ML  additional columns are needed in starting with column  ML+MU+2  for elements generated during the triangularization.  The total number of columns needed in  ABE  is  2*ML+MU+1 . Example:  If the original matrix is 11 12 13  0  0  0 21 22 23 24  0  0 0 32 33 34 35  0 0  0 43 44 45 46 0  0  0 54 55 56 0  0  0  0 65 66 REAL(LDA,NC) on entry, contains the matrix in band storage as described above.  NC  must not be less than not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER not stated by selected source INTEGER not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. vectors, then SNBFS solves the equation A*X=B. A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically  A(I,J) = 0 if  I-J is greater than  ML  or  J-I  is greater than 6, ML = 1, MU = 2, LDA .GE. 5  and ABE should contain 11 12 13  +     , * = not used 21 22 23 24  +     , + = used for pivoting 32 33 34 35  + 43 44 45 46  + 54 55 56  *  + 65 66  *  *  + Argument Description *** 1) INTEGER the order of the matrix A.  N must be greater 5) 6) vectors, then SNBFS solves the equation A*X=B. A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically  A(I,J) = 0 if  I-J is greater than  ML  or  J-I  is greater than 6, ML = 1, MU = 2, LDA .GE. 5  and ABE should contain 11 12 13  +     , * = not used 21 22 23 24  +     , + = used for pivoting 32 33 34 35  + 43 44 45 46  + 54 55 56  *  + 65 66  *  *  + Argument Description *** 1) INTEGER the order of the matrix A.  N must be greater 5) 6) not applicable or not stated by selected source not a workspace argument

## 4. `ML`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are called the lower and upper is the total band width. SNBFS uses less time and storage than the corresponding program for general matrices (SGEFS) if 2*ML+MU .LT. N  . The matrix A is first factored into upper and lower tri- angular matrices U and L using partial pivoting.  These factors and the pivoting information are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, (band width below the diagonal) is cautioned to specify  NC with care since it is not an argument and cannot be checked by SNBFS.  The rows of the original matrix are stored in the rows of  ABE  and the diagonals of the original matrix are stored in columns  1  through  ML+MU+1  of  ABE . on return, contains an upper triangular matrix U and the multipliers necessary to construct a matrix L so that A=L*U. INTEGER the number of diagonals below the main diagonal. must not be less than zero nor greater than or are called the lower and upper is the total band width. SNBFS uses less time and storage than the corresponding program for general matrices (SGEFS) if 2*ML+MU .LT. N  . The matrix A is first factored into upper and lower tri- angular matrices U and L using partial pivoting.  These factors and the pivoting information are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, (band width below the diagonal) is cautioned to specify  NC with care since it is not an argument and cannot be checked by SNBFS.  The rows of the original matrix are stored in the rows of  ABE  and the diagonals of the original matrix are stored in columns  1  through  ML+MU+1  of  ABE . on return, contains an upper triangular matrix U and the multipliers necessary to construct a matrix L so that A=L*U. INTEGER the number of diagonals below the main diagonal. must not be less than zero nor greater than or not applicable or not stated by selected source not a workspace argument

## 5. `MU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are called the lower and upper are called the lower and upper is the total band width. SNBFS uses less time and storage than the corresponding program for general matrices (SGEFS) if 2*ML+MU .LT. N  . The matrix A is first factored into upper and lower tri- angular matrices U and L using partial pivoting.  These factors and the pivoting information are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 is cautioned to specify  NC with care since it is not an argument and cannot be checked by SNBFS.  The rows of the original matrix are stored in the rows of  ABE  and the diagonals of the original matrix are stored in columns  1  through  ML+MU+1  of  ABE . on return, contains an upper triangular matrix U and the multipliers necessary to construct a matrix L so that A=L*U. INTEGER the number of diagonals above the main diagonal. must not be less than zero nor greater than or are called the lower and upper are called the lower and upper is the total band width. SNBFS uses less time and storage than the corresponding program for general matrices (SGEFS) if 2*ML+MU .LT. N  . The matrix A is first factored into upper and lower tri- angular matrices U and L using partial pivoting.  These factors and the pivoting information are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 is cautioned to specify  NC with care since it is not an argument and cannot be checked by SNBFS.  The rows of the original matrix are stored in the rows of  ABE  and the diagonals of the original matrix are stored in columns  1  through  ML+MU+1  of  ABE . on return, contains an upper triangular matrix U and the multipliers necessary to construct a matrix L so that A=L*U. INTEGER the number of diagonals above the main diagonal. must not be less than zero nor greater than or not applicable or not stated by selected source not a workspace argument

## 6. `V`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `ITASK`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1).  IND will not be changed by SNBFS in this case. Band Storage If  A  is a band matrix, the following program segment will set up the input. INTEGER 1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A and IWORK. 3 is printed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1) 2) 5) 6) 3 is printed. INTEGER GT. 0  IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0  See error message corresponding to IND below. 1  terminal   N is greater than LDA. 2  terminal   N is less than 1. 3  terminal   ITASK is less than 1. 4  terminal   the matrix A is computationally singular. A solution has not been computed. 5  terminal   ML is less than zero or is greater than or equal to N . 6  terminal   MU is less than zero or is greater than or equal to N . 10 warning    the solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) a singly subscripted array of dimension at least N. REAL(N) a singly subscripted array of dimension at least N. not applicable or not stated by selected source

## 10. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) a singly subscripted array of dimension at least N. INTEGER(N) a singly subscripted array of dimension at least N. not applicable or not stated by selected source

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
- `V`: not a workspace argument
- `ITASK`: not a workspace argument
- `IND`: not a workspace argument
- `WORK`: REAL(N) a singly subscripted array of dimension at least N.
- `IWORK`: INTEGER(N) a singly subscripted array of dimension at least N.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::snbfs`
- Original SLATEC routine: `SNBFS`
- Native symbol: `snbfs_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [SNBFS](https://www.netlib.org/slatec/src/snbfs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
