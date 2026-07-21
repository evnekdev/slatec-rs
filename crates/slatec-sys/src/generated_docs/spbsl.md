# Purpose

SPBSL solves the real symmetric positive definite band system A*X = B using the factors computed by SPBCO or SPBFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPBSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPBSL](https://www.netlib.org/slatec/lin/spbsl.f).

# Arguments

## 1. `ABD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) the output from SPBCO or SPBFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the number of diagonals above the main diagonal. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `B`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) the right hand side vector. On Return the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero will occur if the input factor contains a zero on the diagonal.  Technically, this indicates singularity, but it is usually caused by improper subroutine arguments.  It will not occur if the subroutines are called correctly and  INFO .EQ. 0 . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL SPBCO(ABD,LDA,N,RCOND,Z,INFO) IF (RCOND is too small .OR. INFO .NE. 0) GO TO ... DO 10 J = 1, P CALL SPBSL(ABD,LDA,N,C(1,J)) 10 CONTINUE

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `M`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::spbsl`
- Original SLATEC routine: `SPBSL`
- Native symbol: `spbsl_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SPBSL](https://www.netlib.org/slatec/lin/spbsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
