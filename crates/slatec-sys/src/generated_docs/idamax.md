# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `IDAMAX`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [IDAMAX](https://www.netlib.org/slatec/lin/idamax.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `DX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DX.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `DX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::idamax`
- Original SLATEC routine: `IDAMAX`
- Native symbol: `idamax_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [IDAMAX](https://www.netlib.org/slatec/lin/idamax.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
