# Purpose

Evaluate EXPREL(X) = (EXP(X) - 1.0) / X. For small ABS(X) the Taylor series is used. If X is negative the reflection formula EXPREL(X) = EXP(X) * EXPREL(ABS(X)) may be used. This reflection formula will be of use when the evaluation for small ABS(X) is done by Chebyshev series rather than Taylor series.

# Description

This canonical unsafe binding exposes original SLATEC routine `DEXPRL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DEXPRL](https://www.netlib.org/slatec/fnlib/dexprl.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1)/X. 1.0) / X.   For small ABS(X) the 1.0) / X.   For small ABS(X) the Taylor series is used.  If X is negative the reflection formula Taylor series is used.  If X is negative the reflection formula EXP(X) * EXPREL(ABS(X)) may be used.  This reflection formula will be of use when the evaluation for small ABS(X) is done by Chebyshev series rather than Taylor series. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::dexprl`
- Original SLATEC routine: `DEXPRL`
- Native symbol: `dexprl_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DEXPRL](https://www.netlib.org/slatec/fnlib/dexprl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
