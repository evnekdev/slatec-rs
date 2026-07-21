# Purpose

This subroutine is a translation of a complex analogue of the ALGOL procedure TRED1, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine reduces a COMPLEX HERMITIAN matrix to a real symmetric tridiagonal matrix using unitary similarity transformations.

# Description

This canonical unsafe binding exposes original SLATEC routine `HTRIDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HTRIDI](https://www.netlib.org/slatec/lin/htridi.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, AR and AI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM.

## `AR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex Hermitian input matrix. Only the lower triangle of the matrix need be supplied. AR and AI are two- dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). some information about the unitary trans- formations used in the reduction in the strict lower triangle of AR and the full lower triangle of AI. The rest of the matrices are unaltered.

## `AI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex Hermitian input matrix. Only the lower triangle of the matrix need be supplied. AR and AI are two- dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). some information about the unitary trans- formations used in the reduction in the strict lower triangle of AR and the full lower triangle of AI. The rest of the matrices are unaltered.

## `D`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the real symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N).

## `E`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the real tridiagonal matrix in its last N-1 positions. E(1) is set to zero. is a one-dimensional REAL array, dimensioned E(N).

## `E2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E. is set to zero. E2 may coincide with E if the squares are not needed. E2 is a one-dimensional REAL array, dimensioned E2(N).

## `TAU`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (2, *).

contains further information about the transformations. is a one-dimensional REAL array, dimensioned TAU(2,N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `TAU`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::htridi`
- Original SLATEC routine: `HTRIDI`
- Native symbol: `htridi_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`
- Exact Netlib source file: [HTRIDI](https://www.netlib.org/slatec/lin/htridi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
