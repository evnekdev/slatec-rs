# Purpose

Written by Carl de Boor and modified by D. E. Amos PPVAL is the PPVALU function of the reference. PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). The Taylor expansion about XI(J) for X in the interval XI(J) .LE. X .LT. XI(J+1) is evaluated, J=1,LXI. Right limiting values at X=XI(J) are obtained. PPVAL will extrapolate beyond XI(1) and XI(LXI+1). To obtain left limiting values (left derivatives) at XI(J), replace LXI by J-1 and set X=XI(J),J=2,LXI+1.

# Description

This canonical unsafe binding exposes original SLATEC routine `PPVAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PPVAL](https://www.netlib.org/slatec/src/ppval.f).

# Arguments

## `LDC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

leading dimension of C matrix, LDC. GE. K.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDC, *).

matrix of dimension at least (K,LXI) containing right derivatives at break points XI(*).

## `XI`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

break point vector of length LXI+1.

## `LXI`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of polynomial pieces.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of B-spline, K. GE. 1.

## `IDERIV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the derivative, 0. LE. IDERIV. K-1 0 gives the B-spline value.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

argument, XI(1). LE. X. XI(LXI+1).

## `INPPV`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

an initialization parameter which must be set to 1 the first time PPVAL is called. INPPV contains information for efficient process- ing after the initial call and INPPV must not be changed by the user. Distinct splines require distinct INPPV parameters. PPVAL - value of the IDERIV-th derivative at X.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `LDC`: not a workspace argument
- `C`: not a workspace argument
- `XI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::ppval`
- Original SLATEC routine: `PPVAL`
- Native symbol: `ppval_`
- ABI fingerprint: `function:f32(mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`
- Exact Netlib source file: [PPVAL](https://www.netlib.org/slatec/src/ppval.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
