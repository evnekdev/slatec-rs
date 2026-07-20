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
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spoir as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sposl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sppco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sppdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sppfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sppsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sptsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sqrdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sqrsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ss2lt as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssd2s as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdbcg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdcg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdcgn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdcgs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdgmr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdomn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssds as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdscl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssgs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssiccg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssico as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssics as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssidi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::ssiev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssifa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssilur as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssilus as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssisl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssjac as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssli as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssli2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssllti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslubc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslucn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslucs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslugm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslui as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslui2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslui4 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssluom as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssluti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssmmi2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssmmti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sspco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sspdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::sspev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sspfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sspsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssvdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::strco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::strdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::strsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tinvit as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tql1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tql2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tqlrat as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::trbak1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::trbak3 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tred1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tred2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tred3 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tridib as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::eigen::numerical::tsturm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ulsia as *const () as usize);
    let _ =
        core::hint::black_box(slatec_sys::approximation::numerical::wnnls as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::xlegf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::xnrmp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::zairy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::zbesh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::zbesi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::zbesj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::zbesk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::zbesy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::numerical::zbiry as *const () as usize);
}
