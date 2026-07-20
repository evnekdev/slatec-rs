//! Representative native smoke coverage for Batch B callback-bearing raw ABIs.
//!
//! These tests prove that selected promoted declarations can cross the raw FFI
//! boundary, invoke Rust-supplied callback symbols, and write observable native
//! outputs. They do not numerically validate every Batch B routine.

#![cfg(all(
    feature = "raw-batch-b-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

use core::sync::atomic::{AtomicUsize, Ordering};
use slatec_sys::FortranInteger;

static QK15_F32_CALLS: AtomicUsize = AtomicUsize::new(0);
static QK15_F64_CALLS: AtomicUsize = AtomicUsize::new(0);
static SCG_MATVEC_CALLS: AtomicUsize = AtomicUsize::new(0);
static SCG_MSOLVE_CALLS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn square_f32(x: *const f32) -> f32 {
    QK15_F32_CALLS.fetch_add(1, Ordering::Relaxed);
    let x = unsafe { *x };
    x * x
}

unsafe extern "C" fn square_f64(x: *const f64) -> f64 {
    QK15_F64_CALLS.fetch_add(1, Ordering::Relaxed);
    let x = unsafe { *x };
    x * x
}

unsafe extern "C" fn scg_matvec(
    n: *mut FortranInteger,
    x: *mut f32,
    y: *mut f32,
    _nelt: *mut FortranInteger,
    _ia: *mut FortranInteger,
    _ja: *mut FortranInteger,
    a: *mut f32,
    _isym: *mut FortranInteger,
) {
    SCG_MATVEC_CALLS.fetch_add(1, Ordering::Relaxed);
    let len = unsafe { *n as usize };
    let x = unsafe { core::slice::from_raw_parts(x, len) };
    let y = unsafe { core::slice::from_raw_parts_mut(y, len) };
    let a = unsafe { *a };
    y[0] = a * x[0];
}

unsafe extern "C" fn scg_msolve(
    n: *mut FortranInteger,
    r: *mut f32,
    z: *mut f32,
    _nelt: *mut FortranInteger,
    _ia: *mut FortranInteger,
    _ja: *mut FortranInteger,
    _a: *mut f32,
    _isym: *mut FortranInteger,
    _rwork: *mut f32,
    _iwork: *mut FortranInteger,
) {
    SCG_MSOLVE_CALLS.fetch_add(1, Ordering::Relaxed);
    let len = unsafe { *n as usize };
    let r = unsafe { core::slice::from_raw_parts(r, len) };
    let z = unsafe { core::slice::from_raw_parts_mut(z, len) };
    z.copy_from_slice(r);
}

#[test]
fn qk15_invokes_single_precision_integrand() {
    slatec_src::ensure_linked();
    QK15_F32_CALLS.store(0, Ordering::Relaxed);
    let mut a = 0.0_f32;
    let mut b = 1.0_f32;
    let mut result = f32::NAN;
    let mut abserr = f32::NAN;
    let mut resabs = f32::NAN;
    let mut resasc = f32::NAN;
    unsafe {
        slatec_sys::quadrature::callbacks::qk15(
            square_f32,
            &mut a,
            &mut b,
            &mut result,
            &mut abserr,
            &mut resabs,
            &mut resasc,
        )
    };
    assert!(QK15_F32_CALLS.load(Ordering::Relaxed) > 0);
    assert!((result - 1.0 / 3.0).abs() < 1.0e-5);
    assert!(abserr.is_finite());
}

#[test]
fn dqk15_invokes_double_precision_integrand() {
    slatec_src::ensure_linked();
    QK15_F64_CALLS.store(0, Ordering::Relaxed);
    let mut a = 0.0_f64;
    let mut b = 1.0_f64;
    let mut result = f64::NAN;
    let mut abserr = f64::NAN;
    let mut resabs = f64::NAN;
    let mut resasc = f64::NAN;
    unsafe {
        slatec_sys::quadrature::callbacks::dqk15(
            square_f64,
            &mut a,
            &mut b,
            &mut result,
            &mut abserr,
            &mut resabs,
            &mut resasc,
        )
    };
    assert!(QK15_F64_CALLS.load(Ordering::Relaxed) > 0);
    assert!((result - 1.0 / 3.0).abs() < 1.0e-12);
    assert!(abserr.is_finite());
}

#[test]
fn scg_invokes_matrix_and_preconditioner_callbacks() {
    slatec_src::ensure_linked();
    SCG_MATVEC_CALLS.store(0, Ordering::Relaxed);
    SCG_MSOLVE_CALLS.store(0, Ordering::Relaxed);

    let mut n: FortranInteger = 1;
    let mut b = [4.0_f32];
    let mut x = [0.0_f32];
    let mut nelt: FortranInteger = 1;
    let mut ia = [1_i32];
    let mut ja = [1_i32];
    let mut a = [2.0_f32];
    let mut isym: FortranInteger = 0;
    let mut itol: FortranInteger = 1;
    let mut tol = 1.0e-5_f32;
    let mut itmax: FortranInteger = 10;
    let mut iter: FortranInteger = -1;
    let mut err = f32::NAN;
    let mut ierr: FortranInteger = -1;
    let mut iunit: FortranInteger = 0;
    let mut r = [0.0_f32];
    let mut z = [0.0_f32];
    let mut p = [0.0_f32];
    let mut dz = [0.0_f32];
    let mut rwork = [0.0_f32; 1];
    let mut iwork = [0_i32; 1];

    unsafe {
        slatec_sys::linear_algebra::sparse::callbacks::scg(
            &mut n,
            b.as_mut_ptr(),
            x.as_mut_ptr(),
            &mut nelt,
            ia.as_mut_ptr(),
            ja.as_mut_ptr(),
            a.as_mut_ptr(),
            &mut isym,
            scg_matvec,
            scg_msolve,
            &mut itol,
            &mut tol,
            &mut itmax,
            &mut iter,
            &mut err,
            &mut ierr,
            &mut iunit,
            r.as_mut_ptr(),
            z.as_mut_ptr(),
            p.as_mut_ptr(),
            dz.as_mut_ptr(),
            rwork.as_mut_ptr(),
            iwork.as_mut_ptr(),
        )
    };

    assert!(matches!(ierr, 0 | 2 | 4));
    assert!(SCG_MATVEC_CALLS.load(Ordering::Relaxed) > 0);
    assert!(SCG_MSOLVE_CALLS.load(Ordering::Relaxed) > 0);
    assert!(iter >= 0);
    assert!(x[0].is_finite());
}
