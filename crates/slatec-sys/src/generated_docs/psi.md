# Purpose

PSI(X) calculates the psi (or digamma) function for real argument X. PSI(X) is the logarithmic derivative of the gamma function of X. Series for PSI on the interval 0. to 1.00000D+00 with weighted error 2.03E-17 log weighted error 16.69 significant figures required 16.39 decimal places required 17.37 Series for APSI on the interval 0. to 2.50000D-01 with weighted error 5.54E-17 log weighted error 16.26 significant figures required 14.42 decimal places required 16.86

# Description

This canonical unsafe binding exposes original SLATEC routine `PSI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PSI](https://www.netlib.org/slatec/fnlib/psi.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the Psi (or Digamma) function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::psi`
- Original SLATEC routine: `PSI`
- Native symbol: `psi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [PSI](https://www.netlib.org/slatec/fnlib/psi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
