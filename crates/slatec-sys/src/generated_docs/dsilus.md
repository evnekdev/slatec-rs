# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM INTEGER NL, IL(NL), JL(NL), NU, IU(NU), JU(NU) INTEGER NROW(N), NCOL(N) DOUBLE PRECISION A(NELT), L(NL), DINV(N), U(NU) CALL DSILUS( N, NELT, IA, JA, A, ISYM, NL, IL, JL, L, $ DINV, NU, IU, JU, U, NROW, NCOL ) IL, JL, L should contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. IU, JU, U should contain the unit upper factor SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double precision array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is,

# Description

This canonical unsafe binding exposes original SLATEC routine `DSILUS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSILUS](https://www.netlib.org/slatec/lin/dsilus.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of the Matrix.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of elements in arrays IA, JA, and A.

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

IA(NELT). A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| ==================== S L A P Row format ==================== This routine requires that the matrix A be stored in the SLAP Row format.

## `JA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

JA(NELT). the first elements of the IROW- th row in JA and A, and JA(IA(IROW+1)-1), A(IA(IROW+1)-1) are the last elements of the IROW-th row. Note that we always have IA(N+1) = NELT+1, where N is the number of rows in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Row storage format for a 5x5 Matrix (in the A and JA arrays '|' denotes the end of a row): 5x5 Matrix SLAP Row format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 |21 22 0 0 0| JA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| IA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| SEE ALSO SILUR.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NELT).

Double Precision A(NELT). These arrays should hold the matrix A in the SLAP Column format. See "Description", below. arrays for the beginning of each row. That is, the first elements of the IROW- th row in JA and A, and JA(IA(IROW+1)-1), A(IA(IROW+1)-1) are the last elements of the IROW-th row. Note that we always have IA(N+1) = NELT+1, where N is the number of rows in the matrix and NELT is the number of non-zeros in the matrix.

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the lower triangle of the matrix is stored.

## `NL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of non-zeros in the L array.

## `IL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NL).

IL(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The Diagonal of ones *IS* stored. See "DESCRIPTION", below for more details about the SLAP format.

## `JL`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NL).

JL(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The Diagonal of ones *IS* stored. See "DESCRIPTION", below for more details about the SLAP format.

## `L`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NL).

Double Precision L(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The Diagonal of ones *IS* stored. See "DESCRIPTION", below for more details about the SLAP format.

## `DINV`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Writable array of at least `N` entries. On return it stores the inverse diagonal factor `D^-1` from the no-fill incomplete LDU preconditioner; it is consumed by the related SLAP solve routines.

## `NU`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of non-zeros in the U array.

## `IU`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NU).

IU(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The Diagonal of ones *IS* stored. See "Description", below for more details about the SLAP.

## `JU`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NU).

JU(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The Diagonal of ones *IS* stored. See "Description", below for more details about the SLAP.

## `U`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NU).

Double Precision U(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The Diagonal of ones *IS* stored. See "Description", below for more details about the SLAP.

## `NROW`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (N).

NROW(N). is the number of non-zero elements in the I-th row of L.

## `NCOL`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (N).

NCOL(N). is the number of non-zero elements in the I-th column of U.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `IL`: not a workspace argument
- `JL`: not a workspace argument
- `L`: not a workspace argument
- `DINV`: not a workspace argument
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
