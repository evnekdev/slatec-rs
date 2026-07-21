# Purpose

Evaluate (Z+0.5)*LOG((Z+1.0)/Z) - 1.0 with relative error accuracy Let Q = 1.0/Z so that (Z+0.5)*LOG(1+1/Z) - 1 = (Z+0.5)*(LOG(1+Q) - Q + Q*Q/2) - Q*Q/4 = (Z+0.5)*Q**3*C9LN2R(Q) - Q**2/4, where C9LN2R is (LOG(1+Q) - Q + 0.5*Q**2) / Q**3.

# Description

This canonical unsafe binding exposes original SLATEC routine `C0LGMC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [C0LGMC](https://www.netlib.org/slatec/fnlib/c0lgmc.f).

# Arguments

## 1. `Z`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. 1.0 with relative 1.0 with relative 1.0 with relative accuracy. accuracy. accuracy. 1.0  with relative error accuracy 1.0  with relative error accuracy 1.0  with relative error accuracy Let Q = 1.0/Z so that Let Q = 1.0/Z so that Let Q = 1.0/Z so that 1 = (Z+0.5)*(LOG(1+Q) - Q + Q*Q/2) - Q*Q/4 1 = (Z+0.5)*(LOG(1+Q) - Q + Q*Q/2) - Q*Q/4 Q**2/4, where  C9LN2R  is (LOG(1+Q) - Q + 0.5*Q**2) / Q**3. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::c0lgmc`
- Original SLATEC routine: `C0LGMC`
- Native symbol: `c0lgmc_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [C0LGMC](https://www.netlib.org/slatec/fnlib/c0lgmc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
