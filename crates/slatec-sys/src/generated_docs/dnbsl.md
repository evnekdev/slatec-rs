# Purpose

DNBSL solves the double precision band system A * X = B or TRANS(A) * X = B using the factors computed by DNBCO or DNBFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DNBSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DNBSL](https://www.netlib.org/slatec/src/dnbsl.f).

# Arguments

## 1. `ABE`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDA, *). DOUBLE PRECISION(LDA, NC) the output from DNBCO or DNBFA. NC must be .GE. 2*ML+MU+1 . DOUBLE PRECISION(LDA, NC) the output from DNBCO or DNBFA. NC must be .GE. 2*ML+MU+1 . not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABE . INTEGER the leading dimension of the array  ABE . INTEGER the leading dimension of the array  ABE . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the original matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ML`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals below the main diagonal. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals above the main diagonal. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from DNBCO or DNBFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `B`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION(N) the right hand side vector. the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0         to solve  A*X = B . = nonzero   to solve  TRANS(A)*X = B , where TRANS(A)  is the transpose. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero will occur if the input factor contains a zero on the diagonal.  Technically this indicates singularity but it is often caused by improper arguments or improper setting of LDA.  It will not occur if the subroutines are called correctly and if DNBCO has set RCOND .GT. 0.0 or DNBFA has set INFO .EQ. 0 . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL DNBCO(ABE,LDA,N,ML,MU,IPVT,RCOND,Z) IF (RCOND is too small) GO TO ... DO 10 J = 1, P CALL DNBSL(ABE,LDA,N,ML,MU,IPVT,C(1,J),0) 10 CONTINUE

# Workspace and array requirements

- `ABE`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `ML`: not a workspace argument
- `MU`: not a workspace argument
- `IPVT`: not a workspace argument
- `B`: not a workspace argument
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dnbsl`
- Original SLATEC routine: `DNBSL`
- Native symbol: `dnbsl_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DNBSL](https://www.netlib.org/slatec/src/dnbsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
