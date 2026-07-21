# Purpose

DTRSL solves systems of the form T * X = B TRANS(T) * X = B where T is a triangular matrix of order N. Here TRANS(T) denotes the transpose of the matrix T. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DTRSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DTRSL](https://www.netlib.org/slatec/lin/dtrsl.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDT, *).

is a triangular matrix. DOUBLE PRECISION(LDT,N) contains the matrix of the system. The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information.

## `LDT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the leading dimension of the array T.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the order of the system.

## `B`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N). contains the right hand side of the system. B contains the solution, if INFO. EQ. 0. Otherwise B is unaltered.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER specifies what kind of system is to be solved. If JOB is 00 solve T*X=B, T lower triangular, 01 solve T*X=B, T upper triangular, 10 solve TRANS(T)*X=B, T lower triangular, 11 solve TRANS(T)*X=B, T upper triangular.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER contains zero if the system is nonsingular. Otherwise INFO contains the index of the first zero diagonal element of T.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

`INFO` is a documented status output; its bounded argument contract states the available source semantics.

# Workspace and array requirements

- `T`: not a workspace argument
- `LDT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dtrsl`
- Original SLATEC routine: `DTRSL`
- Native symbol: `dtrsl_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [DTRSL](https://www.netlib.org/slatec/lin/dtrsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
