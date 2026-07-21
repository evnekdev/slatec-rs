# Purpose

This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval and their associated eigenvectors, using bisection and inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `TSTURM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TSTURM](https://www.netlib.org/slatec/lin/tsturm.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable. must be less than or equal to NM.

## `EPS1`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is an absolute error tolerance for the computed eigen- values. It should be chosen so that the accuracy of these eigenvalues is commensurate with relative perturbations of the order of the relative machine precision in the matrix elements. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. EPS1 is a REAL variable. is unaltered unless it has been reset to its (last) default value.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero.

## `E`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero.

## `E2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E. is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N).

## `LB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables.

## `UB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables.

## `MM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

should be set to an upper bound for the number of eigenvalues in the interval. MM is an INTEGER variable. WARNING - If more than MM eigenvalues are determined to lie in the interval, an error return is made with no values or vectors found.

## `M`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvalues determined to lie in (LB,UB). is an INTEGER variable.

## `W`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the M eigenvalues in ascending order if the matrix does not split. If the matrix splits, the eigenvalues are in ascending order for each submatrix. If a vector error exit is made, W contains those values already found. W is a one-dimensional REAL array, dimensioned W(MM).

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the associated set of orthonormal eigenvectors. If an error exit is made, Z contains those vectors already found. Z is a one-dimensional REAL array, dimensioned.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, 3*N+1 if M exceeds MM no eigenvalues or eigenvectors are computed, 4*N+J if the eigenvector corresponding to the J-th eigenvalue fails to converge in 5 iterations, then the eigenvalues and eigenvectors in W and Z should be correct for indices 1, 2,. , J-1.

## `RV1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and.

## `RV2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and.

## `RV3`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and.

## `RV4`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and.

## `RV5`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and.

## `RV6`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and The ALGOL procedure STURMCNT contained in TRISTURM appears in TSTURM in-line. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `W`: not a workspace argument
- `Z`: not a workspace argument
- `RV1`: not a workspace argument
- `RV2`: not a workspace argument
- `RV3`: not a workspace argument
- `RV4`: not a workspace argument
- `RV5`: not a workspace argument
- `RV6`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tsturm`
- Original SLATEC routine: `TSTURM`
- Native symbol: `tsturm_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [TSTURM](https://www.netlib.org/slatec/lin/tsturm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
