# Purpose

A DOUBLE PRECISION ROUTINE*** On KODE=1, ZBESJ computes an N member sequence of complex Bessel functions CY(L)=J(FNU+L-1,Z) for real nonnegative orders FNU+L-1, L=1,...,N and complex Z in the cut plane -pi<arg(Z)<=pi where Z=ZR+i*ZI. On KODE=2, CBESJ returns the scaled functions CY(L) = exp(-abs(Y))*J(FNU+L-1,Z), L=1,...,N and Y=Im(Z) which remove the exponential growth in both the upper and lower half planes as Z goes to infinity. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

# Description

This canonical unsafe binding exposes original SLATEC routine `ZBESJ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ZBESJ](https://www.netlib.org/slatec/src/zbesj.f).

# Arguments

## `ZR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION real part of argument Z.

## `ZI`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION imag part of argument Z.

## `FNU`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION initial order, FNU>=0.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

A parameter to indicate the scaling option 1 returns CY(L)=J(FNU+L-1,Z), L=1,. ,N =2 returns CY(L)=J(FNU+L-1,Z)*exp(-abs(Y)), L=1,. ,N where Y=Im(Z).

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

Number of underflows set to zero NZ=0 Normal return NZ>0 CY(L)=0, L=N-NZ+1,. ,N.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (Im(Z) too large on KODE=1) 3 Precision warning - COMPUTATION COMPLETED (Result has half precision or less because abs(Z) or FNU+N-1 is large) 4 Precision error - NO COMPUTATION (Result has no precision because abs(Z) or FNU+N-1 is too large) 5 Algorithmic error - NO COMPUTATION (Termination condition not met).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | Normal return , L=N-NZ+1,...,N |

# Workspace and array requirements

- `CYR`: not a workspace argument
- `CYI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::zbesj`
- Original SLATEC routine: `ZBESJ`
- Native symbol: `zbesj_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [ZBESJ](https://www.netlib.org/slatec/src/zbesj.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
