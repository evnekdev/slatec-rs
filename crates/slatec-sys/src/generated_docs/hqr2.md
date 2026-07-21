# Purpose

This subroutine is a translation of the ALGOL procedure HQR2, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). This subroutine finds the eigenvalues and eigenvectors of a REAL UPPER Hessenberg matrix by the QR method. The eigenvectors of a REAL GENERAL matrix can also be found if ELMHES and ELTRAN or ORTHES and ORTRAN have been used to reduce this general matrix to Hessenberg form and to accumulate the similarity transformations.

# Description

This canonical unsafe binding exposes original SLATEC routine `HQR2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HQR2](https://www.netlib.org/slatec/lin/hqr2.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, H and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix H. N is an INTEGER variable. must be less than or equal to NM.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine BALANC. If BALANC has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `H`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the upper Hessenberg matrix. H is a two-dimensional REAL array, dimensioned H(NM,N).

## `WR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues. The eigenvalues are unordered except that complex conjugate pairs of values appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one- dimensional REAL arrays, dimensioned WR(N) and WI(N).

## `WI`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues. The eigenvalues are unordered except that complex conjugate pairs of values appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one- dimensional REAL arrays, dimensioned WR(N) and WI(N).

## `Z`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the transformation matrix produced by ELTRAN after the reduction by ELMHES, or by ORTRAN after the reduction by ORTHES, if performed. If the eigenvectors of the Hessenberg matrix are desired, Z must contain the identity matrix. Z is a two-dimensional REAL array, dimensioned Z(NM,M). contains the real and imaginary parts of the eigenvectors. If the J-th eigenvalue is real, the J-th column of Z contains its eigenvector. If the J-th eigenvalue is complex with positive imaginary part, the J-th and (J+1)-th columns of Z contain the real and imaginary parts of its eigenvector.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2,. , N, but no eigenvectors are computed. Calls CDIV for complex division. Questions and comments should be directed to B. S.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `H`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::hqr2`
- Original SLATEC routine: `HQR2`
- Native symbol: `hqr2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [HQR2](https://www.netlib.org/slatec/lin/hqr2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
