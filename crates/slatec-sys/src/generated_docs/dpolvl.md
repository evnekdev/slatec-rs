# Purpose

Subroutine DPOLVL calculates the value of the polynomial and its first NDER derivatives where the polynomial was produced by a previous call to DPLINT. The variable N and the arrays X and C must not be altered between the call to DPLINT and the call to DPOLVL. Dimensioning Information *******

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOLVL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOLVL](https://www.netlib.org/slatec/src/dpolvl.f).

# Arguments

## `NDER`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the number of derivatives to be evaluated.

## `XX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

the argument at which the polynomial and its derivatives are to be evaluated.

## `YFIT`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

the value of the polynomial at XX.

## `YP`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

must be dimensioned by at least NDER the derivatives of the polynomial at XX. The derivative of order J at XX is stored in YP(J) , J = 1,. ,NDER.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

***** N, X, and C must not be altered between the call.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

must be dimensioned by at least N (see the abstract ) * to DPLINT and the call to DPOLVL.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

must be dimensioned by at least N (see the abstract ) *****.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

must be dimensioned by at least 2*N if NDER is. GT. 0. Note *** If NDER=0, neither YP nor WORK need to be dimensioned variables. If NDER=1, YP does not need to be a dimensioned variable. this is an array to provide internal working storage for DPOLVL. must be dimensioned by at least 2*N if NDER is .GT. 0. Note *** If NDER=0, neither YP nor WORK need to be dimensioned variables. If NDER=1, YP does not need to be a dimensioned variable. this is an array to provide internal working storage for DPOLVL. It must be dimensioned by at least 2*N if NDER is .GT. 0. If NDER=0, WORK does not need to be a dimensioned

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Output error flag with the following possible values. = 1 indicates normal execution.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `YP`: not a workspace argument
- `X`: not a workspace argument
- `C`: not a workspace argument
- `WORK`: must be dimensioned by at least 2*N if NDER is .GT. 0. Note *** If NDER=0, neither YP nor WORK need to be dimensioned variables. If NDER=1, YP does not need to be a dimensioned variable. this is an array to provide internal working storage for DPOLVL. It must be dimensioned by at least 2*N if NDER is .GT. 0. If NDER=0, WORK does not need to be a dimensioned

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpolvl`
- Original SLATEC routine: `DPOLVL`
- Native symbol: `dpolvl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DPOLVL](https://www.netlib.org/slatec/src/dpolvl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
