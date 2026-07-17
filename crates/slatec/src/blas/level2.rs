//! Slice-based safe wrappers for selected real BLAS Level 2 routines.
//!
//! Matrices are column-major backing slices. An element at `row`, `col` has
//! index `row + col * lda`; `lda` may exceed the logical row count.

use core::ffi::c_char;

use slatec_sys::families::blas_level2 as raw;

use super::validation::{
    count, gemv_logical_lengths, increment, input_pointer, output_pointer, validate_matrix,
    validate_vector,
};
use super::{BlasError, Diagonal, Transpose, Triangle};

macro_rules! impl_real_level2 {
    (
        $scalar:ty,
        $gemv:ident,
        $gemv_contiguous:ident,
        $ger:ident,
        $symv:ident,
        $trmv:ident,
        $trsv:ident
    ) => {
        #[doc = concat!(
            "Computes `y = alpha * op(A) * x + beta * y` with the original SLATEC routine `",
            stringify!($gemv),
            "`.\n\nMatrices use column-major storage; `rows` and `cols` describe stored `A`, `lda` is its leading dimension, and `incx`/`incy` describe logical vector strides. The wrapper checks all counts, strides, and backing slices and returns [`BlasError`] without calling Fortran on invalid input.\n\nSee `examples/blas/level2.rs` for a complete, numerically checked example."
        )]
        #[allow(clippy::too_many_arguments)]
        pub fn $gemv(
            transpose: Transpose,
            rows: usize,
            cols: usize,
            alpha: $scalar,
            a: &[$scalar],
            lda: usize,
            x: &[$scalar],
            incx: isize,
            beta: $scalar,
            y: &mut [$scalar],
            incy: isize,
        ) -> Result<(), BlasError> {
            let (x_len, y_len) = gemv_logical_lengths(transpose, rows, cols);
            let mut transpose = transpose.real_character(stringify!($gemv))?;
            validate_matrix("a", rows, cols, lda, a.len())?;
            validate_vector(x_len, incx, x.len(), "x")?;
            validate_vector(y_len, incy, y.len(), "y")?;
            let mut rows_fortran = count(rows, "rows")?;
            let mut cols_fortran = count(cols, "cols")?;
            let mut lda_fortran = count(lda, "lda")?;
            let mut incx_fortran = increment(incx, "incx")?;
            let mut incy_fortran = increment(incy, "incy")?;
            let mut alpha = alpha;
            let mut beta = beta;
            let a = input_pointer(a, a.len(), 1, "a")?;
            let x = input_pointer(x, x_len, incx, "x")?;
            let y = output_pointer(y, y_len, incy, "y")?;
            if rows == 0 || cols == 0 {
                return Ok(());
            }
            // Safety: all dimensions, leading dimension, and vector spans
            // were checked; `y` is uniquely borrowed; selector and trailing
            // length use the validated GNU MinGW character ABI.
            unsafe {
                raw::$gemv(
                    &mut transpose,
                    &mut rows_fortran,
                    &mut cols_fortran,
                    &mut alpha,
                    a,
                    &mut lda_fortran,
                    x,
                    &mut incx_fortran,
                    &mut beta,
                    y,
                    &mut incy_fortran,
                    1,
                )
            };
            Ok(())
        }

        #[doc = concat!(
            "Tightly packed, unit-stride convenience wrapper for [`",
            stringify!($gemv),
            "`]. It infers only `lda = max(1, rows)` and unit vector increments. Original SLATEC routine: `",
            stringify!($gemv),
            "`. See `examples/blas/level2.rs` for a checked matrix-vector example."
        )]
        #[allow(clippy::too_many_arguments)]
        pub fn $gemv_contiguous(
            transpose: Transpose,
            rows: usize,
            cols: usize,
            alpha: $scalar,
            a: &[$scalar],
            x: &[$scalar],
            beta: $scalar,
            y: &mut [$scalar],
        ) -> Result<(), BlasError> {
            $gemv(
                transpose,
                rows,
                cols,
                alpha,
                a,
                rows.max(1),
                x,
                1,
                beta,
                y,
                1,
            )
        }

        #[doc = concat!(
            "Performs the rank-one update `A = alpha * x * y^T + A` with original SLATEC routine `",
            stringify!($ger),
            "`. `A` is an `m` by `n` column-major matrix with leading dimension `lda`; the vectors use `incx` and `incy`. Storage and integer conversions are checked before FFI. See `examples/blas/level2.rs`."
        )]
        #[allow(clippy::too_many_arguments)]
        pub fn $ger(
            m: usize,
            n: usize,
            alpha: $scalar,
            x: &[$scalar],
            incx: isize,
            y: &[$scalar],
            incy: isize,
            a: &mut [$scalar],
            lda: usize,
        ) -> Result<(), BlasError> {
            validate_matrix("a", m, n, lda, a.len())?;
            validate_vector(m, incx, x.len(), "x")?;
            validate_vector(n, incy, y.len(), "y")?;
            let mut m_fortran = count(m, "m")?;
            let mut n_fortran = count(n, "n")?;
            let mut lda_fortran = count(lda, "lda")?;
            let mut incx_fortran = increment(incx, "incx")?;
            let mut incy_fortran = increment(incy, "incy")?;
            let mut alpha = alpha;
            let x = input_pointer(x, m, incx, "x")?;
            let y = input_pointer(y, n, incy, "y")?;
            let a = output_pointer(a, a.len(), 1, "a")?;
            if m == 0 || n == 0 {
                return Ok(());
            }
            // Safety: matrix and vectors have checked BLAS spans, `a` is a
            // unique mutable borrow, and all integer arguments fit the ABI.
            unsafe {
                raw::$ger(
                    &mut m_fortran,
                    &mut n_fortran,
                    &mut alpha,
                    x,
                    &mut incx_fortran,
                    y,
                    &mut incy_fortran,
                    a,
                    &mut lda_fortran,
                )
            };
            Ok(())
        }

        #[doc = concat!(
            "Computes `y = alpha * A * x + beta * y` for a symmetric matrix using original SLATEC routine `",
            stringify!($symv),
            "`. `triangle` selects the stored half of the `n` by `n` column-major matrix; `lda`, vector increments, and slice lengths are validated. See `examples/blas/level2.rs`."
        )]
        #[allow(clippy::too_many_arguments)]
        pub fn $symv(
            triangle: Triangle,
            n: usize,
            alpha: $scalar,
            a: &[$scalar],
            lda: usize,
            x: &[$scalar],
            incx: isize,
            beta: $scalar,
            y: &mut [$scalar],
            incy: isize,
        ) -> Result<(), BlasError> {
            let mut triangle = triangle.character();
            validate_matrix("a", n, n, lda, a.len())?;
            validate_vector(n, incx, x.len(), "x")?;
            validate_vector(n, incy, y.len(), "y")?;
            let mut n_fortran = count(n, "n")?;
            let mut lda_fortran = count(lda, "lda")?;
            let mut incx_fortran = increment(incx, "incx")?;
            let mut incy_fortran = increment(incy, "incy")?;
            let mut alpha = alpha;
            let mut beta = beta;
            let a = input_pointer(a, a.len(), 1, "a")?;
            let x = input_pointer(x, n, incx, "x")?;
            let y = output_pointer(y, n, incy, "y")?;
            if n == 0 {
                return Ok(());
            }
            // Safety: the selected triangle, matrix storage, vector spans,
            // character length, and unique output vector were checked.
            unsafe {
                raw::$symv(
                    &mut triangle,
                    &mut n_fortran,
                    &mut alpha,
                    a,
                    &mut lda_fortran,
                    x,
                    &mut incx_fortran,
                    &mut beta,
                    y,
                    &mut incy_fortran,
                    1,
                )
            };
            Ok(())
        }

        #[doc = concat!(
            "Replaces `x` with `op(A) * x` for a triangular matrix using original SLATEC routine `",
            stringify!($trmv),
            "`. The selectors define the stored triangle, transpose, and diagonal; `A` is column-major and `x` is a checked strided mutable slice. See `examples/blas/level2.rs`."
        )]
        #[allow(clippy::too_many_arguments)]
        pub fn $trmv(
            triangle: Triangle,
            transpose: Transpose,
            diagonal: Diagonal,
            n: usize,
            a: &[$scalar],
            lda: usize,
            x: &mut [$scalar],
            incx: isize,
        ) -> Result<(), BlasError> {
            triangular_vector(
                stringify!($trmv),
                triangle,
                transpose,
                diagonal,
                n,
                a,
                lda,
                x,
                incx,
                raw::$trmv,
            )
        }

        #[doc = concat!(
            "Solves `op(A) * x = b` in place for triangular `A` using original SLATEC routine `",
            stringify!($trsv),
            "`. On success `x` contains the solution; selectors, column-major storage, stride, and integer bounds are checked. Singular diagonals are a numerical precondition of the original BLAS routine. See `examples/blas/level2.rs`."
        )]
        #[allow(clippy::too_many_arguments)]
        pub fn $trsv(
            triangle: Triangle,
            transpose: Transpose,
            diagonal: Diagonal,
            n: usize,
            a: &[$scalar],
            lda: usize,
            x: &mut [$scalar],
            incx: isize,
        ) -> Result<(), BlasError> {
            triangular_vector(
                stringify!($trsv),
                triangle,
                transpose,
                diagonal,
                n,
                a,
                lda,
                x,
                incx,
                raw::$trsv,
            )
        }
    };
}

