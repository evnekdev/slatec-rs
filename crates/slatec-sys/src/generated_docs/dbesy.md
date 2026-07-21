# Purpose

Abstract **** a double precision routine **** DBESY implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions Y/sub(FNU+I-1)/(X), I=1,N for real X .GT. 0.0D0 and non-negative orders FNU. If FNU .LT. NULIM, orders FNU and FNU+1 are obtained from DBSYNU which computes by a power series for X .LE. 2, the K Bessel function of an imaginary argument for 2 .LT. X .LE. 20 and the asymptotic expansion for

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESY](https://www.netlib.org/slatec/src/dbesy.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

GT. 20. If FNU. GE. NULIM, the uniform asymptotic expansion is coded in DASYJY for orders FNU and FNU+1 to start the recursion. NULIM is 70 or 100 depending on whether N=1 or N.

## `FNU`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

order of the initial Y function, FNU. GE. 0. 0D0.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of members in the sequence, N. GE. 1.

## `Y`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

a vector whose first N components contain values for the sequence Y(I)=Y/sub(FNU+I-1)/(X), I=1,N.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesy`
- Original SLATEC routine: `DBESY`
- Native symbol: `dbesy_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBESY](https://www.netlib.org/slatec/src/dbesy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
