//! Generated family-scoped raw declarations.
//!
//! Regenerate with `slatec-corpus generate-linkage-metadata --offline`.
#![allow(clippy::missing_safety_doc, unused_imports)]

/// Private declaration forwarding for the canonical ABI-sensitive blas namespace.
#[cfg(feature = "raw-family-blas-complex")]
pub mod blas_complex {
    pub use crate::abi_bindings::blas::*;
}

/// Private declaration forwarding for the canonical reviewed BLAS Level 1 namespace.
#[cfg(feature = "raw-family-blas-level1")]
pub mod blas_level1 {
    pub use crate::blas::level1::*;
}

/// Private declaration forwarding for the canonical reviewed BLAS Level 2 namespace.
#[cfg(feature = "raw-family-blas-level2")]
pub mod blas_level2 {
    pub use crate::blas::level2::*;
}

/// Private declaration forwarding for the canonical reviewed BLAS Level 3 namespace.
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
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ai.md"))]
        pub fn ai(x: *mut f32) -> f32;
        #[link_name = "aie_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/aie.md"))]
        pub fn aie(x: *mut f32) -> f32;
        #[link_name = "bi_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bi.md"))]
        pub fn bi(x: *mut f32) -> f32;
        #[link_name = "bie_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bie.md"))]
        pub fn bie(x: *mut f32) -> f32;
        #[link_name = "dai_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dai.md"))]
        pub fn dai(x: *mut f64) -> f64;
        #[link_name = "daie_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/daie.md"))]
        pub fn daie(x: *mut f64) -> f64;
        #[link_name = "dbi_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbi.md"))]
        pub fn dbi(x: *mut f64) -> f64;
        #[link_name = "dbie_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbie.md"))]
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
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi0.md"))]
        pub fn besi0(x: *mut f32) -> f32;
        #[link_name = "besi0e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi0e.md"))]
        pub fn besi0e(x: *mut f32) -> f32;
        #[link_name = "besi1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi1.md"))]
        pub fn besi1(x: *mut f32) -> f32;
        #[link_name = "besi1e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi1e.md"))]
        pub fn besi1e(x: *mut f32) -> f32;
        #[link_name = "besj0_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besj0.md"))]
        pub fn besj0(x: *mut f32) -> f32;
        #[link_name = "besj1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besj1.md"))]
        pub fn besj1(x: *mut f32) -> f32;
        #[link_name = "besk0_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk0.md"))]
        pub fn besk0(x: *mut f32) -> f32;
        #[link_name = "besk0e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk0e.md"))]
        pub fn besk0e(x: *mut f32) -> f32;
        #[link_name = "besk1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk1.md"))]
        pub fn besk1(x: *mut f32) -> f32;
        #[link_name = "besk1e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk1e.md"))]
        pub fn besk1e(x: *mut f32) -> f32;
        #[link_name = "besy0_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besy0.md"))]
        pub fn besy0(x: *mut f32) -> f32;
        #[link_name = "besy1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besy1.md"))]
        pub fn besy1(x: *mut f32) -> f32;
        #[link_name = "dbesi0_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesi0.md"))]
        pub fn dbesi0(x: *mut f64) -> f64;
        #[link_name = "dbesi1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesi1.md"))]
        pub fn dbesi1(x: *mut f64) -> f64;
        #[link_name = "dbesj0_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesj0.md"))]
        pub fn dbesj0(x: *mut f64) -> f64;
        #[link_name = "dbesj1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesj1.md"))]
        pub fn dbesj1(x: *mut f64) -> f64;
        #[link_name = "dbesk0_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesk0.md"))]
        pub fn dbesk0(x: *mut f64) -> f64;
        #[link_name = "dbesk1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesk1.md"))]
        pub fn dbesk1(x: *mut f64) -> f64;
        #[link_name = "dbesy0_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesy0.md"))]
        pub fn dbesy0(x: *mut f64) -> f64;
        #[link_name = "dbesy1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesy1.md"))]
        pub fn dbesy1(x: *mut f64) -> f64;
        #[link_name = "dbsi0e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsi0e.md"))]
        pub fn dbsi0e(x: *mut f64) -> f64;
        #[link_name = "dbsi1e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsi1e.md"))]
        pub fn dbsi1e(x: *mut f64) -> f64;
        #[link_name = "dbsk0e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsk0e.md"))]
        pub fn dbsk0e(x: *mut f64) -> f64;
        #[link_name = "dbsk1e_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsk1e.md"))]
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
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/albeta.md"))]
        pub fn albeta(a: *mut f32, b: *mut f32) -> f32;
        #[link_name = "beta_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/beta.md"))]
        pub fn beta(a: *mut f32, b: *mut f32) -> f32;
        #[link_name = "betai_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/betai.md"))]
        pub fn betai(x: *mut f32, pin: *mut f32, qin: *mut f32) -> f32;
        #[link_name = "dbeta_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbeta.md"))]
        pub fn dbeta(a: *mut f64, b: *mut f64) -> f64;
        #[link_name = "dbetai_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbetai.md"))]
        pub fn dbetai(x: *mut f64, pin: *mut f64, qin: *mut f64) -> f64;
        #[link_name = "dlbeta_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dlbeta.md"))]
        pub fn dlbeta(a: *mut f64, b: *mut f64) -> f64;
    }
}

