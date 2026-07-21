# Purpose

On KODE=1, CAIRY computes the complex Airy function Ai(z) or its derivative dAi/dz on ID=0 or ID=1 respectively. On

# Description

This canonical unsafe binding exposes original SLATEC routine `CAIRY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CAIRY](https://www.netlib.org/slatec/src/cairy.f).

# Arguments

## `Z`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Argument of type COMPLEX.

## `ID`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of derivative, ID=0 or ID=1.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

2, a scaling option exp(zeta)*Ai(z) or exp(zeta)*dAi/dz is provided to remove the exponential decay in -pi/3<arg(z) <pi/3 and the exponential growth in pi/3<abs(arg(z))<pi where zeta=(2/3)*z**(3/2). While the Airy functions Ai(z) and dAi/dz are analytic in the whole z-plane, the corresponding scaled functions defined for KODE=2 have a cut along the negative real axis. A parameter to indicate the scaling option 1 returns.

## `AI`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Ai(z) on ID=0 dAi/dz on ID=1 at z=Z =2 returns exp(zeta)*Ai(z) on ID=0 exp(zeta)*dAi/dz on ID=1 at z=Z where zeta=(2/3)*z**(3/2) Result of type COMPLEX.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Underflow indicator NZ=0 Normal return NZ=1 AI=0 due to underflow in -pi/3<arg(Z)<pi/3 on KODE=1.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (Re(Z) too large with KODE=1) 3 Precision warning - COMPUTATION COMPLETED (Result has less than half precision) 4 Precision error - NO COMPUTATION (Result has no precision) 5 Algorithmic error - NO COMPUTATION (Termination condition not met).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | Normal return |
| `NZ` | `1` | AI=0 due to underflow in |

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::cairy`
- Original SLATEC routine: `CAIRY`
- Native symbol: `cairy_`
- ABI fingerprint: `subroutine:void(mut_complex32,mut_i32,mut_i32,mut_complex32,mut_i32,mut_i32)`
- Exact Netlib source file: [CAIRY](https://www.netlib.org/slatec/src/cairy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
