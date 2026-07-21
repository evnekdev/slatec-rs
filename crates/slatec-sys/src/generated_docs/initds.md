# Purpose

Initialize the orthogonal series, represented by the array OS, so that INITDS is the number of terms needed to insure the error is no

# Description

This canonical unsafe binding exposes original SLATEC routine `INITDS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [INITDS](https://www.netlib.org/slatec/fnlib/initds.f).

# Arguments

## 1. `OS`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). double precision array of NOS coefficients in an orthogonal series. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `NOS`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of coefficients in OS. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `ETA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. tenth tenth machine precision. machine precision. Input Arguments -- Input Arguments -- single precision scalar containing requested accuracy of series. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:i32(mut_f64_ptr_rank1,mut_i32,mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `OS`: not a workspace argument
- `NOS`: not a workspace argument
- `ETA`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::initds`
- Original SLATEC routine: `INITDS`
- Native symbol: `initds_`
- ABI fingerprint: `function:i32(mut_f64_ptr_rank1,mut_i32,mut_f32)`
- Exact Netlib source file: [INITDS](https://www.netlib.org/slatec/fnlib/initds.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
