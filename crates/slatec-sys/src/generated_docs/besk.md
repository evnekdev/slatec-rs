# Purpose

BESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N for real X .GT. 0.0E0 and non-negative orders FNU. If FNU .LT. NULIM, orders FNU and FNU+1 are obtained from BESKNU to start the recursion. If

# Description

This canonical unsafe binding exposes original SLATEC routine `BESK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESK](https://www.netlib.org/slatec/src/besk.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

X. GT. 0. 0E0.

## `FNU`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

GE. NULIM, the uniform asymptotic expansion is used for orders FNU and FNU+1 to start the recursion. NULIM is 35 or 70 depending on whether N=1 or N. 2. Under and overflow tests are made on the leading term of the asymptotic expansion before any extensive computation is done. order of the initial K function, FNU.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

a parameter to indicate the scaling option 1 returns Y(I)= K/sub(FNU+I-1)/(X), I=1,. ,N 2 returns Y(I)=EXP(X)*K/sub(FNU+I-1)/(X),.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of members in the sequence, N. GE. 1.

## `Y`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a vector whose first n components contain values for the sequence K/sub(FNU+I-1)/(X), I=1,. ,N or EXP(X)*K/sub(FNU+I-1)/(X), I=1,. ,N depending on KODE.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of components of Y set to zero due to underflow with KODE=1, NZ=0 , normal return, computation completed. NE. 0, first NZ components of Y set to zero due to underflow, Y(I)=0. 0E0, I=1,. ,NZ.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `1` | , |
| `NZ` | `0` | , normal return, computation completed .0E0, I=1,...,NZ Improper input arguments - a fatal error Overflow - a fatal error |
| `NZ` | `1` | - a non-fatal error (NZ .NE. 0) |

# Workspace and array requirements

- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besk`
- Original SLATEC routine: `BESK`
- Native symbol: `besk_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [BESK](https://www.netlib.org/slatec/src/besk.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
