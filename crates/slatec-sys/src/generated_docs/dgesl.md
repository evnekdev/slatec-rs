# Purpose

DGESL solves the double precision system

# Description

This canonical unsafe binding exposes original SLATEC routine `DGESL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGESL](https://www.netlib.org/slatec/lin/dgesl.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDA, *). B or TRANS(A)*X=B using the factors computed by DGECO or DGEFA. B  or  TRANS(A) * X = B using the factors computed by DGECO or DGEFA. On Entry DOUBLE PRECISION(LDA, N) the output from DGECO or DGEFA. is the transpose. On Return division by zero will occur if the input factor contains a zero on the diagonal.  Technically this indicates singularity but it is often caused by improper arguments or improper setting of LDA .  It will not occur if the subroutines are called correctly and if DGECO has set RCOND .GT. 0.0 or DGEFA has set INFO .EQ. 0 . To compute  INVERSE(A) * C  where  C  is a matrix with  P  columns CALL DGECO(A,LDA,N,IPVT,RCOND,Z) IF (RCOND is too small) GO TO ... DO 10 J = 1, P CALL DGESL(A,LDA,N,IPVT,C(1,J),0) 10 CONTINUE not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from DGECO or DGEFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `B`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION(N) the right hand side vector. the solution vector  X . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0         to solve  A*X = B , = nonzero   to solve  TRANS(A)*X = B  where not stated by selected source not applicable or not stated by selected source not a workspace argument

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

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dgesl`
- Original SLATEC routine: `DGESL`
- Native symbol: `dgesl_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DGESL](https://www.netlib.org/slatec/lin/dgesl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
