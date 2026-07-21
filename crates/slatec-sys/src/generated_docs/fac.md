# Purpose

FAC(N) evaluates the factorial function of N. FAC is single precision. N must be an integer between 0 and 25 inclusive.

# Description

This canonical unsafe binding exposes original SLATEC routine `FAC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [FAC](https://www.netlib.org/slatec/fnlib/fac.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is single is single precision.  N must be an integer between 0 and 25 inclusive. precision.  N must be an integer between 0 and 25 inclusive. is single is single precision.  N must be an integer between 0 and 25 inclusive. precision.  N must be an integer between 0 and 25 inclusive. not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::fac`
- Original SLATEC routine: `FAC`
- Native symbol: `fac_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [FAC](https://www.netlib.org/slatec/fnlib/fac.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
