#![cfg(all(
    feature = "blas-raw-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Direct canonical raw BLAS smoke tests.  These deliberately use neither the
//! safe BLAS wrapper nor another SLATEC routine as an oracle.

use core::ffi::c_char;
use slatec_sys::{Complex32, FortranInteger};

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected}, got {actual}"
    );
}

fn retain_selected_provider() {
    slatec_src::ensure_linked();
}

#[test]
fn level1_axpy_dot_and_rotation_generator_use_canonical_paths() {
    retain_selected_provider();
    let mut n: FortranInteger = 3;
    let mut inc: FortranInteger = 1;
    let mut alpha = 2.0_f64;
    let mut x = [1.0_f64, -2.0, 3.0];
    let mut y = [4.0_f64, 5.0, -6.0];
    unsafe {
        slatec_sys::blas::level1::daxpy(
            &mut n,
            &mut alpha,
            x.as_mut_ptr(),
            &mut inc,
            y.as_mut_ptr(),
            &mut inc,
        );
    }
    assert_eq!(y, [6.0, 1.0, 0.0]);
    let dot = unsafe {
        slatec_sys::blas::level1::ddot(&mut n, x.as_mut_ptr(), &mut inc, y.as_mut_ptr(), &mut inc)
    };
    close(dot, 4.0);

    let mut a = 3.0_f64;
    let mut b = 4.0_f64;
    let mut c = 0.0_f64;
    let mut s = 0.0_f64;
    unsafe { slatec_sys::blas::level1::drotg(&mut a, &mut b, &mut c, &mut s) };
    close(c * c + s * s, 1.0);
}

#[test]
fn level2_gemv_and_triangular_solve_expose_character_lengths() {
    retain_selected_provider();
    let mut trans = b'N' as c_char;
    let mut m: FortranInteger = 2;
    let mut n: FortranInteger = 3;
    let mut alpha = 1.0_f64;
    let mut beta = 0.0_f64;
    let mut lda: FortranInteger = 2;
    let mut inc: FortranInteger = 1;
    let mut matrix = [1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut x = [1.0_f64, 2.0, 3.0];
    let mut y = [0.0_f64, 0.0];
    unsafe {
        slatec_sys::blas::level2::dgemv(
            &mut trans,
            &mut m,
            &mut n,
            &mut alpha,
            matrix.as_mut_ptr(),
            &mut lda,
            x.as_mut_ptr(),
            &mut inc,
            &mut beta,
            y.as_mut_ptr(),
            &mut inc,
            1,
        );
    }
    assert_eq!(y, [22.0, 28.0]);

    let mut uplo = b'U' as c_char;
    let mut diag = b'N' as c_char;
    let mut order: FortranInteger = 2;
    let mut triangular = [2.0_f64, 0.0, 1.0, 3.0];
    let mut rhs = [4.0_f64, 6.0];
    unsafe {
        slatec_sys::blas::level2::dtrsv(
            &mut uplo,
            &mut trans,
            &mut diag,
            &mut order,
            triangular.as_mut_ptr(),
            &mut lda,
            rhs.as_mut_ptr(),
            &mut inc,
            1,
            1,
            1,
        );
    }
    assert_eq!(rhs, [1.0, 2.0]);
}

#[test]
fn level3_real_and_complex_matrix_products_use_column_major_storage() {
    retain_selected_provider();
    let mut trans = b'N' as c_char;
    let mut m: FortranInteger = 2;
    let mut n: FortranInteger = 2;
    let mut k: FortranInteger = 2;
    let mut alpha = 1.0_f64;
    let mut beta = 0.0_f64;
    let mut ld: FortranInteger = 2;
    let mut a = [1.0_f64, 2.0, 3.0, 4.0];
    let mut b = [1.0_f64, 0.0, 0.0, 1.0];
    let mut c = [0.0_f64; 4];
    unsafe {
        slatec_sys::blas::level3::dgemm(
            &mut trans,
            &mut trans,
            &mut m,
            &mut n,
            &mut k,
            &mut alpha,
            a.as_mut_ptr(),
            &mut ld,
            b.as_mut_ptr(),
            &mut ld,
            &mut beta,
            c.as_mut_ptr(),
            &mut ld,
            1,
            1,
        );
    }
    assert_eq!(c, a);

    let mut complex_alpha = Complex32 { re: 1.0, im: 0.0 };
    let mut complex_beta = Complex32 { re: 0.0, im: 0.0 };
    let mut complex_a = [
        Complex32 { re: 1.0, im: 1.0 },
        Complex32 { re: 2.0, im: -1.0 },
        Complex32 { re: 0.0, im: 1.0 },
        Complex32 { re: 3.0, im: 0.0 },
    ];
    let mut complex_identity = [
        Complex32 { re: 1.0, im: 0.0 },
        Complex32 { re: 0.0, im: 0.0 },
        Complex32 { re: 0.0, im: 0.0 },
        Complex32 { re: 1.0, im: 0.0 },
    ];
    let mut complex_c = [Complex32::default(); 4];
    unsafe {
        slatec_sys::blas::level3::cgemm(
            &mut trans,
            &mut trans,
            &mut m,
            &mut n,
            &mut k,
            &mut complex_alpha,
            complex_a.as_mut_ptr(),
            &mut ld,
            complex_identity.as_mut_ptr(),
            &mut ld,
            &mut complex_beta,
            complex_c.as_mut_ptr(),
            &mut ld,
            1,
            1,
        );
    }
    assert_eq!(complex_c, complex_a);
}
