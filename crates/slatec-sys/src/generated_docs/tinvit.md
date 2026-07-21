# Purpose

This subroutine is a translation of the inverse iteration tech- nique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of a TRIDIAGONAL SYMMETRIC matrix corresponding to specified eigenvalues, using inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `TINVIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TINVIT](https://www.netlib.org/slatec/lin/tinvit.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable. must be less than or equal to NM.

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N).

## `E`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned is considered negligible if it is not larger than the product of the relative machine precision and the sum of the magnitudes of D(I) and D(I-1). E2(1) must contain 0. 0e0 if the eigenvalues are in ascending order, or 2. 0e0 if the eigenvalues are in descending order.

## `E2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E, with zeros corresponding to negligible elements of E. is a one-dimensional REAL array, dimensioned E2(N).

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of specified eigenvalues for which eigenvectors are to be determined. M is an INTEGER variable.

## `W`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the M eigenvalues in ascending or descending order. is a one-dimensional REAL array, dimensioned W(M).

## `IND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. If BISECT or TRIDIB has been used to determine the eigenvalues, their output IND array is suitable for input to TINVIT. IND is a one-dimensional INTEGER array, dimensioned IND(M).

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the associated set of orthonormal eigenvectors. Any vector which fails to converge is set to zero. is a two-dimensional REAL array, dimensioned Z(NM,M).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, -J if the eigenvector corresponding to the J-th eigenvalue fails to converge in 5 iterations.

## `RV1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage. They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process. RV1, RV2 and.

## `RV2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage. They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process. RV1, RV2 and.

## `RV3`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage. They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process. RV1, RV2 and are dimensioned RV1(N), RV2(N) and RV3(N).

## `RV4`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage. RV4 holds the multipliers of the Gaussian elimination process. RV6 holds the approximate eigenvectors in this process. RV4 and RV6 are dimensioned RV4(N) and.

## `RV6`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage. RV4 holds the multipliers of the Gaussian elimination process. RV6 holds the approximate eigenvectors in this process. RV4 and RV6 are dimensioned RV4(N) and Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `W`: not a workspace argument
- `IND`: not a workspace argument
- `Z`: not a workspace argument
- `RV1`: not a workspace argument
- `RV2`: not a workspace argument
- `RV3`: not a workspace argument
- `RV4`: not a workspace argument
- `RV6`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tinvit`
- Original SLATEC routine: `TINVIT`
- Native symbol: `tinvit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [TINVIT](https://www.netlib.org/slatec/lin/tinvit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