/// Private declaration forwarding for the canonical ABI-sensitive special namespace.
#[cfg(feature = "raw-family-special-complex")]
pub mod special_complex {
    pub use crate::abi_bindings::special::*;
}

/// Reviewed declarations required by `special-elementary`.
#[cfg(feature = "raw-family-special-elementary")]
pub mod special_elementary {
    use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
    use core::ffi::c_char;

    unsafe extern "C" {
        #[link_name = "alnrel_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/alnrel.md"))]
        pub fn alnrel(x: *mut f32) -> f32;
        #[link_name = "cbrt_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbrt.md"))]
        pub fn cbrt(x: *mut f32) -> f32;
        #[link_name = "cosdg_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cosdg.md"))]
        pub fn cosdg(x: *mut f32) -> f32;
        #[link_name = "daws_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/daws.md"))]
        pub fn daws(x: *mut f32) -> f32;
        #[link_name = "dcbrt_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcbrt.md"))]
        pub fn dcbrt(x: *mut f64) -> f64;
        #[link_name = "dcosdg_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcosdg.md"))]
        pub fn dcosdg(x: *mut f64) -> f64;
        #[link_name = "ddaws_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ddaws.md"))]
        pub fn ddaws(x: *mut f64) -> f64;
        #[link_name = "dexprl_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dexprl.md"))]
        pub fn dexprl(x: *mut f64) -> f64;
        #[link_name = "dlnrel_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dlnrel.md"))]
        pub fn dlnrel(x: *mut f64) -> f64;
        #[link_name = "dsindg_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsindg.md"))]
        pub fn dsindg(x: *mut f64) -> f64;
        #[link_name = "exprel_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/exprel.md"))]
        pub fn exprel(x: *mut f32) -> f32;
        #[link_name = "sindg_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sindg.md"))]
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
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/derf.md"))]
        pub fn derf(x: *mut f64) -> f64;
        #[link_name = "derfc_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/derfc.md"))]
        pub fn derfc(x: *mut f64) -> f64;
        #[link_name = "erf_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/erf.md"))]
        pub fn erf(x: *mut f32) -> f32;
        #[link_name = "erfc_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/erfc.md"))]
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
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/alngam.md"))]
        pub fn alngam(x: *mut f32) -> f32;
        #[link_name = "binom_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/binom.md"))]
        pub fn binom(n: *mut FortranInteger, m: *mut FortranInteger) -> f32;
        #[link_name = "dbinom_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbinom.md"))]
        pub fn dbinom(n: *mut FortranInteger, m: *mut FortranInteger) -> f64;
        #[link_name = "dfac_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dfac.md"))]
        pub fn dfac(n: *mut FortranInteger) -> f64;
        #[link_name = "dgami_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgami.md"))]
        pub fn dgami(a: *mut f64, x: *mut f64) -> f64;
        #[link_name = "dgamic_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamic.md"))]
        pub fn dgamic(a: *mut f64, x: *mut f64) -> f64;
        #[link_name = "dgamit_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamit.md"))]
        pub fn dgamit(a: *mut f64, x: *mut f64) -> f64;
        #[link_name = "dgamma_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamma.md"))]
        pub fn dgamma(x: *mut f64) -> f64;
        #[link_name = "dgamr_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamr.md"))]
        pub fn dgamr(x: *mut f64) -> f64;
        #[link_name = "dlngam_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dlngam.md"))]
        pub fn dlngam(x: *mut f64) -> f64;
        #[link_name = "dpsi_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpsi.md"))]
        pub fn dpsi(x: *mut f64) -> f64;
        #[link_name = "fac_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/fac.md"))]
        pub fn fac(n: *mut FortranInteger) -> f32;
        #[link_name = "gami_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gami.md"))]
        pub fn gami(a: *mut f32, x: *mut f32) -> f32;
        #[link_name = "gamic_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamic.md"))]
        pub fn gamic(a: *mut f32, x: *mut f32) -> f32;
        #[link_name = "gamit_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamit.md"))]
        pub fn gamit(a: *mut f32, x: *mut f32) -> f32;
        #[link_name = "gamma_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamma.md"))]
        pub fn gamma(x: *mut f32) -> f32;
        #[link_name = "gamr_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamr.md"))]
        pub fn gamr(x: *mut f32) -> f32;
        #[link_name = "psi_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/psi.md"))]
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
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/de1.md"))]
        pub fn de1(x: *mut f64) -> f64;
        #[link_name = "dei_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dei.md"))]
        pub fn dei(x: *mut f64) -> f64;
        #[link_name = "e1_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/e1.md"))]
        pub fn e1(x: *mut f32) -> f32;
        #[link_name = "ei_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ei.md"))]
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
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csevl.md"))]
        pub fn csevl(x: *mut f32, cs: *mut f32, n: *mut FortranInteger) -> f32;
        #[link_name = "dcsevl_"]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcsevl.md"))]
        pub fn dcsevl(x: *mut f64, cs: *mut f64, n: *mut FortranInteger) -> f64;
    }
}
