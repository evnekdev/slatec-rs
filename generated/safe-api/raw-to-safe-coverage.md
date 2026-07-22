# Raw-to-safe coverage reconciliation

Every canonical public raw routine has exactly one safe-coverage disposition. A raw path remains public independently of whether a safe wrapper is appropriate. Dense, banded, packed, sparse, and eigenproblem safe APIs are intentionally left to the established Rust numerical ecosystem; this policy does not remove the reviewed `slatec-sys` path.

## Counts

- `blocked-by-safe-design`: 2
- `covered-by-higher-level-safe-api`: 49
- `direct-safe-wrapper`: 242
- `expert-raw-only`: 214
- `intentionally-excluded-external-ecosystem`: 314

## Canonical raw routine records

| Raw routine | Canonical raw path | Family | Safe path | Coverage | Feature | Higher abstraction | Disposition | Priority |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `ACOSH` | `slatec_sys::special::acosh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `AI` | `slatec_sys::special::airy::ai` | Special functions | slatec::special::airy::airy_ai_f32 | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `AIE` | `slatec_sys::special::airy::aie` | Special functions | slatec::special::airy::airy_ai_scaled_f32 | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ALBETA` | `slatec_sys::special::beta::albeta` | Special functions | slatec::special::gamma::log_beta_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ALGAMS` | `slatec_sys::special::algams` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ALI` | `slatec_sys::special::ali` | Special functions | slatec::special::scalar_expanded::logarithmic_integral_f32 | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ALNGAM` | `slatec_sys::special::gamma::alngam` | Special functions | slatec::special::gamma::log_gamma_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ALNREL` | `slatec_sys::special::elementary::alnrel` | Elementary and transcendental functions | slatec::special::elementary::log1p_f32 | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ASINH` | `slatec_sys::special::asinh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ATANH` | `slatec_sys::special::atanh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `AVINT` | `slatec_sys::quadrature::avint` | Numerical quadrature | slatec::quadrature::integrate_tabulated_f32 | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BAKVEC` | `slatec_sys::linear_algebra::eigen::bakvec` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BALANC` | `slatec_sys::linear_algebra::eigen::balanc` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BALBAK` | `slatec_sys::linear_algebra::eigen::balbak` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BANDR` | `slatec_sys::linear_algebra::eigen::bandr` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BANDV` | `slatec_sys::linear_algebra::eigen::bandv` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BESI` | `slatec_sys::special::bessel::besi` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BESI0` | `slatec_sys::special::bessel::besi0` | Special functions | slatec::special::bessel::bessel_i0_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESI0E` | `slatec_sys::special::bessel::besi0e` | Special functions | slatec::special::bessel::bessel_i0_scaled_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESI1` | `slatec_sys::special::bessel::besi1` | Special functions | slatec::special::bessel::bessel_i1_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESI1E` | `slatec_sys::special::bessel::besi1e` | Special functions | slatec::special::bessel::bessel_i1_scaled_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESJ` | `slatec_sys::special::bessel::besj` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BESJ0` | `slatec_sys::special::bessel::besj0` | Special functions | slatec::special::bessel::bessel_j0_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESJ1` | `slatec_sys::special::bessel::besj1` | Special functions | slatec::special::bessel::bessel_j1_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESK` | `slatec_sys::special::bessel::besk` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BESK0` | `slatec_sys::special::bessel::besk0` | Special functions | slatec::special::bessel::bessel_k0_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESK0E` | `slatec_sys::special::bessel::besk0e` | Special functions | slatec::special::bessel::bessel_k0_scaled_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESK1` | `slatec_sys::special::bessel::besk1` | Special functions | slatec::special::bessel::bessel_k1_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESK1E` | `slatec_sys::special::bessel::besk1e` | Special functions | slatec::special::bessel::bessel_k1_scaled_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESKES` | `slatec_sys::special::bessel::beskes` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BESKS` | `slatec_sys::special::bessel::besks` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BESY` | `slatec_sys::special::bessel::besy` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BESY0` | `slatec_sys::special::bessel::besy0` | Special functions | slatec::special::bessel::bessel_y0_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BESY1` | `slatec_sys::special::bessel::besy1` | Special functions | slatec::special::bessel::bessel_y1_f32 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BETA` | `slatec_sys::special::beta::beta` | Special functions | slatec::special::gamma::beta_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BETAI` | `slatec_sys::special::beta::betai` | Special functions | slatec::special::gamma::regularized_beta_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BI` | `slatec_sys::special::airy::bi` | Special functions | slatec::special::airy::airy_bi_f32 | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BIE` | `slatec_sys::special::airy::bie` | Special functions | slatec::special::airy::airy_bi_scaled_f32 | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BINOM` | `slatec_sys::special::gamma::binom` | Special functions | slatec::special::gamma::binomial_coefficient_f32 | `direct-safe-wrapper` | special-gamma | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BINT4` | `slatec_sys::interpolation::bint4` | Interpolation | slatec::interpolation::bspline::BSpline::interpolate_cubic | `direct-safe-wrapper` | bspline-cubic-interpolation | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BINTK` | `slatec_sys::interpolation::bintk` | Interpolation | slatec::interpolation::bspline::BSpline::interpolate_with_knots | `direct-safe-wrapper` | bspline | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BISECT` | `slatec_sys::linear_algebra::eigen::bisect` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BLKTRI` | `slatec_sys::pde::fishpack::blktri` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BNDACC` | `slatec_sys::linear_algebra::banded::bndacc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BNDSOL` | `slatec_sys::linear_algebra::banded::bndsol` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BQR` | `slatec_sys::linear_algebra::eigen::bqr` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `BSKIN` | `slatec_sys::special::bskin` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `BSPDR` | `slatec_sys::interpolation::bspdr` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `BSPEV` | `slatec_sys::interpolation::bspev` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `BSPPP` | `slatec_sys::interpolation::bsppp` | Interpolation | slatec::interpolation::bspline::BSpline::to_piecewise_polynomial | `direct-safe-wrapper` | piecewise-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BSPVD` | `slatec_sys::interpolation::bspvd` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `BSPVN` | `slatec_sys::interpolation::bspvn` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `BSQAD` | `slatec_sys::quadrature::bsqad` | Numerical quadrature | slatec::interpolation::bspline::BSpline::integrate | `direct-safe-wrapper` | bspline | — | `maintain_checked_safe_wrapper` | `maintain` |
| `BVALU` | `slatec_sys::interpolation::bvalu` | Interpolation | slatec::interpolation::bspline::BSpline::derivative; slatec::interpolation::bspline::BSpline::evaluate; slatec::interpolation::bspline::BSpline::evaluate_into | `direct-safe-wrapper` | bspline | — | `maintain_checked_safe_wrapper` | `maintain` |
| `C0LGMC` | `slatec_sys::special::complex::c0lgmc` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CACOS` | `slatec_sys::special::complex::cacos` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CACOSH` | `slatec_sys::special::complex::cacosh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CAIRY` | `slatec_sys::special::complex::cairy` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CASIN` | `slatec_sys::special::complex::casin` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CASINH` | `slatec_sys::special::complex::casinh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CATAN` | `slatec_sys::special::complex::catan` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CATAN2` | `slatec_sys::special::complex::catan2` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CATANH` | `slatec_sys::special::complex::catanh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CAXPY` | `slatec_sys::blas::level1::caxpy` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBABK2` | `slatec_sys::linear_algebra::eigen::cbabk2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CBAL` | `slatec_sys::linear_algebra::eigen::cbal` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CBESH` | `slatec_sys::special::complex::cbesh` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBESI` | `slatec_sys::special::complex::cbesi` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBESJ` | `slatec_sys::special::complex::cbesj` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBESK` | `slatec_sys::special::complex::cbesk` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBESY` | `slatec_sys::special::complex::cbesy` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBETA` | `slatec_sys::special::complex::cbeta` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBIRY` | `slatec_sys::special::complex::cbiry` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBLKTR` | `slatec_sys::pde::fishpack::complex::cblktr` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CBRT` | `slatec_sys::special::elementary::cbrt` | Special functions | slatec::special::elementary::cbrt_f32 | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CCBRT` | `slatec_sys::special::complex::ccbrt` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CCHDC` | `slatec_sys::linear_algebra::dense::complex::cchdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CCHDD` | `slatec_sys::linear_algebra::dense::complex::cchdd` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CCHEX` | `slatec_sys::linear_algebra::dense::complex::cchex` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CCHUD` | `slatec_sys::linear_algebra::dense::complex::cchud` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CCOPY` | `slatec_sys::blas::level1::ccopy` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CCOSH` | `slatec_sys::special::complex::ccosh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CCOT` | `slatec_sys::special::complex::ccot` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CDCDOT` | `slatec_sys::blas::level1::cdcdot` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CDOTC` | `slatec_sys::blas::level1::cdotc` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CDOTU` | `slatec_sys::blas::level1::cdotu` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CDRIV1` | `slatec_sys::ode::cdriv1` | ODE solvers | slatec::ode::ComplexDriv1Session::integrate_to | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CDRIV2` | `slatec_sys::ode::cdriv2` | ODE solvers | slatec::ode::ComplexDriv2Session::integrate_to_with_events | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CEXPRL` | `slatec_sys::special::complex::cexprl` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CFFTB1` | `slatec_sys::fftpack::cfftb1` | FFTPACK transforms | slatec::transforms::fft::complex::ComplexFftPlan32::backward | `direct-safe-wrapper` | fftpack-complex | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CFFTF1` | `slatec_sys::fftpack::cfftf1` | FFTPACK transforms | slatec::transforms::fft::complex::ComplexFftPlan32::forward | `direct-safe-wrapper` | fftpack-complex | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CFFTI1` | `slatec_sys::fftpack::cffti1` | FFTPACK transforms | slatec::transforms::fft::complex::ComplexFftPlan32::new | `direct-safe-wrapper` | fftpack-complex | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CG` | `slatec_sys::linear_algebra::eigen::cg` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGAMMA` | `slatec_sys::special::complex::cgamma` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CGAMR` | `slatec_sys::special::complex::cgamr` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CGBCO` | `slatec_sys::linear_algebra::banded::complex::cgbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGBDI` | `slatec_sys::linear_algebra::banded::complex::cgbdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGBFA` | `slatec_sys::linear_algebra::banded::complex::cgbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGBMV` | `slatec_sys::blas::level2::cgbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CGBSL` | `slatec_sys::linear_algebra::banded::complex::cgbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGECO` | `slatec_sys::linear_algebra::dense::complex::cgeco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGEDI` | `slatec_sys::linear_algebra::dense::complex::cgedi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGEEV` | `slatec_sys::linear_algebra::eigen::cgeev` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGEFA` | `slatec_sys::linear_algebra::dense::complex::cgefa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGEFS` | `slatec_sys::linear_algebra::dense::complex::cgefs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGEIR` | `slatec_sys::linear_algebra::dense::complex::cgeir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGEMM` | `slatec_sys::blas::level3::cgemm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CGEMV` | `slatec_sys::blas::level2::cgemv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CGERC` | `slatec_sys::blas::level2::cgerc` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CGERU` | `slatec_sys::blas::level2::cgeru` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CGESL` | `slatec_sys::linear_algebra::dense::complex::cgesl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CGTSL` | `slatec_sys::linear_algebra::banded::complex::cgtsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CH` | `slatec_sys::linear_algebra::eigen::ch` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHBMV` | `slatec_sys::blas::level2::chbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHEMM` | `slatec_sys::blas::level3::chemm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHEMV` | `slatec_sys::blas::level2::chemv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHER` | `slatec_sys::blas::level2::cher` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHER2` | `slatec_sys::blas::level2::cher2` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHER2K` | `slatec_sys::blas::level3::cher2k` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHERK` | `slatec_sys::blas::level3::cherk` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHFDV` | `slatec_sys::interpolation::chfdv` | PCHIP | — | `covered-by-higher-level-safe-api` | — | slatec::pchip::PiecewiseCubicHermite | `maintain_higher_level_abstraction` | `maintain` |
| `CHFEV` | `slatec_sys::interpolation::chfev` | PCHIP | — | `covered-by-higher-level-safe-api` | — | slatec::pchip::PiecewiseCubicHermite | `maintain_higher_level_abstraction` | `maintain` |
| `CHICO` | `slatec_sys::linear_algebra::dense::complex::chico` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHIDI` | `slatec_sys::linear_algebra::dense::complex::chidi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHIEV` | `slatec_sys::linear_algebra::eigen::chiev` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHIFA` | `slatec_sys::linear_algebra::dense::complex::chifa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHISL` | `slatec_sys::linear_algebra::dense::complex::chisl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHKDER` | `slatec_sys::nonlinear::jacobian_check::chkder` | Nonlinear equations | slatec::nonlinear::check_jacobian_f32 | `direct-safe-wrapper` | nonlinear-jacobian-check | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CHPCO` | `slatec_sys::linear_algebra::packed::complex::chpco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHPDI` | `slatec_sys::linear_algebra::packed::complex::chpdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHPFA` | `slatec_sys::linear_algebra::packed::complex::chpfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHPMV` | `slatec_sys::blas::level2::chpmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHPR` | `slatec_sys::blas::level2::chpr` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHPR2` | `slatec_sys::blas::level2::chpr2` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CHPSL` | `slatec_sys::linear_algebra::packed::complex::chpsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CHU` | `slatec_sys::special::chu` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CINVIT` | `slatec_sys::linear_algebra::eigen::cinvit` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CLBETA` | `slatec_sys::special::complex::clbeta` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CLNGAM` | `slatec_sys::special::complex::clngam` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CLNREL` | `slatec_sys::special::complex::clnrel` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CLOG10` | `slatec_sys::special::complex::clog10` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CMGNBN` | `slatec_sys::pde::fishpack::complex::cmgnbn` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CNBCO` | `slatec_sys::linear_algebra::banded::complex::cnbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CNBDI` | `slatec_sys::linear_algebra::banded::complex::cnbdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CNBFA` | `slatec_sys::linear_algebra::banded::complex::cnbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CNBFS` | `slatec_sys::linear_algebra::banded::complex::cnbfs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CNBIR` | `slatec_sys::linear_algebra::banded::complex::cnbir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CNBSL` | `slatec_sys::linear_algebra::banded::complex::cnbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `COMBAK` | `slatec_sys::linear_algebra::eigen::combak` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `COMHES` | `slatec_sys::linear_algebra::eigen::comhes` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `COMLR` | `slatec_sys::linear_algebra::eigen::comlr` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `COMLR2` | `slatec_sys::linear_algebra::eigen::comlr2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `COMQR` | `slatec_sys::linear_algebra::eigen::comqr` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `COMQR2` | `slatec_sys::linear_algebra::eigen::comqr2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CORTB` | `slatec_sys::linear_algebra::eigen::cortb` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CORTH` | `slatec_sys::linear_algebra::eigen::corth` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `COSDG` | `slatec_sys::special::elementary::cosdg` | Elementary and transcendental functions | slatec::special::elementary::cos_degrees_f32 | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `COSQB` | `slatec_sys::fftpack::cosqb` | FFTPACK transforms | slatec::fftpack::QuarterWaveCosinePlan::backward | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `COSQF` | `slatec_sys::fftpack::cosqf` | FFTPACK transforms | slatec::fftpack::QuarterWaveCosinePlan::forward | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `COSQI` | `slatec_sys::fftpack::cosqi` | FFTPACK transforms | slatec::fftpack::QuarterWaveCosinePlan::new | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `COST` | `slatec_sys::fftpack::cost` | FFTPACK transforms | slatec::fftpack::CosineTransformPlan::transform | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `COSTI` | `slatec_sys::fftpack::costi` | FFTPACK transforms | slatec::fftpack::CosineTransformPlan::new | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `COT` | `slatec_sys::special::cot` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CPBCO` | `slatec_sys::linear_algebra::banded::complex::cpbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPBDI` | `slatec_sys::linear_algebra::banded::complex::cpbdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPBFA` | `slatec_sys::linear_algebra::banded::complex::cpbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPBSL` | `slatec_sys::linear_algebra::banded::complex::cpbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPOCO` | `slatec_sys::linear_algebra::dense::complex::cpoco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPODI` | `slatec_sys::linear_algebra::dense::complex::cpodi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPOFA` | `slatec_sys::linear_algebra::dense::complex::cpofa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPOFS` | `slatec_sys::linear_algebra::dense::complex::cpofs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPOIR` | `slatec_sys::linear_algebra::dense::complex::cpoir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPOSL` | `slatec_sys::linear_algebra::dense::complex::cposl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPPCO` | `slatec_sys::linear_algebra::packed::complex::cppco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPPDI` | `slatec_sys::linear_algebra::packed::complex::cppdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPPFA` | `slatec_sys::linear_algebra::packed::complex::cppfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPPSL` | `slatec_sys::linear_algebra::packed::complex::cppsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPQR79` | `slatec_sys::roots::complex::cpqr79` | Nonlinear equations | slatec::roots::complex_polynomial_roots_with_method | `direct-safe-wrapper` | roots-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CPSI` | `slatec_sys::special::complex::cpsi` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CPTSL` | `slatec_sys::linear_algebra::banded::complex::cptsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CPZERO` | `slatec_sys::roots::complex::cpzero` | Nonlinear equations | slatec::roots::complex_polynomial_roots | `direct-safe-wrapper` | roots-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CQRDC` | `slatec_sys::linear_algebra::dense::complex::cqrdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CQRSL` | `slatec_sys::linear_algebra::dense::complex::cqrsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CROTG` | `slatec_sys::blas::level1::crotg` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSCAL` | `slatec_sys::blas::level1::cscal` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSEVL` | `slatec_sys::special::csevl` | Special functions | slatec::polynomials::chebyshev::chebyshev_series_f32 | `direct-safe-wrapper` | special-polynomials | — | `maintain_checked_safe_wrapper` | `maintain` |
| `CSICO` | `slatec_sys::linear_algebra::dense::complex::csico` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSIDI` | `slatec_sys::linear_algebra::dense::complex::csidi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSIFA` | `slatec_sys::linear_algebra::dense::complex::csifa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSINH` | `slatec_sys::special::complex::csinh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSISL` | `slatec_sys::linear_algebra::dense::complex::csisl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSPCO` | `slatec_sys::linear_algebra::packed::complex::cspco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSPDI` | `slatec_sys::linear_algebra::packed::complex::cspdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSPFA` | `slatec_sys::linear_algebra::packed::complex::cspfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSPSL` | `slatec_sys::linear_algebra::packed::complex::cspsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSROT` | `slatec_sys::blas::level1::csrot` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSSCAL` | `slatec_sys::blas::level1::csscal` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSVDC` | `slatec_sys::linear_algebra::dense::complex::csvdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CSWAP` | `slatec_sys::blas::level1::cswap` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSYMM` | `slatec_sys::blas::level3::csymm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSYR2K` | `slatec_sys::blas::level3::csyr2k` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CSYRK` | `slatec_sys::blas::level3::csyrk` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTAN` | `slatec_sys::special::complex::ctan` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTANH` | `slatec_sys::special::complex::ctanh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTBMV` | `slatec_sys::blas::level2::ctbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTBSV` | `slatec_sys::blas::level2::ctbsv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTPMV` | `slatec_sys::blas::level2::ctpmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTPSV` | `slatec_sys::blas::level2::ctpsv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTRCO` | `slatec_sys::linear_algebra::dense::complex::ctrco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CTRDI` | `slatec_sys::linear_algebra::dense::complex::ctrdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CTRMM` | `slatec_sys::blas::level3::ctrmm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTRMV` | `slatec_sys::blas::level2::ctrmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTRSL` | `slatec_sys::linear_algebra::dense::complex::ctrsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `CTRSM` | `slatec_sys::blas::level3::ctrsm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CTRSV` | `slatec_sys::blas::level2::ctrsv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `CV` | `slatec_sys::statistics::cv` | Probability and statistics | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DACOSH` | `slatec_sys::special::dacosh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DAI` | `slatec_sys::special::airy::dai` | Special functions | slatec::special::airy::airy_ai | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DAIE` | `slatec_sys::special::airy::daie` | Special functions | slatec::special::airy::airy_ai_scaled | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DASINH` | `slatec_sys::special::dasinh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DASUM` | `slatec_sys::blas::level1::dasum` | Linear algebra kernels | slatec::blas::level1::dasum; slatec::blas::level1::dasum_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DATANH` | `slatec_sys::special::datanh` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DAVINT` | `slatec_sys::quadrature::davint` | Numerical quadrature | slatec::quadrature::integrate_tabulated | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DAWS` | `slatec_sys::special::elementary::daws` | Special functions | slatec::special::elementary::dawson_f32 | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DAXPY` | `slatec_sys::blas::level1::daxpy` | Linear algebra kernels | slatec::blas::level1::daxpy; slatec::blas::level1::daxpy_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBCG` | `slatec_sys::linear_algebra::sparse::callbacks::dbcg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DBESI` | `slatec_sys::special::bessel::dbesi` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DBESI0` | `slatec_sys::special::bessel::dbesi0` | Special functions | slatec::special::bessel::bessel_i0 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBESI1` | `slatec_sys::special::bessel::dbesi1` | Special functions | slatec::special::bessel::bessel_i1 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBESJ` | `slatec_sys::special::bessel::dbesj` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DBESJ0` | `slatec_sys::special::bessel::dbesj0` | Special functions | slatec::special::bessel::bessel_j0 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBESJ1` | `slatec_sys::special::bessel::dbesj1` | Special functions | slatec::special::bessel::bessel_j1 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBESK` | `slatec_sys::special::bessel::dbesk` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DBESK0` | `slatec_sys::special::bessel::dbesk0` | Special functions | slatec::special::bessel::bessel_k0 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBESK1` | `slatec_sys::special::bessel::dbesk1` | Special functions | slatec::special::bessel::bessel_k1 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBESKS` | `slatec_sys::special::bessel::dbesks` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DBESY` | `slatec_sys::special::bessel::dbesy` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DBESY0` | `slatec_sys::special::bessel::dbesy0` | Special functions | slatec::special::bessel::bessel_y0 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBESY1` | `slatec_sys::special::bessel::dbesy1` | Special functions | slatec::special::bessel::bessel_y1 | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBETA` | `slatec_sys::special::beta::dbeta` | Special functions | slatec::special::gamma::beta | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBETAI` | `slatec_sys::special::beta::dbetai` | Special functions | slatec::special::gamma::regularized_beta | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBI` | `slatec_sys::special::airy::dbi` | Special functions | slatec::special::airy::airy_bi | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBIE` | `slatec_sys::special::airy::dbie` | Special functions | slatec::special::airy::airy_bi_scaled | `direct-safe-wrapper` | special-airy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBINOM` | `slatec_sys::special::gamma::dbinom` | Special functions | slatec::special::gamma::binomial_coefficient | `direct-safe-wrapper` | special-gamma | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBINT4` | `slatec_sys::interpolation::dbint4` | Interpolation | slatec::interpolation::bspline::BSpline::interpolate_cubic | `direct-safe-wrapper` | bspline-cubic-interpolation | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBINTK` | `slatec_sys::interpolation::dbintk` | Interpolation | slatec::interpolation::bspline::BSpline::interpolate_with_knots | `direct-safe-wrapper` | bspline | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBNDAC` | `slatec_sys::linear_algebra::banded::dbndac` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DBNDSL` | `slatec_sys::linear_algebra::banded::dbndsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DBOCLS` | `slatec_sys::approximation::dbocls` | Approximation | slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares | `direct-safe-wrapper` | least-squares-linear-bounded-constrained | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBOLS` | `slatec_sys::approximation::dbols` | Approximation | slatec::bounded_least_squares::solve_bounded_least_squares | `direct-safe-wrapper` | least-squares-linear-bounded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBSI0E` | `slatec_sys::special::dbsi0e` | Special functions | slatec::special::bessel::bessel_i0_scaled | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBSI1E` | `slatec_sys::special::dbsi1e` | Special functions | slatec::special::bessel::bessel_i1_scaled | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBSK0E` | `slatec_sys::special::dbsk0e` | Special functions | slatec::special::bessel::bessel_k0_scaled | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBSK1E` | `slatec_sys::special::dbsk1e` | Special functions | slatec::special::bessel::bessel_k1_scaled | `direct-safe-wrapper` | special-bessel | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBSKES` | `slatec_sys::special::dbskes` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DBSKIN` | `slatec_sys::special::dbskin` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DBSPDR` | `slatec_sys::interpolation::dbspdr` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `DBSPEV` | `slatec_sys::interpolation::dbspev` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `DBSPPP` | `slatec_sys::interpolation::dbsppp` | Interpolation | slatec::interpolation::bspline::BSpline::to_piecewise_polynomial | `direct-safe-wrapper` | piecewise-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBSPVD` | `slatec_sys::interpolation::dbspvd` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `DBSPVN` | `slatec_sys::interpolation::dbspvn` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `DBSQAD` | `slatec_sys::quadrature::dbsqad` | Numerical quadrature | slatec::interpolation::bspline::BSpline::integrate | `direct-safe-wrapper` | bspline | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DBVALU` | `slatec_sys::interpolation::dbvalu` | Interpolation | slatec::interpolation::bspline::BSpline::derivative; slatec::interpolation::bspline::BSpline::evaluate; slatec::interpolation::bspline::BSpline::evaluate_into | `direct-safe-wrapper` | bspline | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DCBRT` | `slatec_sys::special::elementary::dcbrt` | Special functions | slatec::special::elementary::cbrt | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DCDOT` | `slatec_sys::blas::level1::dcdot` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DCG` | `slatec_sys::linear_algebra::sparse::callbacks::dcg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DCGN` | `slatec_sys::linear_algebra::sparse::callbacks::dcgn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DCGS` | `slatec_sys::linear_algebra::sparse::callbacks::dcgs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DCHDC` | `slatec_sys::linear_algebra::dense::dchdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DCHDD` | `slatec_sys::linear_algebra::dense::dchdd` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DCHEX` | `slatec_sys::linear_algebra::dense::dchex` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DCHFDV` | `slatec_sys::interpolation::dchfdv` | PCHIP | — | `covered-by-higher-level-safe-api` | — | slatec::pchip::PiecewiseCubicHermite | `maintain_higher_level_abstraction` | `maintain` |
| `DCHFEV` | `slatec_sys::interpolation::dchfev` | PCHIP | — | `covered-by-higher-level-safe-api` | — | slatec::pchip::PiecewiseCubicHermite | `maintain_higher_level_abstraction` | `maintain` |
| `DCHU` | `slatec_sys::special::dchu` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DCHUD` | `slatec_sys::linear_algebra::dense::dchud` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DCKDER` | `slatec_sys::nonlinear::jacobian_check::dckder` | Nonlinear equations | slatec::nonlinear::check_jacobian | `direct-safe-wrapper` | nonlinear-jacobian-check | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DCOPY` | `slatec_sys::blas::level1::dcopy` | Linear algebra kernels | slatec::blas::level1::dcopy; slatec::blas::level1::dcopy_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DCOPYM` | `slatec_sys::blas::level1::dcopym` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DCOSDG` | `slatec_sys::special::elementary::dcosdg` | Elementary and transcendental functions | slatec::special::elementary::cos_degrees | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DCOT` | `slatec_sys::special::dcot` | Elementary and transcendental functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DCOV` | `slatec_sys::least_squares::dcov` | Approximation | slatec::least_squares::covariance_from_expert_fit; slatec::least_squares::estimate_covariance; slatec::least_squares::estimate_covariance_finite_difference | `direct-safe-wrapper` | least-squares-covariance; least-squares-covariance + least-squares-nonlinear-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DCSEVL` | `slatec_sys::special::dcsevl` | Special functions | slatec::polynomials::chebyshev::chebyshev_series | `direct-safe-wrapper` | special-polynomials | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DCV` | `slatec_sys::statistics::dcv` | Probability and statistics | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DDASSL` | `slatec_sys::dassl::ddassl` | ODE solvers | slatec::dassl::DaeSession::<f64, F>::advance_to | `direct-safe-wrapper` | dassl | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DDAWS` | `slatec_sys::special::elementary::ddaws` | Special functions | slatec::special::elementary::dawson | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DDEABM` | `slatec_sys::ode::callbacks::ddeabm` | ODE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DDERKF` | `slatec_sys::ode::callbacks::dderkf` | ODE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DDOT` | `slatec_sys::blas::level1::ddot` | Linear algebra kernels | slatec::blas::level1::ddot; slatec::blas::level1::ddot_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DDRIV1` | `slatec_sys::ode::ddriv1` | ODE solvers | slatec::ode::Driv1Session::<f64>::integrate_to | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DDRIV2` | `slatec_sys::ode::ddriv2` | ODE solvers | slatec::ode::Driv2Session::<f64>::integrate_to_with_events | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DDRIV3` | `slatec_sys::ode::ddriv3` | ODE solvers | slatec::ode::OdeSession::<f64, F, E>::integrate_to | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DE1` | `slatec_sys::special::de1` | Special functions | slatec::special::integrals::exponential_integral_e1 | `direct-safe-wrapper` | special-integrals | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DEFC` | `slatec_sys::approximation::defc` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DEI` | `slatec_sys::special::dei` | Special functions | slatec::special::integrals::exponential_integral_ei | `direct-safe-wrapper` | special-integrals | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DERF` | `slatec_sys::special::error::derf` | Special functions | slatec::special::error_functions::erf | `direct-safe-wrapper` | special-error | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DERFC` | `slatec_sys::special::error::derfc` | Special functions | slatec::special::error_functions::erfc | `direct-safe-wrapper` | special-error | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DERKF` | `slatec_sys::ode::callbacks::derkf` | ODE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DEXINT` | `slatec_sys::special::dexint` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DEXPRL` | `slatec_sys::special::elementary::dexprl` | Elementary and transcendental functions | slatec::special::elementary::exprel | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DFAC` | `slatec_sys::special::gamma::dfac` | Special functions | slatec::special::gamma::factorial | `direct-safe-wrapper` | special-gamma | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DFC` | `slatec_sys::approximation::dfc` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DFZERO` | `slatec_sys::roots::scalar::dfzero` | Nonlinear equations | slatec::roots::find_root | `direct-safe-wrapper` | roots-scalar | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGAMI` | `slatec_sys::special::gamma::dgami` | Special functions | slatec::special::gamma::incomplete_gamma_lower | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGAMIC` | `slatec_sys::special::gamma::dgamic` | Special functions | slatec::special::gamma::incomplete_gamma_upper | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGAMIT` | `slatec_sys::special::gamma::dgamit` | Special functions | slatec::special::gamma::tricomi_incomplete_gamma | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGAMLM` | `slatec_sys::special::dgamlm` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DGAMMA` | `slatec_sys::special::gamma::dgamma` | Special functions | slatec::special::gamma::gamma | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGAMR` | `slatec_sys::special::gamma::dgamr` | Special functions | slatec::special::gamma::reciprocal_gamma | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGAUS8` | `slatec_sys::quadrature::dgaus8` | Numerical quadrature | — | `blocked-by-safe-design` | — | existing QUADPACK options-based integration | `retain_raw_only` | `none` |
| `DGBCO` | `slatec_sys::linear_algebra::banded::dgbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGBDI` | `slatec_sys::linear_algebra::banded::dgbdi` | Dense linear algebra | slatec::linear_algebra::banded::BandLu64::scaled_determinant | `direct-safe-wrapper` | banded-linear-systems | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGBFA` | `slatec_sys::linear_algebra::banded::dgbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGBMV` | `slatec_sys::blas::level2::dgbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DGBSL` | `slatec_sys::linear_algebra::banded::dgbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGECO` | `slatec_sys::linear_algebra::dense::dgeco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGEDI` | `slatec_sys::linear_algebra::dense::dgedi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGEFA` | `slatec_sys::linear_algebra::dense::dgefa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGEFS` | `slatec_sys::linear_algebra::dense::dgefs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGEMM` | `slatec_sys::blas::level3::dgemm` | Linear algebra kernels | slatec::blas::level3::dgemm; slatec::blas::level3::dgemm_contiguous | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGEMV` | `slatec_sys::blas::level2::dgemv` | Linear algebra kernels | slatec::blas::level2::dgemv; slatec::blas::level2::dgemv_contiguous | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGER` | `slatec_sys::blas::level2::dger` | Linear algebra kernels | slatec::blas::level2::dger | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DGESL` | `slatec_sys::linear_algebra::dense::dgesl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGLSS` | `slatec_sys::linear_algebra::dense::dglss` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGMRES` | `slatec_sys::linear_algebra::sparse::callbacks::dgmres` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DGTSL` | `slatec_sys::linear_algebra::banded::dgtsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DHFTI` | `slatec_sys::linear_algebra::banded::dhfti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DINTP` | `slatec_sys::ode::dintp` | ODE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DINTRV` | `slatec_sys::interpolation::dintrv` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `DIR` | `slatec_sys::linear_algebra::sparse::callbacks::dir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DLBETA` | `slatec_sys::special::beta::dlbeta` | Special functions | slatec::special::gamma::log_beta | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DLGAMS` | `slatec_sys::special::dlgams` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DLI` | `slatec_sys::special::dli` | Special functions | slatec::special::scalar_expanded::logarithmic_integral | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DLLSIA` | `slatec_sys::linear_algebra::dense::dllsia` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DLLTI2` | `slatec_sys::linear_algebra::dense::dllti2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DLNGAM` | `slatec_sys::special::gamma::dlngam` | Special functions | slatec::special::gamma::log_gamma | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DLNREL` | `slatec_sys::special::elementary::dlnrel` | Elementary and transcendental functions | slatec::special::elementary::log1p | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DLSEI` | `slatec_sys::approximation::dlsei` | Approximation | slatec::constrained_least_squares::solve_constrained_least_squares | `direct-safe-wrapper` | least-squares-linear-constrained | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DNBCO` | `slatec_sys::linear_algebra::banded::dnbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DNBDI` | `slatec_sys::linear_algebra::banded::dnbdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DNBFA` | `slatec_sys::linear_algebra::banded::dnbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DNBFS` | `slatec_sys::linear_algebra::banded::dnbfs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DNBSL` | `slatec_sys::linear_algebra::banded::dnbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DNLS1` | `slatec_sys::least_squares::dnls1` | Approximation | slatec::least_squares::least_squares_expert; slatec::least_squares::least_squares_with_jacobian | `direct-safe-wrapper` | least-squares-nonlinear-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DNLS1E` | `slatec_sys::least_squares::dnls1e` | Approximation | slatec::least_squares::least_squares | `direct-safe-wrapper` | least-squares-nonlinear-easy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DNRM2` | `slatec_sys::blas::level1::dnrm2` | Linear algebra kernels | slatec::blas::level1::dnrm2; slatec::blas::level1::dnrm2_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DNSQ` | `slatec_sys::nonlinear::dnsq` | Nonlinear equations | slatec::nonlinear::solve_system_expert; slatec::nonlinear::solve_system_with_jacobian | `direct-safe-wrapper` | nonlinear-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DNSQE` | `slatec_sys::nonlinear::dnsqe` | Nonlinear equations | slatec::nonlinear::solve_system | `direct-safe-wrapper` | nonlinear-easy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DOMN` | `slatec_sys::linear_algebra::sparse::callbacks::domn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DP1VLU` | `slatec_sys::approximation::dp1vlu` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPBCO` | `slatec_sys::linear_algebra::banded::dpbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPBDI` | `slatec_sys::linear_algebra::banded::dpbdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPBFA` | `slatec_sys::linear_algebra::banded::dpbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPBSL` | `slatec_sys::linear_algebra::banded::dpbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPCHBS` | `slatec_sys::interpolation::dpchbs` | PCHIP | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPCHCM` | `slatec_sys::interpolation::dpchcm` | PCHIP | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPCHFD` | `slatec_sys::interpolation::dpchfd` | PCHIP | slatec::pchip::PiecewiseCubicHermite::evaluate_with_derivatives_into | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPCHFE` | `slatec_sys::interpolation::dpchfe` | PCHIP | slatec::pchip::PiecewiseCubicHermite::evaluate_into | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPCHIA` | `slatec_sys::interpolation::dpchia` | PCHIP | slatec::pchip::PiecewiseCubicHermite::integrate | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPCHIC` | `slatec_sys::interpolation::dpchic` | PCHIP | slatec::pchip::PiecewiseCubicHermite::monotone_with_conditions | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPCHID` | `slatec_sys::interpolation::dpchid` | PCHIP | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPCHIM` | `slatec_sys::interpolation::dpchim` | PCHIP | slatec::pchip::PiecewiseCubicHermite::monotone | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPCHSP` | `slatec_sys::interpolation::dpchsp` | PCHIP | slatec::pchip::PiecewiseCubicHermite::spline | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPCOEF` | `slatec_sys::approximation::dpcoef` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPFQAD` | `slatec_sys::quadrature::dpfqad` | Numerical quadrature | slatec::quadrature::integrate_piecewise_polynomial | `direct-safe-wrapper` | quadrature-piecewise-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPLINT` | `slatec_sys::interpolation::dplint` | Interpolation | slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPOCH` | `slatec_sys::special::dpoch` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPOCH1` | `slatec_sys::special::dpoch1` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPOCO` | `slatec_sys::linear_algebra::dense::dpoco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPODI` | `slatec_sys::linear_algebra::dense::dpodi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPOFA` | `slatec_sys::linear_algebra::dense::dpofa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPOFS` | `slatec_sys::linear_algebra::dense::dpofs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPOLCF` | `slatec_sys::interpolation::dpolcf` | Interpolation | slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPOLFT` | `slatec_sys::approximation::dpolft` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPOLVL` | `slatec_sys::interpolation::dpolvl` | Interpolation | slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate; slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate_with_derivatives | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPOSL` | `slatec_sys::linear_algebra::dense::dposl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPPCO` | `slatec_sys::linear_algebra::packed::dppco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPPDI` | `slatec_sys::linear_algebra::packed::dppdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPPFA` | `slatec_sys::linear_algebra::packed::dppfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPPQAD` | `slatec_sys::quadrature::dppqad` | Numerical quadrature | slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::integrate | `direct-safe-wrapper` | piecewise-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPPSL` | `slatec_sys::linear_algebra::packed::dppsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DPPVAL` | `slatec_sys::interpolation::dppval` | Interpolation | slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into | `direct-safe-wrapper` | piecewise-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPSI` | `slatec_sys::special::gamma::dpsi` | Special functions | slatec::special::gamma::digamma | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DPSIFN` | `slatec_sys::special::dpsifn` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DPTSL` | `slatec_sys::linear_algebra::banded::dptsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DQAG` | `slatec_sys::quadrature::dqag` | Numerical quadrature | slatec::quadrature::integrate | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAGI` | `slatec_sys::quadrature::dqagi` | Numerical quadrature | slatec::quadrature::integrate_infinite | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAGIE` | `slatec_sys::quadrature::callbacks::dqagie` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQAGP` | `slatec_sys::quadrature::dqagp` | Numerical quadrature | slatec::quadrature::integrate_with_breakpoints | `direct-safe-wrapper` | quadrature-breakpoints | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAGPE` | `slatec_sys::quadrature::callbacks::dqagpe` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQAGS` | `slatec_sys::quadrature::dqags` | Numerical quadrature | slatec::quadrature::integrate_singular | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAGSE` | `slatec_sys::quadrature::callbacks::dqagse` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQAWC` | `slatec_sys::quadrature::dqawc` | Numerical quadrature | slatec::quadrature::integrate_principal_value | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAWCE` | `slatec_sys::quadrature::callbacks::dqawce` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQAWF` | `slatec_sys::quadrature::dqawf` | Numerical quadrature | slatec::quadrature::integrate_fourier_tail | `direct-safe-wrapper` | quadrature-fourier | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAWO` | `slatec_sys::quadrature::dqawo` | Numerical quadrature | slatec::quadrature::integrate_oscillatory | `direct-safe-wrapper` | quadrature-oscillatory | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAWS` | `slatec_sys::quadrature::dqaws` | Numerical quadrature | slatec::quadrature::integrate_weighted_endpoints | `direct-safe-wrapper` | quadrature-weighted | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQAWSE` | `slatec_sys::quadrature::callbacks::dqawse` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQC25C` | `slatec_sys::quadrature::callbacks::dqc25c` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQC25S` | `slatec_sys::quadrature::callbacks::dqc25s` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQDOTA` | `slatec_sys::blas::level1::dqdota` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DQDOTI` | `slatec_sys::blas::level1::dqdoti` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DQK15` | `slatec_sys::quadrature::callbacks::dqk15` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQK15I` | `slatec_sys::quadrature::callbacks::dqk15i` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQK15W` | `slatec_sys::quadrature::callbacks::dqk15w` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQK21` | `slatec_sys::quadrature::callbacks::dqk21` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQK31` | `slatec_sys::quadrature::callbacks::dqk31` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQK41` | `slatec_sys::quadrature::callbacks::dqk41` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQK51` | `slatec_sys::quadrature::callbacks::dqk51` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQK61` | `slatec_sys::quadrature::callbacks::dqk61` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQMOMO` | `slatec_sys::quadrature::dqmomo` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `DQNC79` | `slatec_sys::quadrature::dqnc79` | Numerical quadrature | slatec::quadrature::integrate_nc79 | `direct-safe-wrapper` | quadrature-nonadaptive | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQNG` | `slatec_sys::quadrature::dqng` | Numerical quadrature | slatec::quadrature::integrate_non_adaptive | `direct-safe-wrapper` | quadrature-nonadaptive | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DQRDC` | `slatec_sys::linear_algebra::dense::dqrdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DQRSL` | `slatec_sys::linear_algebra::dense::dqrsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DRC` | `slatec_sys::special::drc` | Special functions | slatec::special::scalar_expanded::carlson_rc | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DRC3JJ` | `slatec_sys::special::drc3jj` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DRC3JM` | `slatec_sys::special::drc3jm` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DRC6J` | `slatec_sys::special::drc6j` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DRD` | `slatec_sys::special::drd` | Special functions | slatec::special::scalar_expanded::carlson_rd | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DRF` | `slatec_sys::special::drf` | Special functions | slatec::special::scalar_expanded::carlson_rf | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DRJ` | `slatec_sys::special::drj` | Special functions | slatec::special::scalar_expanded::carlson_rj | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DROT` | `slatec_sys::blas::level1::drot` | Linear algebra kernels | slatec::blas::level1::drot; slatec::blas::level1::drot_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DROTG` | `slatec_sys::blas::level1::drotg` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DROTM` | `slatec_sys::blas::level1::drotm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DROTMG` | `slatec_sys::blas::level1::drotmg` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DS2LT` | `slatec_sys::linear_algebra::dense::ds2lt` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DS2Y` | `slatec_sys::blas::level1::ds2y` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSBMV` | `slatec_sys::blas::level2::dsbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSCAL` | `slatec_sys::blas::level1::dscal` | Linear algebra kernels | slatec::blas::level1::dscal; slatec::blas::level1::dscal_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DSD2S` | `slatec_sys::linear_algebra::dense::dsd2s` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDBCG` | `slatec_sys::linear_algebra::sparse::dsdbcg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDCG` | `slatec_sys::linear_algebra::sparse::dsdcg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDCGN` | `slatec_sys::linear_algebra::sparse::dsdcgn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDCGS` | `slatec_sys::linear_algebra::sparse::dsdcgs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDGMR` | `slatec_sys::linear_algebra::sparse::dsdgmr` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDI` | `slatec_sys::blas::level1::dsdi` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSDOMN` | `slatec_sys::linear_algebra::sparse::dsdomn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDOT` | `slatec_sys::blas::level1::dsdot` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSDS` | `slatec_sys::linear_algebra::dense::dsds` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSDSCL` | `slatec_sys::linear_algebra::dense::dsdscl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSGS` | `slatec_sys::linear_algebra::sparse::dsgs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSICCG` | `slatec_sys::linear_algebra::sparse::dsiccg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSICO` | `slatec_sys::linear_algebra::dense::dsico` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSICS` | `slatec_sys::linear_algebra::dense::dsics` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSIDI` | `slatec_sys::linear_algebra::dense::dsidi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSIFA` | `slatec_sys::linear_algebra::dense::dsifa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSILUR` | `slatec_sys::linear_algebra::sparse::dsilur` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSILUS` | `slatec_sys::linear_algebra::dense::dsilus` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSINDG` | `slatec_sys::special::elementary::dsindg` | Elementary and transcendental functions | slatec::special::elementary::sin_degrees | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DSISL` | `slatec_sys::linear_algebra::dense::dsisl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSJAC` | `slatec_sys::linear_algebra::sparse::dsjac` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLI` | `slatec_sys::linear_algebra::dense::dsli` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLI2` | `slatec_sys::linear_algebra::dense::dsli2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLLTI` | `slatec_sys::linear_algebra::dense::dsllti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUBC` | `slatec_sys::linear_algebra::sparse::dslubc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUCN` | `slatec_sys::linear_algebra::sparse::dslucn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUCS` | `slatec_sys::linear_algebra::dense::dslucs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUGM` | `slatec_sys::linear_algebra::sparse::dslugm` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUI` | `slatec_sys::linear_algebra::dense::dslui` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUI2` | `slatec_sys::linear_algebra::dense::dslui2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUI4` | `slatec_sys::linear_algebra::dense::dslui4` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUOM` | `slatec_sys::linear_algebra::sparse::dsluom` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSLUTI` | `slatec_sys::linear_algebra::dense::dsluti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSMMI2` | `slatec_sys::linear_algebra::dense::dsmmi2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSMMTI` | `slatec_sys::linear_algebra::dense::dsmmti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSMTV` | `slatec_sys::blas::level1::dsmtv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSMV` | `slatec_sys::blas::level1::dsmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSOS` | `slatec_sys::nonlinear::systems::dsos` | Nonlinear equations | slatec::nonlinear::solve_scalar_equations | `direct-safe-wrapper` | nonlinear-systems | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DSPCO` | `slatec_sys::linear_algebra::packed::dspco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSPDI` | `slatec_sys::linear_algebra::packed::dspdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSPENC` | `slatec_sys::special::dspenc` | Special functions | slatec::special::scalar_expanded::spence_integral | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DSPFA` | `slatec_sys::linear_algebra::packed::dspfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSPLP` | `slatec_sys::linear_programming::dsplp` | Optimization and least squares | slatec::linear_programming::LinearProgram::<f64>::solve | `direct-safe-wrapper` | optimization-linear-programming-in-memory | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DSPMV` | `slatec_sys::blas::level2::dspmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSPR` | `slatec_sys::blas::level2::dspr` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSPR2` | `slatec_sys::blas::level2::dspr2` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSPSL` | `slatec_sys::linear_algebra::packed::dspsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSVDC` | `slatec_sys::linear_algebra::dense::dsvdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DSWAP` | `slatec_sys::blas::level1::dswap` | Linear algebra kernels | slatec::blas::level1::dswap; slatec::blas::level1::dswap_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DSYMM` | `slatec_sys::blas::level3::dsymm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSYMV` | `slatec_sys::blas::level2::dsymv` | Linear algebra kernels | slatec::blas::level2::dsymv | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DSYR` | `slatec_sys::blas::level2::dsyr` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSYR2` | `slatec_sys::blas::level2::dsyr2` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSYR2K` | `slatec_sys::blas::level3::dsyr2k` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DSYRK` | `slatec_sys::blas::level3::dsyrk` | Linear algebra kernels | slatec::blas::level3::dsyrk | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DTBMV` | `slatec_sys::blas::level2::dtbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DTBSV` | `slatec_sys::blas::level2::dtbsv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DTPMV` | `slatec_sys::blas::level2::dtpmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DTPSV` | `slatec_sys::blas::level2::dtpsv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DTRCO` | `slatec_sys::linear_algebra::dense::dtrco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DTRDI` | `slatec_sys::linear_algebra::dense::dtrdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DTRMM` | `slatec_sys::blas::level3::dtrmm` | Linear algebra kernels | slatec::blas::level3::dtrmm | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DTRMV` | `slatec_sys::blas::level2::dtrmv` | Linear algebra kernels | slatec::blas::level2::dtrmv | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DTRSL` | `slatec_sys::linear_algebra::dense::dtrsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DTRSM` | `slatec_sys::blas::level3::dtrsm` | Linear algebra kernels | slatec::blas::level3::dtrsm | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DTRSV` | `slatec_sys::blas::level2::dtrsv` | Linear algebra kernels | slatec::blas::level2::dtrsv | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DULSIA` | `slatec_sys::linear_algebra::dense::dulsia` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `DWNNLS` | `slatec_sys::approximation::dwnnls` | Approximation | slatec::linear_least_squares::solve_nonnegative_least_squares | `direct-safe-wrapper` | least-squares-linear-nonnegative | — | `maintain_checked_safe_wrapper` | `maintain` |
| `DXLEGF` | `slatec_sys::special::dxlegf` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `DXNRMP` | `slatec_sys::special::dxnrmp` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `E1` | `slatec_sys::special::e1` | Special functions | slatec::special::integrals::exponential_integral_e1_f32 | `direct-safe-wrapper` | special-integrals | — | `maintain_checked_safe_wrapper` | `maintain` |
| `EFC` | `slatec_sys::approximation::efc` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `EI` | `slatec_sys::special::ei` | Special functions | slatec::special::integrals::exponential_integral_ei_f32 | `direct-safe-wrapper` | special-integrals | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ELMBAK` | `slatec_sys::linear_algebra::eigen::elmbak` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ELMHES` | `slatec_sys::linear_algebra::eigen::elmhes` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ELTRAN` | `slatec_sys::linear_algebra::eigen::eltran` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ERF` | `slatec_sys::special::error::erf` | Special functions | slatec::special::error_functions::erf_f32 | `direct-safe-wrapper` | special-error | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ERFC` | `slatec_sys::special::error::erfc` | Special functions | slatec::special::error_functions::erfc_f32 | `direct-safe-wrapper` | special-error | — | `maintain_checked_safe_wrapper` | `maintain` |
| `EXINT` | `slatec_sys::special::exint` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `EXPREL` | `slatec_sys::special::elementary::exprel` | Elementary and transcendental functions | slatec::special::elementary::exprel_f32 | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `EZFFTB` | `slatec_sys::fftpack::ezfftb` | FFTPACK transforms | slatec::fftpack::EasyRealFftPlan::backward | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `EZFFTF` | `slatec_sys::fftpack::ezfftf` | FFTPACK transforms | slatec::fftpack::EasyRealFftPlan::forward | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `EZFFTI` | `slatec_sys::fftpack::ezffti` | FFTPACK transforms | slatec::fftpack::EasyRealFftPlan::new | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `FAC` | `slatec_sys::special::gamma::fac` | Special functions | slatec::special::gamma::factorial_f32 | `direct-safe-wrapper` | special-gamma | — | `maintain_checked_safe_wrapper` | `maintain` |
| `FC` | `slatec_sys::approximation::fc` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `FIGI` | `slatec_sys::linear_algebra::eigen::figi` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `FIGI2` | `slatec_sys::linear_algebra::eigen::figi2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `FZERO` | `slatec_sys::roots::scalar::fzero` | Nonlinear equations | slatec::roots::find_root_f32 | `direct-safe-wrapper` | roots-scalar | — | `maintain_checked_safe_wrapper` | `maintain` |
| `GAMI` | `slatec_sys::special::gamma::gami` | Special functions | slatec::special::gamma::incomplete_gamma_lower_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `GAMIC` | `slatec_sys::special::gamma::gamic` | Special functions | slatec::special::gamma::incomplete_gamma_upper_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `GAMIT` | `slatec_sys::special::gamma::gamit` | Special functions | slatec::special::gamma::tricomi_incomplete_gamma_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `GAMLIM` | `slatec_sys::special::gamlim` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `GAMMA` | `slatec_sys::special::gamma::gamma` | Special functions | slatec::special::gamma::gamma_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `GAMR` | `slatec_sys::special::gamma::gamr` | Special functions | slatec::special::gamma::reciprocal_gamma_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `GAUS8` | `slatec_sys::quadrature::gaus8` | Numerical quadrature | — | `blocked-by-safe-design` | — | existing QUADPACK options-based integration | `retain_raw_only` | `none` |
| `GENBUN` | `slatec_sys::pde::fishpack::genbun` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HFTI` | `slatec_sys::linear_algebra::dense::hfti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `HQR` | `slatec_sys::linear_algebra::eigen::hqr` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `HQR2` | `slatec_sys::linear_algebra::eigen::hqr2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `HSTCRT` | `slatec_sys::pde::fishpack::hstcrt` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HSTCSP` | `slatec_sys::pde::fishpack::hstcsp` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HSTCYL` | `slatec_sys::pde::fishpack::hstcyl` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HSTPLR` | `slatec_sys::pde::fishpack::hstplr` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HSTSSP` | `slatec_sys::pde::fishpack::hstssp` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HTRIB3` | `slatec_sys::linear_algebra::eigen::htrib3` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `HTRIBK` | `slatec_sys::linear_algebra::eigen::htribk` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `HTRID3` | `slatec_sys::linear_algebra::eigen::htrid3` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `HTRIDI` | `slatec_sys::linear_algebra::eigen::htridi` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `HW3CRT` | `slatec_sys::pde::fishpack::hw3crt` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HWSCRT` | `slatec_sys::pde::fishpack::hwscrt` | FISHPACK elliptic PDE solvers | slatec::differential_equations::pde::CartesianHelmholtz2d::solve | `direct-safe-wrapper` | fishpack-cartesian-2d | — | `maintain_checked_safe_wrapper` | `maintain` |
| `HWSCSP` | `slatec_sys::pde::fishpack::hwscsp` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HWSCYL` | `slatec_sys::pde::fishpack::hwscyl` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HWSPLR` | `slatec_sys::pde::fishpack::hwsplr` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `HWSSSP` | `slatec_sys::pde::fishpack::hwsssp` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ICAMAX` | `slatec_sys::blas::level1::icamax` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ICOPY` | `slatec_sys::blas::level1::icopy` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `IDAMAX` | `slatec_sys::blas::level1::idamax` | Linear algebra kernels | slatec::blas::level1::idamax; slatec::blas::level1::idamax_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `IMTQL1` | `slatec_sys::linear_algebra::eigen::imtql1` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `IMTQL2` | `slatec_sys::linear_algebra::eigen::imtql2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `IMTQLV` | `slatec_sys::linear_algebra::eigen::imtqlv` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `INITDS` | `slatec_sys::special::initds` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `INITS` | `slatec_sys::special::inits` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `INTRV` | `slatec_sys::interpolation::intrv` | Interpolation | — | `covered-by-higher-level-safe-api` | — | slatec::interpolation::bspline::BSpline | `maintain_higher_level_abstraction` | `maintain` |
| `INVIT` | `slatec_sys::linear_algebra::eigen::invit` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ISAMAX` | `slatec_sys::blas::level1::isamax` | Linear algebra kernels | slatec::blas::level1::isamax; slatec::blas::level1::isamax_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `ISWAP` | `slatec_sys::blas::level1::iswap` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `LLSIA` | `slatec_sys::linear_algebra::dense::llsia` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `LSEI` | `slatec_sys::approximation::lsei` | Approximation | slatec::constrained_least_squares::solve_constrained_least_squares_f32 | `direct-safe-wrapper` | least-squares-linear-constrained | — | `maintain_checked_safe_wrapper` | `maintain` |
| `MINFIT` | `slatec_sys::linear_algebra::dense::minfit` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ORTBAK` | `slatec_sys::linear_algebra::eigen::ortbak` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ORTHES` | `slatec_sys::linear_algebra::eigen::orthes` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ORTRAN` | `slatec_sys::linear_algebra::eigen::ortran` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `PCHBS` | `slatec_sys::interpolation::pchbs` | PCHIP | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `PCHCM` | `slatec_sys::interpolation::pchcm` | PCHIP | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `PCHFD` | `slatec_sys::interpolation::pchfd` | PCHIP | slatec::pchip::PiecewiseCubicHermite::evaluate_with_derivatives_into | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PCHFE` | `slatec_sys::interpolation::pchfe` | PCHIP | slatec::pchip::PiecewiseCubicHermite::evaluate_into | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PCHIA` | `slatec_sys::interpolation::pchia` | PCHIP | slatec::pchip::PiecewiseCubicHermite::integrate | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PCHIC` | `slatec_sys::interpolation::pchic` | PCHIP | slatec::pchip::PiecewiseCubicHermite::monotone_with_conditions | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PCHID` | `slatec_sys::interpolation::pchid` | PCHIP | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `PCHIM` | `slatec_sys::interpolation::pchim` | PCHIP | slatec::pchip::PiecewiseCubicHermite::monotone | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PCHSP` | `slatec_sys::interpolation::pchsp` | PCHIP | slatec::pchip::PiecewiseCubicHermite::spline | `direct-safe-wrapper` | pchip | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PCOEF` | `slatec_sys::approximation::pcoef` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `POCH` | `slatec_sys::special::poch` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `POCH1` | `slatec_sys::special::poch1` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `POIS3D` | `slatec_sys::pde::fishpack::pois3d` | FISHPACK elliptic PDE solvers | slatec::differential_equations::pde::Pois3dProblem::solve | `direct-safe-wrapper` | fishpack-pois3d | — | `maintain_checked_safe_wrapper` | `maintain` |
| `POISTG` | `slatec_sys::pde::fishpack::poistg` | FISHPACK elliptic PDE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `POLCOF` | `slatec_sys::interpolation::polcof` | Interpolation | slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `POLFIT` | `slatec_sys::approximation::polfit` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `POLINT` | `slatec_sys::interpolation::polint` | Interpolation | slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `POLYVL` | `slatec_sys::interpolation::polyvl` | Interpolation | slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate; slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate_with_derivatives | `direct-safe-wrapper` | tabulated-data | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PPQAD` | `slatec_sys::quadrature::ppqad` | Numerical quadrature | slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::integrate | `direct-safe-wrapper` | piecewise-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PPVAL` | `slatec_sys::interpolation::ppval` | Interpolation | slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into | `direct-safe-wrapper` | piecewise-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PSI` | `slatec_sys::special::gamma::psi` | Special functions | slatec::special::gamma::digamma_f32 | `direct-safe-wrapper` | special | — | `maintain_checked_safe_wrapper` | `maintain` |
| `PSIFN` | `slatec_sys::special::psifn` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `PVALUE` | `slatec_sys::approximation::pvalue` | Approximation | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `QAG` | `slatec_sys::quadrature::qag` | Numerical quadrature | slatec::quadrature::integrate_f32 | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAGI` | `slatec_sys::quadrature::qagi` | Numerical quadrature | slatec::quadrature::integrate_infinite_f32 | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAGIE` | `slatec_sys::quadrature::callbacks::qagie` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QAGP` | `slatec_sys::quadrature::qagp` | Numerical quadrature | slatec::quadrature::integrate_with_breakpoints_f32 | `direct-safe-wrapper` | quadrature-breakpoints | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAGPE` | `slatec_sys::quadrature::callbacks::qagpe` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QAGS` | `slatec_sys::quadrature::qags` | Numerical quadrature | slatec::quadrature::integrate_singular_f32 | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAGSE` | `slatec_sys::quadrature::callbacks::qagse` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QAWC` | `slatec_sys::quadrature::qawc` | Numerical quadrature | slatec::quadrature::integrate_principal_value_f32 | `direct-safe-wrapper` | quadrature-basic | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAWCE` | `slatec_sys::quadrature::callbacks::qawce` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QAWF` | `slatec_sys::quadrature::qawf` | Numerical quadrature | slatec::quadrature::integrate_fourier_tail_f32 | `direct-safe-wrapper` | quadrature-fourier | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAWO` | `slatec_sys::quadrature::qawo` | Numerical quadrature | slatec::quadrature::integrate_oscillatory_f32 | `direct-safe-wrapper` | quadrature-oscillatory | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAWS` | `slatec_sys::quadrature::qaws` | Numerical quadrature | slatec::quadrature::integrate_weighted_endpoints_f32 | `direct-safe-wrapper` | quadrature-weighted | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QAWSE` | `slatec_sys::quadrature::callbacks::qawse` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QC25C` | `slatec_sys::quadrature::callbacks::qc25c` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QC25S` | `slatec_sys::quadrature::callbacks::qc25s` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK15` | `slatec_sys::quadrature::callbacks::qk15` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK15I` | `slatec_sys::quadrature::callbacks::qk15i` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK15W` | `slatec_sys::quadrature::callbacks::qk15w` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK21` | `slatec_sys::quadrature::callbacks::qk21` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK31` | `slatec_sys::quadrature::callbacks::qk31` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK41` | `slatec_sys::quadrature::callbacks::qk41` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK51` | `slatec_sys::quadrature::callbacks::qk51` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QK61` | `slatec_sys::quadrature::callbacks::qk61` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QMOMO` | `slatec_sys::quadrature::qmomo` | Numerical quadrature | — | `covered-by-higher-level-safe-api` | — | slatec::quadrature options-based adaptive integration | `maintain_higher_level_abstraction` | `maintain` |
| `QNC79` | `slatec_sys::quadrature::qnc79` | Numerical quadrature | slatec::quadrature::integrate_nc79_f32 | `direct-safe-wrapper` | quadrature-nonadaptive | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QNG` | `slatec_sys::quadrature::qng` | Numerical quadrature | slatec::quadrature::integrate_non_adaptive_f32 | `direct-safe-wrapper` | quadrature-nonadaptive | — | `maintain_checked_safe_wrapper` | `maintain` |
| `QZHES` | `slatec_sys::linear_algebra::eigen::qzhes` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `QZIT` | `slatec_sys::linear_algebra::eigen::qzit` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `QZVAL` | `slatec_sys::linear_algebra::eigen::qzval` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `QZVEC` | `slatec_sys::linear_algebra::eigen::qzvec` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RAND` | `slatec_sys::statistics::rand` | Probability and statistics | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `RATQR` | `slatec_sys::linear_algebra::eigen::ratqr` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RC` | `slatec_sys::special::rc` | Special functions | slatec::special::scalar_expanded::carlson_rc_f32 | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `RC3JJ` | `slatec_sys::special::rc3jj` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `RC3JM` | `slatec_sys::special::rc3jm` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `RC6J` | `slatec_sys::special::rc6j` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `RD` | `slatec_sys::special::rd` | Special functions | slatec::special::scalar_expanded::carlson_rd_f32 | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `REBAK` | `slatec_sys::linear_algebra::eigen::rebak` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `REBAKB` | `slatec_sys::linear_algebra::eigen::rebakb` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `REDUC` | `slatec_sys::linear_algebra::eigen::reduc` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `REDUC2` | `slatec_sys::linear_algebra::eigen::reduc2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RF` | `slatec_sys::special::rf` | Special functions | slatec::special::scalar_expanded::carlson_rf_f32 | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `RFFTB1` | `slatec_sys::fftpack::rfftb1` | FFTPACK transforms | — | `covered-by-higher-level-safe-api` | — | slatec::fftpack owned transform plans | `maintain_higher_level_abstraction` | `maintain` |
| `RFFTF1` | `slatec_sys::fftpack::rfftf1` | FFTPACK transforms | — | `covered-by-higher-level-safe-api` | — | slatec::fftpack owned transform plans | `maintain_higher_level_abstraction` | `maintain` |
| `RFFTI1` | `slatec_sys::fftpack::rffti1` | FFTPACK transforms | — | `covered-by-higher-level-safe-api` | — | slatec::fftpack owned transform plans | `maintain_higher_level_abstraction` | `maintain` |
| `RG` | `slatec_sys::linear_algebra::eigen::rg` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RGAUSS` | `slatec_sys::statistics::rgauss` | Probability and statistics | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `RGG` | `slatec_sys::linear_algebra::eigen::rgg` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RJ` | `slatec_sys::special::rj` | Special functions | slatec::special::scalar_expanded::carlson_rj_f32 | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `RPQR79` | `slatec_sys::roots::complex::rpqr79` | Nonlinear equations | slatec::roots::real_polynomial_roots_with_method | `direct-safe-wrapper` | roots-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `RPZERO` | `slatec_sys::roots::complex::rpzero` | Nonlinear equations | slatec::roots::real_polynomial_roots | `direct-safe-wrapper` | roots-polynomial | — | `maintain_checked_safe_wrapper` | `maintain` |
| `RS` | `slatec_sys::linear_algebra::eigen::rs` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RSB` | `slatec_sys::linear_algebra::eigen::rsb` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RSG` | `slatec_sys::linear_algebra::eigen::rsg` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RSGAB` | `slatec_sys::linear_algebra::eigen::rsgab` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RSGBA` | `slatec_sys::linear_algebra::eigen::rsgba` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RSP` | `slatec_sys::linear_algebra::eigen::rsp` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RST` | `slatec_sys::linear_algebra::eigen::rst` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RT` | `slatec_sys::linear_algebra::eigen::rt` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `RUNIF` | `slatec_sys::statistics::runif` | Probability and statistics | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SASUM` | `slatec_sys::blas::level1::sasum` | Linear algebra kernels | slatec::blas::level1::sasum; slatec::blas::level1::sasum_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SAXPY` | `slatec_sys::blas::level1::saxpy` | Linear algebra kernels | slatec::blas::level1::saxpy; slatec::blas::level1::saxpy_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SBCG` | `slatec_sys::linear_algebra::sparse::callbacks::sbcg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SBOCLS` | `slatec_sys::approximation::sbocls` | Approximation | slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32 | `direct-safe-wrapper` | least-squares-linear-bounded-constrained | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SBOLS` | `slatec_sys::approximation::sbols` | Approximation | slatec::bounded_least_squares::solve_bounded_least_squares_f32 | `direct-safe-wrapper` | least-squares-linear-bounded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SCASUM` | `slatec_sys::blas::level1::scasum` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SCG` | `slatec_sys::linear_algebra::sparse::callbacks::scg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SCGN` | `slatec_sys::linear_algebra::sparse::callbacks::scgn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SCGS` | `slatec_sys::linear_algebra::sparse::callbacks::scgs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SCHDC` | `slatec_sys::linear_algebra::dense::schdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SCHDD` | `slatec_sys::linear_algebra::dense::schdd` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SCHEX` | `slatec_sys::linear_algebra::dense::schex` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SCHUD` | `slatec_sys::linear_algebra::dense::schud` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SCNRM2` | `slatec_sys::blas::level1::scnrm2` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SCOPY` | `slatec_sys::blas::level1::scopy` | Linear algebra kernels | slatec::blas::level1::scopy; slatec::blas::level1::scopy_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SCOPYM` | `slatec_sys::blas::level1::scopym` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SCOV` | `slatec_sys::least_squares::scov` | Approximation | slatec::least_squares::covariance_from_expert_fit_f32; slatec::least_squares::estimate_covariance_f32; slatec::least_squares::estimate_covariance_finite_difference_f32 | `direct-safe-wrapper` | least-squares-covariance; least-squares-covariance + least-squares-nonlinear-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SDASSL` | `slatec_sys::dassl::sdassl` | ODE solvers | slatec::dassl::DaeSession::<f32, F>::advance_to | `direct-safe-wrapper` | dassl | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SDOT` | `slatec_sys::blas::level1::sdot` | Linear algebra kernels | slatec::blas::level1::sdot; slatec::blas::level1::sdot_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SDRIV1` | `slatec_sys::ode::sdriv1` | ODE solvers | slatec::ode::Driv1Session::<f32>::integrate_to | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SDRIV2` | `slatec_sys::ode::sdriv2` | ODE solvers | slatec::ode::Driv2Session::<f32>::integrate_to_with_events | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SDRIV3` | `slatec_sys::ode::sdriv3` | ODE solvers | slatec::ode::OdeSession::<f32, F, E>::integrate_to | `direct-safe-wrapper` | ode-sdrive-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SDSDOT` | `slatec_sys::blas::level1::sdsdot` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SGBCO` | `slatec_sys::linear_algebra::banded::sgbco` | Dense linear algebra | slatec::linear_algebra::banded::BandMatrixRef::factorize_with_condition_estimate | `direct-safe-wrapper` | banded-linear-systems | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SGBDI` | `slatec_sys::linear_algebra::banded::sgbdi` | Dense linear algebra | slatec::linear_algebra::banded::BandLu32::scaled_determinant | `direct-safe-wrapper` | banded-linear-systems | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SGBFA` | `slatec_sys::linear_algebra::banded::sgbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGBMV` | `slatec_sys::blas::level2::sgbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SGBSL` | `slatec_sys::linear_algebra::banded::sgbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGECO` | `slatec_sys::linear_algebra::dense::sgeco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGEDI` | `slatec_sys::linear_algebra::dense::sgedi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGEEV` | `slatec_sys::linear_algebra::eigen::sgeev` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGEFA` | `slatec_sys::linear_algebra::dense::sgefa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGEFS` | `slatec_sys::linear_algebra::dense::sgefs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGEIR` | `slatec_sys::linear_algebra::dense::sgeir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGEMM` | `slatec_sys::blas::level3::sgemm` | Linear algebra kernels | slatec::blas::level3::sgemm; slatec::blas::level3::sgemm_contiguous | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SGEMV` | `slatec_sys::blas::level2::sgemv` | Linear algebra kernels | slatec::blas::level2::sgemv; slatec::blas::level2::sgemv_contiguous | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SGER` | `slatec_sys::blas::level2::sger` | Linear algebra kernels | slatec::blas::level2::sger | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SGESL` | `slatec_sys::linear_algebra::dense::sgesl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGLSS` | `slatec_sys::linear_algebra::dense::sglss` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGMRES` | `slatec_sys::linear_algebra::sparse::callbacks::sgmres` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SGTSL` | `slatec_sys::linear_algebra::banded::sgtsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SINDG` | `slatec_sys::special::elementary::sindg` | Elementary and transcendental functions | slatec::special::elementary::sin_degrees_f32 | `direct-safe-wrapper` | special-elementary | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SINQB` | `slatec_sys::fftpack::sinqb` | FFTPACK transforms | slatec::fftpack::QuarterWaveSinePlan::backward | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SINQF` | `slatec_sys::fftpack::sinqf` | FFTPACK transforms | slatec::fftpack::QuarterWaveSinePlan::forward | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SINQI` | `slatec_sys::fftpack::sinqi` | FFTPACK transforms | slatec::fftpack::QuarterWaveSinePlan::new | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SINT` | `slatec_sys::fftpack::sint` | FFTPACK transforms | slatec::fftpack::SineTransformPlan::transform | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SINTI` | `slatec_sys::fftpack::sinti` | FFTPACK transforms | slatec::fftpack::SineTransformPlan::new | `direct-safe-wrapper` | fftpack-real | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SINTRP` | `slatec_sys::ode::sintrp` | ODE solvers | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SIR` | `slatec_sys::linear_algebra::sparse::callbacks::sir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SLLTI2` | `slatec_sys::linear_algebra::dense::sllti2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SNBCO` | `slatec_sys::linear_algebra::banded::snbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SNBDI` | `slatec_sys::linear_algebra::banded::snbdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SNBFA` | `slatec_sys::linear_algebra::banded::snbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SNBFS` | `slatec_sys::linear_algebra::banded::snbfs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SNBIR` | `slatec_sys::linear_algebra::banded::snbir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SNBSL` | `slatec_sys::linear_algebra::banded::snbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SNLS1` | `slatec_sys::least_squares::snls1` | Approximation | slatec::least_squares::least_squares_expert_f32; slatec::least_squares::least_squares_with_jacobian_f32 | `direct-safe-wrapper` | least-squares-nonlinear-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SNLS1E` | `slatec_sys::least_squares::snls1e` | Approximation | slatec::least_squares::least_squares_f32 | `direct-safe-wrapper` | least-squares-nonlinear-easy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SNRM2` | `slatec_sys::blas::level1::snrm2` | Linear algebra kernels | slatec::blas::level1::snrm2; slatec::blas::level1::snrm2_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SNSQ` | `slatec_sys::nonlinear::snsq` | Nonlinear equations | slatec::nonlinear::solve_system_expert_f32; slatec::nonlinear::solve_system_with_jacobian_f32 | `direct-safe-wrapper` | nonlinear-expert | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SNSQE` | `slatec_sys::nonlinear::snsqe` | Nonlinear equations | slatec::nonlinear::solve_system_f32 | `direct-safe-wrapper` | nonlinear-easy | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SOMN` | `slatec_sys::linear_algebra::sparse::callbacks::somn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SOS` | `slatec_sys::nonlinear::systems::sos` | Nonlinear equations | slatec::nonlinear::solve_scalar_equations_f32 | `direct-safe-wrapper` | nonlinear-systems | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SPBCO` | `slatec_sys::linear_algebra::banded::spbco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPBDI` | `slatec_sys::linear_algebra::banded::spbdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPBFA` | `slatec_sys::linear_algebra::banded::spbfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPBSL` | `slatec_sys::linear_algebra::banded::spbsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPENC` | `slatec_sys::special::spenc` | Special functions | slatec::special::scalar_expanded::spence_integral_f32 | `direct-safe-wrapper` | special-scalar-expanded | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SPLP` | `slatec_sys::linear_programming::splp` | Optimization and least squares | slatec::linear_programming::LinearProgram::<f32>::solve | `direct-safe-wrapper` | optimization-linear-programming-in-memory | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SPOCO` | `slatec_sys::linear_algebra::dense::spoco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPODI` | `slatec_sys::linear_algebra::dense::spodi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPOFA` | `slatec_sys::linear_algebra::dense::spofa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPOFS` | `slatec_sys::linear_algebra::dense::spofs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPOIR` | `slatec_sys::linear_algebra::dense::spoir` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPOSL` | `slatec_sys::linear_algebra::dense::sposl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPPCO` | `slatec_sys::linear_algebra::packed::sppco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPPDI` | `slatec_sys::linear_algebra::packed::sppdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPPFA` | `slatec_sys::linear_algebra::packed::sppfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPPSL` | `slatec_sys::linear_algebra::packed::sppsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SPTSL` | `slatec_sys::linear_algebra::banded::sptsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SQRDC` | `slatec_sys::linear_algebra::dense::sqrdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SQRSL` | `slatec_sys::linear_algebra::dense::sqrsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SROT` | `slatec_sys::blas::level1::srot` | Linear algebra kernels | slatec::blas::level1::srot; slatec::blas::level1::srot_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SROTG` | `slatec_sys::blas::level1::srotg` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SROTM` | `slatec_sys::blas::level1::srotm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SROTMG` | `slatec_sys::blas::level1::srotmg` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SS2LT` | `slatec_sys::linear_algebra::dense::ss2lt` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SS2Y` | `slatec_sys::blas::level1::ss2y` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSBMV` | `slatec_sys::blas::level2::ssbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSCAL` | `slatec_sys::blas::level1::sscal` | Linear algebra kernels | slatec::blas::level1::sscal; slatec::blas::level1::sscal_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SSD2S` | `slatec_sys::linear_algebra::dense::ssd2s` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDBCG` | `slatec_sys::linear_algebra::sparse::ssdbcg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDCG` | `slatec_sys::linear_algebra::sparse::ssdcg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDCGN` | `slatec_sys::linear_algebra::sparse::ssdcgn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDCGS` | `slatec_sys::linear_algebra::sparse::ssdcgs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDGMR` | `slatec_sys::linear_algebra::sparse::ssdgmr` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDI` | `slatec_sys::blas::level1::ssdi` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSDOMN` | `slatec_sys::linear_algebra::sparse::ssdomn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDS` | `slatec_sys::linear_algebra::dense::ssds` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSDSCL` | `slatec_sys::linear_algebra::dense::ssdscl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSGS` | `slatec_sys::linear_algebra::sparse::ssgs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSICCG` | `slatec_sys::linear_algebra::sparse::ssiccg` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSICO` | `slatec_sys::linear_algebra::dense::ssico` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSICS` | `slatec_sys::linear_algebra::dense::ssics` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSIDI` | `slatec_sys::linear_algebra::dense::ssidi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSIEV` | `slatec_sys::linear_algebra::eigen::ssiev` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSIFA` | `slatec_sys::linear_algebra::dense::ssifa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSILUR` | `slatec_sys::linear_algebra::sparse::ssilur` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSILUS` | `slatec_sys::linear_algebra::dense::ssilus` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSISL` | `slatec_sys::linear_algebra::dense::ssisl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSJAC` | `slatec_sys::linear_algebra::sparse::ssjac` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLI` | `slatec_sys::linear_algebra::dense::ssli` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLI2` | `slatec_sys::linear_algebra::dense::ssli2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLLTI` | `slatec_sys::linear_algebra::dense::ssllti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUBC` | `slatec_sys::linear_algebra::sparse::sslubc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUCN` | `slatec_sys::linear_algebra::sparse::sslucn` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUCS` | `slatec_sys::linear_algebra::dense::sslucs` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUGM` | `slatec_sys::linear_algebra::sparse::sslugm` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUI` | `slatec_sys::linear_algebra::dense::sslui` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUI2` | `slatec_sys::linear_algebra::dense::sslui2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUI4` | `slatec_sys::linear_algebra::dense::sslui4` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUOM` | `slatec_sys::linear_algebra::sparse::ssluom` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSLUTI` | `slatec_sys::linear_algebra::dense::ssluti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSMMI2` | `slatec_sys::linear_algebra::dense::ssmmi2` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSMMTI` | `slatec_sys::linear_algebra::dense::ssmmti` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSMTV` | `slatec_sys::blas::level1::ssmtv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSMV` | `slatec_sys::blas::level1::ssmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSPCO` | `slatec_sys::linear_algebra::packed::sspco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSPDI` | `slatec_sys::linear_algebra::packed::sspdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSPEV` | `slatec_sys::linear_algebra::eigen::sspev` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSPFA` | `slatec_sys::linear_algebra::packed::sspfa` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSPMV` | `slatec_sys::blas::level2::sspmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSPR` | `slatec_sys::blas::level2::sspr` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSPR2` | `slatec_sys::blas::level2::sspr2` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSPSL` | `slatec_sys::linear_algebra::packed::sspsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSVDC` | `slatec_sys::linear_algebra::dense::ssvdc` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `SSWAP` | `slatec_sys::blas::level1::sswap` | Linear algebra kernels | slatec::blas::level1::sswap; slatec::blas::level1::sswap_strided | `direct-safe-wrapper` | blas-level1 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SSYMM` | `slatec_sys::blas::level3::ssymm` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSYMV` | `slatec_sys::blas::level2::ssymv` | Linear algebra kernels | slatec::blas::level2::ssymv | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `SSYR` | `slatec_sys::blas::level2::ssyr` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSYR2` | `slatec_sys::blas::level2::ssyr2` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSYR2K` | `slatec_sys::blas::level3::ssyr2k` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `SSYRK` | `slatec_sys::blas::level3::ssyrk` | Linear algebra kernels | slatec::blas::level3::ssyrk | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `STBMV` | `slatec_sys::blas::level2::stbmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `STBSV` | `slatec_sys::blas::level2::stbsv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `STPMV` | `slatec_sys::blas::level2::stpmv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `STPSV` | `slatec_sys::blas::level2::stpsv` | Linear algebra kernels | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `STRCO` | `slatec_sys::linear_algebra::dense::strco` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `STRDI` | `slatec_sys::linear_algebra::dense::strdi` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `STRMM` | `slatec_sys::blas::level3::strmm` | Linear algebra kernels | slatec::blas::level3::strmm | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `STRMV` | `slatec_sys::blas::level2::strmv` | Linear algebra kernels | slatec::blas::level2::strmv | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `STRSL` | `slatec_sys::linear_algebra::dense::strsl` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `STRSM` | `slatec_sys::blas::level3::strsm` | Linear algebra kernels | slatec::blas::level3::strsm | `direct-safe-wrapper` | blas-level3 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `STRSV` | `slatec_sys::blas::level2::strsv` | Linear algebra kernels | slatec::blas::level2::strsv | `direct-safe-wrapper` | blas-level2 | — | `maintain_checked_safe_wrapper` | `maintain` |
| `TINVIT` | `slatec_sys::linear_algebra::eigen::tinvit` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TQL1` | `slatec_sys::linear_algebra::eigen::tql1` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TQL2` | `slatec_sys::linear_algebra::eigen::tql2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TQLRAT` | `slatec_sys::linear_algebra::eigen::tqlrat` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TRBAK1` | `slatec_sys::linear_algebra::eigen::trbak1` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TRBAK3` | `slatec_sys::linear_algebra::eigen::trbak3` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TRED1` | `slatec_sys::linear_algebra::eigen::tred1` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TRED2` | `slatec_sys::linear_algebra::eigen::tred2` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TRED3` | `slatec_sys::linear_algebra::eigen::tred3` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TRIDIB` | `slatec_sys::linear_algebra::eigen::tridib` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `TSTURM` | `slatec_sys::linear_algebra::eigen::tsturm` | Eigenvalue problems | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `ULSIA` | `slatec_sys::linear_algebra::dense::ulsia` | Dense linear algebra | — | `intentionally-excluded-external-ecosystem` | — | — | `retain_raw_only` | `none` |
| `WNNLS` | `slatec_sys::approximation::wnnls` | Approximation | slatec::linear_least_squares::solve_nonnegative_least_squares_f32 | `direct-safe-wrapper` | least-squares-linear-nonnegative | — | `maintain_checked_safe_wrapper` | `maintain` |
| `XLEGF` | `slatec_sys::special::xlegf` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `XNRMP` | `slatec_sys::special::xnrmp` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ZAIRY` | `slatec_sys::special::zairy` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ZBESH` | `slatec_sys::special::bessel::zbesh` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ZBESI` | `slatec_sys::special::bessel::zbesi` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ZBESJ` | `slatec_sys::special::bessel::zbesj` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ZBESK` | `slatec_sys::special::bessel::zbesk` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ZBESY` | `slatec_sys::special::bessel::zbesy` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
| `ZBIRY` | `slatec_sys::special::zbiry` | Special functions | — | `expert-raw-only` | — | — | `retain_raw_only_pending_family_review` | `none` |
