//! Focused native evidence for the small raw-candidate promotion batch.

#![cfg(all(
    feature = "small-raw-candidate-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

use core::sync::atomic::{AtomicUsize, Ordering};
use slatec_sys::{Complex32, FortranInteger};

static QUADRATURE_CALLS: AtomicUsize = AtomicUsize::new(0);
static REAL_F32_CALLS: AtomicUsize = AtomicUsize::new(0);
static REAL_F64_CALLS: AtomicUsize = AtomicUsize::new(0);
static COMPLEX_CALLS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn square(x: *const f64) -> f64 {
    QUADRATURE_CALLS.fetch_add(1, Ordering::Relaxed);
    let x = unsafe { *x };
    x * x
}

unsafe extern "C" fn rhs_f32(n: *mut FortranInteger, _t: *mut f32, y: *mut f32, ydot: *mut f32) {
    REAL_F32_CALLS.fetch_add(1, Ordering::Relaxed);
    let n = unsafe { *n as usize };
    let y = unsafe { core::slice::from_raw_parts(y, n) };
    let ydot = unsafe { core::slice::from_raw_parts_mut(ydot, n) };
    ydot.copy_from_slice(y);
}

unsafe extern "C" fn rhs_f64(n: *mut FortranInteger, _t: *mut f64, y: *mut f64, ydot: *mut f64) {
    REAL_F64_CALLS.fetch_add(1, Ordering::Relaxed);
    let n = unsafe { *n as usize };
    let y = unsafe { core::slice::from_raw_parts(y, n) };
    let ydot = unsafe { core::slice::from_raw_parts_mut(ydot, n) };
    ydot.copy_from_slice(y);
}

unsafe extern "C" fn unused_root_f32(
    _n: *mut FortranInteger,
    _t: *mut f32,
    _y: *mut f32,
    _iroot: *mut FortranInteger,
) -> f32 {
    1.0
}

unsafe extern "C" fn complex_rhs(
    n: *mut FortranInteger,
    _t: *mut f32,
    y: *mut Complex32,
    ydot: *mut Complex32,
) {
    COMPLEX_CALLS.fetch_add(1, Ordering::Relaxed);
    let n = unsafe { *n as usize };
    let y = unsafe { core::slice::from_raw_parts(y, n) };
    let ydot = unsafe { core::slice::from_raw_parts_mut(ydot, n) };
    for (out, input) in ydot.iter_mut().zip(y) {
        *out = Complex32 {
            re: -input.im,
            im: input.re,
        };
    }
}

unsafe extern "C" fn unused_complex_root(
    _n: *mut FortranInteger,
    _t: *mut f32,
    _y: *mut Complex32,
    _iroot: *mut FortranInteger,
) -> f32 {
    1.0
}

fn sdrive1_workspace_len(n: usize) -> usize {
    n * n + 11 * n + 300
}

fn sdrive2_workspace_len(n: usize, root_count: usize, method: FortranInteger) -> usize {
    match method {
        1 => 16 * n + 2 * root_count + 250,
        2 => n * n + 10 * n + 2 * root_count + 250,
        3 => n * n + 17 * n + 2 * root_count + 250,
        _ => unreachable!("test uses a source-documented method selector"),
    }
}

#[test]
fn promoted_symbols_link_through_their_exact_provider_closures() {
    slatec_src::ensure_linked();
    for symbol in [
        slatec_sys::quadrature::dpfqad as *const () as usize,
        slatec_sys::ode::sdriv1 as *const () as usize,
        slatec_sys::ode::ddriv1 as *const () as usize,
        slatec_sys::ode::sdriv2 as *const () as usize,
        slatec_sys::ode::ddriv2 as *const () as usize,
        slatec_sys::ode::cdriv1 as *const () as usize,
        slatec_sys::ode::cdriv2 as *const () as usize,
    ] {
        assert_ne!(core::hint::black_box(symbol), 0);
    }
}

