# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `CDOTC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CDOTC](https://www.netlib.org/slatec/lin/cdotc.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `CX`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

complex vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of CX.

## `CY`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

complex vector with N elements.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of CY.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_i32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `CX`: not a workspace argument
- `CY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::cdotc`
- Original SLATEC routine: `CDOTC`
- Native symbol: `cdotc_`
- ABI fingerprint: `function:complex32(mut_i32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CDOTC](https://www.netlib.org/slatec/lin/cdotc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
