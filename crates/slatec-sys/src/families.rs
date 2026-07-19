//! Generated family-scoped raw declarations.
//!
//! Regenerate with `slatec-corpus generate-linkage-metadata --offline`.
#![allow(clippy::missing_safety_doc, unused_imports)]

/// Compatibility re-exports for the canonical reviewed BLAS Level 1 namespace.
#[cfg(feature = "raw-family-blas-level1")]
pub mod blas_level1 {
    pub use crate::blas::level1::*;
}

/// Compatibility re-exports for the canonical reviewed BLAS Level 2 namespace.
#[cfg(feature = "raw-family-blas-level2")]
pub mod blas_level2 {
    pub use crate::blas::level2::*;
}

/// Compatibility re-exports for the canonical reviewed BLAS Level 3 namespace.
#[cfg(feature = "raw-family-blas-level3")]
pub mod blas_level3 {
    pub use crate::blas::level3::*;
}

/// Reviewed declarations required by `special-airy`.
#[cfg(feature = "raw-family-special-airy")]
pub mod special_airy {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "ai_"]
        pub fn ai(x: *mut f32) -> f32;
        #[link_name = "aie_"]
        pub fn aie(x: *mut f32) -> f32;
        #[link_name = "bi_"]
        pub fn bi(x: *mut f32) -> f32;
        #[link_name = "bie_"]
        pub fn bie(x: *mut f32) -> f32;
        #[link_name = "dai_"]
        pub fn dai(x: *mut f64) -> f64;
        #[link_name = "daie_"]
        pub fn daie(x: *mut f64) -> f64;
        #[link_name = "dbi_"]
        pub fn dbi(x: *mut f64) -> f64;
        #[link_name = "dbie_"]
        pub fn dbie(x: *mut f64) -> f64;
    }
}

/// Reviewed declarations required by `special-bessel`.
#[cfg(feature = "raw-family-special-bessel")]
pub mod special_bessel {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "besi0_"]
        pub fn besi0(x: *mut f32) -> f32;
        #[link_name = "besi0e_"]
        pub fn besi0e(x: *mut f32) -> f32;
        #[link_name = "besi1_"]
        pub fn besi1(x: *mut f32) -> f32;
        #[link_name = "besi1e_"]
        pub fn besi1e(x: *mut f32) -> f32;
        #[link_name = "besj0_"]
        pub fn besj0(x: *mut f32) -> f32;
        #[link_name = "besj1_"]
        pub fn besj1(x: *mut f32) -> f32;
        #[link_name = "besk0_"]
        pub fn besk0(x: *mut f32) -> f32;
        #[link_name = "besk0e_"]
        pub fn besk0e(x: *mut f32) -> f32;
        #[link_name = "besk1_"]
        pub fn besk1(x: *mut f32) -> f32;
        #[link_name = "besk1e_"]
        pub fn besk1e(x: *mut f32) -> f32;
        #[link_name = "besy0_"]
        pub fn besy0(x: *mut f32) -> f32;
        #[link_name = "besy1_"]
        pub fn besy1(x: *mut f32) -> f32;
        #[link_name = "dbesi0_"]
        pub fn dbesi0(x: *mut f64) -> f64;
        #[link_name = "dbesi1_"]
        pub fn dbesi1(x: *mut f64) -> f64;
        #[link_name = "dbesj0_"]
        pub fn dbesj0(x: *mut f64) -> f64;
        #[link_name = "dbesj1_"]
        pub fn dbesj1(x: *mut f64) -> f64;
        #[link_name = "dbesk0_"]
        pub fn dbesk0(x: *mut f64) -> f64;
        #[link_name = "dbesk1_"]
        pub fn dbesk1(x: *mut f64) -> f64;
        #[link_name = "dbesy0_"]
        pub fn dbesy0(x: *mut f64) -> f64;
        #[link_name = "dbesy1_"]
        pub fn dbesy1(x: *mut f64) -> f64;
        #[link_name = "dbsi0e_"]
        pub fn dbsi0e(x: *mut f64) -> f64;
        #[link_name = "dbsi1e_"]
        pub fn dbsi1e(x: *mut f64) -> f64;
        #[link_name = "dbsk0e_"]
        pub fn dbsk0e(x: *mut f64) -> f64;
        #[link_name = "dbsk1e_"]
        pub fn dbsk1e(x: *mut f64) -> f64;
    }
}

/// Reviewed declarations required by `special-beta`.
#[cfg(feature = "raw-family-special-beta")]
pub mod special_beta {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "albeta_"]
        pub fn albeta(a: *mut f32, b: *mut f32) -> f32;
        #[link_name = "beta_"]
        pub fn beta(a: *mut f32, b: *mut f32) -> f32;
        #[link_name = "betai_"]
        pub fn betai(x: *mut f32, pin: *mut f32, qin: *mut f32) -> f32;
        #[link_name = "dbeta_"]
        pub fn dbeta(a: *mut f64, b: *mut f64) -> f64;
        #[link_name = "dbetai_"]
        pub fn dbetai(x: *mut f64, pin: *mut f64, qin: *mut f64) -> f64;
        #[link_name = "dlbeta_"]
        pub fn dlbeta(a: *mut f64, b: *mut f64) -> f64;
    }
}