#[test]
fn dpfqad_integrates_a_polynomial_times_a_constant_piecewise_polynomial() {
    slatec_src::ensure_linked();
    QUADRATURE_CALLS.store(0, Ordering::Relaxed);
    let mut leading_dimension: FortranInteger = 1;
    let mut coefficients = [1.0_f64];
    let mut breakpoints = [0.0_f64, 1.0_f64];
    let mut piece_count: FortranInteger = 1;
    let mut order: FortranInteger = 1;
    let mut derivative_order: FortranInteger = 0;
    let mut lower = 0.0_f64;
    let mut upper = 1.0_f64;
    let mut tolerance = 1.0e-10_f64;
    let mut integral = 0.0_f64;
    let mut status: FortranInteger = 0;

    unsafe {
        slatec_sys::quadrature::dpfqad(
            square,
            &mut leading_dimension,
            coefficients.as_mut_ptr(),
            breakpoints.as_mut_ptr(),
            &mut piece_count,
            &mut order,
            &mut derivative_order,
            &mut lower,
            &mut upper,
            &mut tolerance,
            &mut integral,
            &mut status,
        );
    }

    assert_eq!(status, 1);
    assert!((integral - 1.0 / 3.0).abs() < 1.0e-8);
    assert!(QUADRATURE_CALLS.load(Ordering::Relaxed) >= 8);
}

#[test]
fn real_sdrive_convenience_and_advanced_drivers_converge() {
    slatec_src::ensure_linked();
    REAL_F32_CALLS.store(0, Ordering::Relaxed);
    REAL_F64_CALLS.store(0, Ordering::Relaxed);

    let mut n: FortranInteger = 1;
    let mut t = 0.0_f32;
    let mut y = [1.0_f32];
    let mut tout = 1.0_f32;
    let mut mstate: FortranInteger = 1;
    let mut eps = 1.0e-5_f32;
    let mut work = vec![0.0_f32; sdrive1_workspace_len(1)];
    let mut lenw = work.len() as FortranInteger;
    let mut ierflg: FortranInteger = 0;
    assert_eq!(work.len(), sdrive1_workspace_len(1));
    unsafe {
        slatec_sys::ode::sdriv1(
            &mut n,
            &mut t,
            y.as_mut_ptr(),
            rhs_f32,
            &mut tout,
            &mut mstate,
            &mut eps,
            work.as_mut_ptr(),
            &mut lenw,
            &mut ierflg,
        )
    };
    assert_eq!(mstate, 2);
    assert_eq!(ierflg, 0);
    assert!((y[0] - core::f32::consts::E).abs() < 3.0e-3);

    let mut nroot: FortranInteger = 0;
    let mut method: FortranInteger = 1;
    let mut ewt = 1.0_f32;
    let mut iwork = vec![FortranInteger::default(); 50];
    let mut leniw = iwork.len() as FortranInteger;
    let mut work2 = vec![0.0_f32; sdrive2_workspace_len(1, 0, method)];
    let mut lenw2 = work2.len() as FortranInteger;
    let mut t2 = 0.0_f32;
    let mut y2 = [1.0_f32];
    let mut tout2 = 1.0_f32;
    let mut mstate2: FortranInteger = 1;
    let mut eps2 = 1.0e-5_f32;
    let mut ierflg2: FortranInteger = 0;
    assert_eq!(work2.len(), 266);
    unsafe {
        slatec_sys::ode::sdriv2(
            &mut n,
            &mut t2,
            y2.as_mut_ptr(),
            rhs_f32,
            &mut tout2,
            &mut mstate2,
            &mut nroot,
            &mut eps2,
            &mut ewt,
            &mut method,
            work2.as_mut_ptr(),
            &mut lenw2,
            iwork.as_mut_ptr(),
            &mut leniw,
            unused_root_f32,
            &mut ierflg2,
        )
    };
    assert_eq!(mstate2, 2);
    assert_eq!(ierflg2, 0);
    assert!((y2[0] - core::f32::consts::E).abs() < 3.0e-3);

    let mut td = 0.0_f64;
    let mut yd = [1.0_f64];
    let mut toutd = 1.0_f64;
    let mut mstated: FortranInteger = 1;
    let mut epsd = 1.0e-10_f64;
    let mut workd = vec![0.0_f64; sdrive1_workspace_len(1)];
    let mut lenwd = workd.len() as FortranInteger;
    let mut ierflgd: FortranInteger = 0;
    unsafe {
        slatec_sys::ode::ddriv1(
            &mut n,
            &mut td,
            yd.as_mut_ptr(),
            rhs_f64,
            &mut toutd,
            &mut mstated,
            &mut epsd,
            workd.as_mut_ptr(),
            &mut lenwd,
            &mut ierflgd,
        )
    };
    assert_eq!(mstated, 2);
    assert_eq!(ierflgd, 0);
    assert!((yd[0] - core::f64::consts::E).abs() < 3.0e-8);

    let mut ewt_d = 1.0_f64;
    let mut iwork_d = vec![FortranInteger::default(); 50];
    let mut leniw_d = iwork_d.len() as FortranInteger;
    let mut workd2 = vec![0.0_f64; sdrive2_workspace_len(1, 0, method)];
    let mut lenwd2 = workd2.len() as FortranInteger;
    let mut td2 = 0.0_f64;
    let mut yd2 = [1.0_f64];
    let mut toutd2 = 1.0_f64;
    let mut mstated2: FortranInteger = 1;
    let mut epsd2 = 1.0e-10_f64;
    let mut ierflgd2: FortranInteger = 0;
    unsafe {
        slatec_sys::ode::ddriv2(
            &mut n,
            &mut td2,
            yd2.as_mut_ptr(),
            rhs_f64,
            &mut toutd2,
            &mut mstated2,
            &mut nroot,
            &mut epsd2,
            &mut ewt_d,
            &mut method,
            workd2.as_mut_ptr(),
            &mut lenwd2,
            iwork_d.as_mut_ptr(),
            &mut leniw_d,
            unused_root_f64,
            &mut ierflgd2,
        )
    };
    assert_eq!(mstated2, 2);
    assert_eq!(ierflgd2, 0);
    assert!((yd2[0] - core::f64::consts::E).abs() < 3.0e-8);
    assert!(REAL_F32_CALLS.load(Ordering::Relaxed) > 1);
    assert!(REAL_F64_CALLS.load(Ordering::Relaxed) > 1);
}

