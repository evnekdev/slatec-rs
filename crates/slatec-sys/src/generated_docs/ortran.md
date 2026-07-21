# Purpose

This subroutine is a translation of the ALGOL procedure ORTRANS, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). This subroutine accumulates the orthogonal similarity transformations used in the reduction of a REAL GENERAL matrix to upper Hessenberg form by ORTHES.

# Description

This canonical unsafe binding exposes original SLATEC routine `ORTRAN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ORTRAN](https://www.netlib.org/slatec/lin/ortran.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains some information about the orthogonal trans- formations used in the reduction to Hessenberg form by ORTHES in its strict lower triangle. A is a two-dimensional REAL array, dimensioned A(NM,IGH).

## `ORT`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains further information about the orthogonal trans- formations used in the reduction by ORTHES. Only elements LOW through IGH are used. ORT is a one-dimensional REAL array, dimensioned ORT(IGH).

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the transformation matrix produced in the reduction by ORTHES to the upper Hessenberg form. Z is a two- dimensional REAL array, dimensioned Z(NM,N). ORT has been used for temporary storage as is not restored. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `ORT`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ortran`
- Original SLATEC routine: `ORTRAN`
- Native symbol: `ortran_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`
- Exact Netlib source file: [ORTRAN](https://www.netlib.org/slatec/lin/ortran.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
