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
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::spoir as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sposl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sppco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sppdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sppfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sppsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::banded::sptsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sqrdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sqrsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ss2lt as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssd2s as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssdbcg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssdcg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssdcgn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssdcgs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssdgmr as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssdomn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssds as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssdscl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssgs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssiccg as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssico as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssics as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssidi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::ssiev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssifa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssilur as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssilus as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssisl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssjac as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssli as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssli2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssllti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::sslubc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::sslucn as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslucs as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::sslugm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslui as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslui2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::sslui4 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::sparse::ssluom as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssluti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssmmi2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssmmti as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sspco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sspdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::sspev as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sspfa as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::packed::sspsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ssvdc as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::strco as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::strdi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::strsl as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tinvit as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tql1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tql2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tqlrat as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::trbak1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::trbak3 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tred1 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tred2 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tred3 as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tridib as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::eigen::tsturm as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::linear_algebra::dense::ulsia as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::approximation::wnnls as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::xlegf as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::xnrmp as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::zairy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::zbesh as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::zbesi as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::zbesj as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::zbesk as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::bessel::zbesy as *const () as usize);
    let _ = core::hint::black_box(slatec_sys::special::zbiry as *const () as usize);
}