/// Reviewed declarations required by `special-elementary`.
#[cfg(feature = "raw-family-special-elementary")]
pub mod special_elementary {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "alnrel_"]
        pub fn alnrel(x: *mut f32) -> f32;
        #[link_name = "cbrt_"]
        pub fn cbrt(x: *mut f32) -> f32;
        #[link_name = "cosdg_"]
        pub fn cosdg(x: *mut f32) -> f32;
        #[link_name = "daws_"]
        pub fn daws(x: *mut f32) -> f32;
        #[link_name = "dcbrt_"]
        pub fn dcbrt(x: *mut f64) -> f64;
        #[link_name = "dcosdg_"]
        pub fn dcosdg(x: *mut f64) -> f64;
        #[link_name = "ddaws_"]
        pub fn ddaws(x: *mut f64) -> f64;
        #[link_name = "dexprl_"]
        pub fn dexprl(x: *mut f64) -> f64;
        #[link_name = "dlnrel_"]
        pub fn dlnrel(x: *mut f64) -> f64;
        #[link_name = "dsindg_"]
        pub fn dsindg(x: *mut f64) -> f64;
        #[link_name = "exprel_"]
        pub fn exprel(x: *mut f32) -> f32;
        #[link_name = "sindg_"]
        pub fn sindg(x: *mut f32) -> f32;
    }
}

/// Reviewed declarations required by `special-error`.
#[cfg(feature = "raw-family-special-error")]
pub mod special_error {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "derf_"]
        pub fn derf(x: *mut f64) -> f64;
        #[link_name = "derfc_"]
        pub fn derfc(x: *mut f64) -> f64;
        #[link_name = "erf_"]
        pub fn erf(x: *mut f32) -> f32;
        #[link_name = "erfc_"]
        pub fn erfc(x: *mut f32) -> f32;
    }
}

/// Reviewed declarations required by `special-gamma`.
#[cfg(feature = "raw-family-special-gamma")]
pub mod special_gamma {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "alngam_"]
        pub fn alngam(x: *mut f32) -> f32;
        #[link_name = "binom_"]
        pub fn binom(n: *mut FortranInteger, m: *mut FortranInteger) -> f32;
        #[link_name = "dbinom_"]
        pub fn dbinom(n: *mut FortranInteger, m: *mut FortranInteger) -> f64;
        #[link_name = "dfac_"]
        pub fn dfac(n: *mut FortranInteger) -> f64;
        #[link_name = "dgami_"]
        pub fn dgami(a: *mut f64, x: *mut f64) -> f64;
        #[link_name = "dgamic_"]
        pub fn dgamic(a: *mut f64, x: *mut f64) -> f64;
        #[link_name = "dgamit_"]
        pub fn dgamit(a: *mut f64, x: *mut f64) -> f64;
        #[link_name = "dgamma_"]
        pub fn dgamma(x: *mut f64) -> f64;
        #[link_name = "dgamr_"]
        pub fn dgamr(x: *mut f64) -> f64;
        #[link_name = "dlngam_"]
        pub fn dlngam(x: *mut f64) -> f64;
        #[link_name = "dpsi_"]
        pub fn dpsi(x: *mut f64) -> f64;
        #[link_name = "fac_"]
        pub fn fac(n: *mut FortranInteger) -> f32;
        #[link_name = "gami_"]
        pub fn gami(a: *mut f32, x: *mut f32) -> f32;
        #[link_name = "gamic_"]
        pub fn gamic(a: *mut f32, x: *mut f32) -> f32;
        #[link_name = "gamit_"]
        pub fn gamit(a: *mut f32, x: *mut f32) -> f32;
        #[link_name = "gamma_"]
        pub fn gamma(x: *mut f32) -> f32;
        #[link_name = "gamr_"]
        pub fn gamr(x: *mut f32) -> f32;
        #[link_name = "psi_"]
        pub fn psi(x: *mut f32) -> f32;
    }
}

/// Reviewed declarations required by `special-integrals`.
#[cfg(feature = "raw-family-special-integrals")]
pub mod special_integrals {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "de1_"]
        pub fn de1(x: *mut f64) -> f64;
        #[link_name = "dei_"]
        pub fn dei(x: *mut f64) -> f64;
        #[link_name = "e1_"]
        pub fn e1(x: *mut f32) -> f32;
        #[link_name = "ei_"]
        pub fn ei(x: *mut f32) -> f32;
    }
}

/// Reviewed declarations required by `special-polynomials`.
#[cfg(feature = "raw-family-special-polynomials")]
pub mod special_polynomials {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "csevl_"]
        pub fn csevl(x: *mut f32, cs: *mut f32, n: *mut FortranInteger) -> f32;
        #[link_name = "dcsevl_"]
        pub fn dcsevl(x: *mut f64, cs: *mut f64, n: *mut FortranInteger) -> f64;
    }
}
