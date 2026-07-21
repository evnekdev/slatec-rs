# Purpose

Abstract **** a double precision routine **** DBESJ computes an N member sequence of J Bessel functions J/sub(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane. For values of (NU,X) not covered by one of these formulae, the order is incremented or decremented by integer values into a region where one of the formulae apply. Backward recursion is applied to reduce orders by integer values except where the entire sequence lies in the oscillatory region. In this case forward recursion is stable and values from the asymptotic expansion for X to infinity start the recursion when it is efficient to do so. Leading terms of the series and uniform expansion are tested for underflow. If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all members are set to zero. Overflow cannot occur. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic.

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESJ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESJ](https://www.netlib.org/slatec/src/dbesj.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

X. GE. 0. 0D0.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

order of first member of the sequence,. GE. 0. 0D0.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of members in the sequence, N. GE. 1 Output Y is double precision.

## `Y`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

a vector whose first N components contain values for J/sub(ALPHA+K-1)/(X), K=1,. ,N 0. 0D0, K=N-NZ+1,. ,N.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of components of Y set to zero due to underflow, NZ=0 , normal return, computation completed. NE. 0, last NZ components of Y set to zero,.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | , normal return, computation completed |

# Workspace and array requirements

- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesj`
- Original SLATEC routine: `DBESJ`
- Native symbol: `dbesj_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DBESJ](https://www.netlib.org/slatec/src/dbesj.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
