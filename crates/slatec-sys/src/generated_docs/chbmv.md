# Purpose

CHBMV performs the matrix-vector operation

# Description

This canonical unsafe binding exposes original SLATEC routine `CHBMV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHBMV](https://www.netlib.org/slatec/lin/chbmv.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the band matrix A is being supplied as follows: 'U' or 'u'   The upper triangular part of A is being supplied. 'L' or 'l'   The lower triangular part of A is being supplied. Unchanged on exit. 'U' or 'u', the leading ( k + 1 ) 'L' or 'l', the leading ( k + 1 ) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. diagonals. diagonals. Parameters Parameters ========== ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. contain the upper triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row contain the lower triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer the lower triangular part of a hermitian band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the vector x. Unchanged on exit. 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the diagonals. diagonals. Parameters Parameters ========== ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. contain the upper triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row contain the lower triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer the lower triangular part of a hermitian band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the vector x. Unchanged on exit. 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the not applicable or not stated by selected source not a workspace argument

## 3. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. diagonals. Parameters ========== INTEGER. diagonals of the must satisfy  0 .le. K. Unchanged on exit. diagonal starting at position 2 in row k, and so on. The top left k by k triangle of the array A is not referenced. The following program segment will transfer the upper triangular part of a hermitian band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N J DO 10, I = MAX( 1, J - K ), J diagonals. Parameters ========== INTEGER. diagonals of the must satisfy  0 .le. K. Unchanged on exit. diagonal starting at position 2 in row k, and so on. The top left k by k triangle of the array A is not referenced. The following program segment will transfer the upper triangular part of a hermitian band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N J DO 10, I = MAX( 1, J - K ), J not applicable or not stated by selected source not a workspace argument

## 4. `ALPHA`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. are scalars, x and y are n element vectors and COMPLEX         . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). diagonals. Parameters ========== must satisfy  0 .le. K. Unchanged on exit. COMPLEX          array of DIMENSION ( LDA, n ). contain the upper triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row matrix( I, J ) 10    CONTINUE 20 CONTINUE contain the lower triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer the lower triangular part of a hermitian band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) matrix( I, J ) 10    CONTINUE 20 CONTINUE Note that the imaginary parts of the diagonal elements need not be set and are assumed to be zero. Unchanged on exit. diagonals. Parameters ========== must satisfy  0 .le. K. Unchanged on exit. COMPLEX          array of DIMENSION ( LDA, n ). contain the upper triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row matrix( I, J ) 10    CONTINUE 20 CONTINUE contain the lower triangular band part of the hermitian matrix, supplied column by column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the array A is not referenced. The following program segment will transfer the lower triangular part of a hermitian band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = 1 - J DO 10, I = J, MIN( N, J + K ) matrix( I, J ) 10    CONTINUE 20 CONTINUE Note that the imaginary parts of the diagonal elements need not be set and are assumed to be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( k + 1 ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( k + 1 ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( k + 1 ). Unchanged on exit. not a workspace argument

## 7. `X`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX          array of DIMENSION at least must not be zero. Unchanged on exit. COMPLEX          array of DIMENSION at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 8. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 9. `BETA`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. are scalars, x and y are n element vectors and COMPLEX         . On entry, BETA specifies the scalar beta. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `Y`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). = alpha*A*x + beta*y, COMPLEX          array of DIMENSION at least is overwritten by the updated vector y. is overwritten by the updated vector y. must not be zero. Unchanged on exit. = alpha*A*x + beta*y, COMPLEX          array of DIMENSION at least is overwritten by the updated vector y. is overwritten by the updated vector y. must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 11. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `UPLO`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `ALPHA`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument
- `BETA`: not a workspace argument
- `Y`: not a workspace argument
- `INCY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::chbmv`
- Original SLATEC routine: `CHBMV`
- Native symbol: `chbmv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CHBMV](https://www.netlib.org/slatec/lin/chbmv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
