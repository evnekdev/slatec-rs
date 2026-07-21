# Purpose

This routine scales (and unscales) the system Ax = b by symmetric diagonal scaling. The new system is: -1/2 -1/2 1/2 -1/2 D AD (D x) = D b when scaling is selected with the JOB parameter. When unscaling is selected this process is reversed. The true solution is also scaled or unscaled if ITOL is set appropriately, see below. Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, JOB, ITOL REAL A(NELT), X(N), B(N), DINV(N) CALL SSDSCL( N, NELT, IA, JA, A, ISYM, X, B, DINV, JOB, ITOL ) =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have

# Description

This canonical unsafe binding exposes original SLATEC routine `SSDSCL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSDSCL](https://www.netlib.org/slatec/lin/ssdscl.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of the Matrix.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of elements in arrays IA, JA, and A.

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

IA(NELT).

## `JA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

JA(NELT). NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| With the SLAP format all of the "inner loops" of this routine should vectorize on machines with hardware support for vector gather/scatter operations. Your compiler may require a compiler directive to convince it that there are no implicit vector dependencies. Compiler directives for the Alliant FX/Fortran and CRI CFT/CFT77 compilers are supplied with the standard SLAP distribution.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (NELT).

A(NELT). These arrays should hold the matrix A in the SLAP Column format. See "Description", below.

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

X(N). Initial guess that will be later used in the iterative solution. of the scaled system.

## `B`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

B(N). Right hand side vector.

## `DINV`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

DINV(N). Upon return this array holds 1. /DIAG(A). This is an input if JOB = 0.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag indicating whether to scale or not. JOB non-zero means do scaling. 0 means do unscaling.

## `ITOL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag indicating what type of error estimation to do in the iterative method. When ITOL = 11 the exact solution from common block SSLBLK will be used. When the system is scaled then the true solution must also be scaled. If ITOL is not 11 then this vector is not referenced. Common Blocks: SOLN :INOUT Real SOLN(N).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `X`: not a workspace argument
- `B`: not a workspace argument
- `DINV`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssdscl`
- Original SLATEC routine: `SSDSCL`
- Native symbol: `ssdscl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SSDSCL](https://www.netlib.org/slatec/lin/ssdscl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
