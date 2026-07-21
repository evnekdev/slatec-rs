# Purpose

SGBMV performs one of the matrix-vector operations

# Description

This canonical unsafe binding exposes original SLATEC routine `SGBMV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGBMV](https://www.netlib.org/slatec/lin/sgbmv.f).

# Arguments

## 1. `TRANS`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: = alpha*A*x + beta*y. = alpha*A'*x + beta*y. = alpha*A'*x + beta*y. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. diagonals and ku super-diagonals. Parameters ========== INTEGER. On entry, M specifies the number of rows of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ) otherwise. Before entry, the incremented array X must contain the vector x. Unchanged on exit. 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' and at least diagonals and ku super-diagonals. Parameters ========== INTEGER. On entry, M specifies the number of rows of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ) otherwise. Before entry, the incremented array X must contain the vector x. Unchanged on exit. 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' and at least not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. diagonals and ku super-diagonals. Parameters ========== = alpha*A*x + beta*y. = alpha*A*x + beta*y. INTEGER. On entry, N specifies the number of columns of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' and at least 1 )*abs( INCY ) ) otherwise. Before entry, the incremented array Y must contain the diagonals and ku super-diagonals. Parameters ========== = alpha*A*x + beta*y. = alpha*A*x + beta*y. INTEGER. On entry, N specifies the number of columns of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' and at least 1 )*abs( INCY ) ) otherwise. Before entry, the incremented array Y must contain the not applicable or not stated by selected source not a workspace argument

## 4. `KL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. diagonals and ku super-diagonals. Parameters ========== INTEGER. diagonals of the must satisfy  0 .le. KL. Unchanged on exit. diagonals and ku super-diagonals. Parameters ========== INTEGER. diagonals of the must satisfy  0 .le. KL. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `KU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. diagonals of the must satisfy  0 .le. KU. Unchanged on exit. diagonal diagonal starting at position 1 in row ( ku + 2 ), and so on. Elements in the array A that do not correspond to elements in the band matrix (such as the top left ku by ku triangle) are not referenced. The following program segment will transfer a band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N J DO 10, I = MAX( 1, J - KU ), MIN( M, J + KL ) INTEGER. diagonals of the must satisfy  0 .le. KU. Unchanged on exit. diagonal diagonal starting at position 1 in row ( ku + 2 ), and so on. Elements in the array A that do not correspond to elements in the band matrix (such as the top left ku by ku triangle) are not referenced. The following program segment will transfer a band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N J DO 10, I = MAX( 1, J - KU ), MIN( M, J + KL ) not applicable or not stated by selected source not a workspace argument

## 6. `ALPHA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is an REAL            . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). is an must satisfy  0 .le. KL. Unchanged on exit. must satisfy  0 .le. KU. Unchanged on exit. REAL             array of DIMENSION ( LDA, n ). Before entry, the leading ( kl + ku + 1 ) by n part of the array A must contain the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in matrix( I, J ) 10    CONTINUE 20 CONTINUE Unchanged on exit. is an must satisfy  0 .le. KL. Unchanged on exit. must satisfy  0 .le. KU. Unchanged on exit. REAL             array of DIMENSION ( LDA, n ). Before entry, the leading ( kl + ku + 1 ) by n part of the array A must contain the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in matrix( I, J ) 10    CONTINUE 20 CONTINUE Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 8. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( kl + ku + 1 ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( kl + ku + 1 ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( kl + ku + 1 ). Unchanged on exit. not a workspace argument

## 9. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is an REAL             array of DIMENSION at least must not be zero. Unchanged on exit. is an REAL             array of DIMENSION at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 10. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 11. `BETA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is an REAL            . On entry, BETA specifies the scalar beta. When BETA is supplied as zero then Y need not be set on input. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `Y`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). = alpha*A*x + beta*y,   or   y := alpha*A'*x + beta*y, is an = alpha*A*x + beta*y. = alpha*A'*x + beta*y. = alpha*A'*x + beta*y. Unchanged on exit. REAL             array of DIMENSION at least is overwritten by the updated vector y. is overwritten by the updated vector y. must not be zero. Unchanged on exit. = alpha*A*x + beta*y,   or   y := alpha*A'*x + beta*y, is an = alpha*A*x + beta*y. = alpha*A'*x + beta*y. = alpha*A'*x + beta*y. Unchanged on exit. REAL             array of DIMENSION at least is overwritten by the updated vector y. is overwritten by the updated vector y. must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 13. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `TRANS`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `KL`: not a workspace argument
- `KU`: not a workspace argument
- `ALPHA`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument
- `BETA`: not a workspace argument
- `Y`: not a workspace argument
- `INCY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::sgbmv`
- Original SLATEC routine: `SGBMV`
- Native symbol: `sgbmv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SGBMV](https://www.netlib.org/slatec/lin/sgbmv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
