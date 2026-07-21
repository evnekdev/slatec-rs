use slatec_sys::blas::level3 as raw;

use super::super::validation::{
    count, gemm_stored_shapes, input_pointer, output_pointer, validate_matrix,
};
use super::super::{BlasError, Transpose};

/// Computes `C = alpha * op(A) * op(B) + beta * C` with original SLATEC
/// routine `DGEMM`; stored shapes and column-major leading dimensions are
/// validated before the native call.
#[allow(clippy::too_many_arguments)]
pub fn dgemm(
    trans_a: Transpose,
    trans_b: Transpose,
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
) -> Result<(), BlasError> {
    let ((a_rows, a_cols), (b_rows, b_cols)) = gemm_stored_shapes(trans_a, trans_b, m, n, k);
    let mut trans_a = trans_a.real_character("dgemm")?;
    let mut trans_b = trans_b.real_character("dgemm")?;
    validate_matrix("a", a_rows, a_cols, lda, a.len())?;
    validate_matrix("b", b_rows, b_cols, ldb, b.len())?;
    validate_matrix("c", m, n, ldc, c.len())?;
    let mut m_fortran = count(m, "m")?;
    let mut n_fortran = count(n, "n")?;
    let mut k_fortran = count(k, "k")?;
    let mut lda_fortran = count(lda, "lda")?;
    let mut ldb_fortran = count(ldb, "ldb")?;
    let mut ldc_fortran = count(ldc, "ldc")?;
    let mut alpha = alpha;
    let mut beta = beta;
    let a = input_pointer(a, a.len(), 1, "a")?;
    let b = input_pointer(b, b.len(), 1, "b")?;
    let c = output_pointer(c, c.len(), 1, "c")?;
    if m == 0 || n == 0 {
        return Ok(());
    }
    // Safety: all stored shapes, leading dimensions, ranges, selectors, and
    // the mutable output matrix were checked for the supported raw ABI.
    unsafe {
        raw::dgemm(
            &mut trans_a,
            &mut trans_b,
            &mut m_fortran,
            &mut n_fortran,
            &mut k_fortran,
            &mut alpha,
            a,
            &mut lda_fortran,
            b,
            &mut ldb_fortran,
            &mut beta,
            c,
            &mut ldc_fortran,
            1,
            1,
        )
    };
    Ok(())
}

/// Tightly packed column-major convenience wrapper for [`dgemm`].
#[allow(clippy::too_many_arguments)]
pub fn dgemm_contiguous(
    trans_a: Transpose,
    trans_b: Transpose,
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    b: &[f64],
    beta: f64,
    c: &mut [f64],
) -> Result<(), BlasError> {
    let a_rows = if trans_a.is_transposed() { k } else { m };
    let b_rows = if trans_b.is_transposed() { n } else { k };
    dgemm(
        trans_a,
        trans_b,
        m,
        n,
        k,
        alpha,
        a,
        a_rows.max(1),
        b,
        b_rows.max(1),
        beta,
        c,
        m.max(1),
    )
}
