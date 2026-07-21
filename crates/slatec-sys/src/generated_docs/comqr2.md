# Purpose

This subroutine is a translation of a unitary analogue of the ALGOL procedure COMLR2, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). The unitary analogue substitutes the QR algorithm of Francis (COMP. JOUR. 4, 332-345(1962)) for the LR algorithm. This subroutine finds the eigenvalues and eigenvectors of a COMPLEX UPPER Hessenberg matrix by the QR method. The eigenvectors of a COMPLEX GENERAL matrix can also be found if CORTH has been used to reduce this general matrix to Hessenberg form.

# Description

This canonical unsafe binding exposes original SLATEC routine `COMQR2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COMQR2](https://www.netlib.org/slatec/lin/comqr2.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, HR, HI, ZR, and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix H=(HR,HI). N is an INTEGER variable. N must be less than or equal to NM.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `ORTR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

information about the unitary trans- formations used in the reduction by CORTH, if performed. Only elements LOW through IGH are used. If the eigenvectors of the Hessenberg matrix are desired, set ORTR(J) and ORTI(J) to 0. 0E0 for these elements. ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and.

## `ORTI`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

information about the unitary trans- formations used in the reduction by CORTH, if performed. Only elements LOW through IGH are used. If the eigenvectors of the Hessenberg matrix are desired, set ORTR(J) and ORTI(J) to 0. 0E0 for these elements. ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and.

## `HR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. Their lower triangles below the subdiagonal contain information about the unitary transformations used in the reduction by CORTH, if performed. If the eigenvectors of the Hessenberg matrix are desired, these elements may be arbitrary. HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and.

## `HI`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. Their lower triangles below the subdiagonal contain information about the unitary transformations used in the reduction by CORTH, if performed. If the eigenvectors of the Hessenberg matrix are desired, these elements may be arbitrary. HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and.

## `WR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one- dimensional REAL arrays, dimensioned WR(N) and WI(N).

## `WI`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one- dimensional REAL arrays, dimensioned WR(N) and WI(N).

## `ZR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors. The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been found. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N).

## `ZI`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors. The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been found. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2,. , N, but no eigenvectors are computed. Calls CSROOT for complex square root. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `ORTR`: not a workspace argument
- `ORTI`: not a workspace argument
- `HR`: not a workspace argument
- `HI`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::comqr2`
- Original SLATEC routine: `COMQR2`
- Native symbol: `comqr2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [COMQR2](https://www.netlib.org/slatec/lin/comqr2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
