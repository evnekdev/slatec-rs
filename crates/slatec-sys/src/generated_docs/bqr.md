# Purpose

This subroutine is a translation of the ALGOL procedure BQR, NUM. MATH. 16, 85-92(1970) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL II-LINEAR ALGEBRA, 266-272(1971). This subroutine finds the eigenvalue of smallest (usually) magnitude of a REAL SYMMETRIC BAND matrix using the QR algorithm with shifts of origin. Consecutive calls can be made to find further eigenvalues.

# Description

This canonical unsafe binding exposes original SLATEC routine `BQR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BQR](https://www.netlib.org/slatec/lin/bqr.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM.

## `MB`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the (half) band width of the matrix, defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix. MB is an INTEGER variable. must be less than or equal to N on first call. should not be altered even when it exceeds the current N. Calls PYTHAG(A,B) for SQRT(A**2 + B**2). Questions and comments should be directed to B.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the lower triangle of the symmetric band input matrix stored as an N by MB array. Its lowest subdiagonal is stored in the last N+1-MB positions of the first column, its next subdiagonal in the last N+2-MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of the last column. Contents of storages not part of the matrix are arbitrary. On a subsequent call, its output contents from the previous call should be passed. A is a two-dimensional REAL array, dimensioned A(NM,MB). contains the transformed band matrix.

## `T`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

specifies the shift (of eigenvalues) applied to the diagonal of A in forming the input matrix. What is actually determined is the eigenvalue of A+TI (I is the identity matrix) nearest to T. On a subsequent call, the output value of T from the previous call should be passed if the next nearest eigenvalue is sought. T is a REAL variable. contains the computed eigenvalue of A+TI (if IERR is zero), where I is the identity matrix.

## `R`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

should be specified as zero on the first call, and as its output value from the previous call on a subsequent call. It is used to determine when the last row and column of the transformed band matrix can be regarded as negligible. is a REAL variable. contains the maximum of its input value and the norm of the last column of the input matrix A.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30 iterations.

## `NV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the dimension of the array parameter RV as declared in the calling program dimension statement. is an INTEGER variable.

## `RV`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is a one-dimensional REAL array of dimension NV which is at least (2*MB**2+4*MB-3), used for temporary storage. The first (3*MB-2) locations correspond to the ALGOL array B, the next (2*MB-1) locations correspond to the ALGOL array H, and the final (2*MB**2-MB) locations correspond to the MB by (2*MB-1) ALGOL array U. NOTE. For a subsequent call, N should be replaced by N-1, but.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `RV`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bqr`
- Original SLATEC routine: `BQR`
- Native symbol: `bqr_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BQR](https://www.netlib.org/slatec/lin/bqr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
