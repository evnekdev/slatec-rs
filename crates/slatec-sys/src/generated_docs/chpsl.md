# Purpose

CHISL solves the complex Hermitian system A * X = B using the factors computed by CHPFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CHPSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHPSL](https://www.netlib.org/slatec/lin/chpsl.f).

# Arguments

## 1. `AP`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N*(N+1)/2) the output from CHPFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from CHPFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `B`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) the right hand side vector. On Return the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero may occur if  CHPCO  has set RCOND .EQ. 0.0 or  CHPFA  has set INFO .NE. 0  . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL CHPFA(AP,N,KVPT,INFO) IF (INFO .NE. 0) GO TO ... DO 10 J = 1, P CALL CHPSL(AP,N,KVPT,C(1,J)) 10 CONTINUE

# Workspace and array requirements

- `AP`: not a workspace argument
- `N`: not a workspace argument
- `KPVT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::chpsl`
- Original SLATEC routine: `CHPSL`
- Native symbol: `chpsl_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_i32_array_rank1,mut_complex32_array_rank1)`
- Exact Netlib source file: [CHPSL](https://www.netlib.org/slatec/lin/chpsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
