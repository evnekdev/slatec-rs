# Purpose

B L A S Subprogram Description of parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DQDOTI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQDOTI](https://www.netlib.org/slatec/src/dqdoti.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `DB`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision scalar to be added to inner product.

## `QC`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (30).

extended precision scalar to be added extended precision result D. P. dot product with extended precision accumulation (and result) QC and DQDOTI are set = DB + sum for I = 0 to N-1 of DX(LX+I*INCX) * DY(LY+I*INCY), where QC is an extended precision result which can be used as input to DQDOTA, and LX = 1 if INCX. GE. 0, else LX = (-INCX)*N, and LY is defined in a similar way using INCY. The MP package by Richard P.

## `DX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DX.

## `DY`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DY.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `QC`: not a workspace argument
- `DX`: not a workspace argument
- `DY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dqdoti`
- Original SLATEC routine: `DQDOTI`
- Native symbol: `dqdoti_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQDOTI](https://www.netlib.org/slatec/src/dqdoti.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
