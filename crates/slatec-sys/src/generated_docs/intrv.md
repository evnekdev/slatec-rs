# Purpose

Written by Carl de Boor and modified by D. E. Amos INTRV is the INTERV routine of the reference. INTRV computes the largest integer ILEFT in 1 .LE. ILEFT .LE. LXT such that XT(ILEFT) .LE. X where XT(*) is a subdivision of the X interval. Precisely,

# Description

This canonical unsafe binding exposes original SLATEC routine `INTRV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [INTRV](https://www.netlib.org/slatec/src/intrv.f).

# Arguments

## `XT`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

LE. X LXT 1, That is, when multiplicities are present in the break point to the left of X, the largest index is taken for ILEFT. XT is a knot or break point vector of length LXT.

## `LXT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

length of the XT vector.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

LT. XT(1) 1 -1 if XT(I). LE. X. XT(I+1) then ILEFT=I , MFLAG=0 argument.

## `ILO`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

an initialization parameter which must be set to 1 the first time the spline array XT is processed by INTRV. ILO contains information for efficient process- ing after the initial call, and ILO must not be changed by the user. Distinct splines require distinct ILO parameters.

## `ILEFT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

largest integer satisfying XT(ILEFT). LE. X.

## `MFLAG`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

signals when X lies out of bounds.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `XT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::intrv`
- Original SLATEC routine: `INTRV`
- Native symbol: `intrv_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [INTRV](https://www.netlib.org/slatec/src/intrv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
