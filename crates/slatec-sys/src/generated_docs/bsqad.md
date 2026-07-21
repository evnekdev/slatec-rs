# Purpose

BSQAD computes the integral on (X1,X2) of a K-th order B-spline using the B-representation (T,BCOEF,N,K). Orders K as high as 20 are permitted by applying a 2, 6, or 10 point Gauss formula on subintervals of (X1,X2) which are formed by included (distinct) knots. If orders K greater than 20 are needed, use BFQAD with F(X) = 1.

# Description

This canonical unsafe binding exposes original SLATEC routine `BSQAD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSQAD](https://www.netlib.org/slatec/src/bsqad.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

knot array of length N+K. LE. X. T(N+1).

## `BCOEF`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

B-spline coefficient array of length N.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

length of coefficient array.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of B-spline, 1. LE. K. 20.

## `X1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

end points of quadrature interval in.

## `X2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

end points of quadrature interval in.

## `BQUAD`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

integral of the B-spline over (X1,X2).

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

work vector of length 3*K.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `BCOEF`: not a workspace argument
- `WORK`: work vector of length 3*K

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::bsqad`
- Original SLATEC routine: `BSQAD`
- Native symbol: `bsqad_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BSQAD](https://www.netlib.org/slatec/src/bsqad.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
