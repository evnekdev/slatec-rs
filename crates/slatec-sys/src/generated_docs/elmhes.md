# Purpose

This subroutine is a translation of the ALGOL procedure ELMHES, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). Given a REAL GENERAL matrix, this subroutine reduces a submatrix situated in rows and columns LOW through IGH to upper Hessenberg form by stabilized elementary similarity transformations.

# Description

This canonical unsafe binding exposes original SLATEC routine `ELMHES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ELMHES](https://www.netlib.org/slatec/lin/elmhes.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix, A. N is an INTEGER variable. must be less than or equal to NM.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the input matrix. A is a two-dimensional REAL array, dimensioned A(NM,N). contains the upper Hessenberg matrix. The multipliers which were used in the reduction are stored in the remaining triangle under the Hessenberg matrix.

## `INT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

contains information on the rows and columns interchanged in the reduction. Only elements LOW through IGH are used. is a one-dimensional INTEGER array, dimensioned INT(IGH). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `INT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::elmhes`
- Original SLATEC routine: `ELMHES`
- Native symbol: `elmhes_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1)`
- Exact Netlib source file: [ELMHES](https://www.netlib.org/slatec/lin/elmhes.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
