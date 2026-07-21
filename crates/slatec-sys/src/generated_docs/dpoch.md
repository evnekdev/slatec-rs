# Purpose

Evaluate a double precision generalization of Pochhammer's symbol (A)-sub-X = GAMMA(A+X)/GAMMA(A) for double precision A and X. For X a non-negative integer, POCH(A,X) is just Pochhammer's symbol. This is a preliminary version that does not handle wrong arguments properly and may not properly handle the case when the result is computed to less than half of double precision.

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOCH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOCH](https://www.netlib.org/slatec/fnlib/dpoch.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate a generalization of Pochhammer's symbol

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate a generalization of Pochhammer's symbol

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dpoch`
- Original SLATEC routine: `DPOCH`
- Native symbol: `dpoch_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64)`
- Exact Netlib source file: [DPOCH](https://www.netlib.org/slatec/fnlib/dpoch.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
