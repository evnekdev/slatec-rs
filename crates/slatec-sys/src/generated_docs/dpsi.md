# Purpose

DPSI calculates the double precision Psi (or Digamma) function for double precision argument X. PSI(X) is the logarithmic derivative of the Gamma function of X. Series for PSI on the interval 0. to 1.00000E+00 with weighted error 5.79E-32 log weighted error 31.24 significant figures required 30.93 decimal places required 32.05 Series for APSI on the interval 0. to 1.00000E-02 with weighted error 7.75E-33 log weighted error 32.11 significant figures required 28.88 decimal places required 32.71

# Description

This canonical unsafe binding exposes original SLATEC routine `DPSI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPSI](https://www.netlib.org/slatec/fnlib/dpsi.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the Psi (or Digamma) function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dpsi`
- Original SLATEC routine: `DPSI`
- Native symbol: `dpsi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DPSI](https://www.netlib.org/slatec/fnlib/dpsi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
