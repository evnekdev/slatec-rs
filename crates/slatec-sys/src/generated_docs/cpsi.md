# Purpose

PSI(X) calculates the psi (or digamma) function of X. PSI(X) is the logarithmic derivative of the gamma function of X.

# Description

This canonical unsafe binding exposes original SLATEC routine `CPSI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPSI](https://www.netlib.org/slatec/fnlib/cpsi.f).

# Arguments

## `ZIN`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input complex argument to the digamma function. The routine evaluates the logarithmic derivative of gamma at this value; poles and values too near a negative integer can trigger the SLATEC error handler.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::cpsi`
- Original SLATEC routine: `CPSI`
- Native symbol: `cpsi_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [CPSI](https://www.netlib.org/slatec/fnlib/cpsi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
