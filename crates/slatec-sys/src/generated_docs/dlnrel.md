# Purpose

DLNREL(X) calculates the double precision natural logarithm of (1.0+X) for double precision argument X. This routine should be used when X is small and accurate to calculate the logarithm accurately (in the relative error sense) in the neighborhood of 1.0. Series for ALNR on the interval -3.75000E-01 to 3.75000E-01 with weighted error 6.35E-32 log weighted error 31.20 significant figures required 30.93 decimal places required 32.01

# Description

This canonical unsafe binding exposes original SLATEC routine `DLNREL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DLNREL](https://www.netlib.org/slatec/fnlib/dlnrel.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate ln(1+X) accurate in the sense of relative error

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::dlnrel`
- Original SLATEC routine: `DLNREL`
- Native symbol: `dlnrel_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DLNREL](https://www.netlib.org/slatec/fnlib/dlnrel.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
