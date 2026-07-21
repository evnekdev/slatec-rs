# Purpose

The following definitions are used in DBSKIN: Definition 1 KI(0,X) = K-zero Bessel function. Definition 2 KI(N,X) = Bickley Function = integral from X to infinity of KI(N-1,t)dt for X .ge. 0 and N = 1,2,... _____________________________________________________________________ DBSKIN computes a sequence of Bickley functions (repeated integrals of the K0 Bessel function); i.e. for fixed X and N and for K=1,..., DBSKIN computes the sequence

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSKIN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSKIN](https://www.netlib.org/slatec/src/dbskin.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Argument, X. ge. 0. 0D0.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of first member of the sequence N. ge. 0.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Selection parameter 1 returns Y(K)= KI(N+K-1,X), K=1,M = 2 returns Y(K)=EXP(X)*KI(N+K-1,X), K=1,M 1. Y(K)=0. 0D0, K=1,. ,M is returned 1 AND Y(K)=0. 0E0, K=1,. ,M IS RETURNED.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of members in the sequence, M. ge. 1 OUTPUT Y is a DOUBLE PRECISION VECTOR.

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

KI(N+K-1,X) for KODE=1 EXP(X)*KI(N+K-1,X) for KODE=2, for N. ge. 0 and X. 0 (N and X cannot be zero simultaneously). INPUT X is DOUBLE PRECISION A vector of dimension at least M containing the sequence selected by KODE.

## `NZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Underflow flag NZ = 0 means computation completed = 1 means an exponential underflow occurred on.

## `IERR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0, Normal return, computation completed 1, Input error, no computation 2, Error, no computation Algorithm termination condition not met The nominal computational accuracy is the maximum of unit roundoff (=D1MACH(4)) and 1. 0D-18 since critical constants are given to only 18 digits. BSKIN is the single precision version of DBSKIN.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | 0 means computation completed |
| `NZ` | `1` | 1 means an exponential underflow occurred on |

# Workspace and array requirements

- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dbskin`
- Original SLATEC routine: `DBSKIN`
- Native symbol: `dbskin_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [DBSKIN](https://www.netlib.org/slatec/src/dbskin.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
