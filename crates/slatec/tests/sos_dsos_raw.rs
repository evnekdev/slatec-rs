//! Native callback and convergence coverage for the raw SOS/DSOS drivers.

#![cfg(all(
    feature = "nonlinear-systems-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

use core::sync::atomic::{AtomicUsize, Ordering};
use slatec_sys::FortranInteger;

static SOS_CALLS: AtomicUsize = AtomicUsize::new(0);
static DSOS_CALLS: AtomicUsize = AtomicUsize::new(0);
static SOS_K_MASK: AtomicUsize = AtomicUsize::new(0);
static DSOS_K_MASK: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn equations_f32(x: *const f32, k: *const FortranInteger) -> f32 {
    SOS_CALLS.fetch_add(1, Ordering::Relaxed);
    let x = unsafe { core::slice::from_raw_parts(x, 2) };
    match unsafe { *k } {
        1 => {
            SOS_K_MASK.fetch_or(1, Ordering::Relaxed);
            x[0] * x[0] + x[1] * x[1] - 1.0
        }
        2 => {
            SOS_K_MASK.fetch_or(2, Ordering::Relaxed);
            x[0] - x[1]
        }
        _ => f32::NAN,
    }
}

unsafe extern "C" fn equations_f64(x: *const f64, k: *const FortranInteger) -> f64 {
    DSOS_CALLS.fetch_add(1, Ordering::Relaxed);
    let x = unsafe { core::slice::from_raw_parts(x, 2) };
    match unsafe { *k } {
        1 => {
            DSOS_K_MASK.fetch_or(1, Ordering::Relaxed);
            x[0] * x[0] + x[1] * x[1] - 1.0
        }
        2 => {
            DSOS_K_MASK.fetch_or(2, Ordering::Relaxed);
            x[0] - x[1]
        }
        _ => f64::NAN,
    }
}

fn real_workspace_len(equation_count: usize) -> usize {
    1 + 6 * equation_count + equation_count * (equation_count + 1) / 2
}

#[test]
fn sos_and_dsos_link_through_their_exact_provider_closure() {
    slatec_src::ensure_linked();
    let _ = core::hint::black_box(slatec_sys::nonlinear::systems::sos as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::nonlinear::systems::dsos as *const () as usize);
}

#[test]
fn sos_converges_and_uses_the_one_based_equation_callback_abi() {
    slatec_src::ensure_linked();
    SOS_CALLS.store(0, Ordering::Relaxed);
    SOS_K_MASK.store(0, Ordering::Relaxed);
    let mut equation_count: FortranInteger = 2;
    let mut x = [0.8_f32, 0.6_f32];
    let mut rtolx = 1.0e-5_f32;
    let mut atolx = 1.0e-6_f32;
    let mut tolf = 1.0e-5_f32;
    let mut iflag: FortranInteger = 0;
    let mut rw = vec![0.0_f32; real_workspace_len(2)];
    let mut lrw = rw.len() as FortranInteger;
    let mut iw = vec![0_i32; 3 + 2];
    let mut liw = iw.len() as FortranInteger;

    unsafe {
        slatec_sys::nonlinear::systems::sos(
            equations_f32,
            &mut equation_count,
            x.as_mut_ptr(),
            &mut rtolx,
            &mut atolx,
            &mut tolf,
            &mut iflag,
            rw.as_mut_ptr(),
            &mut lrw,
            iw.as_mut_ptr(),
            &mut liw,
        );
    }

    let expected = 0.5_f32.sqrt();
    assert!(matches!(iflag, 1..=3), "SOS IFLAG={iflag}");
    assert!((x[0] - expected).abs() < 2.0e-3);
    assert!((x[1] - expected).abs() < 2.0e-3);
    assert!(rw[0].is_finite());
    assert!(iw[2] > 0);
    assert!(SOS_CALLS.load(Ordering::Relaxed) > 2);
    assert_eq!(SOS_K_MASK.load(Ordering::Relaxed), 3);
}

#[test]
fn dsos_converges_and_uses_the_one_based_equation_callback_abi() {
    slatec_src::ensure_linked();
    DSOS_CALLS.store(0, Ordering::Relaxed);
    DSOS_K_MASK.store(0, Ordering::Relaxed);
    let mut equation_count: FortranInteger = 2;
    let mut x = [0.8_f64, 0.6_f64];
    let mut rtolx = 1.0e-11_f64;
    let mut atolx = 1.0e-12_f64;
    let mut tolf = 1.0e-11_f64;
    let mut iflag: FortranInteger = 0;
    let mut rw = vec![0.0_f64; real_workspace_len(2)];
    let mut lrw = rw.len() as FortranInteger;
    let mut iw = vec![0_i32; 3 + 2];
    let mut liw = iw.len() as FortranInteger;

    unsafe {
        slatec_sys::nonlinear::systems::dsos(
            equations_f64,
            &mut equation_count,
            x.as_mut_ptr(),
            &mut rtolx,
            &mut atolx,
            &mut tolf,
            &mut iflag,
            rw.as_mut_ptr(),
            &mut lrw,
            iw.as_mut_ptr(),
            &mut liw,
        );
    }

    let expected = 0.5_f64.sqrt();
    assert!(matches!(iflag, 1..=3), "DSOS IFLAG={iflag}");
    assert!((x[0] - expected).abs() < 2.0e-9);
    assert!((x[1] - expected).abs() < 2.0e-9);
    assert!(rw[0].is_finite());
    assert!(iw[2] > 0);
    assert!(DSOS_CALLS.load(Ordering::Relaxed) > 2);
    assert_eq!(DSOS_K_MASK.load(Ordering::Relaxed), 3);
}
