# Purpose

Initialize the orthogonal series, represented by the array OS, so that INITDS is the number of terms needed to insure the error is no larger than ETA. Ordinarily, ETA will be chosen to be one-tenth machine precision. Input Arguments --

# Description

This canonical unsafe binding exposes original SLATEC routine `INITDS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [INITDS](https://www.netlib.org/slatec/fnlib/initds.f).

# Arguments

## `OS`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision array of NOS coefficients in an orthogonal series.

## `NOS`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of coefficients in OS.

## `ETA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scalar containing requested accuracy of series.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:i32(mut_f64_ptr_rank1,mut_i32,mut_f32)`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `OS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::initds`
- Original SLATEC routine: `INITDS`
- Native symbol: `initds_`
- ABI fingerprint: `function:i32(mut_f64_ptr_rank1,mut_i32,mut_f32)`
- Exact Netlib source file: [INITDS](https://www.netlib.org/slatec/fnlib/initds.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
