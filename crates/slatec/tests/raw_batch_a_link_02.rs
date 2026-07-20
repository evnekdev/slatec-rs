//! Generated Batch A native-link probe.
//! Regenerate with `slatec-corpus generate-raw-batch-a --offline`.

#![cfg(all(
    feature = "raw-canonical-link-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

#[test]
fn canonical_symbols_link() {
    slatec_src::ensure_linked();
    let _ = core::hint::black_box(slatec_sys::approximation::dbols as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dbsi0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dbsi1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dbsk0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dbsk1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dbskes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dbskin as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbspdr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbspev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbsppp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbspvd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbspvn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::dbsqad as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbvalu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchdd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchex as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dchfdv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dchfev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dchu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dchud as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::nonlinear::jacobian_check::dckder as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dcot as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dcsevl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::statistics::dcv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::de1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::defc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dei as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dexint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::dfc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dgamlm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::dgaus8 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dgbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dgbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dgbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dgbsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgeco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgedi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgefa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgefs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dgesl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dglss as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dgtsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dhfti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::ode::dintp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dintrv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dlgams as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dli as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dllsia as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dllti2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::dlsei as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dnbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dnbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dnbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dnbfs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dnbsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::dp1vlu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dpbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dpbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dpbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dpbsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchbs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchcm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchfd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchfe as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchia as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchic as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchid as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchim as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpchsp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::dpcoef as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dplint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dpoch as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dpoch1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpoco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpodi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpofa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dpofs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpolcf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::dpolft as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dpolvl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dposl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::dppco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::dppdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::dppfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::dppqad as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::dppsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dppval as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dpsifn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dptsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::dqmomo as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dqrdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dqrsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::drc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::drc3jj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::drc3jm as *const () as usize);
}
