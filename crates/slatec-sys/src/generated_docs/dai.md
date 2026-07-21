# Purpose

DAI(X) calculates the double precision Airy function for double precision argument X. Series for AIF on the interval -1.00000E+00 to 1.00000E+00 with weighted error 8.37E-33 log weighted error 32.08 significant figures required 30.87 decimal places required 32.63 Series for AIG on the interval -1.00000E+00 to 1.00000E+00 with weighted error 7.47E-34 log weighted error 33.13 significant figures required 31.50 decimal places required 33.68

# Description

This canonical unsafe binding exposes original SLATEC routine `DAI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DAI](https://www.netlib.org/slatec/fnlib/dai.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision Airy function for double precision argument X. Series for AIF        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   8.37E-33 log weighted error  32.08 significant figures required  30.87 decimal places required  32.63 Series for AIG        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   7.47E-34 log weighted error  33.13 significant figures required  31.50 decimal places required  33.68 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::dai`
- Original SLATEC routine: `DAI`
- Native symbol: `dai_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DAI](https://www.netlib.org/slatec/fnlib/dai.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
