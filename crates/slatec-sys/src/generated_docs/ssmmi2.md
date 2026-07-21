# Purpose

Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) This routine is supplied with the SLAP package as a routine to perform the MSOLVE operation in the SBCGN iteration routine for the driver SSLUCN. It must be called via the SLAP MSOLVE calling sequence convention interface routine SSMMTI. THIS ROUTINE ITSELF DOES NOT CONFORM TO THE **** SLAP MSOLVE CALLING CONVENTION **** IL, JL, L should contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. IU, JU, U should contain the unit upper factor SLAP Column format This ILU factorization can be computed by the SSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| ==================== S L A P Row format ==================== SLAP Row format. In this format the non-zeros are stored counting across rows (except for the diagonal entry, which must appear first in each "row") and are stored in the real array A. In other words, for each row in the matrix put the elements going across the row (except the diagonal) in order. The JA array holds the column index for each non-zero. The IA array holds the offsets into the JA, A arrays for the beginning of each row. That is, JA(IA(IROW)), A(IA(IROW)) points to the beginning of the IROW-th row in JA and A. JA(IA(IROW+1)-1), A(IA(IROW+1)-1) points to the end of the IROW-th row. Note that we always have IA(N+1) = NELT+1, where N is the number of rows in Here is an example of the SLAP Row storage format for a 5x5 Matrix (in the A and JA arrays '|' denotes the end of a row): 5x5 Matrix SLAP Row format for 5x5 matrix on left. |11 12 0 0 15| A: 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 |21 22 0 0 0| JA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| IA: 1 4 6 8 9 12 With the SLAP format the "inner loops" of this routine should vectorize on machines with hardware support for vector gather/scatter operations. Your compiler may require a compiler directive to convince it that there are no implicit vector dependencies. Compiler directives for the Alliant FX/Fortran and CRI CFT/CFT77 compilers are supplied with the standard SLAP distribution. SEE ALSO SSILUS

# Description

This canonical unsafe binding exposes original SLATEC routine `SSMMI2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSMMI2](https://www.netlib.org/slatec/lin/ssmmi2.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of the Matrix.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

B(N). Right hand side.

## `X`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

X(N). Solution of (L*D*U)(L*D*U)trans x = b.

## `IL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

IL(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array.

## `JL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

JL(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array.

## `L`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

L(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array.

## `DINV`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

DINV(N). Inverse of the diagonal matrix D.

## `IU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

IU(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array.

## `JU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

JU(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array.

## `U`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

U(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `B`: not a workspace argument
- `X`: not a workspace argument
- `IL`: not a workspace argument
- `JL`: not a workspace argument
- `L`: not a workspace argument
- `DINV`: not a workspace argument
- `IU`: not a workspace argument
- `JU`: not a workspace argument
- `U`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssmmi2`
- Original SLATEC routine: `SSMMI2`
- Native symbol: `ssmmi2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SSMMI2](https://www.netlib.org/slatec/lin/ssmmi2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
