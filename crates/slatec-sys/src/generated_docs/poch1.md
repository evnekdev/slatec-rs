# Purpose

Evaluate a generalization of Pochhammer's symbol for special situations that require especially accurate values when X is small in POCH1(A,X) = (POCH(A,X)-1)/X = (GAMMA(A+X)/GAMMA(A) - 1.0)/X . This specification is particularly suited for stably computing expressions such as (GAMMA(A+X)/GAMMA(A) - GAMMA(B+X)/GAMMA(B))/X = POCH1(A,X) - POCH1(B,X) Note that POCH1(A,0.0) = PSI(A) When ABS(X) is so small that substantial cancellation will occur if the straightforward formula is used, we use an expansion due to Fields and discussed by Y. L. Luke, The Special Functions and Their Approximations, Vol. 1, Academic Press, 1969, page 34. The ratio POCH(A,X) = GAMMA(A+X)/GAMMA(A) is written by Luke as (A+(X-1)/2)**X * polynomial in (A+(X-1)/2)**(-2) . In order to maintain significance in POCH1, we write for positive A

# Description

This canonical unsafe binding exposes original SLATEC routine `POCH1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POCH1](https://www.netlib.org/slatec/fnlib/poch1.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate a generalization of Pochhammer's symbol starting from first order

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

EXP(X*LOG(A+(X-1)/2)) = EXP(Q) = 1. 0 + Q*EXPREL(Q). Likewise the polynomial is written POLY = 1. 0 + X*POLY1(A,X). Thus, POCH1(A,X) = (POCH(A,X) - 1) / X = EXPREL(Q)*(Q/X + Q*POLY1(A,X)) + POLY1(A,X).

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::poch1`
- Original SLATEC routine: `POCH1`
- Native symbol: `poch1_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32)`
- Exact Netlib source file: [POCH1](https://www.netlib.org/slatec/fnlib/poch1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
