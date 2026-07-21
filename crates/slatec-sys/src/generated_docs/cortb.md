# Purpose

This subroutine is a translation of a complex analogue of the ALGOL procedure ORTBAK, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). This subroutine forms the eigenvectors of a COMPLEX GENERAL matrix by back transforming those of the corresponding upper Hessenberg matrix determined by CORTH.

# Description

This canonical unsafe binding exposes original SLATEC routine `CORTB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CORTB](https://www.netlib.org/slatec/lin/cortb.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix.

## `AR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

information about the unitary trans- formations used in the reduction by CORTH in their strict lower triangles. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH).

## `AI`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

information about the unitary trans- formations used in the reduction by CORTH in their strict lower triangles. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH).

## `ORTR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

further information about the unitary transformations used in the reduction by CORTH. Only elements LOW through IGH are used. ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and.

## `ORTI`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

further information about the unitary transformations used in the reduction by CORTH. Only elements LOW through IGH are used. ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of columns of Z=(ZR,ZI) to be back transformed. is an INTEGER variable.

## `ZR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first M columns. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). of the transformed eigenvectors in their first M columns. ORTR and ORTI have been altered. Note that CORTB preserves vector Euclidean norms. Questions and comments should be directed to B.

## `ZI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first M columns. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). of the transformed eigenvectors in their first M columns. ORTR and ORTI have been altered. Note that CORTB preserves vector Euclidean norms. Questions and comments should be directed to B.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `ORTR`: not a workspace argument
- `ORTI`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cortb`
- Original SLATEC routine: `CORTB`
- Native symbol: `cortb_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`
- Exact Netlib source file: [CORTB](https://www.netlib.org/slatec/lin/cortb.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
