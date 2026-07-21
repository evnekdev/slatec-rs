# Purpose

DSISL solves the double precision symmetric system

# Description

This canonical unsafe binding exposes original SLATEC routine `DSISL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSISL](https://www.netlib.org/slatec/lin/dsisl.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDA, *). B using the factors computed by DSIFA. On Entry DOUBLE PRECISION(LDA,N) the output from DSIFA. division by zero may occur if  DSICO  has set RCOND .EQ. 0.0 or  DSIFA  has set INFO .NE. 0  . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL DSIFA(A,LDA,N,KPVT,INFO) IF (INFO .NE. 0) GO TO ... DO 10 J = 1, P CALL DSISL(A,LDA,N,KPVT,C(1,J)) 10 CONTINUE not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `KPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from DSIFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `B`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION(N) the right hand side vector. On Return the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `KPVT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsisl`
- Original SLATEC routine: `DSISL`
- Native symbol: `dsisl_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DSISL](https://www.netlib.org/slatec/lin/dsisl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
