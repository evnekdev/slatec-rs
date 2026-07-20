//! Generated Batch B native-link probe.
//! Regenerate with `slatec-corpus generate-raw-batch-b --offline`.

#![cfg(all(
    feature = "raw-callback-link-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

#[test]
fn callback_symbols_link() {
    slatec_src::ensure_linked();
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::dbcg as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::dcg as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::dcgn as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::dcgs as *const () as usize,
    );
    let _ = core::hint::black_box(slatec_sys::ode::callbacks::ddeabm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::ode::callbacks::dderkf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::ode::callbacks::derkf as *const () as usize);
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::dgmres as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::dir as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::domn as *const () as usize,
    );
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqagie as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqagpe as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqagse as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqawce as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqawse as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqc25c as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqc25s as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk15 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk15i as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk15w as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk21 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk31 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk41 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk51 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::dqk61 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qagie as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qagpe as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qagse as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qawce as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qawse as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qc25c as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qc25s as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk15 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk15i as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk15w as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk21 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk31 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk41 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk51 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::callbacks::qk61 as *const () as usize);
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::sbcg as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::scg as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::scgn as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::scgs as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::sgmres as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::sir as *const () as usize,
    );
    let _ = core::hint::black_box(
        slatec_sys::linear_algebra::sparse::callbacks::somn as *const () as usize,
    );
}
