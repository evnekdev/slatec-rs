# Purpose

This subroutine is a translation of the ALGOL procedure REBAKA, NUM. MATH. 11, 99-110(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 303-314(1971). This subroutine forms the eigenvectors of a generalized SYMMETRIC eigensystem by back transforming those of the derived symmetric matrix determined by REDUC or REDUC2.

# Description

This canonical unsafe binding exposes original SLATEC routine `REBAK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [REBAK](https://www.netlib.org/slatec/lin/rebak.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, B and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix system. N is an INTEGER variable. N must be less than or equal to NM.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains information about the similarity transformation (Cholesky decomposition) used in the reduction by REDUC or REDUC2 in its strict lower triangle. B is a two- dimensional REAL array, dimensioned B(NM,N).

## `DL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains further information about the transformation. is a one-dimensional REAL array, dimensioned DL(N).

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvectors to be back transformed. is an INTEGER variable.

## `Z`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the eigenvectors to be back transformed in its first M columns. Z is a two-dimensional REAL array dimensioned Z(NM,M). contains the transformed eigenvectors in its first Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `B`: not a workspace argument
- `DL`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rebak`
- Original SLATEC routine: `REBAK`
- Native symbol: `rebak_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [REBAK](https://www.netlib.org/slatec/lin/rebak.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