type TriangularVectorFn<T> = unsafe extern "C" fn(
    *mut c_char,
    *mut c_char,
    *mut c_char,
    *mut slatec_sys::FortranInteger,
    *mut T,
    *mut slatec_sys::FortranInteger,
    *mut T,
    *mut slatec_sys::FortranInteger,
    slatec_sys::FortranCharacterLength,
    slatec_sys::FortranCharacterLength,
    slatec_sys::FortranCharacterLength,
);

#[allow(clippy::too_many_arguments)]
fn triangular_vector<T>(
    operation: &'static str,
    triangle: Triangle,
    transpose: Transpose,
    diagonal: Diagonal,
    n: usize,
    a: &[T],
    lda: usize,
    x: &mut [T],
    incx: isize,
    routine: TriangularVectorFn<T>,
) -> Result<(), BlasError> {
    let mut triangle = triangle.character();
    let mut transpose = transpose.real_character(operation)?;
    let mut diagonal = diagonal.character();
    validate_matrix("a", n, n, lda, a.len())?;
    validate_vector(n, incx, x.len(), "x")?;
    let mut n_fortran = count(n, "n")?;
    let mut lda_fortran = count(lda, "lda")?;
    let mut incx_fortran = increment(incx, "incx")?;
    let a = input_pointer(a, a.len(), 1, "a")?;
    let x = output_pointer(x, n, incx, "x")?;
    if n == 0 {
        return Ok(());
    }
    // Safety: selected triangle/transpose/diagonal characters and their
    // trailing lengths match the profile; matrix and in-place vector spans
    // were checked and `x` is uniquely borrowed.
    unsafe {
        routine(
            &mut triangle,
            &mut transpose,
            &mut diagonal,
            &mut n_fortran,
            a,
            &mut lda_fortran,
            x,
            &mut incx_fortran,
            1,
            1,
            1,
        )
    };
    Ok(())
}

impl_real_level2!(f32, sgemv, sgemv_contiguous, sger, ssymv, strmv, strsv);
impl_real_level2!(f64, dgemv, dgemv_contiguous, dger, dsymv, dtrmv, dtrsv);
