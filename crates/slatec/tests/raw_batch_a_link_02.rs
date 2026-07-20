//! Generated Batch A native-link probe.
//! Regenerate with `slatec-corpus generate-raw-batch-a --offline`.

#![cfg(all(
    feature = "raw-batch-a-link-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

#[test]
fn canonical_symbols_link() {
    slatec_src::ensure_linked();
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::dbols as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbsi0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbsi1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbsk0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbsk1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbskes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbskin as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbspdr as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbspev as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbsppp as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbspvd as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbspvn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::dbsqad as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbvalu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchdd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchex as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dchfdv as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dchfev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dchu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchud as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::nonlinear::numerical::dckder as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dcot as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dcsevl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::statistics::numerical::dcv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::de1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::numerical::defc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dei as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dexint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::numerical::dfc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dgamlm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::dgaus8 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgbsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgeco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgedi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgefa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgefs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgesl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dglss as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgtsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dhfti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::ode::numerical::dintp as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dintrv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dlgams as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dli as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dllsia as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dllti2 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::dlsei as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dnbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dnbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dnbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dnbfs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dnbsl as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::dp1vlu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpbsl as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchbs as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchcm as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchfd as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchfe as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchia as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchic as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchid as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchim as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpchsp as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::dpcoef as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dplint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dpoch as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dpoch1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpoco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpodi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpofa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpofs as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpolcf as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::dpolft as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dpolvl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dposl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dppco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dppdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dppfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::dppqad as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dppsl as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dppval as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dpsifn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dptsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::dqmomo as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dqrdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dqrsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::drc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::drc3jj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::drc3jm as *const () as usize);
}
