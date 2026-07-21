# Purpose

CTRDI computes the determinant and inverse of a complex triangular matrix. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CTRDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTRDI](https://www.netlib.org/slatec/lin/ctrdi.f).

# Arguments

## `T`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDT, *).

COMPLEX(LDT,N) contains the triangular matrix. The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information. inverse of original matrix if requested. Otherwise unchanged.

## `LDT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the leading dimension of the array T.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the order of the system.

## `DET`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (2).

COMPLEX(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 010 no det, inverse of lower triangular. = 011 no det, inverse of upper triangular. = 100 det, no inverse. = 110 det, inverse of lower triangular. = 111 det, inverse of upper triangular.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER contains zero if the system is nonsingular and the inverse is requested. Otherwise INFO contains the index of a zero diagonal element of T.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

`INFO` is a documented status output; its bounded argument contract states the available source semantics.

# Workspace and array requirements

- `T`: not a workspace argument
- `LDT`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::ctrdi`
- Original SLATEC routine: `CTRDI`
- Native symbol: `ctrdi_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CTRDI](https://www.netlib.org/slatec/lin/ctrdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
