# Purpose

SSISL solves the real symmetric system A * X = B using the factors computed by SSPFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SSPSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSPSL](https://www.netlib.org/slatec/lin/sspsl.f).

# Arguments

## 1. `AP`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N*(N+1)/2) the output from SSPFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from SSPFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `B`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) the right hand side vector. On Return the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero may occur if  SSPCO  has set RCOND .EQ. 0.0 or  SSPFA  has set INFO .NE. 0  . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL SSPFA(AP,N,KPVT,INFO) IF (INFO .NE. 0) GO TO ... DO 10 J = 1, P CALL SSPSL(AP,N,KPVT,C(1,J)) 10 CONTINUE

# Workspace and array requirements

- `AP`: not a workspace argument
- `N`: not a workspace argument
- `KPVT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::sspsl`
- Original SLATEC routine: `SSPSL`
- Native symbol: `sspsl_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SSPSL](https://www.netlib.org/slatec/lin/sspsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
