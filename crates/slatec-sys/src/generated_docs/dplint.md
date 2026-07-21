# Purpose

Subroutine DPLINT is designed to produce the polynomial which interpolates the data (X(I),Y(I)), I=1,...,N. DPLINT sets up information in the array C which can be used by subroutine DPOLVL to evaluate the polynomial and its derivatives and by subroutine DPOLCF to produce the coefficients.

# Description

This canonical unsafe binding exposes original SLATEC routine `DPLINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPLINT](https://www.netlib.org/slatec/src/dplint.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the number of data points (N. GE. 1).

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

the array of abscissas (all of which must be distinct).

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

the array of ordinates.

## `C`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

an array of information used by subroutines Dimensioning Information ******* Arrays X,Y, and C must be dimensioned at least N in the calling program.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `C`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dplint`
- Original SLATEC routine: `DPLINT`
- Native symbol: `dplint_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DPLINT](https://www.netlib.org/slatec/src/dplint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
