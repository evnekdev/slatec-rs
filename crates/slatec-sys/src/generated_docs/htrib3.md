# Purpose

This subroutine is a translation of a complex analogue of the ALGOL procedure TRBAK3, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine forms the eigenvectors of a COMPLEX HERMITIAN matrix by back transforming those of the corresponding real symmetric tridiagonal matrix determined by HTRID3.

# Description

This canonical unsafe binding exposes original SLATEC routine `HTRIB3`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HTRIB3](https://www.netlib.org/slatec/lin/htrib3.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A, ZR, and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable. must be less than or equal to NM.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains some information about the unitary transformations used in the reduction by HTRID3. A is a two-dimensional REAL array, dimensioned A(NM,N).

## `TAU`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (2, *).

contains further information about the transformations. is a one-dimensional REAL array, dimensioned TAU(2,N).

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvectors to be back transformed. is an INTEGER variable.

## `ZR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the eigenvectors to be back transformed in its first M columns. The contents of ZI are immaterial. ZR and the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. NOTE that the last component of each returned vector is real and that vector Euclidean norms are preserved. Questions and comments should be directed to B. S.

## `ZI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

are two-dimensional REAL arrays, dimensioned ZR(NM,M) and the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. NOTE that the last component of each returned vector is real and that vector Euclidean norms are preserved. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `TAU`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::htrib3`
- Original SLATEC routine: `HTRIB3`
- Native symbol: `htrib3_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`
- Exact Netlib source file: [HTRIB3](https://www.netlib.org/slatec/lin/htrib3.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
