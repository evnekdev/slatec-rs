# Purpose

SGBSL solves the real band system A * X = B or TRANS(A) * X = B using the factors computed by SBGCO or SGBFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SGBSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGBSL](https://www.netlib.org/slatec/lin/sgbsl.f).

# Arguments

## 1. `ABD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) the output from SBGCO or SGBFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the original matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ML`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals below the main diagonal. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals above the main diagonal. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from SBGCO or SGBFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `B`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) the right hand side vector. the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0         to solve  A*X = B , = nonzero   to solve  TRANS(A)*X = B , where TRANS(A)  is the transpose. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero will occur if the input factor contains a zero on the diagonal.  Technically, this indicates singularity, but it is often caused by improper arguments or improper setting of LDA .  It will not occur if the subroutines are called correctly and if SBGCO has set RCOND .GT. 0.0 or SGBFA has set INFO .EQ. 0 . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL SBGCO(ABD,LDA,N,ML,MU,IPVT,RCOND,Z) If (RCOND is too small) GO TO ... DO 10 J = 1, P CALL SGBSL(ABD,LDA,N,ML,MU,IPVT,C(1,J),0) 10 CONTINUE

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `ML`: not a workspace argument
- `MU`: not a workspace argument
- `IPVT`: not a workspace argument
- `B`: not a workspace argument
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::sgbsl`
- Original SLATEC routine: `SGBSL`
- Native symbol: `sgbsl_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SGBSL](https://www.netlib.org/slatec/lin/sgbsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
