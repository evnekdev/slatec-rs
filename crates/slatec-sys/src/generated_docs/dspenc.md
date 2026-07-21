# Purpose

DSPENC(X) calculates the double precision Spence's integral for double precision argument X. Spence's function defined by integral from 0 to X of -LOG(1-Y)/Y DY. For ABS(X) .LE. 1, the uniformly convergent expansion DSPENC = sum K=1,infinity X**K / K**2 is valid. This is a form of Spence's integral due to K. Mitchell which differs from the definition in the NBS Handbook of Mathematical Functions. Spence's function can be used to evaluate much more general integral forms. For example, integral from 0 to Z of LOG(A*X+B)/(C*X+D) DX = LOG(ABS(B-A*D/C))*LOG(ABS(A*(C*X+D)/(A*D-B*C)))/C - DSPENC (A*(C*Z+D)/(A*D-B*C)) / C. Ref -- K. Mitchell, Philosophical Magazine, 40, p.351 (1949). Stegun and Abromowitz, AMS 55, p.1004. Series for SPEN on the interval 0. to 5.00000E-01 with weighted error 4.74E-32 log weighted error 31.32 significant figures required 30.37 decimal places required 32.11

# Description

This canonical unsafe binding exposes original SLATEC routine `DSPENC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSPENC](https://www.netlib.org/slatec/fnlib/dspenc.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute a form of Spence's integral due to K. Mitchell

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dspenc`
- Original SLATEC routine: `DSPENC`
- Native symbol: `dspenc_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DSPENC](https://www.netlib.org/slatec/fnlib/dspenc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