unsafe extern "C" fn unused_root_f64(
    _n: *mut FortranInteger,
    _t: *mut f64,
    _y: *mut f64,
    _iroot: *mut FortranInteger,
) -> f64 {
    1.0
}

#[test]
fn complex_sdrive_wrappers_validate_complex_callback_layout() {
    slatec_src::ensure_linked();
    COMPLEX_CALLS.store(0, Ordering::Relaxed);
    let mut n: FortranInteger = 1;
    let mut t = 0.0_f32;
    let mut y = [Complex32 { re: 1.0, im: 0.0 }];
    let mut tout = 1.0_f32;
    let mut mstate: FortranInteger = 1;
    let mut eps = 1.0e-5_f32;
    let mut work = vec![Complex32::default(); sdrive1_workspace_len(1)];
    let mut lenw = work.len() as FortranInteger;
    let mut ierflg: FortranInteger = 0;
    unsafe {
        slatec_sys::ode::cdriv1(
            &mut n,
            &mut t,
            y.as_mut_ptr(),
            complex_rhs,
            &mut tout,
            &mut mstate,
            &mut eps,
            work.as_mut_ptr(),
            &mut lenw,
            &mut ierflg,
        )
    };
    assert_eq!(mstate, 2);
    assert_eq!(ierflg, 0);
    assert!((y[0].re - 1.0_f32.cos()).abs() < 3.0e-3);
    assert!((y[0].im - 1.0_f32.sin()).abs() < 3.0e-3);

    let mut nroot: FortranInteger = 0;
    let mut ewt = 1.0_f32;
    let mut method: FortranInteger = 1;
    let mut work2 = vec![Complex32::default(); sdrive2_workspace_len(1, 0, method)];
    let mut lenw2 = work2.len() as FortranInteger;
    let mut iwork = vec![FortranInteger::default(); 50];
    let mut leniw = iwork.len() as FortranInteger;
    let mut t2 = 0.0_f32;
    let mut y2 = [Complex32 { re: 1.0, im: 0.0 }];
    let mut tout2 = 1.0_f32;
    let mut mstate2: FortranInteger = 1;
    let mut eps2 = 1.0e-5_f32;
    let mut ierflg2: FortranInteger = 0;
    unsafe {
        slatec_sys::ode::cdriv2(
            &mut n,
            &mut t2,
            y2.as_mut_ptr(),
            complex_rhs,
            &mut tout2,
            &mut mstate2,
            &mut nroot,
            &mut eps2,
            &mut ewt,
            &mut method,
            work2.as_mut_ptr(),
            &mut lenw2,
            iwork.as_mut_ptr(),
            &mut leniw,
            unused_complex_root,
            &mut ierflg2,
        )
    };
    assert_eq!(mstate2, 2);
    assert_eq!(ierflg2, 0);
    assert!((y2[0].re - 1.0_f32.cos()).abs() < 3.0e-3);
    assert!((y2[0].im - 1.0_f32.sin()).abs() < 3.0e-3);
    assert!(COMPLEX_CALLS.load(Ordering::Relaxed) > 1);
}
