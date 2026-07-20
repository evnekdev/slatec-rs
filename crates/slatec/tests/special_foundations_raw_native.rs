#![cfg(all(
    feature = "special-raw-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Direct canonical raw special-function tests using independent constants and
//! identities rather than another SLATEC entry as an oracle.

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

fn retain_selected_provider() {
    slatec_src::ensure_linked();
}

#[test]
fn elementary_foundations_match_definitions_and_degree_identities() {
    retain_selected_provider();
    let mut x = 1.0e-8_f64;
    let log1p = unsafe { slatec_sys::special::elementary::dlnrel(&mut x) };
    close(log1p, 9.999_999_950_000_000e-9, 1.0e-20);

    let mut zero = 0.0_f64;
    close(
        unsafe { slatec_sys::special::elementary::dexprl(&mut zero) },
        1.0,
        1.0e-15,
    );
    let mut eight = 8.0_f64;
    close(
        unsafe { slatec_sys::special::elementary::dcbrt(&mut eight) },
        2.0,
        1.0e-15,
    );
    let mut degrees = 90.0_f64;
    close(
        unsafe { slatec_sys::special::elementary::dsindg(&mut degrees) },
        1.0,
        1.0e-15,
    );
}

#[test]
fn gamma_foundations_match_independent_constants() {
    retain_selected_provider();
    let mut five = 5.0_f64;
    close(
        unsafe { slatec_sys::special::gamma::dgamma(&mut five) },
        24.0,
        1.0e-13,
    );
    let mut half = 0.5_f64;
    close(
        unsafe { slatec_sys::special::gamma::dgamma(&mut half) },
        core::f64::consts::PI.sqrt(),
        1.0e-13,
    );
    let mut negative_half = -0.5_f64;
    close(
        unsafe { slatec_sys::special::gamma::dgamma(&mut negative_half) },
        -2.0 * core::f64::consts::PI.sqrt(),
        2.0e-12,
    );
}

#[test]
fn beta_and_error_foundations_match_constants_and_identities() {
    retain_selected_provider();
    let mut half = 0.5_f64;
    let mut second_half = 0.5_f64;
    close(
        unsafe { slatec_sys::special::beta::dbeta(&mut half, &mut second_half) },
        core::f64::consts::PI,
        1.0e-13,
    );
    let mut x = 0.5_f64;
    let mut p = 0.5_f64;
    let mut q = 0.5_f64;
    close(
        unsafe { slatec_sys::special::beta::dbetai(&mut x, &mut p, &mut q) },
        0.5,
        1.0e-13,
    );

    let mut one = 1.0_f64;
    let erf = unsafe { slatec_sys::special::error::derf(&mut one) };
    close(erf, 0.842_700_792_949_714_9, 1.0e-14);
    let mut one_again = 1.0_f64;
    let erfc = unsafe { slatec_sys::special::error::derfc(&mut one_again) };
    close(erf + erfc, 1.0, 1.0e-14);
}
