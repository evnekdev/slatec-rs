# Purpose

This subprogram solves a linearly constrained least squares problem with both equality and inequality constraints, and, if the user requests, obtains a covariance matrix of the solution

# Description

This canonical unsafe binding exposes original SLATEC routine `DLSEI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DLSEI](https://www.netlib.org/slatec/src/dlsei.f).

# Arguments

## `W`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (MDW, *).

where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for DLSEI( ) are The array W(*,*) is doubly subscripted with The array W(*,*) contains the N by N symmetric covariance matrix of the solution parameters, provided this was requested on input with the option vector PRGOPT(*) and the output flag is returned with MODE = 0 or 1.

## `MDW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The array W(*,*) is doubly subscripted with must satisfy MDW. GE. M. The condition. LT. M is an error.

## `ME`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then.

## `MA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then.

## `MG`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then.

## `PRGOPT`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for DLSEI( ) are This real-valued array is the option vector. If the user is satisfied with the nominal subprogram features set 1 (or PRGOPT(1)=1.

## `X`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for DLSEI( ) are The array X(*) contains the solution parameters.

## `RNORME`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

The array X(*) contains the solution parameters.

## `RNORML`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

if the integer output flag MODE = 0 or 1. The definition of MODE is given directly below. When MODE = 0 or 1, RNORME and RNORML respectively contain the residual vector Euclidean lengths of F - EX and B - AX. When.

## `MODE`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

1 the equality constraint equations EX=F are contradictory, so RNORME. NE. 0. The residual vector F-EX has minimal Euclidean length. For. GE.

## `WS`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for DLSEI( ) are These are respectively type real and type integer working arrays. Their required minimal lengths are given above.

## `IP`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (3).

where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for DLSEI( ) are The amounts of working storage actually allocated for the working arrays WS(*) and IP(*), respectively. These quantities are compared with the actual amounts of storage needed by DLSEI( ).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `W`: not a workspace argument
- `PRGOPT`: not a workspace argument
- `X`: not a workspace argument
- `WS`: not a workspace argument
- `IP`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::dlsei`
- Original SLATEC routine: `DLSEI`
- Native symbol: `dlsei_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DLSEI](https://www.netlib.org/slatec/src/dlsei.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
