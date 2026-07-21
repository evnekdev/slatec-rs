# Purpose

XLEGF: Extended-range Single-precision Legendre Functions A feature of the XLEGF subroutine for Legendre functions is the use of extended-range arithmetic, a software extension of ordinary floating-point arithmetic that greatly increases the exponent range of the representable numbers. This avoids the need for scaling the solutions to lie within the exponent range of the most restrictive manufacturer's hardware. The increased exponent range is achieved by allocating an integer storage location together with each floating-point storage location. The interpretation of the pair (X,I) where X is floating-point and I is integer is X*(IR**I) where IR is the internal radix of the computer arithmetic. This subroutine computes one of the following vectors: 1. Legendre function of the first kind of negative order, either a. P(-MU1,NU,X), P(-MU1-1,NU,X), ..., P(-MU2,NU,X) or b. P(-MU,NU1,X), P(-MU,NU1+1,X), ..., P(-MU,NU2,X) 2. Legendre function of the second kind, either a. Q(MU1,NU,X), Q(MU1+1,NU,X), ..., Q(MU2,NU,X) or b. Q(MU,NU1,X), Q(MU,NU1+1,X), ..., Q(MU,NU2,X) 3. Legendre function of the first kind of positive order, either a. P(MU1,NU,X), P(MU1+1,NU,X), ..., P(MU2,NU,X) or b. P(MU,NU1,X), P(MU,NU1+1,X), ..., P(MU,NU2,X) 4. Normalized Legendre polynomials, either a. PN(MU1,NU,X), PN(MU1+1,NU,X), ..., PN(MU2,NU,X) or b. PN(MU,NU1,X), PN(MU,NU1+1,X), ..., PN(MU,NU2,X) where X = COS(THETA). The input values to XLEGF are DNU1, NUDIFF, MU1, MU2, THETA, and ID. These must satisfy

# Description

This canonical unsafe binding exposes original SLATEC routine `XLEGF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [XLEGF](https://www.netlib.org/slatec/src/xlegf.f).

# Arguments

## `DNU1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is REAL and greater than or equal to -0. 5;.

## `NUDIFF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is INTEGER and non-negative;.

## `MU1`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is INTEGER and non-negative;.

## `MU2`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is INTEGER and greater than or equal to MU1;.

## `THETA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is REAL and in the half-open interval (0,PI/2\];.

## `ID`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is INTEGER and equal to 1, 2, 3 or 4; and additionally either NUDIFF = 0 or MU2 = MU1. If ID=1 and NUDIFF=0, a vector of type 1a above is computed with NU=DNU1. If ID=1 and MU1=MU2, a vector of type 1b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. If ID=2 and NUDIFF=0, a vector of type 2a above is computed If ID=2 and MU1=MU2, a vector of type 2b above is computed If ID=3 and NUDIFF=0, a vector of type 3a above is computed If ID=3 and MU1=MU2, a vector of type 3b above is computed If ID=4 and NUDIFF=0, a vector of type 4a above is computed If ID=4 and MU1=MU2, a vector of type 4b above is computed In each case the vector of computed Legendre function values is returned in the extended-range vector (PQA(I),IPQA(I)). The length of this vector is either MU2-MU1+1 or NUDIFF+1. Where possible, XLEGF returns IPQA(I) as zero.

## `PQA`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Writable extended-range mantissa array. Together with `IPQA`, it returns the requested Legendre-function vector; element `I` represents `PQA\[I\] * radix^IPQA\[I\]`. Its required length is `MU2-MU1+1` or `NUDIFF+1`, according to the selected vector form.

## `IPQA`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Writable extended-range exponent array paired element-for-element with `PQA`. A zero entry means the corresponding value is directly representable in the routine precision; callers must inspect nonzero entries before treating `PQA` alone as the result.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an error indicator. If no errors are detected, IERROR=0 when control returns to the calling routine. If an error is detected, is returned as nonzero. The calling routine must check the value of IERROR. If IERROR=110 or 111, invalid input was provided to XLEGF. If IERROR=101,102,103, or 104, invalid input was provided to XSET.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `110` | or 111, invalid input was provided to XLEGF. |
| `IERROR` | `101` | ,102,103, or 104, invalid input was provided to XSET. |
| `IERROR` | `105` | or 106, an internal consistency error occurred in XSET (probably due to a software malfunction in the library routine I1MACH). |
| `IERROR` | `107` | , an overflow or underflow of an extended-range number was detected in XADJ. |
| `IERROR` | `108` | , an overflow or underflow of an extended-range number was detected in XC210. SEE ALSO XSET |

# Workspace and array requirements

- `PQA`: not a workspace argument
- `IPQA`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::xlegf`
- Original SLATEC routine: `XLEGF`
- Native symbol: `xlegf_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [XLEGF](https://www.netlib.org/slatec/src/xlegf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
