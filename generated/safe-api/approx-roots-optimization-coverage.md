# all safe-coverage disposition

This generated inventory joins canonical raw status with raw-to-safe coverage. `expert-raw-only` and blocked records are explicit decisions, not missing data. Call-graph fields are recorded as unavailable where the committed authoritative inputs do not contain a complete Fortran call graph.

## Disposition counts

- `blocked-by-abi`: 2
- `candidate-implemented-in-this-milestone`: 6
- `covered-by-higher-level-safe-api`: 10
- `direct-safe-wrapper`: 40
- `expert-raw-only`: 119

## Routine records

| Routine | Role | Raw path | Safe path | Provider feature | Docs | Link | Runtime | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `BINT4` | historically_user_callable_driver | `slatec_sys::interpolation::bint4` | `slatec::interpolation::bspline::BSpline::interpolate_cubic` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
| `BINTK` | historically_user_callable_driver | `slatec_sys::interpolation::bintk` | `slatec::interpolation::bspline::BSpline::interpolate_with_knots` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `BNFAC` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `BNSLV` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `BSPDOC` | historically_user_callable_driver | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `BSPDR` | historically_user_callable_driver | `slatec_sys::interpolation::bspdr` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `BSPEV` | historically_user_callable_driver | `slatec_sys::interpolation::bspev` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `BSPLVD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `BSPLVN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `BSPPP` | historically_user_callable_driver | `slatec_sys::interpolation::bsppp` | `slatec::interpolation::bspline::BSpline::to_piecewise_polynomial` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `BSPVD` | historically_user_callable_driver | `slatec_sys::interpolation::bspvd` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `BSPVN` | historically_user_callable_driver | `slatec_sys::interpolation::bspvn` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `BVALU` | historically_user_callable_driver | `slatec_sys::interpolation::bvalu` | `slatec::interpolation::bspline::BSpline::derivative; slatec::interpolation::bspline::BSpline::evaluate; slatec::interpolation::bspline::BSpline::evaluate_into` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DBINT4` | historically_user_callable_driver | `slatec_sys::interpolation::dbint4` | `slatec::interpolation::bspline::BSpline::interpolate_cubic` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
| `DBINTK` | historically_user_callable_driver | `slatec_sys::interpolation::dbintk` | `slatec::interpolation::bspline::BSpline::interpolate_with_knots` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DBNFAC` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DBNSLV` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DBOCLS` | historically_user_callable_driver | `slatec_sys::approximation::dbocls` | `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DBOLS` | historically_user_callable_driver | `slatec_sys::approximation::dbols` | `slatec::bounded_least_squares::solve_bounded_least_squares` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DBOLSM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DBSPDR` | historically_user_callable_driver | `slatec_sys::interpolation::dbspdr` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `DBSPEV` | historically_user_callable_driver | `slatec_sys::interpolation::dbspev` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `DBSPPP` | historically_user_callable_driver | `slatec_sys::interpolation::dbsppp` | `slatec::interpolation::bspline::BSpline::to_piecewise_polynomial` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DBSPVD` | historically_user_callable_driver | `slatec_sys::interpolation::dbspvd` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `DBSPVN` | historically_user_callable_driver | `slatec_sys::interpolation::dbspvn` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `DBVALU` | historically_user_callable_driver | `slatec_sys::interpolation::dbvalu` | `slatec::interpolation::bspline::BSpline::derivative; slatec::interpolation::bspline::BSpline::evaluate; slatec::interpolation::bspline::BSpline::evaluate_into` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DCOV` | historically_user_callable_driver | `slatec_sys::least_squares::dcov` | `slatec::least_squares::covariance_from_expert_fit; slatec::least_squares::estimate_covariance; slatec::least_squares::estimate_covariance_finite_difference` | `least-squares-covariance` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DEFC` | historically_user_callable_driver | `slatec_sys::approximation::defc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `DEFCMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFC` | historically_user_callable_driver | `slatec_sys::approximation::dfc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `DFCMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFDJC3` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFSPVD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFSPVN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DINTRV` | historically_user_callable_driver | `slatec_sys::interpolation::dintrv` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `DLPDP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DLSEI` | historically_user_callable_driver | `slatec_sys::approximation::dlsei` | `slatec::constrained_least_squares::solve_constrained_least_squares` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DLSI` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DMOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `DMPAR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DNLS1` | historically_user_callable_driver | `slatec_sys::least_squares::dnls1` | `slatec::least_squares::least_squares_expert; slatec::least_squares::least_squares_with_jacobian` | `least-squares-nonlinear-expert` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DNLS1E` | historically_user_callable_driver | `slatec_sys::least_squares::dnls1e` | `slatec::least_squares::least_squares` | `least-squares-nonlinear-easy` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DP1VLU` | historically_user_callable_driver | `slatec_sys::approximation::dp1vlu` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `DPCOEF` | historically_user_callable_driver | `slatec_sys::approximation::dpcoef` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `DPLINT` | historically_user_callable_driver | `slatec_sys::interpolation::dplint` | `slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DPOLCF` | historically_user_callable_driver | `slatec_sys::interpolation::dpolcf` | `slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DPOLFT` | historically_user_callable_driver | `slatec_sys::approximation::dpolft` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `DPOLVL` | historically_user_callable_driver | `slatec_sys::interpolation::dpolvl` | `slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate; slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate_with_derivatives` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DPPVAL` | historically_user_callable_driver | `slatec_sys::interpolation::dppval` | `slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DQRSLV` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DWNLIT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DWNLSM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DWNLT1` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DWNLT2` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DWNLT3` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DWNNLS` | historically_user_callable_driver | `slatec_sys::approximation::dwnnls` | `slatec::linear_least_squares::solve_nonnegative_least_squares` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DWUPDT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `EFC` | historically_user_callable_driver | `slatec_sys::approximation::efc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `EFCMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FC` | historically_user_callable_driver | `slatec_sys::approximation::fc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `FCMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FDJAC3` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `INTRV` | historically_user_callable_driver | `slatec_sys::interpolation::intrv` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `LMPAR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LPDP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LSEI` | historically_user_callable_driver | `slatec_sys::approximation::lsei` | `slatec::constrained_least_squares::solve_constrained_least_squares_f32` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `LSI` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PCOEF` | historically_user_callable_driver | `slatec_sys::approximation::pcoef` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `POLCOF` | historically_user_callable_driver | `slatec_sys::interpolation::polcof` | `slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `POLFIT` | historically_user_callable_driver | `slatec_sys::approximation::polfit` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `POLINT` | historically_user_callable_driver | `slatec_sys::interpolation::polint` | `slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `POLYVL` | historically_user_callable_driver | `slatec_sys::interpolation::polyvl` | `slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate; slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate_with_derivatives` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `PPVAL` | historically_user_callable_driver | `slatec_sys::interpolation::ppval` | `slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `PVALUE` | historically_user_callable_driver | `slatec_sys::approximation::pvalue` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `expert-raw-only` |
| `QRSOLV` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `RWUPDT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SBOCLS` | historically_user_callable_driver | `slatec_sys::approximation::sbocls` | `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `SBOLS` | historically_user_callable_driver | `slatec_sys::approximation::sbols` | `slatec::bounded_least_squares::solve_bounded_least_squares_f32` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `SBOLSM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SCOV` | historically_user_callable_driver | `slatec_sys::least_squares::scov` | `slatec::least_squares::covariance_from_expert_fit_f32; slatec::least_squares::estimate_covariance_f32; slatec::least_squares::estimate_covariance_finite_difference_f32` | `least-squares-covariance` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SMOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `SNLS1` | historically_user_callable_driver | `slatec_sys::least_squares::snls1` | `slatec::least_squares::least_squares_expert_f32; slatec::least_squares::least_squares_with_jacobian_f32` | `least-squares-nonlinear-expert` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SNLS1E` | historically_user_callable_driver | `slatec_sys::least_squares::snls1e` | `slatec::least_squares::least_squares_f32` | `least-squares-nonlinear-easy` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `WNLIT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `WNLSM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `WNLT1` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `WNLT2` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `WNLT3` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `WNNLS` | historically_user_callable_driver | `slatec_sys::approximation::wnnls` | `slatec::linear_least_squares::solve_nonnegative_least_squares_f32` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `CPQR79` | historically_user_callable_driver | `slatec_sys::roots::complex::cpqr79` | `slatec::roots::complex_polynomial_roots_with_method` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `representative_batch_smoke_only` | `candidate-implemented-in-this-milestone` |
| `CPZERO` | historically_user_callable_driver | `slatec_sys::roots::complex::cpzero` | `slatec::roots::complex_polynomial_roots` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `representative_batch_smoke_only` | `candidate-implemented-in-this-milestone` |
| `DFZERO` | historically_user_callable_driver | `slatec_sys::roots::scalar::dfzero` | `slatec::roots::find_root` | `roots-scalar` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `FZERO` | historically_user_callable_driver | `slatec_sys::roots::scalar::fzero` | `slatec::roots::find_root_f32` | `roots-scalar` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `RPQR79` | historically_user_callable_driver | `slatec_sys::roots::complex::rpqr79` | `slatec::roots::real_polynomial_roots_with_method` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `representative_batch_smoke_only` | `candidate-implemented-in-this-milestone` |
| `RPZERO` | historically_user_callable_driver | `slatec_sys::roots::complex::rpzero` | `slatec::roots::real_polynomial_roots` | `nonlinear-complex` | `complete_generated_abi_contract` | `passed` | `representative_batch_smoke_only` | `candidate-implemented-in-this-milestone` |
| `CHKDER` | historically_user_callable_driver | `slatec_sys::nonlinear::jacobian_check::chkder` | `slatec::nonlinear::check_jacobian_f32` | `nonlinear-jacobian` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `CPEVL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `CPEVLR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `D1MPYQ` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `D1UPDT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DCKDER` | historically_user_callable_driver | `slatec_sys::nonlinear::jacobian_check::dckder` | `slatec::nonlinear::check_jacobian` | `nonlinear-jacobian` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DDOGLG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DENORM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFDJC1` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFULMT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DNSQ` | historically_user_callable_driver | `slatec_sys::nonlinear::dnsq` | `slatec::nonlinear::solve_system_expert; slatec::nonlinear::solve_system_with_jacobian` | `nonlinear-expert` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DNSQE` | historically_user_callable_driver | `slatec_sys::nonlinear::dnsqe` | `slatec::nonlinear::solve_system` | `nonlinear-easy` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DOGLEG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPCHNG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPINCW` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPINIT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPINTM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPCE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPDM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPFE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPFL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `blocked-by-abi` |
| `DPLPMU` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPUP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPNNZR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPOPT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPRWPG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPRWVR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DQFORM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DREADP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DSOS` | historically_user_callable_driver | `slatec_sys::nonlinear::systems::dsos` | `slatec::nonlinear::solve_scalar_equations` | `nonlinear-systems` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DSOSEQ` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DSOSSL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DSPLP` | historically_user_callable_driver | `slatec_sys::linear_programming::dsplp` | `slatec::linear_programming::LinearProgram::<f64>::solve` | `optimization-linear-programming-in-memory` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DUSRMT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DVOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `DWRITP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FDJAC1` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FULMAT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `IDLOC` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `IPLOC` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `IVOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `LA05AD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05AS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05BD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05BS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05CD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05CS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05ED` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05ES` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `MC20AD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `MC20AS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PCHNGS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PINITM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PNNZRS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PRWPGE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PRWVIR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `QFORM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `R1MPYQ` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `R1UPDT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SCLOSM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SNSQ` | historically_user_callable_driver | `slatec_sys::nonlinear::snsq` | `slatec::nonlinear::solve_system_expert_f32; slatec::nonlinear::solve_system_with_jacobian_f32` | `nonlinear-expert` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SNSQE` | historically_user_callable_driver | `slatec_sys::nonlinear::snsqe` | `slatec::nonlinear::solve_system_f32` | `nonlinear-easy` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SOPENM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SOS` | historically_user_callable_driver | `slatec_sys::nonlinear::systems::sos` | `slatec::nonlinear::solve_scalar_equations_f32` | `nonlinear-systems` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SOSEQS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SOSSOL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPINCW` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPINIT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLP` | historically_user_callable_driver | `slatec_sys::linear_programming::splp` | `slatec::linear_programming::LinearProgram::<f32>::solve` | `optimization-linear-programming-in-memory` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SPLPCE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPDM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPFE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPFL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `blocked-by-abi` |
| `SPLPMU` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPUP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPOPT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SREADP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SVOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `SWRITP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `USRMAT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
