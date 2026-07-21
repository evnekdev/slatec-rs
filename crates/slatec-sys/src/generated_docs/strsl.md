# Purpose

STRSL solves systems of the form

# Description

This canonical unsafe binding exposes original SLATEC routine `STRSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [STRSL](https://www.netlib.org/slatec/lin/strsl.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDT, *). B or TRANS(T)*X=B, where is a triangular matrix. B or B where T is a triangular matrix of order N.  Here TRANS(T) denotes the transpose of the matrix T. On Entry REAL(LDT,N) contains the matrix of the system.  The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information. B, T lower triangular, B, T upper triangular, B, T lower triangular, B, T upper triangular. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER is the leading dimension of the array T. INTEGER is the leading dimension of the array T. INTEGER is the leading dimension of the array T. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER is the order of the system. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `B`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N). contains the right hand side of the system. contains the solution, if INFO .EQ. 0. contains the solution, if INFO .EQ. 0. Otherwise B is unaltered. Otherwise B is unaltered. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER specifies what kind of system is to be solved. If JOB is not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER contains zero if the system is nonsingular. Otherwise INFO contains the index of the first zero diagonal element of T. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `B`: not a workspace argument
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::strsl`
- Original SLATEC routine: `STRSL`
- Native symbol: `strsl_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [STRSL](https://www.netlib.org/slatec/lin/strsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
