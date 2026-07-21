# Purpose

This subroutine is a translation of the ALGOL procedure COMBAK, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). This subroutine forms the eigenvectors of a COMPLEX GENERAL matrix by back transforming those of the corresponding upper Hessenberg matrix determined by COMHES.

# Description

This canonical unsafe binding exposes original SLATEC routine `COMBAK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COMBAK](https://www.netlib.org/slatec/lin/combak.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix.

## `AR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the multipliers which were used in the reduction by COMHES in their lower triangles below the subdiagonal. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH).

## `AI`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the multipliers which were used in the reduction by COMHES in their lower triangles below the subdiagonal. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH).

## `INT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

contains information on the rows and columns interchanged in the reduction by COMHES. Only elements LOW through IGH are used. INT is a one-dimensional INTEGER array, dimensioned INT(IGH).

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvectors to be back transformed. is an INTEGER variable.

## `ZR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first M columns. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). of the transformed eigenvectors in their first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

## `ZI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first M columns. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). of the transformed eigenvectors in their first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `INT`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::combak`
- Original SLATEC routine: `COMBAK`
- Native symbol: `combak_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`
- Exact Netlib source file: [COMBAK](https://www.netlib.org/slatec/lin/combak.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
