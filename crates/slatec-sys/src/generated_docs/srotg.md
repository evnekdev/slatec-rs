# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SROTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SROTG](https://www.netlib.org/slatec/lin/srotg.f).

# Arguments

## `SA`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scalar single precision result R.

## `SB`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scalar single precision result Z.

## `SC`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision result.

## `SS`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision result Construct the Givens transformation ( SC SS ) G = ( ) , SC**2 + SS**2 = 1 , (-SS SC ) which zeros the second entry of the 2-vector (SA,SB)**T. The quantity R = (+/-)SQRT(SA**2 + SB**2) overwrites SA in storage. The value of SB is overwritten by a value Z which allows SC and SS to be recovered by the following algorithm: If Z=1 set SC=0. 0 and SS=1. 0 If ABS(Z). LT.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::srotg`
- Original SLATEC routine: `SROTG`
- Native symbol: `srotg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SROTG](https://www.netlib.org/slatec/lin/srotg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
