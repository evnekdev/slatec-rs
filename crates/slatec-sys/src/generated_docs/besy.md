# Purpose

BESY implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel

# Description

This canonical unsafe binding exposes original SLATEC routine `BESY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESY](https://www.netlib.org/slatec/src/besy.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. negative orders FNU. .GT. 20. If FNU .GE. NULIM, the uniform asymptotic expansion is coded in ASYJY for orders FNU and FNU+1 to start the recursion. X .GT. 0.0E0 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `FNU`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1)/(X), I=1,...,N for real, positive 1)/(X), I=1,N for real X .GT. 0.0E0 and non-negative orders FNU.  If FNU .LT. NULIM, orders FNU and are obtained from BESYNU which computes by a power series for X .LE. 2, the K Bessel function of an imaginary argument for 2 .LT. X .LE. 20 and the asymptotic expansion for order of the initial Y function, FNU .GE. 0.0E0 1)/(X), I=1,N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 or N .GE. 2.  An overflow test is made on the leading term of the asymptotic expansion before any extensive computation is done. number of members in the sequence, N .GE. 1 M. Temme, On the numerical evaluation of the modified Bessel function of the third kind, Journal of Computational Physics 19, (1975), pp. 324-337. M. Temme, On the numerical evaluation of the ordinary Bessel function of the second kind, Journal of Computational Physics 21, (1976), pp. 343-350. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `Y`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1)/(X), I=1,...,N for real, positive 1)/(X), I=1,N for real X .GT. 0.0E0 and non-negative orders FNU.  If FNU .LT. NULIM, orders FNU and a vector whose first N components contain values 1)/(X), I=1,N. 1)/(X), I=1,N. 1)/(X), I=1,...,N for real, positive 1)/(X), I=1,N for real X .GT. 0.0E0 and non-negative orders FNU.  If FNU .LT. NULIM, orders FNU and a vector whose first N components contain values 1)/(X), I=1,N. 1)/(X), I=1,N. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input arguments - a fatal error Overflow - a fatal error

# Workspace and array requirements

- `X`: not a workspace argument
- `FNU`: not a workspace argument
- `N`: not a workspace argument
- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besy`
- Original SLATEC routine: `BESY`
- Native symbol: `besy_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BESY](https://www.netlib.org/slatec/src/besy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
