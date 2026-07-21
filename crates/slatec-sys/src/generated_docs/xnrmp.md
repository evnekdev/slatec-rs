# Purpose

SUBROUTINE TO CALCULATE NORMALIZED LEGENDRE POLYNOMIALS (DXNRMP is double-precision version) XNRMP calculates normalized Legendre polynomials of varying order and fixed argument and degree. The order MU and degree

# Description

This canonical unsafe binding exposes original SLATEC routine `XNRMP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [XNRMP](https://www.netlib.org/slatec/src/xnrmp.f).

# Arguments

## `NU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

are non-negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J. M. Smith, F. W.

## `MU1`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

GE. 0 specifies the lowest-order normalized Legendre polynomial that is wanted.

## `MU2`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

GE. MU1 specifies the highest-order normalized Leg- endre polynomial that is wanted.

## `SARG`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input real argument. With `MODE=1` it is the Legendre argument and must lie in \[-1,1\]; with `MODE=2` it is an angle strictly between -pi and pi and the routine uses its cosine.

## `MODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

1 and -1. 0. LE. SARG. 1. 0 specifies that Normalized Legendre(NU, MU, SARG) is wanted for MU = MU1, MU1 + 1,.

## `SPN`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Writable extended-range mantissa array. Together with `IPN`, element `I` represents the normalized Legendre value for order `MU1+I-1` in the selected extended-range representation.

## `IPN`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

0 the value of the normalized Legendre polynomial is contained entirely in SPN(I) and subsequent single-precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I). NE. 0 the corre- sponding value of the normalized Legendre polynomial cannot be represented in single-precision because of overflow or under- flow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case that IPN(I) is nonzero, the user should try using double pre- cision if it has a wider exponent range.

## `ISIG`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Writable output estimate of decimal digits lost to rounding in the extended-range normalized-Legendre calculation. Subtract it from the significant digits in the input argument to estimate retained result precision away from zeros.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an error indicator. If no errors are detected, IERROR=0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. If IERROR=112 or 113, invalid input was provided to XNRMP. If IERROR=101,102,103, or 104, invalid input was provided to XSET.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. |
| `IERROR` | `112` | or 113, invalid input was provided to XNRMP. |
| `IERROR` | `101` | ,102,103, or 104, invalid input was provided to XSET. |
| `IERROR` | `105` | or 106, an internal consistency error occurred in XSET (probably due to a software malfunction in the |

# Workspace and array requirements

- `SPN`: not a workspace argument
- `IPN`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::xnrmp`
- Original SLATEC routine: `XNRMP`
- Native symbol: `xnrmp_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [XNRMP](https://www.netlib.org/slatec/src/xnrmp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
