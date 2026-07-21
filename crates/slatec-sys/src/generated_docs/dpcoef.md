# Purpose

DPOLFT computes the least squares polynomial fit of degree L as a sum of orthogonal polynomials. DPCOEF changes this fit to its Taylor expansion about any point C , i.e. writes the polynomial as a sum of powers of (X-C). Taking C=0. gives the polynomial in powers of X, but a suitable non-zero C often leads to polynomials which are better scaled and more accurately evaluated. The parameters for DPCOEF are

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCOEF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCOEF](https://www.netlib.org/slatec/src/dpcoef.f).

# Arguments

## `L`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the degree of polynomial to be changed to its Taylor expansion. To obtain the Taylor coefficients in reverse order, input L as the negative of the degree desired. The absolute value of L must be less than or equal to NDEG, the highest degree polynomial fitted by DPOLFT.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

The point about which the Taylor expansion is to be made.

## `TC`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Vector containing the first LL+1 Taylor coefficients where LL=ABS(L). If L. GT. 0 , the coefficients are in the usual Taylor series order, i. e. P(X) = TC(1) + TC(2)*(X-C) +.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Work and output array containing values from last call to DPOLFT.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `TC`: not a workspace argument
- `A`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::dpcoef`
- Original SLATEC routine: `DPCOEF`
- Native symbol: `dpcoef_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DPCOEF](https://www.netlib.org/slatec/src/dpcoef.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
