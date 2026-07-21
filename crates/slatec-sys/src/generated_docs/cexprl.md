# Purpose

Evaluate (EXP(Z)-1)/Z . For small ABS(Z), we use the Taylor series. We could instead use the expression CEXPRL(Z) = (EXP(X)*EXP(I*Y)-1)/Z = (X*EXPREL(X) * (1 - 2*SIN(Y/2)**2) - 2*SIN(Y/2)**2 + I*SIN(Y)*(1+X*EXPREL(X))) / Z

# Description

This canonical unsafe binding exposes original SLATEC routine `CEXPRL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CEXPRL](https://www.netlib.org/slatec/fnlib/cexprl.f).

# Arguments

## `Z`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the relative error exponential (EXP(X)-1)/X

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::cexprl`
- Original SLATEC routine: `CEXPRL`
- Native symbol: `cexprl_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [CEXPRL](https://www.netlib.org/slatec/fnlib/cexprl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
