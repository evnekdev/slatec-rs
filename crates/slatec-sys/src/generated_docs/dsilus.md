# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM INTEGER NL, IL(NL), JL(NL), NU, IU(NU), JU(NU) INTEGER NROW(N), NCOL(N) DOUBLE PRECISION A(NELT), L(NL), DINV(N), U(NU) CALL DSILUS( N, NELT, IA, JA, A, ISYM, NL, IL, JL, L, $ DINV, NU, IU, JU, U, NROW, NCOL )

# Description

This canonical unsafe binding exposes original SLATEC routine `DSILUS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSILUS](https://www.netlib.org/slatec/lin/dsilus.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer Order of the Matrix. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a NELT+1, where N is the number of rows not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of elements in arrays IA, JA, and A. zeros  in the matrix. Here is an example of the SLAP Row storage format for a  5x5 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). IN       Integer IA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we NELT+1, where N is the number of rows 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). IN       Integer JA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), 1) points to  the  end of the   ICOL-th column. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| ==================== S L A P Row format ==================== This routine requires  that the matrix A  be  stored  in the SLAP  Row format.   In this format  the non-zeros are stored counting across  rows (except for the diagonal  entry, which must  appear first  in each  "row")  and  are stored  in the double precision  array A.  In other words, for each row  in the matrix  put the diagonal  entry in A.   Then put in  the other  non-zero elements  going across  the row  (except the diagonal) in order.  The JA array holds the column index for each non-zero.  The IA array holds the offsets  into the JA, 1), A(IA(IROW+1)-1) 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 IN       Integer JA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), 1) points to  the  end of the   ICOL-th column. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| ==================== S L A P Row format ==================== This routine requires  that the matrix A  be  stored  in the SLAP  Row format.   In this format  the non-zeros are stored counting across  rows (except for the diagonal  entry, which must  appear first  in each  "row")  and  are stored  in the double precision  array A.  In other words, for each row  in the matrix  put the diagonal  entry in A.   Then put in  the other  non-zero elements  going across  the row  (except the diagonal) in order.  The JA array holds the column index for each non-zero.  The IA array holds the offsets  into the JA, 1), A(IA(IROW+1)-1) 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NELT). IN       Double Precision A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. arrays  for  the  beginning  of each   column.   That  is, 1) points to  the  end of the   ICOL-th column. denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 arrays  for  the   beginning  of  each  row.    That  is, 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  3    4  5    6  7    8    9 10 11 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the lower triangle of the matrix is stored. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `NL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT      Integer. Number of non-zeros in the L array. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `IL`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NL). OUT      Integer IL(NL). contain the unit lower triangular factor of  the incomplete decomposition  of some  matrix stored  in   SLAP Row format.     The   Diagonal  of ones  *IS*  stored.  See "DESCRIPTION", below for more details about the SLAP format. contain the unit  lower triangular factor of the incomplete decomposition of the A matrix  stored in SLAP not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `JL`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NL). OUT      Integer JL(NL). contain the unit lower triangular factor of  the incomplete decomposition  of some  matrix stored  in   SLAP Row format.     The   Diagonal  of ones  *IS*  stored.  See "DESCRIPTION", below for more details about the SLAP format. contain the unit  lower triangular factor of the incomplete decomposition of the A matrix  stored in SLAP not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `L`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NL). 60 Livermore, CA 94550 (510) 423-3141 seager@llnl.gov OUT      Double Precision L(NL). contain the unit lower triangular factor of  the incomplete decomposition  of some  matrix stored  in   SLAP Row format.     The   Diagonal  of ones  *IS*  stored.  See "DESCRIPTION", below for more details about the SLAP format. contain the unit  lower triangular factor of the incomplete decomposition of the A matrix  stored in SLAP not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `DINV`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). Writable array of at least `N` entries. On return it stores the inverse diagonal factor `D^-1` from the no-fill incomplete LDU preconditioner; it is consumed by the related SLAP solve routines. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `NU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT      Integer. Number of non-zeros in the U array. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `IU`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NU). OUT      Integer IU(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, OUT      Integer IU(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, not applicable or not stated by selected source not a workspace argument

## 14. `JU`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NU). OUT      Integer JU(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, OUT      Integer JU(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, not applicable or not stated by selected source not a workspace argument

## 15. `U`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NU). OUT      Double Precision     U(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, OUT      Double Precision     U(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, not applicable or not stated by selected source not a workspace argument

## 16. `NROW`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (N). WORK     Integer NROW(N). zero elements in the I-th row of L. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `NCOL`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (N). WORK     Integer NCOL(N). zero elements in the I-th column of U. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `NL`: not a workspace argument
- `IL`: not a workspace argument
- `JL`: not a workspace argument
- `L`: not a workspace argument
- `DINV`: not a workspace argument
- `NU`: not a workspace argument
- `IU`: not a workspace argument
- `JU`: not a workspace argument
- `U`: not a workspace argument
- `NROW`: not a workspace argument
- `NCOL`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsilus`
- Original SLATEC routine: `DSILUS`
- Native symbol: `dsilus_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DSILUS](https://www.netlib.org/slatec/lin/dsilus.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
