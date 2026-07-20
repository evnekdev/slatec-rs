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
    let _ = core::hint::black_box(slatec_sys::special::numerical::acosh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::algams as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::ali as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::asinh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::atanh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::avint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::bakvec as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::balanc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::balbak as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::bandr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::bandv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besi0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besi0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besi1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besi1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besj0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besj1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besk0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besk0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besk1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besk1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::beskes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besks as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besy0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::besy1 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bint4 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bintk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::bisect as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::blktri as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::bndacc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::bndsol as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::bqr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::bskin as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bspdr as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bspev as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bsppp as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bspvd as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bspvn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::bsqad as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::bvalu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::cbabk2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::cbal as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::cfftb1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::cfftf1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::cffti1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::cg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::cgeev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::ch as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::chfdv as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::chfev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::chiev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::nonlinear::numerical::chkder as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::chu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::cinvit as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::combak as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::comhes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::comlr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::comlr2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::comqr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::comqr2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::cortb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::corth as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::cosqb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::cosqf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::cosqi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::cost as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::costi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::cot as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::csevl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::statistics::numerical::cv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dacosh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dasinh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::datanh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::davint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesi0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesi1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesj0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesj1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesk0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesk1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesks as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesy0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dbesy1 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbint4 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::dbintk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dbndac as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dbndsl as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::dbocls as *const () as usize);
}
