# Purpose

PSI(X) calculates the psi (or digamma) function for real argument X. PSI(X) is the logarithmic derivative of the gamma function of X. Series for PSI on the interval 0. to 1.00000D+00 with weighted error 2.03E-17 log weighted error 16.69 significant figures required 16.39 decimal places required 17.37 Series for APSI on the interval 0. to 2.50000D-01 with weighted error 5.54E-17 log weighted error 16.26 significant figures required 14.42 decimal places required 16.86

# Description

This canonical unsafe binding exposes original SLATEC routine `PSI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PSI](https://www.netlib.org/slatec/fnlib/psi.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. calculates the psi (or digamma) function for real argument X. is the logarithmic derivative of the gamma function of X. Series for PSI        on the interval  0.          to  1.00000D+00 with weighted error   2.03E-17 log weighted error  16.69 significant figures required  16.39 decimal places required  17.37 Series for APSI       on the interval  0.          to  2.50000D-01 with weighted error   5.54E-17 log weighted error  16.26 significant figures required  14.42 decimal places required  16.86 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::psi`
- Original SLATEC routine: `PSI`
- Native symbol: `psi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [PSI](https://www.netlib.org/slatec/fnlib/psi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
