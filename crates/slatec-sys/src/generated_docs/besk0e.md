# Purpose

BESK0E(X) computes the exponentially scaled modified (hyperbolic) Bessel function of third kind of order zero for real argument

# Description

This canonical unsafe binding exposes original SLATEC routine `BESK0E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESK0E](https://www.netlib.org/slatec/fnlib/besk0e.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

GT. 0. 0, i. e. , EXP(X)*K0(X). Series for BK0 on the interval 0.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besk0e`
- Original SLATEC routine: `BESK0E`
- Native symbol: `besk0e_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESK0E](https://www.netlib.org/slatec/fnlib/besk0e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
