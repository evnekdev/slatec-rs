# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM REAL A(NELT), DINV(N) CALL SSDS( N, NELT, IA, JA, A, ISYM, DINV ) =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix

# Description

This canonical unsafe binding exposes original SLATEC routine `SSDS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSDS](https://www.netlib.org/slatec/lin/ssds.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Order of the Matrix. NELT+1, where  N  is the number of columns in  the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of elements in arrays IA, JA, and A. zeros in the matrix. Here is an example of the  SLAP Column  storage format for a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer IA(NELT). zero. The JA array holds the offsets into the IA, A arrays for the beginning of   each    column.    That  is,    IA(JA(ICOL)), 1),  A(JA(ICOL+1)-1) points to the 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer JA(NELT). th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| With the SLAP  format  all  of  the   "inner  loops" of this routine should vectorize  on  machines with hardware support for vector   gather/scatter  operations.  Your compiler  may require a compiler directive to  convince it that  there are no  implicit  vector  dependencies.  Compiler directives for the Alliant    FX/Fortran and CRI   CFT/CFT77 compilers  are supplied with the standard SLAP distribution. Cautions: not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NELT). INOUT    Real A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. zero elements going down   the  column (except  the diagonal)  in th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 zero not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `DINV`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). OUT      Real DINV(N). Upon return this array holds 1./DIAG(A). 1.0/DIAG(A) will not underflow or overflow.    This  is done so that the  loop  vectorizes. Matrices  with zero or near zero or very  large entries will have numerical difficulties  and  must  be fixed before this routine is called. OUT      Real DINV(N). Upon return this array holds 1./DIAG(A). 1.0/DIAG(A) will not underflow or overflow.    This  is done so that the  loop  vectorizes. Matrices  with zero or near zero or very  large entries will have numerical difficulties  and  must  be fixed before this routine is called. not applicable or not stated by selected source not a workspace argument

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
- `DINV`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssds`
- Original SLATEC routine: `SSDS`
- Native symbol: `ssds_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SSDS](https://www.netlib.org/slatec/lin/ssds.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
