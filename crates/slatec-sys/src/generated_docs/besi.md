# Purpose

BESI computes an N member sequence of I Bessel functions I/sub(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity, and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane. For values not covered by one of these formulae, the order is incremented by an integer so that one of these formulae apply. Backward recursion is used to reduce orders by integer values. The asymptotic expansion for X to infinity is used only when the entire sequence (specifically the last member) lies within the region covered by the expansion. Leading terms of these expansions are used to test for over or underflow where appropriate. If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all are set to zero. An overflow cannot occur with scaling.

# Description

This canonical unsafe binding exposes original SLATEC routine `BESI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESI](https://www.netlib.org/slatec/src/besi.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

X. GE. 0. 0E0.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

order of first member of the sequence,. GE. 0. 0E0.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

a parameter to indicate the scaling option 1 returns 2 returns.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of members in the sequence, N. GE. 1.

## `Y`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

I/sub(ALPHA+K-1)/(X), K=1,. ,N EXP(-X)*I/sub(ALPHA+K-1)/(X), a vector whose first N components contain values for I/sub(ALPHA+K-1)/(X) or scaled values for EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,. ,N depending on KODE 0. 0E0, K=N-NZ+1,. ,N.

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

- Canonical Rust path: `slatec_sys::special::bessel::besi`
- Original SLATEC routine: `BESI`
- Native symbol: `besi_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [BESI](https://www.netlib.org/slatec/src/besi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
