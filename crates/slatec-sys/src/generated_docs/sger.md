# Purpose

SGER performs the rank 1 operation

# Description

This canonical unsafe binding exposes original SLATEC routine `SGER`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGER](https://www.netlib.org/slatec/lin/sger.f).

# Arguments

## 1. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, M specifies the number of rows of the matrix A. must be at least zero. Unchanged on exit. 1)*abs( INCX)). Before entry, the incremented array X must contain the m element vector x. Unchanged on exit. INTEGER. On entry, M specifies the number of rows of the matrix A. must be at least zero. Unchanged on exit. 1)*abs( INCX)). Before entry, the incremented array X must contain the m element vector x. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, N specifies the number of columns of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the n element vector y. Unchanged on exit. INTEGER. On entry, N specifies the number of columns of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the n element vector y. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 3. `ALPHA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL            . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL             array of dimension at least must not be zero. Unchanged on exit. REAL             array of dimension at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `Y`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL             array of dimension at least must not be zero. Unchanged on exit. REAL             array of dimension at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 7. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 8. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). = alpha*x*y' + A, where alpha is a scalar, x is an m element vector, y is an n element vector and A is an m by n matrix. Parameters ========== REAL             array of DIMENSION ( LDA, n ). Before entry, the leading m by n part of the array A must contain the matrix of coefficients. On exit, A is overwritten by the updated matrix. = alpha*x*y' + A, where alpha is a scalar, x is an m element vector, y is an n element vector and A is an m by n matrix. Parameters ========== REAL             array of DIMENSION ( LDA, n ). Before entry, the leading m by n part of the array A must contain the matrix of coefficients. On exit, A is overwritten by the updated matrix. not applicable or not stated by selected source not a workspace argument

## 9. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, m ). Unchanged on exit. not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `M`: not a workspace argument
- `N`: not a workspace argument
- `ALPHA`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument
- `Y`: not a workspace argument
- `INCY`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::sger`
- Original SLATEC routine: `SGER`
- Native symbol: `sger_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SGER](https://www.netlib.org/slatec/lin/sger.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
