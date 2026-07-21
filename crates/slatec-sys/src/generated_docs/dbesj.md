# Purpose

Abstract **** a double precision routine **** DBESJ computes an N member sequence of J Bessel functions

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESJ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESJ](https://www.netlib.org/slatec/src/dbesj.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are double precision X .GE. 0.0D0 and J(NU,X), X .GE. 0, NU .GE. 0, ACM Transactions on Mathematical Software 3, (1977), pp. 76-92. F. W. J. Olver, Tables of Bessel Functions of Moderate or Large Orders, NPL Mathematical Tables 6, Her Majesty's Stationery Office, London, 1962. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `ALPHA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1)/(X), K=1,...,N for non-negative ALPHA and X. 1)/(X), K=1,...,N for non-negative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane.  For values of (NU,X) not covered by one of these formulae, the order is incremented or decremented by integer values into a region where one of the formulae apply. Backward recursion is applied to reduce orders by integer values except where the entire sequence lies in the oscillatory region.  In this case forward recursion is stable and values from the asymptotic expansion for X to infinity start the recursion when it is efficient to do so. Leading terms of the series and uniform expansion are tested for underflow.  If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all members are set to zero. Overflow cannot occur. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic. are double precision order of first member of the sequence, .GE. 0.0D0 1)/(X), K=1,...,N not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of members in the sequence, N .GE. 1 Output     Y is double precision NZ+1,...,N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `Y`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). a vector whose first N components contain NZ+1,...,N. a vector whose first N components contain NZ+1,...,N. not applicable or not stated by selected source not a workspace argument

## 5. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of components of Y set to zero due to underflow, 0   , normal return, computation completed .NE. 0, last NZ components of Y set to zero, number of components of Y set to zero due to underflow, 0   , normal return, computation completed .NE. 0, last NZ components of Y set to zero, not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input arguments - a fatal error Underflow  - a non-fatal error (NZ .NE. 0)

# Workspace and array requirements

- `X`: not a workspace argument
- `ALPHA`: not a workspace argument
- `N`: not a workspace argument
- `Y`: not a workspace argument
- `NZ`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesj`
- Original SLATEC routine: `DBESJ`
- Native symbol: `dbesj_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DBESJ](https://www.netlib.org/slatec/src/dbesj.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
