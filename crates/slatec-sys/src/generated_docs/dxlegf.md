# Purpose

DXLEGF: Extended-range Double-precision Legendre Functions A feature of the DXLEGF subroutine for Legendre functions is the use of extended-range arithmetic, a software extension of ordinary floating-point arithmetic that greatly increases the exponent range of the representable numbers. This avoids the need for scaling the solutions to lie within the exponent range of the most restrictive manufacturer's hardware. The increased exponent range is achieved by allocating an integer storage location together with each floating-point storage location. The interpretation of the pair (X,I) where X is floating-point and I is integer is X*(IR**I) where IR is the internal radix of the computer arithmetic. This subroutine computes one of the following vectors: 1. Legendre function of the first kind of negative order, either a. P(-MU1,NU,X), P(-MU1-1,NU,X), ..., P(-MU2,NU,X) or b. P(-MU,NU1,X), P(-MU,NU1+1,X), ..., P(-MU,NU2,X) 2. Legendre function of the second kind, either a. Q(MU1,NU,X), Q(MU1+1,NU,X), ..., Q(MU2,NU,X) or b. Q(MU,NU1,X), Q(MU,NU1+1,X), ..., Q(MU,NU2,X) 3. Legendre function of the first kind of positive order, either a. P(MU1,NU,X), P(MU1+1,NU,X), ..., P(MU2,NU,X) or b. P(MU,NU1,X), P(MU,NU1+1,X), ..., P(MU,NU2,X) 4. Normalized Legendre polynomials, either a. PN(MU1,NU,X), PN(MU1+1,NU,X), ..., PN(MU2,NU,X) or b. PN(MU,NU1,X), PN(MU,NU1+1,X), ..., PN(MU,NU2,X) where X = COS(THETA). The input values to DXLEGF are DNU1, NUDIFF, MU1, MU2, THETA, and ID. These must satisfy

# Description

This canonical unsafe binding exposes original SLATEC routine `DXLEGF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DXLEGF](https://www.netlib.org/slatec/src/dxlegf.f).

# Arguments

## 1. `DNU1`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 0.5; not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `NUDIFF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. negative; 0 or MU2 = MU1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `MU1`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. negative; not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `MU2`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is INTEGER and greater than or equal to MU1; MU1+1 or NUDIFF+1. Where possible, DXLEGF returns IPQA(I) as zero. In this case the value of the Legendre function is contained entirely in PQA(I), so it can be used in subsequent computations without further consideration of extended-range arithmetic. If IPQA(I) is nonzero, then the value of the Legendre function is not representable in floating-point because of underflow or overflow. The program that calls DXLEGF must test IPQA(I) to ensure correct usage. is INTEGER and greater than or equal to MU1; MU1+1 or NUDIFF+1. Where possible, DXLEGF returns IPQA(I) as zero. In this case the value of the Legendre function is contained entirely in PQA(I), so it can be used in subsequent computations without further consideration of extended-range arithmetic. If IPQA(I) is nonzero, then the value of the Legendre function is not representable in floating-point because of underflow or overflow. The program that calls DXLEGF must test IPQA(I) to ensure correct usage. not applicable or not stated by selected source not a workspace argument

## 5. `THETA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. open interval (0,PI/2\]; not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ID`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is INTEGER and equal to 1, 2, 3 or 4; 1 and NUDIFF=0, a vector of type 1a above is computed with NU=DNU1. 1 and MU1=MU2, a vector of type 1b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. 2 and NUDIFF=0, a vector of type 2a above is computed with NU=DNU1. 2 and MU1=MU2, a vector of type 2b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. 3 and NUDIFF=0, a vector of type 3a above is computed with NU=DNU1. 3 and MU1=MU2, a vector of type 3b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. 4 and NUDIFF=0, a vector of type 4a above is computed with NU=DNU1. 4 and MU1=MU2, a vector of type 4b above is computed with NU1=DNU1, NU2=DNU1+NUDIFF and MU=MU1. In each case the vector of computed Legendre function values is returned in the extended-range vector (PQA(I),IPQA(I)). The not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `PQA`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Writable extended-range mantissa array. Together with `IPQA`, it returns the requested Legendre-function vector; element `I` represents `PQA\[I\] * radix^IPQA\[I\]`. Its required length is `MU2-MU1+1` or `NUDIFF+1`, according to the selected vector form. Writable extended-range mantissa array. Together with `IPQA`, it returns the requested Legendre-function vector; element `I` represents `PQA\[I\] * radix^IPQA\[I\]`. Its required length is `MU2-MU1+1` or `NUDIFF+1`, according to the selected vector form. not applicable or not stated by selected source not a workspace argument

## 8. `IPQA`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Writable extended-range exponent array paired element-for-element with `PQA`. A zero entry means the corresponding value is directly representable in the routine precision; callers must inspect nonzero entries before treating `PQA` alone as the result. Writable extended-range exponent array paired element-for-element with `PQA`. A zero entry means the corresponding value is directly representable in the routine precision; callers must inspect nonzero entries before treating `PQA` alone as the result. not applicable or not stated by selected source not a workspace argument

## 9. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 0 0 when control returns to the calling routine. If an error is detected, when control returns to the calling routine. If an error is detected, is returned as nonzero. The calling routine must check the value of IERROR. 210 or 211, invalid input was provided to DXLEGF. 201,202,203, or 204, invalid input was provided to DXSET. 205 or 206, an internal consistency error occurred in DXSET (probably due to a software malfunction in the library routine I1MACH). range number was detected in DXADJ. range number was detected in DXC210. 0 0 when control returns to the calling routine. If an error is detected, when control returns to the calling routine. If an error is detected, is returned as nonzero. The calling routine must check the value of IERROR. 210 or 211, invalid input was provided to DXLEGF. 201,202,203, or 204, invalid input was provided to DXSET. 205 or 206, an internal consistency error occurred in DXSET (probably due to a software malfunction in the library routine I1MACH). range number was detected in DXADJ. range number was detected in DXC210. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `DNU1`: not a workspace argument
- `NUDIFF`: not a workspace argument
- `MU1`: not a workspace argument
- `MU2`: not a workspace argument
- `THETA`: not a workspace argument
- `ID`: not a workspace argument
- `PQA`: not a workspace argument
- `IPQA`: not a workspace argument
- `IERROR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dxlegf`
- Original SLATEC routine: `DXLEGF`
- Native symbol: `dxlegf_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DXLEGF](https://www.netlib.org/slatec/src/dxlegf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
