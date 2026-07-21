# Purpose

Written by Carl de Boor and modified by D. E. Amos BSPVN is the BSPLVN routine of the reference. BSPVN calculates the value of all (possibly) nonzero basis functions at X of order MAX(JHIGH,(J+1)*(INDEX-1)), where

# Description

This canonical unsafe binding exposes original SLATEC routine `BSPVN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSPVN](https://www.netlib.org/slatec/src/bspvn.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

LE. X. T(N+1) and J=IWORK is set inside the routine on the first call when INDEX=1. ILEFT is such that T(ILEFT). LT. T(ILEFT+1).

## `JHIGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of B-spline, 1. LE. JHIGH. K.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

highest possible order.

## `INDEX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INDEX = 1 gives basis functions of order JHIGH = 2 denotes previous entry with WORK, IWORK values saved for subsequent calls to BSPVN.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

argument of basis functions,.

## `ILEFT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

largest integer such that.

## `VNIKX`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

vector of length K for spline values.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work vector of length 2*K.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

a work parameter. Both WORK and IWORK contain information necessary to continue for INDEX = 2. When INDEX = 1 exclusively, these are scratch variables and can be used for other purposes.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `VNIKX`: not a workspace argument
- `WORK`: a work vector of length 2*K
- `IWORK`: a work parameter. Both WORK and IWORK contain information necessary to continue for INDEX = 2. When INDEX = 1 exclusively, these are scratch variables and can be used for other purposes.

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bspvn`
- Original SLATEC routine: `BSPVN`
- Native symbol: `bspvn_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [BSPVN](https://www.netlib.org/slatec/src/bspvn.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
