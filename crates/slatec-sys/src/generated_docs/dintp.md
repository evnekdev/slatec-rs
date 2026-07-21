# Purpose

The methods in subroutine DSTEPS approximate the solution near X by a polynomial. Subroutine DINTP approximates the solution at

# Description

This canonical unsafe binding exposes original SLATEC routine `DINTP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DINTP](https://www.netlib.org/slatec/src/dintp.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input current integration abscissa from `DSTEPS`. It and the history arguments must be from the same unmodified `DSTEPS` state.

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Readable current solution vector from `DSTEPS`, with at least `NEQN` elements.

## `XOUT`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

by evaluating the polynomial there. Information defining this polynomial is passed from DSTEPS so DINTP cannot be used alone. Subroutine DSTEPS is completely explained and documented in the text "Computer Solution of Ordinary Differential Equations, the Initial Value Problem" by L. F. Shampine and M. K.

## `YOUT`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

solution at XOUT.

## `YPOUT`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

derivative of solution at XOUT The remaining parameters are returned unaltered from their input values. Integration with DSTEPS may be continued.

## `NEQN`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input number of differential equations. It is the required length of `Y`, `YOUT`, and `YPOUT` and the first dimension of `PHI`.

## `KOLD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input interpolation order saved by `DSTEPS`. It controls how many columns of the `PHI` history are used and must be passed unchanged from that integrator state.

## `PHI`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (NEQN, 16).

Readable `DSTEPS` history matrix with Fortran shape `(NEQN, 16)`. It defines the local interpolation polynomial and must not be synthesized independently.

## `IVC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Readable interpolation-cache control index supplied by `DSTEPS`; it selects cached data in `IV` and `OW` when the order changes.

## `IV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (10).

Readable integer interpolation cache of length 10 supplied by `DSTEPS`. It is part of the persistent integrator state and must be passed unchanged.

## `KGI`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Readable `DSTEPS` interpolation-history order marker used to decide whether cached `GI` values apply.

## `GI`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (11).

Readable interpolation cache of length 11 supplied by `DSTEPS`. It stores precomputed integral factors and must remain consistent with `KGI` and `KOLD`.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (12).

Readable `DSTEPS` coefficient array of length 12 used to reconstruct the interpolation factors.

## `OG`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (13).

Readable `DSTEPS` interpolation-history array of length 13 used when evaluating the local polynomial.

## `OW`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (12).

Readable `DSTEPS` interpolation-cache array of length 12. It is indexed through `IVC` and `IV` and is not independent workspace.

## `OX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Readable previous integration abscissa from `DSTEPS`; together with `X` it defines the interpolation interval.

## `OY`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Readable previous solution vector from `DSTEPS`, with at least `NEQN` elements. It supplies the endpoint data for the smooth interpolant.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `Y`: not a workspace argument
- `YOUT`: not a workspace argument
- `YPOUT`: not a workspace argument
- `PHI`: not a workspace argument
- `IV`: not a workspace argument
- `GI`: not a workspace argument
- `ALPHA`: not a workspace argument
- `OG`: not a workspace argument
- `OW`: Readable `DSTEPS` interpolation-cache array of length 12. It is indexed through `IVC` and `IV` and is not independent workspace.
- `OY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::ode::dintp`
- Original SLATEC routine: `DINTP`
- Native symbol: `dintp_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DINTP](https://www.netlib.org/slatec/src/dintp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
