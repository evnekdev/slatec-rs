# Purpose

The following definitions are used in BSKIN: Definition 1 KI(0,X) = K-zero Bessel function. Definition 2 KI(N,X) = Bickley Function = integral from X to infinity of KI(N-1,t)dt for X .ge. 0 and N = 1,2,... ____________________________________________________________________ BSKIN computes sequences of Bickley functions (repeated integrals of the K0 Bessel function); i.e. for fixed X and N and K=1,..., BSKIN computes the M-member sequence

# Description

This canonical unsafe binding exposes original SLATEC routine `BSKIN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSKIN](https://www.netlib.org/slatec/src/bskin.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Argument, X. ge. 0. 0E0.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of first member of the sequence N. ge. 0.

## `KODE`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Selection parameter 1 returns Y(K)= KI(N+K-1,X), K=1,M = 2 returns Y(K)=EXP(X)*KI(N+K-1,X), K=1,M 1. Y(K)=0. 0E0, K=1,. ,M is returned.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of members in the sequence, M. ge. 1.

## `Y`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

KI(N+K-1,X) for KODE=1 EXP(X)*KI(N+K-1,X) for KODE=2, for N. ge. 0 and X. 0 (N and X cannot be zero simultaneously). A vector of dimension at least M containing the sequence selected by KODE.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Underflow flag NZ = 0 means computation completed = M means an exponential underflow occurred on.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0, Normal return, computation completed. = 1, Input error, no computation. = 2, Error, no computation. The termination condition was not met. The nominal computational accuracy is the maximum of unit roundoff (=R1MACH(4)) and 1. 0e-18 since critical constants are given to only 18 digits.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | 0 means computation completed = M means an exponential underflow occurred on |

# Workspace and array requirements

- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bskin`
- Original SLATEC routine: `BSKIN`
- Native symbol: `bskin_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [BSKIN](https://www.netlib.org/slatec/src/bskin.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
