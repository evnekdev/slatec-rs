# Purpose

Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) This routine is supplied with the SLAP package as a routine to perform the MSOLVE operation in the SIR iteration routine for the driver routine SSGS. It must be called via the SLAP MSOLVE calling sequence convention interface routine SSLI. THIS ROUTINE ITSELF DOES NOT CONFORM TO THE **** SLAP MSOLVE CALLING CONVENTION **** ==================== S L A P Row format ==================== This routine requires that the matrix A be stored in the SLAP Row format. In this format the non-zeros are stored counting across rows (except for the diagonal entry, which must appear first in each "row") and are stored in the real array A. In other words, for each row in the matrix put the diagonal entry in A. Then put in the other non-zero elements going across the row (except the diagonal) in order. The JA array holds the column index for each non-zero. The IA array holds the offsets into the JA, A arrays for the beginning of each row. That is, JA(IA(IROW)), A(IA(IROW)) points to the beginning of the IROW-th row in JA and A. JA(IA(IROW+1)-1), A(IA(IROW+1)-1) points to the end of the IROW-th row. Note that we always

# Description

This canonical unsafe binding exposes original SLATEC routine `SSLI2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSLI2](https://www.netlib.org/slatec/lin/ssli2.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer Order of the Matrix. NELT+1, where  N  is  the number of rows  in the matrix  and NELT  is the  number   of  non-zeros in  the matrix. Here is an example of the SLAP Row storage format for a  5x5 Matrix (in the A and JA arrays '|' denotes the end of a row): 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 |11 12  0  0 15|   A: 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 |21 22  0  0  0|  JA:  1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 | 0  0 33  0 35|  IA:  1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| With  the SLAP  Row format  the "inner loop" of this routine should vectorize   on machines with   hardware  support  for vector gather/scatter operations.  Your compiler may require a  compiler directive  to  convince   it that there  are  no implicit vector  dependencies.  Compiler directives  for the Alliant FX/Fortran and CRI CFT/CFT77 compilers  are supplied with the standard SLAP distribution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). IN       Real B(N). Right hand side vector. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `X`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). OUT      Real X(N). Solution to Lx = b. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NEL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of non-zeros in the EL array. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IEL`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NEL). IN       Integer IEL(NEL). contain the unit lower triangular factor   of the incomplete decomposition   of the A  matrix  stored in SLAP Row format.  The diagonal of  ones *IS* stored.  This structure can be  set up by the  SS2LT  routine.  See  the "Description", below, for more details about the  SLAP Row format. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JEL`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NEL). IN       Integer JEL(NEL). contain the unit lower triangular factor   of the incomplete decomposition   of the A  matrix  stored in SLAP Row format.  The diagonal of  ones *IS* stored.  This structure can be  set up by the  SS2LT  routine.  See  the "Description", below, for more details about the  SLAP Row format. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `EL`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NEL). IN       Real     EL(NEL). contain the unit lower triangular factor   of the incomplete decomposition   of the A  matrix  stored in SLAP Row format.  The diagonal of  ones *IS* stored.  This structure can be  set up by the  SS2LT  routine.  See  the "Description", below, for more details about the  SLAP Row format. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `B`: not a workspace argument
- `X`: not a workspace argument
- `NEL`: not a workspace argument
- `IEL`: not a workspace argument
- `JEL`: not a workspace argument
- `EL`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssli2`
- Original SLATEC routine: `SSLI2`
- Native symbol: `ssli2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SSLI2](https://www.netlib.org/slatec/lin/ssli2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
