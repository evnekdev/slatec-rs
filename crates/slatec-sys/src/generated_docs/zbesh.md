# Purpose

A DOUBLE PRECISION ROUTINE*** On KODE=1, ZBESH computes an N member sequence of complex Hankel (Bessel) functions CY(L)=H(M,FNU+L-1,Z) for super- script M=1 or 2, real nonnegative orders FNU+L-1, L=1,..., N, and complex nonzero Z in the cut plane -pi<arg(Z)<=pi where Z=ZR+i*ZI. On KODE=2, CBESH returns the scaled CY(L) = H(M,FNU+L-1,Z)*exp(-(3-2*M)*Z*i), i**2=-1 which removes the exponential behavior in both the upper and lower half planes. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

# Description

This canonical unsafe binding exposes original SLATEC routine `ZBESH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ZBESH](https://www.netlib.org/slatec/src/zbesh.f).

# Arguments

## `ZR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION real part of nonzero argument Z.

## `ZI`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION imag part of nonzero argument Z.

## `FNU`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION initial order, FNU>=0.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

A parameter to indicate the scaling option 1 returns CY(L)=H(M,FNU+L-1,Z), L=1,. ,N =2 returns CY(L)=H(M,FNU+L-1,Z)*exp(-(3-2M)*Z*i),.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Superscript of Hankel function, M=1 or 2.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of terms in the sequence, N>=1.

## `CYR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

DOUBLE PRECISION real part of result vector.

## `CYI`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

DOUBLE PRECISION imag part of result vector.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of underflows set to zero NZ=0 Normal return NZ>0 CY(L)=0 for NZ values of L (if M=1 and Im(Z)>0 or if M=2 and Im(Z)<0, then CY(L)=0 for L=1,. ,NZ; in the com- plementary half planes, the underflows may not be in an uninterrupted sequence).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (abs(Z) too small and/or FNU+N-1 too large) 3 Precision warning - COMPUTATION COMPLETED (Result has half precision or less because abs(Z) or FNU+N-1 is large) 4 Precision error - NO COMPUTATION (Result has no precision because abs(Z) or FNU+N-1 is too large) 5 Algorithmic error - NO COMPUTATION (Termination condition not met).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | Normal return for NZ values of L (if M=1 and |
| `NZ` | `2` | and Im(Z)<0, then |
| `NZ` | `0` | for L=1,...,NZ; in the com- plementary half planes, the underflows may not be in an uninterrupted sequence) |

# Workspace and array requirements

- `CYR`: not a workspace argument
- `CYI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::zbesh`
- Original SLATEC routine: `ZBESH`
- Native symbol: `zbesh_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [ZBESH](https://www.netlib.org/slatec/src/zbesh.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
