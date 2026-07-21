# Purpose

Generate a normally distributed random number, i.e., generate random numbers with a Gaussian distribution. These random numbers are not exceptionally good -- especially in the tails of the distribution, but this implementation is simple and suitable for most applications. See R. W. Hamming, Numerical Methods for Scientists and Engineers, McGraw-Hill, 1962, pages 34 and 389. Input Arguments --

# Description

This canonical unsafe binding exposes original SLATEC routine `RGAUSS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RGAUSS](https://www.netlib.org/slatec/fnlib/rgauss.f).

# Arguments

## 1. `XMEAN`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. the mean of the Guassian distribution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `SD`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. the standard deviation of the Guassian function EXP (-1/2 * (X-XMEAN)**2 / SD**2) not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `XMEAN`: not a workspace argument
- `SD`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::statistics::rgauss`
- Original SLATEC routine: `RGAUSS`
- Native symbol: `rgauss_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32)`
- Exact Netlib source file: [RGAUSS](https://www.netlib.org/slatec/fnlib/rgauss.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
