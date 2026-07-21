# Purpose

CTBMV performs one of the matrix-vector operations

# Description

This canonical unsafe binding exposes original SLATEC routine `CTBMV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTBMV](https://www.netlib.org/slatec/lin/ctbmv.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, UPLO specifies whether the matrix is an upper or lower triangular matrix as follows: 'U' or 'u'   A is an upper triangular matrix. 'L' or 'l'   A is a lower triangular matrix. Unchanged on exit. 'U' or 'u', K specifies the number of super-diagonals of the matrix A. 'L' or 'l', K specifies the number of sub-diagonals of the matrix A. 'U' or 'u', the leading ( k + 1 ) 'L' or 'l', the leading ( k + 1 ) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `TRANS`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: = A*x. = A'*x. = conjg( A' )*x. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `DIAG`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. is unit triangular as follows: 'U' or 'u'   A is assumed to be unit triangular. 'N' or 'n'   A is not assumed to be unit triangular. Unchanged on exit. 'U' or 'u' the elements of the array A corresponding to the diagonal elements of the matrix are not referenced, but are assumed to be unity. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. unit, unit, unit, upper or lower triangular band matrix, with ( k + 1 ) diagonals. upper or lower triangular band matrix, with ( k + 1 ) diagonals. upper or lower triangular band matrix, with ( k + 1 ) diagonals. Parameters Parameters Parameters ========== ========== ========== = A*x. = A*x. INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. contain the upper triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row contain the lower triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer a lower triangular band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n unit, unit, unit, upper or lower triangular band matrix, with ( k + 1 ) diagonals. upper or lower triangular band matrix, with ( k + 1 ) diagonals. upper or lower triangular band matrix, with ( k + 1 ) diagonals. Parameters Parameters Parameters ========== ========== ========== = A*x. = A*x. INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. contain the upper triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row contain the lower triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer a lower triangular band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n not applicable or not stated by selected source not a workspace argument

## 5. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. must satisfy  0 .le. K. Unchanged on exit. diagonal starting at position 2 in row k, and so on. The top left k by k triangle of the array A is not referenced. The following program segment will transfer an upper triangular band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N J DO 10, I = MAX( 1, J - K ), J INTEGER. must satisfy  0 .le. K. Unchanged on exit. diagonal starting at position 2 in row k, and so on. The top left k by k triangle of the array A is not referenced. The following program segment will transfer an upper triangular band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N J DO 10, I = MAX( 1, J - K ), J not applicable or not stated by selected source not a workspace argument

## 6. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). unit, upper or lower triangular band matrix, with ( k + 1 ) diagonals. Parameters ========== is unit triangular as follows: COMPLEX          array of DIMENSION ( LDA, n ). contain the upper triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row matrix( I, J ) 10    CONTINUE 20 CONTINUE contain the lower triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer a lower triangular band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) matrix( I, J ) 10    CONTINUE 20 CONTINUE unit, upper or lower triangular band matrix, with ( k + 1 ) diagonals. Parameters ========== is unit triangular as follows: COMPLEX          array of DIMENSION ( LDA, n ). contain the upper triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row matrix( I, J ) 10    CONTINUE 20 CONTINUE contain the lower triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer a lower triangular band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) matrix( I, J ) 10    CONTINUE 20 CONTINUE not applicable or not stated by selected source not a workspace argument

## 7. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( k + 1 ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( k + 1 ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( k + 1 ). Unchanged on exit. not a workspace argument

## 8. `X`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). = A*x,   or   x := A'*x,   or   x := conjg( A')*x, unit, upper or lower triangular band matrix, with ( k + 1 ) diagonals. Parameters ========== = A*x. = A'*x. = conjg( A' )*x. Unchanged on exit. COMPLEX          array of dimension at least is overwritten with the is overwritten with the transformed vector x. transformed vector x. must not be zero. Unchanged on exit. = A*x,   or   x := A'*x,   or   x := conjg( A')*x, unit, upper or lower triangular band matrix, with ( k + 1 ) diagonals. Parameters ========== = A*x. = A'*x. = conjg( A' )*x. Unchanged on exit. COMPLEX          array of dimension at least is overwritten with the is overwritten with the transformed vector x. transformed vector x. must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 9. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `UPLO`: not a workspace argument
- `TRANS`: not a workspace argument
- `DIAG`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::ctbmv`
- Original SLATEC routine: `CTBMV`
- Native symbol: `ctbmv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CTBMV](https://www.netlib.org/slatec/lin/ctbmv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
