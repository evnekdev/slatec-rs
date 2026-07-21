# Purpose

DLI(X) calculates the double precision logarithmic integral for double precision argument X.

# Description

This canonical unsafe binding exposes original SLATEC routine `DLI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DLI](https://www.netlib.org/slatec/fnlib/dli.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the logarithmic integral

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dli`
- Original SLATEC routine: `DLI`
- Native symbol: `dli_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DLI](https://www.netlib.org/slatec/fnlib/dli.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
