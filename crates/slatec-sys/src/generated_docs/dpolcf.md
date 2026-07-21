# Purpose

Subroutine DPOLCF computes the coefficients of the polynomial fit (including Hermite polynomial fits ) produced by a previous call to DPLINT. The coefficients of the polynomial, expanded about XX, are stored in the array D. The expansion is of the form P(Z) = D(1) + D(2)*(Z-XX) +D(3)*((Z-XX)**2) + ... +

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOLCF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOLCF](https://www.netlib.org/slatec/src/dpolcf.f).

# Arguments

## `XX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

The point about which the Taylor expansion is to be made.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

**** N, X, and C must remain unchanged between the.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

* call to DPLINT and the call to DPOLCF.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

**** OUTPUT PARAMETER All TYPE REAL variables are DOUBLE PRECISION ***.

## `D`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Between the call to DPLINT and the call to DPOLCF the variable N and the arrays X and C must not be altered. The array of coefficients for the Taylor expansion as explained in the abstract STORAGE PARAMETER.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. Note - There are two methods for evaluating the fit produced by DPLINT. You may call DPOLVL to perform the task, or you may call DPOLCF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by DPLINT, much more accuracy may be expected by calling DPOLVL as opposed to writing your own scheme.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `C`: not a workspace argument
- `D`: not a workspace argument
- `WORK`: This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. Note - There are two methods for evaluating the fit produced by DPLINT. You may call DPOLVL to perform the task, or you may call DPOLCF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by DPLINT, much more accuracy may be expected by calling DPOLVL as opposed to writing your own scheme.

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpolcf`
- Original SLATEC routine: `DPOLCF`
- Native symbol: `dpolcf_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DPOLCF](https://www.netlib.org/slatec/src/dpolcf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
