# Purpose

Evaluate a generalization of Pochhammer's symbol (A)-sub-X = GAMMA(A+X)/GAMMA(A). For X a non-negative integer, POCH(A,X) is just Pochhammer's symbol. A and X are single precision. This is a preliminary version. Error handling when POCH(A,X) is less than half precision is probably incorrect. Grossly incorrect arguments are not handled properly.

# Description

This canonical unsafe binding exposes original SLATEC routine `POCH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POCH](https://www.netlib.org/slatec/fnlib/poch.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate a generalization of Pochhammer's symbol

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate a generalization of Pochhammer's symbol

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::poch`
- Original SLATEC routine: `POCH`
- Native symbol: `poch_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32)`
- Exact Netlib source file: [POCH](https://www.netlib.org/slatec/fnlib/poch.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
