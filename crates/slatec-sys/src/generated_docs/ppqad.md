# Purpose

PPQAD computes the integral on (X1,X2) of a K-th order B-spline using the piecewise polynomial representation (C,XI,LXI,K). Here the Taylor expansion about the left end point XI(J) of the J-th interval is integrated and evaluated on subintervals of (X1,X2) which are formed by included break points. Integration outside (XI(1),XI(LXI+1)) is permitted.

# Description

This canonical unsafe binding exposes original SLATEC routine `PPQAD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PPQAD](https://www.netlib.org/slatec/src/ppqad.f).

# Arguments

## `LDC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

leading dimension of matrix C, LDC. GE. K.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDC, *).

right Taylor derivatives at XI(J), I=1,K , J=1,LXI.

## `XI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

break point array of length LXI+1. LE. X. XI(LXI+1).

## `LXI`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of polynomial pieces.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of B-spline, K. GE. 1.

## `X1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

end points of quadrature interval, normally in.

## `X2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

end points of quadrature interval, normally in.

## `PQUAD`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

integral of the PP representation over (X1,X2).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `LDC`: not a workspace argument
- `C`: not a workspace argument
- `XI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::ppqad`
- Original SLATEC routine: `PPQAD`
- Native symbol: `ppqad_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [PPQAD](https://www.netlib.org/slatec/src/ppqad.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
