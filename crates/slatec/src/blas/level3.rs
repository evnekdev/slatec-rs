//! Slice-based safe wrappers for selected real BLAS Level 3 routines.
//!
//! Every matrix is a column-major slice with explicit leading dimension.

use slatec_sys::generated::character;

use super::validation::{
    count, gemm_stored_shapes, input_pointer, output_pointer, triangular_order, validate_matrix,
};
use super::{BlasError, Diagonal, Side, Transpose, Triangle};

macro_rules! impl_real_level3 {
    (
        $scalar:ty,
        $gemm:ident,
        $gemm_contiguous:ident,
        $trmm:ident,
        $trsm:ident,
        $syrk:ident
    ) => {
        #[allow(clippy::too_many_arguments)]
        pub fn $gemm(
            trans_a: Transpose,
            trans_b: Transpose,
            m: usize,
            n: usize,
            k: usize,
            alpha: $scalar,
            a: &[$scalar],
            lda: usize,
            b: &[$scalar],
            ldb: usize,
            beta: $scalar,
            c: &mut [$scalar],
            ldc: usize,
        ) -> Result<(), BlasError> {
            let ((a_rows, a_cols), (b_rows, b_cols)) =
                gemm_stored_shapes(trans_a, trans_b, m, n, k);
            let mut trans_a = trans_a.real_character(stringify!($gemm))?;
            let mut trans_b = trans_b.real_character(stringify!($gemm))?;
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
            // Safety: stored shapes, leading dimensions, and all backing
            // ranges are checked; `c` is uniquely borrowed; selectors and
            // trailing lengths follow the validated GNU MinGW ABI profile.
            unsafe {
                character::$gemm(
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

        #[allow(clippy::too_many_arguments)]
        pub fn $gemm_contiguous(
            trans_a: Transpose,
            trans_b: Transpose,
            m: usize,
            n: usize,
            k: usize,
            alpha: $scalar,
            a: &[$scalar],
            b: &[$scalar],
            beta: $scalar,
            c: &mut [$scalar],
        ) -> Result<(), BlasError> {
            let a_rows = if trans_a.is_transposed() { k } else { m };
            let b_rows = if trans_b.is_transposed() { n } else { k };
            $gemm(
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

        #[allow(clippy::too_many_arguments)]
        pub fn $trmm(
            side: Side,
            triangle: Triangle,
            transpose: Transpose,
            diagonal: Diagonal,
            m: usize,
            n: usize,
            alpha: $scalar,
            a: &[$scalar],
            lda: usize,
            b: &mut [$scalar],
            ldb: usize,
        ) -> Result<(), BlasError> {
            triangular_matrix(
                stringify!($trmm),
                side,
                triangle,
                transpose,
                diagonal,
                m,
                n,
                alpha,
                a,
                lda,
                b,
                ldb,
                character::$trmm,
            )
        }

        #[allow(clippy::too_many_arguments)]
        pub fn $trsm(
            side: Side,
            triangle: Triangle,
            transpose: Transpose,
            diagonal: Diagonal,
            m: usize,
            n: usize,
            alpha: $scalar,
            a: &[$scalar],
            lda: usize,
            b: &mut [$scalar],
            ldb: usize,
        ) -> Result<(), BlasError> {
            triangular_matrix(
                stringify!($trsm),
                side,
                triangle,
                transpose,
                diagonal,
                m,
                n,
                alpha,
                a,
                lda,
                b,
                ldb,
                character::$trsm,
            )
        }

        #[allow(clippy::too_many_arguments)]
        pub fn $syrk(
            triangle: Triangle,
            transpose: Transpose,
            n: usize,
            k: usize,
            alpha: $scalar,
            a: &[$scalar],
            lda: usize,
            beta: $scalar,
            c: &mut [$scalar],
            ldc: usize,
        ) -> Result<(), BlasError> {
            let (a_rows, a_cols) = if transpose.is_transposed() {
                (k, n)
            } else {
                (n, k)
            };
            let mut triangle = triangle.character();
            let mut transpose = transpose.real_character(stringify!($syrk))?;
            validate_matrix("a", a_rows, a_cols, lda, a.len())?;
            validate_matrix("c", n, n, ldc, c.len())?;
            let mut n_fortran = count(n, "n")?;
            let mut k_fortran = count(k, "k")?;
            let mut lda_fortran = count(lda, "lda")?;
            let mut ldc_fortran = count(ldc, "ldc")?;
            let mut alpha = alpha;
            let mut beta = beta;
            let a = input_pointer(a, a.len(), 1, "a")?;
            let c = output_pointer(c, c.len(), 1, "c")?;
            if n == 0 {
                return Ok(());
            }
            // Safety: both stored matrix layouts and the output triangle are
            // checked; `c` is uniquely borrowed; selector lengths are one.
            unsafe {
                character::$syrk(
                    &mut triangle,
                    &mut transpose,
                    &mut n_fortran,
                    &mut k_fortran,
                    &mut alpha,
                    a,
                    &mut lda_fortran,
                    &mut beta,
                    c,
                    &mut ldc_fortran,
                    1,
                    1,
                )
            };
            Ok(())
        }
    };
}

type TriangularMatrixFn<T> = unsafe extern "C" fn(
    *mut core::ffi::c_char,
    *mut core::ffi::c_char,
    *mut core::ffi::c_char,
    *mut core::ffi::c_char,
    *mut slatec_sys::FortranInteger,
    *mut slatec_sys::FortranInteger,
    *mut T,
    *mut T,
    *mut slatec_sys::FortranInteger,
    *mut T,
    *mut slatec_sys::FortranInteger,
    slatec_sys::FortranCharacterLength,
    slatec_sys::FortranCharacterLength,
    slatec_sys::FortranCharacterLength,
    slatec_sys::FortranCharacterLength,
);

#[allow(clippy::too_many_arguments)]
fn triangular_matrix<T>(
    operation: &'static str,
    side: Side,
    triangle: Triangle,
    transpose: Transpose,
    diagonal: Diagonal,
    m: usize,
    n: usize,
    alpha: T,
    a: &[T],
    lda: usize,
    b: &mut [T],
    ldb: usize,
    routine: TriangularMatrixFn<T>,
) -> Result<(), BlasError> {
    let order = triangular_order(side, m, n);
    let mut side = side.character();
    let mut triangle = triangle.character();
    let mut transpose = transpose.real_character(operation)?;
    let mut diagonal = diagonal.character();
    validate_matrix("a", order, order, lda, a.len())?;
    validate_matrix("b", m, n, ldb, b.len())?;
    let mut m_fortran = count(m, "m")?;
    let mut n_fortran = count(n, "n")?;
    let mut lda_fortran = count(lda, "lda")?;
    let mut ldb_fortran = count(ldb, "ldb")?;
    let mut alpha = alpha;
    let a = input_pointer(a, a.len(), 1, "a")?;
    let b = output_pointer(b, b.len(), 1, "b")?;
    if m == 0 || n == 0 {
        return Ok(());
    }
    // Safety: side determines the checked square A order, B is a unique
    // mutable column-major matrix, and all four selector lengths are one.
    unsafe {
        routine(
            &mut side,
            &mut triangle,
            &mut transpose,
            &mut diagonal,
            &mut m_fortran,
            &mut n_fortran,
            &mut alpha,
            a,
            &mut lda_fortran,
            b,
            &mut ldb_fortran,
            1,
            1,
            1,
            1,
        )
    };
    Ok(())
}

impl_real_level3!(f32, sgemm, sgemm_contiguous, strmm, strsm, ssyrk);
impl_real_level3!(f64, dgemm, dgemm_contiguous, dtrmm, dtrsm, dsyrk);
