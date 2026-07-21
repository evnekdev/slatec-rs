# Purpose

DTRCO estimates the condition of a double precision triangular matrix. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DTRCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DTRCO](https://www.netlib.org/slatec/lin/dtrco.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDT, *). DOUBLE PRECISION(LDT,N) contains the triangular matrix.  The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information. B , relative perturbations in  T  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  T  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate underflows. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER is the leading dimension of the array T. INTEGER is the leading dimension of the array T. INTEGER is the leading dimension of the array T. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER is the order of the system. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `RCOND`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. DOUBLE PRECISION an estimate of the reciprocal condition of  T . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `Z`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION(N) a work vector whose contents are usually unimportant. If  T  is close to a singular matrix, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0         T  is lower triangular. = nonzero   T  is upper triangular. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `T`: not a workspace argument
- `LDT`: not a workspace argument
- `N`: not a workspace argument
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dtrco`
- Original SLATEC routine: `DTRCO`
- Native symbol: `dtrco_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DTRCO](https://www.netlib.org/slatec/lin/dtrco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
