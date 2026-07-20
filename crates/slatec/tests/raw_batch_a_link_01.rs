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
    let _ = core::hint::black_box(slatec_sys::special::acosh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::algams as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::ali as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::asinh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::atanh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::avint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::bakvec as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::balanc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::balbak as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::bandr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::bandv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besi0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besi0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besi1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besi1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besj0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besj1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besk0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besk0e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besk1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besk1e as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::beskes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besks as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besy0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::besy1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bint4 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bintk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::bisect as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::pde::fishpack::blktri as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::bndacc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::bndsol as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::bqr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bskin as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bspdr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bspev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bsppp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bspvd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bspvn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::bsqad as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::bvalu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::cbabk2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::cbal as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::cfftb1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::cfftf1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::cffti1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::cg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::cgeev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::ch as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::chfdv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::chfev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::chiev as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::nonlinear::jacobian_check::chkder as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::chu as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::cinvit as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::combak as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::comhes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::comlr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::comlr2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::comqr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::comqr2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::cortb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::corth as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::cosqb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::cosqf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::cosqi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::cost as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::costi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::cot as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::csevl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::statistics::cv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dacosh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::dasinh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::datanh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::davint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesi0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesi1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesj0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesj1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesk0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesk1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesks as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesy0 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::dbesy1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbint4 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::interpolation::dbintk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dbndac as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::dbndsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::dbocls as *const () as usize);
}
