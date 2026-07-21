# Purpose

This routine scales (and unscales) the system Ax = b by symmetric diagonal scaling. The new system is: -1/2 -1/2 1/2 -1/2 =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double precision array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA,

# Description

This canonical unsafe binding exposes original SLATEC routine `DSDSCL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSDSCL](https://www.netlib.org/slatec/lin/dsdscl.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer Order of the Matrix. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of elements in arrays IA, JA, and A. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). IN       Integer IA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). IN       Integer JA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), 1) points to  the  end of the   ICOL-th column. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| With the SLAP  format  all  of  the   "inner  loops" of this routine should vectorize  on  machines with hardware support for vector   gather/scatter  operations.  Your compiler  may require a compiler directive to  convince it that  there are no  implicit  vector  dependencies.  Compiler directives for the Alliant    FX/Fortran and CRI   CFT/CFT77 compilers  are supplied with the standard SLAP distribution. Cautions: not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `A`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NELT). IN       Double Precision A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. arrays  for  the  beginning  of each   column.   That  is, 1) points to  the  end of the   ICOL-th column. denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 zero flow or overflow. This is done so that the loop  vectorizes. Matrices  with zero or near zero or very  large entries will have numerical difficulties  and  must  be fixed before this routine is called. IN       Double Precision A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. arrays  for  the  beginning  of each   column.   That  is, 1) points to  the  end of the   ICOL-th column. denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 zero flow or overflow. This is done so that the loop  vectorizes. Matrices  with zero or near zero or very  large entries will have numerical difficulties  and  must  be fixed before this routine is called. not applicable or not stated by selected source not a workspace argument

## 6. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `X`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). D    b when scaling is selected with the JOB parameter.  When unscaling is selected this process is reversed.  The true solution is also scaled or unscaled if ITOL is set appropriately, see below. Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, JOB, ITOL DOUBLE PRECISION A(NELT), X(N), B(N), DINV(N) CALL DSDSCL( N, NELT, IA, JA, A, ISYM, X, B, DINV, JOB, ITOL ) INOUT    Double Precision X(N). Initial guess that will be later used in the iterative solution. of the scaled system. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `B`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). INOUT    Double Precision B(N). Right hand side vector. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `DINV`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). INOUT    Double Precision DINV(N). Upon return this array holds 1./DIAG(A). flow or overflow. This is done so that the loop  vectorizes. Matrices  with zero or near zero or very  large entries will have numerical difficulties  and  must  be fixed before this routine is called. INOUT    Double Precision DINV(N). Upon return this array holds 1./DIAG(A). flow or overflow. This is done so that the loop  vectorizes. Matrices  with zero or near zero or very  large entries will have numerical difficulties  and  must  be fixed before this routine is called. not applicable or not stated by selected source not a workspace argument

## 10. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 0. IN       Integer. Flag indicating whether to scale or not. zero means do scaling. 0 means do unscaling. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `ITOL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag indicating what type of error estimation to do in the 11 the exact solution from common block DSLBLK will be used.  When the system is scaled then the true solution must also be scaled.  If ITOL is not 11 then this vector is not referenced. Common Blocks: SOLN    :INOUT   Double Precision SOLN(N).  COMMON BLOCK /DSLBLK/ The true solution, SOLN, is scaled (or unscaled) if ITOL is set to 11, see above. IN       Integer. Flag indicating what type of error estimation to do in the 11 the exact solution from common block DSLBLK will be used.  When the system is scaled then the true solution must also be scaled.  If ITOL is not 11 then this vector is not referenced. Common Blocks: SOLN    :INOUT   Double Precision SOLN(N).  COMMON BLOCK /DSLBLK/ The true solution, SOLN, is scaled (or unscaled) if ITOL is set to 11, see above. not applicable or not stated by selected source not a workspace argument

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
- `X`: not a workspace argument
- `B`: not a workspace argument
- `DINV`: not a workspace argument
- `JOB`: not a workspace argument
- `ITOL`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsdscl`
- Original SLATEC routine: `DSDSCL`
- Native symbol: `dsdscl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [DSDSCL](https://www.netlib.org/slatec/lin/dsdscl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
