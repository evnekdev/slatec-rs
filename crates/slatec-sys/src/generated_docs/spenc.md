# Purpose

Evaluate a form of Spence's function defined by

# Description

This canonical unsafe binding exposes original SLATEC routine `SPENC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPENC](https://www.netlib.org/slatec/fnlib/spenc.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. LOG(1-Y)/Y  DY. For ABS(X) .LE. 1, the uniformly convergent expansion SPENC = sum K=1,infinity  X**K / K**2     is valid. Spence's function can be used to evaluate much more general integral forms.  For example, - SPENC (A*(C*Z+D)/(A*D-B*C)) / C. Ref -- K. Mitchell, Philosophical Magazine, 40, p. 351 (1949). Stegun and Abromowitz, AMS 55, p. 1004. Series for SPEN       on the interval  0.          to  5.00000D-01 with weighted error   6.82E-17 log weighted error  16.17 significant figures required  15.22 decimal places required  16.81 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::spenc`
- Original SLATEC routine: `SPENC`
- Native symbol: `spenc_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [SPENC](https://www.netlib.org/slatec/fnlib/spenc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
