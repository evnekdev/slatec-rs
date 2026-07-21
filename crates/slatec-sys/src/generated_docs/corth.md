# Purpose

This subroutine is a translation of a complex analogue of the ALGOL procedure ORTHES, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). Given a COMPLEX GENERAL matrix, this subroutine reduces a submatrix situated in rows and columns LOW through IGH to upper Hessenberg form by unitary similarity transformations.

# Description

This canonical unsafe binding exposes original SLATEC routine `CORTH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CORTH](https://www.netlib.org/slatec/lin/corth.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, AR and AI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM.

## `LOW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `IGH`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N.

## `AR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex input matrix. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). of the Hessenberg matrix. Information about the unitary transformations used in the reduction is stored in the remaining triangles under the Hessenberg matrix.

## `AI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex input matrix. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). of the Hessenberg matrix. Information about the unitary transformations used in the reduction is stored in the remaining triangles under the Hessenberg matrix.

## `ORTR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

further information about the unitary transformations. Only elements LOW through IGH are used. one-dimensional REAL arrays, dimensioned ORTR(IGH) and ORTI(IGH). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S.

## `ORTI`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

further information about the unitary transformations. Only elements LOW through IGH are used. one-dimensional REAL arrays, dimensioned ORTR(IGH) and ORTI(IGH). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `ORTR`: not a workspace argument
- `ORTI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::corth`
- Original SLATEC routine: `CORTH`
- Native symbol: `corth_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [CORTH](https://www.netlib.org/slatec/lin/corth.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
