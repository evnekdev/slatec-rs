# Purpose

A DOUBLE PRECISION ROUTINE*** On KODE=1, ZBIRY computes the complex Airy function Bi(z) or its derivative dBi/dz on ID=0 or ID=1 respectively. On KODE=2, a scaling option exp(abs(Re(zeta)))*Bi(z) or exp(abs(Re(zeta)))*dBi/dz is provided to remove the exponential behavior in both the left and right half planes where zeta=(2/3)*z**(3/2). The Airy functions Bi(z) and dBi/dz are analytic in the whole z-plane, and the scaling option does not destroy this property.

# Description

This canonical unsafe binding exposes original SLATEC routine `ZBIRY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ZBIRY](https://www.netlib.org/slatec/src/zbiry.f).

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

A parameter to indicate the scaling option 1 returns BI=Bi(z) on ID=0 BI=dBi/dz on ID=1 at z=Z =2 returns BI=exp(abs(Re(zeta)))*Bi(z) on ID=0 BI=exp(abs(Re(zeta)))*dBi/dz on ID=1 at z=Z where zeta=(2/3)*z**(3/2).

## `BIR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION real part of result.

## `BII`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION imag part of result.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (Re(Z) too large with KODE=1) 3 Precision warning - COMPUTATION COMPLETED (Result has less than half precision) 4 Precision error - NO COMPUTATION (Result has no precision) 5 Algorithmic error - NO COMPUTATION (Termination condition not met).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::special::zbiry`
- Original SLATEC routine: `ZBIRY`
- Native symbol: `zbiry_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [ZBIRY](https://www.netlib.org/slatec/src/zbiry.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
