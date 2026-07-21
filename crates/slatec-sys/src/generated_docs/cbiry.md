# Purpose

On KODE=1, CBIRY computes the complex Airy function Bi(z) or its derivative dBi/dz on ID=0 or ID=1 respectively. On KODE=2, a scaling option exp(abs(Re(zeta)))*Bi(z) or exp(abs(Re(zeta)))*dBi/dz is provided to remove the exponential behavior in both the left and right half planes where zeta=(2/3)*z**(3/2). The Airy functions Bi(z) and dBi/dz are analytic in the whole z-plane, and the scaling option does not destroy this property.

# Description

This canonical unsafe binding exposes original SLATEC routine `CBIRY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBIRY](https://www.netlib.org/slatec/src/cbiry.f).

# Arguments

## `Z`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Argument of type COMPLEX.

## `ID`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of derivative, ID=0 or ID=1.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

A parameter to indicate the scaling option 1 returns.

## `BI`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Bi(z) on ID=0 dBi/dz on ID=1 at z=Z =2 returns exp(abs(Re(zeta)))*Bi(z) on ID=0 exp(abs(Re(zeta)))*dBi/dz on ID=1 at z=Z where zeta=(2/3)*z**(3/2) Result of type COMPLEX.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (Re(Z) too large with KODE=1) 3 Precision warning - COMPUTATION COMPLETED (Result has less than half precision) 4 Precision error - NO COMPUTATION (Result has no precision) 5 Algorithmic error - NO COMPUTATION (Termination condition not met).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::cbiry`
- Original SLATEC routine: `CBIRY`
- Native symbol: `cbiry_`
- ABI fingerprint: `subroutine:void(mut_complex32,mut_i32,mut_i32,mut_complex32,mut_i32)`
- Exact Netlib source file: [CBIRY](https://www.netlib.org/slatec/src/cbiry.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
