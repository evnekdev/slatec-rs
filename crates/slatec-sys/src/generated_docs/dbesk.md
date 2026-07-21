# Purpose

Abstract **** a double precision routine **** DBESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESK](https://www.netlib.org/slatec/src/dbesk.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1)/(X), I=1,...,N for real, positive negative orders FNU. 1)/(X), I=1,..,N for real X .GT. 0.0D0 and non-negative orders FNU.  If FNU .LT. NULIM, orders FNU and are double precision X .GT. 0.0D0 1)/(X), 1)/(X), I=1,...,N depending on KODE not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `FNU`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1)/(X), or scaled Bessel functions 1)/(X), I=1,...,N for real, positive 1)/(X), or scaled Bessel functions 1)/(X), I=1,..,N for real X .GT. 0.0D0 and non-negative orders FNU.  If FNU .LT. NULIM, orders FNU and are obtained from DBSKNU to start the recursion.  If .GE. NULIM, the uniform asymptotic expansion is used for is 35 or is 35 or are double precision order of the initial K function, FNU .GE. 0.0D0 1)/(X), 1)/(X), 1)/(X), I=1,...,N  or 1)/(X), I=1,...,N depending on KODE not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. a parameter to indicate the scaling option 1)/(X), 1)/(X), 1, a non-fatal error (NZ .NE. 0) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 or N .GE. 2.  Under and overflow tests are made on the leading term of the asymptotic expansion before any extensive computation is done. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic. number of members in the sequence, N .GE. 1 Output     Y is double precision M. Temme, On the numerical evaluation of the modified Bessel function of the third kind, Journal of Computational Physics 19, (1975), pp. 324-337. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `Y`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). 1)/(X), 1)/(X), a vector whose first N components contain values for the sequence 1)/(X), I=1,...,N  or 1)/(X), I=1,...,N depending on KODE 0.0D0, I=1,...,NZ 1)/(X), 1)/(X), a vector whose first N components contain values for the sequence 1)/(X), I=1,...,N  or 1)/(X), I=1,...,N depending on KODE 0.0D0, I=1,...,NZ not applicable or not stated by selected source not a workspace argument

## 6. `NZ`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of components of Y set to zero due to 0   , normal return, computation completed .NE. 0, first NZ components of Y set to zero number of components of Y set to zero due to 0   , normal return, computation completed .NE. 0, first NZ components of Y set to zero not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input arguments - a fatal error Overflow - a fatal error

# Workspace and array requirements

- `X`: not a workspace argument
- `FNU`: not a workspace argument
- `KODE`: not a workspace argument
- `N`: not a workspace argument
- `Y`: not a workspace argument
- `NZ`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesk`
- Original SLATEC routine: `DBESK`
- Native symbol: `dbesk_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DBESK](https://www.netlib.org/slatec/src/dbesk.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
