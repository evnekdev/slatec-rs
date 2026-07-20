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
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::ortran as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchbs as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchcm as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchfd as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchfe as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchia as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchic as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchid as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchim as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::pchsp as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::pcoef as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::poch as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::poch1 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::poistg as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::polcof as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::polfit as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::polint as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::polyvl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::ppqad as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::ppval as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::psifn as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::pvalue as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::qmomo as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::qzhes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::qzit as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::qzval as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::qzvec as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::statistics::numerical::rand as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::ratqr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::rc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::rc3jj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::rc3jm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::rc6j as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::rd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rebak as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rebakb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::reduc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::reduc2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::rf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::rfftb1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::rfftf1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::rffti1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::statistics::numerical::rgauss as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rgg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::rj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rsb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rsg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rsgab as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rsgba as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rsp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rst as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::rt as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::statistics::numerical::runif as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::sbocls as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::sbols as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::schdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::schdd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::schex as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::schud as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgbsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgeco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgedi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::sgeev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgefa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgefs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgeir as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgesl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sglss as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sgtsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::sinqb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::sinqf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::sinqi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::sint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::sinti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::ode::numerical::sintrp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sllti2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::snbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::snbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::snbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::snbfs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::snbir as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::snbsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spbco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spbdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spbfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spbsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::spenc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spoco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spodi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spofa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spofs as *const () as usize);
}
