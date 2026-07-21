# Purpose

This routine computes all zeros of a polynomial of degree NDEG with real coefficients by computing the eigenvalues of the companion matrix. Description of Parameters The user must dimension all arrays appearing in the call list COEFF(NDEG+1), ROOT(NDEG), WORK(NDEG*(NDEG+2))

# Description

This canonical unsafe binding exposes original SLATEC routine `RPQR79`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RPQR79](https://www.netlib.org/slatec/src/rpqr79.f).

# Arguments

## `NDEG`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

degree of polynomial.

## `COEFF`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL coefficients in descending order. i. e. , P(Z)= COEFF(1)*(Z**NDEG) + COEFF(NDEG)*Z + COEFF(NDEG+1).

## `ROOT`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX vector of roots.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Output Error Code - Normal Code 0 means the roots were computed. - Abnormal Codes 1 more than 30 QR iterations on some eigenvalue of the companion matrix 2 COEFF(1)=0. 0 3 NDEG is invalid (less than or equal to 0).

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL work array of dimension at least NDEG*(NDEG+2).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `COEFF`: not a workspace argument
- `ROOT`: not a workspace argument
- `WORK`: REAL work array of dimension at least NDEG*(NDEG+2).

# ABI notes

- Canonical Rust path: `slatec_sys::roots::complex::rpqr79`
- Original SLATEC routine: `RPQR79`
- Native symbol: `rpqr79_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`
- Exact Netlib source file: [RPQR79](https://www.netlib.org/slatec/src/rpqr79.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
