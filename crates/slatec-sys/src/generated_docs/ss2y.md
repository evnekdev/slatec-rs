# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM REAL A(NELT) CALL SS2Y( N, NELT, IA, JA, A, ISYM ) The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the SLAP Triad format or 2) the SLAP Column format. The user can hand this routine either of the of these data structures. If the SLAP Triad format is give as input then this routine transforms it into SLAP Column format. The way this routine tells which format is given as

# Description

This canonical unsafe binding exposes original SLATEC routine `SS2Y`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SS2Y](https://www.netlib.org/slatec/lin/ss2y.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer Order of the Matrix. NELT+1 then we NELT+1 then we have the SLAP Column format.  If that equality does not hold have the SLAP Column format.  If that equality does not hold NELT+1, where  N  is the number of columns in  the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of non-zeros stored in A. is  the number  of is  the number  of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For each non-zero the user puts the row and column index of that each non-zero the user puts the row and column index of that matrix element  in the IA and  JA arrays.  The  value of the matrix element  in the IA and  JA arrays.  The  value of the non-zero   matrix  element is  placed  in  the corresponding non-zero   matrix  element is  placed  in  the corresponding location of the A array.   This is  an  extremely  easy data location of the A array.   This is  an  extremely  easy data structure to generate.  On  the  other hand it   is  not too structure to generate.  On  the  other hand it   is  not too efficient on vector computers for  the iterative solution of efficient on vector computers for  the iterative solution of linear systems.  Hence,   SLAP changes   this  input    data linear systems.  Hence,   SLAP changes   this  input    data structure to the SLAP Column format  for  the iteration (but structure to the SLAP Column format  for  the iteration (but does not change it back). does not change it back). Here is an example of the  SLAP Triad   storage format for a Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  3  4  5  6  7  8  9 10 11 zeros in the matrix. Here is an example of the  SLAP Column  storage format for a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `IA`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer IA(NELT). contain the SLAP Triad format. =================== S L A P Triad format =================== This routine requires that the  matrix A be   stored in  the SLAP  Triad format.  In  this format only the non-zeros  are stored.  They may appear in  *ANY* order.  The user supplies 5  1  1  3  1  5  5  2  3  4  2 zero. The JA array holds the offsets into the IA, A arrays for the beginning of   each    column.    That  is,    IA(JA(ICOL)), 1),  A(JA(ICOL+1)-1) points to the 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `JA`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer JA(NELT). NELT+1 then we NELT+1 then we have the SLAP Column format.  If that equality does not hold have the SLAP Column format.  If that equality does not hold contain the SLAP Triad format. =================== S L A P Triad format =================== This routine requires that the  matrix A be   stored in  the SLAP  Triad format.  In  this format only the non-zeros  are stored.  They may appear in  *ANY* order.  The user supplies 1  2  1  3  5  3  5  2  5  4  1 | 0  0  0 44  0| |51  0 53  0 55| =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the real array A.  In other words, for each column in the matrix th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| INOUT    Integer JA(NELT). NELT+1 then we NELT+1 then we have the SLAP Column format.  If that equality does not hold have the SLAP Column format.  If that equality does not hold contain the SLAP Triad format. =================== S L A P Triad format =================== This routine requires that the  matrix A be   stored in  the SLAP  Triad format.  In  this format only the non-zeros  are stored.  They may appear in  *ANY* order.  The user supplies 1  2  1  3  5  3  5  2  5  4  1 | 0  0  0 44  0| |51  0 53  0 55| =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the real array A.  In other words, for each column in the matrix th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| not applicable or not stated by selected source not a workspace argument

## 5. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NELT). INOUT    Real A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format.  See "Description", below.  If the SLAP Triad format is used, this format is translated to the SLAP Column format by this routine. contain the SLAP Triad format. =================== S L A P Triad format =================== This routine requires that the  matrix A be   stored in  the SLAP  Triad format.  In  this format only the non-zeros  are stored.  They may appear in  *ANY* order.  The user supplies 51 12 11 33 15 53 55 22 35 44 21 zero elements going down   the  column (except  the diagonal)  in th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the lower triangle of the matrix is stored. not stated by selected source not applicable or not stated by selected source not a workspace argument

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

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::ss2y`
- Original SLATEC routine: `SS2Y`
- Native symbol: `ss2y_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SS2Y](https://www.netlib.org/slatec/lin/ss2y.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
