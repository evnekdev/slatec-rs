# Purpose

This subroutine is a companion routine to SNLS1,SNLS1E,SNSQ,and SNSQE which may be used to check the calculation of the Jacobian. SUBROUTINE CHKDER This subroutine checks the gradients of M nonlinear functions in N variables, evaluated at a point X, for consistency with the functions themselves. The user must call CKDER twice, first with MODE = 1 and then with MODE = 2.

# Description

This canonical unsafe binding exposes original SLATEC routine `CHKDER`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHKDER](https://www.netlib.org/slatec/src/chkder.f).

# Arguments

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable set to the number of functions.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable set to the number of variables.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an input array of length N.

## `FVEC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an array of length M. On input when MODE = 2, must contain the functions evaluated at X.

## `FJAC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDFJAC, *).

is an M by N array. On input when MODE = 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X.

## `LDFJAC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input parameter not less than M which specifies the leading dimension of the array FJAC.

## `XP`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an array of length N. On output when MODE = 1, is set to a neighboring point of X.

## `FVECP`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an array of length M. On input when MODE = 2, must contain the functions evaluated at XP.

## `MODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

1. On input, X must contain the point of evaluation. On output, XP is set to a neighboring point. 2. On input, FVEC must contain the functions and the rows of FJAC must contain the gradients of the respective functions each evaluated at X, and FVECP must contain the functions evaluated at XP. On output, ERR contains measures of correctness of the respective gradients.

## `ERR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an array of length M. On output when MODE = 2, contains measures of correctness of the respective gradients. If there is no severe loss of significance, then if ERR(I) is 1. 0 the I-th gradient is correct, while if ERR(I) is 0. 0 the I-th gradient is incorrect. For values of ERR between 0.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `FJAC`: not a workspace argument
- `LDFJAC`: not a workspace argument
- `XP`: not a workspace argument
- `FVECP`: not a workspace argument
- `ERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::nonlinear::jacobian_check::chkder`
- Original SLATEC routine: `CHKDER`
- Native symbol: `chkder_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [CHKDER](https://www.netlib.org/slatec/src/chkder.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
