# Purpose

This subprogram solves a linearly constrained least squares problem. Suppose there are given matrices E and A of respective dimensions ME by N and MA by N, and vectors F and B of respective lengths ME and MA. This subroutine solves the problem EX = F, (equations to be exactly satisfied) AX = B, (equations to be approximately satisfied, in the least squares sense) subject to components L+1,...,N nonnegative Any values ME.GE.0, MA.GE.0 and 0.LE. L .LE.N are permitted. The problem is reposed as problem DWNNLS

# Description

This canonical unsafe binding exposes original SLATEC routine `DWNNLS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DWNNLS](https://www.netlib.org/slatec/src/dwnnls.f).

# Arguments

## `W`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (MDW, *).

The array W(*,*) is double subscripted with first.

## `MDW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array W(*,*) is double subscripted with first.

## `ME`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW.

## `MA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW.

## `L`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW.

## `PRGOPT`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

This double precision array is the option vector. If the user is satisfied with the nominal subprogram features set 1 (or PRGOPT(1)=1. 0) Otherwise PRGOPT(*) is a linked list consisting of groups of data of the following form LINK KEY DATA SET The parameters LINK and KEY are each one word. The DATA SET can be comprised of several words. The number of items depends on the value of KEY. The value of LINK points to the first entry of the next group of data within The exception is when there are no more options to change.

## `X`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

(WT*F) ( A) ( B), (least squares) subject to components L+1,. ,N nonnegative. The subprogram chooses the heavy weight (or penalty parameter) WT. The parameters for DWNNLS are An array dimensioned at least N, which will contain the N components of the solution vector.

## `RNORM`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

The residual norm of the solution. The value of contains the residual vector length of the equality constraints and least squares equations.

## `MODE`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The value of MODE indicates the success or failure of the subprogram. 0 Subprogram completed successfully. = 1 Max. number of iterations (equal to 3*(N-L)) exceeded. Nearly all problems should complete in fewer than this number of iterations. An approximate solution and its corresponding residual vector length are in X(*) and RNORM.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

The amounts of working storage actually allocated for the working arrays WORK(*) and IWORK(*), respectively. These quantities are compared with the actual amounts of storage needed for DWNNLS( ). Insufficient storage allocated for either WORK(*) or IWORK(*) is considered an error. This feature was included in DWNNLS( ) because miscalculating the storage formulas for WORK(*) and IWORK(*) might very well lead to subtle and hard-to-find execution errors. The length of WORK(*) must be at least LW = ME+MA+5*N This test will not be made if IWORK(1). LE. The amounts of working storage actually allocated for the working arrays WORK(*) and IWORK(*), respectively. These quantities are compared with the actual amounts of storage needed for DWNNLS( ). Insufficient storage allocated for either WORK(*) or IWORK(*) is considered an error. This feature was included in DWNNLS( ) because miscalculating the storage formulas for WORK(*) and IWORK(*) might very well lead to subtle and hard-to-find execution errors. The length of WORK(*) must be at least LW = ME+MA+5*N This test will not be made if IWORK(1).LE.0. The length of IWORK(*) must be at least LIW = ME+MA+N This test will not be made if IWORK(2).LE.0. An integer-valued working array of length at least M+N.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

A double precision working array of length at least M + 5*N.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `W`: not a workspace argument
- `PRGOPT`: not a workspace argument
- `X`: not a workspace argument
- `IWORK`: The amounts of working storage actually allocated for the working arrays WORK(*) and IWORK(*), respectively. These quantities are compared with the actual amounts of storage needed for DWNNLS( ). Insufficient storage allocated for either WORK(*) or IWORK(*) is considered an error. This feature was included in DWNNLS( ) because miscalculating the storage formulas for WORK(*) and IWORK(*) might very well lead to subtle and hard-to-find execution errors. The length of WORK(*) must be at least LW = ME+MA+5*N This test will not be made if IWORK(1).LE.0. The length of IWORK(*) must be at least LIW = ME+MA+N This test will not be made if IWORK(2).LE.0. An integer-valued working array of length at least M+N.
- `WORK`: A double precision working array of length at least M + 5*N.

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::dwnnls`
- Original SLATEC routine: `DWNNLS`
- Native symbol: `dwnnls_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DWNNLS](https://www.netlib.org/slatec/src/dwnnls.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
