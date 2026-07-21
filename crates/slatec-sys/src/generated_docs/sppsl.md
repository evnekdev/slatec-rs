# Purpose

SPPSL solves the real symmetric positive definite system A * X = B using the factors computed by SPPCO or SPPFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPPSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPPSL](https://www.netlib.org/slatec/lin/sppsl.f).

# Arguments

## 1. `AP`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL (N*(N+1)/2) the output from SPPCO or SPPFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) the right hand side vector. On Return the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero will occur if the input factor contains a zero on the diagonal.  Technically, this indicates singularity, but it is usually caused by improper subroutine arguments.  It will not occur if the subroutines are called correctly and  INFO .EQ. 0 . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL SPPCO(AP,N,RCOND,Z,INFO) IF (RCOND is too small .OR. INFO .NE. 0) GO TO ... DO 10 J = 1, P CALL SPPSL(AP,N,C(1,J)) 10 CONTINUE

# Workspace and array requirements

- `AP`: not a workspace argument
- `N`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::sppsl`
- Original SLATEC routine: `SPPSL`
- Native symbol: `sppsl_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SPPSL](https://www.netlib.org/slatec/lin/sppsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
