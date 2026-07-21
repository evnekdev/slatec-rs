# Purpose

This subroutine is a translation of the ALGOL procedure CBABK2, which is a complex version of BALBAK, NUM. MATH. 13, 293-304(1969) by Parlett and Reinsch. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 315-326(1971). This subroutine forms the eigenvectors of a COMPLEX GENERAL matrix by back transforming those of the corresponding balanced matrix determined by CBAL.

# Description

This canonical unsafe binding exposes original SLATEC routine `CBABK2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBABK2](https://www.netlib.org/slatec/lin/cbabk2.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, ZR and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix Z=(ZR,ZI). N is an INTEGER variable. N must be less than or equal to NM.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER variables determined by CBAL.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER variables determined by CBAL.

## `SCALE`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains information determining the permutations and scaling factors used by CBAL. SCALE is a one-dimensional REAL array, dimensioned SCALE(N).

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvectors to be back transformed. is an INTEGER variable.

## `ZR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first M columns. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). respectively, of the transformed eigenvectors Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

## `ZI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first M columns. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). respectively, of the transformed eigenvectors Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `SCALE`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cbabk2`
- Original SLATEC routine: `CBABK2`
- Native symbol: `cbabk2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`
- Exact Netlib source file: [CBABK2](https://www.netlib.org/slatec/lin/cbabk2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
