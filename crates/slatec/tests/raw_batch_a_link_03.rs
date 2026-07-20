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
    let _ = core::hint::black_box(slatec_sys::special::numerical::drc6j as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::drd as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::drf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::drj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ds2lt as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsd2s as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsdbcg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsdcg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsdcgn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsdcgs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsdgmr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsdomn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsds as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsdscl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsgs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsiccg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsico as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsics as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsidi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsifa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsilur as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsilus as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsisl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsjac as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsli as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsli2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsllti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dslubc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dslucn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dslucs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dslugm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dslui as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dslui2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dslui4 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsluom as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsluti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsmmi2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsmmti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dspco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dspdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dspenc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dspfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dspsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dsvdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dtrco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dtrdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dtrsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::dulsia as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::dwnnls as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dxlegf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::dxnrmp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::e1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::numerical::efc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::ei as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::elmbak as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::elmhes as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::eltran as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::exint as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::ezfftb as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::ezfftf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::fftpack::numerical::ezffti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::numerical::fc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::figi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::figi2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::gamlim as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::quadrature::numerical::gaus8 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::genbun as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::hfti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::hqr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::hqr2 as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hstcrt as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hstcsp as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hstcyl as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hstplr as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hstssp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::htrib3 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::htribk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::htrid3 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::htridi as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hw3crt as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hwscsp as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hwscyl as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hwsplr as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::pde::fishpack::numerical::hwsssp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::imtql1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::imtql2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::imtqlv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::initds as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::inits as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::interpolation::numerical::intrv as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::invit as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::llsia as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::numerical::lsei as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::minfit as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::ortbak as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::orthes as *const () as usize);
}
