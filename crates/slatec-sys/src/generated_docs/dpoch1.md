# Purpose

Evaluate a double precision generalization of Pochhammer's symbol for double precision A and X for special situations that require especially accurate values when X is small in

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOCH1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOCH1](https://www.netlib.org/slatec/fnlib/dpoch1.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1)/X 1)/X 1.0)/X . 1.0)/X . This specification is particularly suited for stably computing This specification is particularly suited for stably computing expressions such as expressions such as GAMMA(B+X)/GAMMA(B))/X GAMMA(B+X)/GAMMA(B))/X POCH1(B,X) PSI(A) When ABS(X) is so small that substantial cancellation will occur if the straightforward formula is used, we use an expansion due to Fields and discussed by Y. L. Luke, The Special Functions and Their Approximations, Vol. 1, Academic Press, 1969, page 34. GAMMA(A+X)/GAMMA(A) is written by Luke as 1)/2)**X * polynomial in (A+(X-1)/2)**(-2) . In order to maintain significance in POCH1, we write for positive a 1)/2)**X = EXP(X*LOG(A+(X-1)/2)) = EXP(Q) = 1.0 + Q*EXPREL(Q) . Likewise the polynomial is written POLY = 1.0 + X*POLY1(A,X) . Thus, 1) / X 1) / X = EXPREL(Q)*(Q/X + Q*POLY1(A,X)) + POLY1(A,X) = EXPREL(Q)*(Q/X + Q*POLY1(A,X)) + POLY1(A,X) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1)/X 1)/X 1.0)/X . This specification is particularly suited for stably computing expressions such as GAMMA(B+X)/GAMMA(B))/X POCH1(B,X) GAMMA(A+X)/GAMMA(A) is written by Luke as 1)/2)**X * polynomial in (A+(X-1)/2)**(-2) . In order to maintain significance in POCH1, we write for positive a 1)/2)**X = EXP(X*LOG(A+(X-1)/2)) = EXP(Q) = 1.0 + Q*EXPREL(Q) . Likewise the polynomial is written POLY = 1.0 + X*POLY1(A,X) . Thus, 1) / X 1) / X = EXPREL(Q)*(Q/X + Q*POLY1(A,X)) + POLY1(A,X) = EXPREL(Q)*(Q/X + Q*POLY1(A,X)) + POLY1(A,X) not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dpoch1`
- Original SLATEC routine: `DPOCH1`
- Native symbol: `dpoch1_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64)`
- Exact Netlib source file: [DPOCH1](https://www.netlib.org/slatec/fnlib/dpoch1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
