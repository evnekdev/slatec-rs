# Purpose

DPSI calculates the double precision Psi (or Digamma) function for

# Description

This canonical unsafe binding exposes original SLATEC routine `DPSI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPSI](https://www.netlib.org/slatec/fnlib/dpsi.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. is the logarithmic derivative is the logarithmic derivative of the Gamma function of X. of the Gamma function of X. Series for PSI        on the interval  0.          to  1.00000E+00 Series for PSI        on the interval  0.          to  1.00000E+00 with weighted error   5.79E-32 with weighted error   5.79E-32 log weighted error  31.24 log weighted error  31.24 significant figures required  30.93 significant figures required  30.93 decimal places required  32.05 decimal places required  32.05 Series for APSI       on the interval  0.          to  1.00000E-02 Series for APSI       on the interval  0.          to  1.00000E-02 with weighted error   7.75E-33 with weighted error   7.75E-33 log weighted error  32.11 log weighted error  32.11 significant figures required  28.88 significant figures required  28.88 decimal places required  32.71 decimal places required  32.71 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dpsi`
- Original SLATEC routine: `DPSI`
- Native symbol: `dpsi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DPSI](https://www.netlib.org/slatec/fnlib/dpsi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
