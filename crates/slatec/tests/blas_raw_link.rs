//! Generated native link retention coverage for every reviewed raw BLAS symbol.
//! Run on the supported GNU MinGW target with `blas-raw-link-tests`.

#![cfg(all(
    feature = "blas-raw-link-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

#[test]
fn every_reviewed_blas_symbol_links_from_its_level_provider_closure() {
    slatec_src::ensure_linked();
    // Level 1
    std::hint::black_box(slatec_sys::blas::level1::caxpy as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::ccopy as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::crotg as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::cscal as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::csrot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::csscal as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::cswap as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dasum as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::daxpy as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dcdot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dcopy as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dcopym as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::ddot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dnrm2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dqdota as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dqdoti as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::drot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::drotg as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::drotm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::drotmg as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::ds2y as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dscal as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dsdi as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dsdot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dsmtv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dsmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::dswap as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::icamax as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::icopy as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::idamax as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::isamax as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::iswap as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::sasum as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::saxpy as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::scasum as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::scnrm2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::scopy as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::scopym as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::sdot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::sdsdot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::snrm2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::srot as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::srotg as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::srotm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::srotmg as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::ss2y as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::sscal as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::ssdi as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::ssmtv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::ssmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level1::sswap as *const () as usize);
    // Level 2
    std::hint::black_box(slatec_sys::blas::level2::cgbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::cgemv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::cgerc as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::cgeru as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::chbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::chemv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::cher as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::cher2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::chpmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::chpr as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::chpr2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ctbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ctbsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ctpmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ctpsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ctrmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ctrsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dgbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dgemv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dger as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dsbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dspmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dspr as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dspr2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dsymv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dsyr as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dsyr2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dtbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dtbsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dtpmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dtpsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dtrmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::dtrsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::sgbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::sgemv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::sger as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ssbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::sspmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::sspr as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::sspr2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ssymv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ssyr as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::ssyr2 as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::stbmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::stbsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::stpmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::stpsv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::strmv as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level2::strsv as *const () as usize);
    // Level 3
    std::hint::black_box(slatec_sys::blas::level3::cgemm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::chemm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::cher2k as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::cherk as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::csymm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::csyr2k as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::csyrk as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::ctrmm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::ctrsm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::dgemm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::dsymm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::dsyr2k as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::dsyrk as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::dtrmm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::dtrsm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::sgemm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::ssymm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::ssyr2k as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::ssyrk as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::strmm as *const () as usize);
    std::hint::black_box(slatec_sys::blas::level3::strsm as *const () as usize);
}
