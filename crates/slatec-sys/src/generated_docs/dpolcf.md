# Purpose

Subroutine DPOLCF computes the coefficients of the polynomial fit (including Hermite polynomial fits ) produced by a previous call to DPLINT. The coefficients of the polynomial, expanded

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOLCF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOLCF](https://www.netlib.org/slatec/src/dpolcf.f).

# Arguments

## 1. `XX`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. is of the form The point about which the Taylor expansion is to be made. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. XX)**(N-1)). Between the call to DPLINT and the call to DPOLCF the variable N **** must remain unchanged between the XX)**(N-1)). Between the call to DPLINT and the call to DPOLCF the variable N **** must remain unchanged between the not applicable or not stated by selected source not a workspace argument

## 3. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). must not be altered. must remain unchanged between the *     call to DPLINT and the call to DPOLCF. must not be altered. must remain unchanged between the *     call to DPLINT and the call to DPOLCF. not applicable or not stated by selected source not a workspace argument

## 4. `C`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). must not be altered. must remain unchanged between the **** OUTPUT PARAMETER All TYPE REAL variables are DOUBLE PRECISION *** must not be altered. must remain unchanged between the **** OUTPUT PARAMETER All TYPE REAL variables are DOUBLE PRECISION *** not applicable or not stated by selected source not a workspace argument

## 5. `D`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). is of the form XX) +D(3)*((Z-XX)**2) + ... + XX) +D(3)*((Z-XX)**2) + ... + XX)**(N-1)). Between the call to DPLINT and the call to DPOLCF the variable N The array of coefficients for the Taylor expansion as explained in the abstract STORAGE PARAMETER not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. Note - There are two methods for evaluating the fit produced by DPLINT. You may call DPOLVL to perform the task, or you may call DPOLCF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by DPLINT, much more accuracy may be expected by calling DPOLVL as opposed to writing your own scheme. This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. Note - There are two methods for evaluating the fit produced by DPLINT. You may call DPOLVL to perform the task, or you may call DPOLCF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by DPLINT, much more accuracy may be expected by calling DPOLVL as opposed to writing your own scheme. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `XX`: not a workspace argument
- `N`: not a workspace argument
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
