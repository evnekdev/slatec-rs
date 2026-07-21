# Purpose

A DOUBLE PRECISION ROUTINE*** On KODE=1, ZAIRY computes the complex Airy function Ai(z) or its derivative dAi/dz on ID=0 or ID=1 respectively. On

# Description

This canonical unsafe binding exposes original SLATEC routine `ZAIRY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ZAIRY](https://www.netlib.org/slatec/src/zairy.f).

# Arguments

## `ZR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION real part of argument Z.

## `ZI`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION imag part of argument Z.

## `ID`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of derivative, ID=0 or ID=1.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

2, a scaling option exp(zeta)*Ai(z) or exp(zeta)*dAi/dz is provided to remove the exponential decay in -pi/3<arg(z) <pi/3 and the exponential growth in pi/3<abs(arg(z))<pi where zeta=(2/3)*z**(3/2). While the Airy functions Ai(z) and dAi/dz are analytic in the whole z-plane, the corresponding scaled functions defined for KODE=2 have a cut along the negative real axis. A parameter to indicate the scaling option 1 returns AI=Ai(z) on ID=0 AI=dAi/dz on ID=1 at z=Z =2 returns AI=exp(zeta)*Ai(z) on ID=0 AI=exp(zeta)*dAi/dz on ID=1 at z=Z where zeta=(2/3)*z**(3/2).

## `AIR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION real part of result.

## `AII`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION imag part of result.

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

- Canonical Rust path: `slatec_sys::special::zairy`
- Original SLATEC routine: `ZAIRY`
- Native symbol: `zairy_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32,mut_i32)`
- Exact Netlib source file: [ZAIRY](https://www.netlib.org/slatec/src/zairy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
