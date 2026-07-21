# Purpose

CGESL solves the complex system

# Description

This canonical unsafe binding exposes original SLATEC routine `CGESL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGESL](https://www.netlib.org/slatec/lin/cgesl.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). B or CTRANS(A)*X=B using the factors computed by CGECO or CGEFA. B  or  CTRANS(A) * X = B using the factors computed by CGECO or CGEFA. On Entry COMPLEX(LDA, N) the output from CGECO or CGEFA. is the conjugate transpose. On Return division by zero will occur if the input factor contains a zero on the diagonal.  Technically this indicates singularity but it is often caused by improper arguments or improper setting of LDA .  It will not occur if the subroutines are called correctly and if CGECO has set RCOND .GT. 0.0 or CGEFA has set INFO .EQ. 0 . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL CGECO(A,LDA,N,IPVT,RCOND,Z) IF (RCOND is too small) GO TO ... DO 10 J = 1, P CALL CGESL(A,LDA,N,IPVT,C(1,J),0) 10 CONTINUE not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from CGECO or CGEFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `B`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) the right hand side vector. the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0         to solve  A*X = B , = nonzero   to solve  CTRANS(A)*X = B  where not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `IPVT`: not a workspace argument
- `B`: not a workspace argument
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cgesl`
- Original SLATEC routine: `CGESL`
- Native symbol: `cgesl_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CGESL](https://www.netlib.org/slatec/lin/cgesl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
