# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPVN is the BSPLVN routine of the reference. DBSPVN calculates the value of all (possibly) nonzero basis functions at X of order MAX(JHIGH,(J+1)*(INDEX-1)), where T(K) .LE. X .LE. T(N+1) and J=IWORK is set inside the routine on the first call when INDEX=1. ILEFT is such that T(ILEFT) .LE.

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSPVN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSPVN](https://www.netlib.org/slatec/src/dbspvn.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K. LE. X. T(N+1). LT. T(ILEFT+1) Output VNIKX, WORK are double precision.

## `JHIGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of B-spline, 1. LE. JHIGH. K.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

highest possible order.

## `INDEX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INDEX = 1 gives basis functions of order JHIGH = 2 denotes previous entry with work, IWORK values saved for subsequent calls to DBSPVN.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

LT. T(ILEFT+1). A call to DINTRV(T,N+1,X,ILO,ILEFT,MFLAG) produces the proper ILEFT. DBSPVN calculates using the basic algorithm needed in DBSPVD. If only basis functions are desired, setting JHIGH=K and INDEX=1 can be faster than calling DBSPVD, but extra coding is required for derivatives (INDEX=2) and DBSPVD is set up for this purpose. Left limiting values are set up as described in DBSPVD.

## `ILEFT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

largest integer such that.

## `VNIKX`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

vector of length K for spline values.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

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

- Canonical Rust path: `slatec_sys::interpolation::dbspvn`
- Original SLATEC routine: `DBSPVN`
- Native symbol: `dbspvn_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DBSPVN](https://www.netlib.org/slatec/src/dbspvn.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
