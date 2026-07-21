# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM INTEGER NEL, IEL(NEL), JEL(NEL), IWARN DOUBLE PRECISION A(NELT), EL(NEL), D(N), R(N) CALL DSICS( N, NELT, IA, JA, A, ISYM, NEL, IEL, JEL, EL, D, R, $ IWARN ) =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double precision array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA,

# Description

This canonical unsafe binding exposes original SLATEC routine `DSICS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSICS](https://www.netlib.org/slatec/lin/dsics.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Order of the Matrix. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a NELT+1, where N is the number of rows not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of elements in arrays IA, JA, and A. zeros  in the matrix. Here is an example of the SLAP Row storage format for a  5x5 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer IA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we NELT+1, where N is the number of rows 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| With the SLAP  format some  of  the   "inner  loops" of this routine should vectorize  on  machines with hardware support for vector   gather/scatter  operations.  Your compiler  may require a compiler directive to  convince it that  there are no  implicit  vector  dependencies.  Compiler directives for the Alliant    FX/Fortran and CRI   CFT/CFT77 compilers  are supplied with the standard SLAP distribution. The IC factorization does not always exist for SPD matrices. In the event that a zero pivot is found it is set  to be 1.0 and the factorization proceeds.   The integer variable IWARN is set to the last row where the Diagonal was fudged.  This eventuality hardly ever occurs in practice. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer JA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), 1) points to  the  end of the   ICOL-th column. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| ==================== S L A P Row format ==================== This routine requires  that the matrix A  be  stored  in the SLAP  Row format.   In this format  the non-zeros are stored counting across  rows (except for the diagonal  entry, which must  appear first  in each  "row")  and  are stored  in the double precision  array A.  In other words, for each row  in the matrix  put the diagonal  entry in A.   Then put in  the other  non-zero elements  going across  the row  (except the diagonal) in order.  The JA array holds the column index for each non-zero.  The IA array holds the offsets  into the JA, 1), A(IA(IROW+1)-1) 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 INOUT    Integer JA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), 1) points to  the  end of the   ICOL-th column. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| ==================== S L A P Row format ==================== This routine requires  that the matrix A  be  stored  in the SLAP  Row format.   In this format  the non-zeros are stored counting across  rows (except for the diagonal  entry, which must  appear first  in each  "row")  and  are stored  in the double precision  array A.  In other words, for each row  in the matrix  put the diagonal  entry in A.   Then put in  the other  non-zero elements  going across  the row  (except the diagonal) in order.  The JA array holds the column index for each non-zero.  The IA array holds the offsets  into the JA, 1), A(IA(IROW+1)-1) 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NELT). INOUT    Double Precision A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. tion goes well.  It is set to the row index corresponding to the last zero pivot found.  See "Description", below. arrays  for  the  beginning  of each   column.   That  is, 1) points to  the  end of the   ICOL-th column. denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 arrays  for  the   beginning  of  each  row.    That  is, 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  3    4  5    6  7    8    9 10 11 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the lower triangle of the matrix is stored. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `NEL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT      Integer. Number of non-zeros in the lower triangle of A.   Also corresponds to the length of the IEL, JEL, EL arrays. OUT      Integer. Number of non-zeros in the lower triangle of A.   Also corresponds to the length of the IEL, JEL, EL arrays. not applicable or not stated by selected source not a workspace argument

## 8. `IEL`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NEL). OUT      Integer IEL(NEL). contain the unit lower triangular factor  of the incomplete decomposition   of the A  matrix  stored  in  SLAP Row format.   The Diagonal of   ones   *IS*   stored.     See "Description", below for more details about the SLAP Row fmt. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `JEL`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NEL). OUT      Integer JEL(NEL). contain the unit lower triangular factor  of the incomplete decomposition   of the A  matrix  stored  in  SLAP Row format.   The Diagonal of   ones   *IS*   stored.     See "Description", below for more details about the SLAP Row fmt. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `EL`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NEL). OUT      Double Precision EL(NEL). contain the unit lower triangular factor  of the incomplete decomposition   of the A  matrix  stored  in  SLAP Row format.   The Diagonal of   ones   *IS*   stored.     See "Description", below for more details about the SLAP Row fmt. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `D`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). trans, of a symmetric positive definite matrix, A, which is stored in SLAP Column format.  The unit lower triangular matrix L is stored by rows, and the inverse of the diagonal matrix D is stored. OUT      Double Precision D(N) 1./DIAG(A). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `R`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). WORK     Double Precision R(N). Temporary double precision workspace needed for the factorization. not stated by selected source not applicable or not stated by selected source

## 13. `IWARN`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT      Integer. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `NELT`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `ISYM`: not a workspace argument
- `NEL`: not a workspace argument
- `IEL`: not a workspace argument
- `JEL`: not a workspace argument
- `EL`: not a workspace argument
- `D`: not a workspace argument
- `R`: WORK     Double Precision R(N). Temporary double precision workspace needed for the factorization.
- `IWARN`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsics`
- Original SLATEC routine: `DSICS`
- Native symbol: `dsics_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DSICS](https://www.netlib.org/slatec/lin/dsics.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
