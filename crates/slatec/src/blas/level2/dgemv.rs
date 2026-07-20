use slatec_sys::families::blas_level2 as raw;

use super::super::validation::{
    count, gemv_logical_lengths, increment, input_pointer, output_pointer, validate_matrix,
    validate_vector,
};
use super::super::{BlasError, Transpose};

/// Computes `y = alpha * op(A) * x + beta * y` with original SLATEC routine
/// `DGEMV`. Matrices are column-major and all dimensions, strides, and backing
/// slices are checked before the native call.
#[allow(clippy::too_many_arguments)]
pub fn dgemv(
    transpose: Transpose,
    rows: usize,
    cols: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    x: &[f64],
    incx: isize,
    beta: f64,
    y: &mut [f64],
    incy: isize,
) -> Result<(), BlasError> {
    let (x_len, y_len) = gemv_logical_lengths(transpose, rows, cols);
    let mut transpose = transpose.real_character("dgemv")?;
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
    // Safety: dimensions, spans, selector, and the uniquely borrowed output
    // are checked; the trailing character length is the supported ABI value.
    unsafe {
        raw::dgemv(
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

/// Tightly packed, unit-stride convenience wrapper for [`dgemv`].
#[allow(clippy::too_many_arguments)]
pub fn dgemv_contiguous(
    transpose: Transpose,
    rows: usize,
    cols: usize,
    alpha: f64,
    a: &[f64],
    x: &[f64],
    beta: f64,
    y: &mut [f64],
) -> Result<(), BlasError> {
    dgemv(
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
