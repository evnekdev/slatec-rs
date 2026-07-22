# approximation safe-coverage disposition

This generated inventory joins canonical raw status with raw-to-safe coverage. `expert-raw-only` and blocked records are explicit decisions, not missing data. Call-graph fields are recorded as unavailable where the committed authoritative inputs do not contain a complete Fortran call graph.

## Disposition counts

- `candidate-implemented-in-this-milestone`: 8
- `covered-by-higher-level-safe-api`: 10
- `deferred-incremental-bspline-preprocessing`: 2
- `deferred-stateful-constrained-bspline`: 2
- `direct-safe-wrapper`: 28
- `expert-raw-only`: 39

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
| `DEFC` | historically_user_callable_driver | `slatec_sys::approximation::defc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `deferred-incremental-bspline-preprocessing` |
| `DEFCMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFC` | historically_user_callable_driver | `slatec_sys::approximation::dfc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `deferred-stateful-constrained-bspline` |
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
| `DP1VLU` | historically_user_callable_driver | `slatec_sys::approximation::dp1vlu` | `slatec::interpolation::approximation::PolynomialFit::evaluate; slatec::interpolation::approximation::PolynomialFit::evaluate_into; slatec::interpolation::approximation::PolynomialFit::evaluate_with_derivatives` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
| `DPCOEF` | historically_user_callable_driver | `slatec_sys::approximation::dpcoef` | `slatec::interpolation::approximation::PolynomialFit::power_coefficients; slatec::interpolation::approximation::PolynomialFit::power_coefficients_at` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
| `DPLINT` | historically_user_callable_driver | `slatec_sys::interpolation::dplint` | `slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DPOLCF` | historically_user_callable_driver | `slatec_sys::interpolation::dpolcf` | `slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DPOLFT` | historically_user_callable_driver | `slatec_sys::approximation::dpolft` | `slatec::interpolation::approximation::PolynomialFit::fit; slatec::interpolation::approximation::PolynomialFit::fit_weighted` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
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
| `EFC` | historically_user_callable_driver | `slatec_sys::approximation::efc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `deferred-incremental-bspline-preprocessing` |
| `EFCMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FC` | historically_user_callable_driver | `slatec_sys::approximation::fc` | `none` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `deferred-stateful-constrained-bspline` |
| `FCMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FDJAC3` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `INTRV` | historically_user_callable_driver | `slatec_sys::interpolation::intrv` | `none` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `covered-by-higher-level-safe-api` |
| `LMPAR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LPDP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LSEI` | historically_user_callable_driver | `slatec_sys::approximation::lsei` | `slatec::constrained_least_squares::solve_constrained_least_squares_f32` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `LSI` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PCOEF` | historically_user_callable_driver | `slatec_sys::approximation::pcoef` | `slatec::interpolation::approximation::PolynomialFit::power_coefficients; slatec::interpolation::approximation::PolynomialFit::power_coefficients_at` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
| `POLCOF` | historically_user_callable_driver | `slatec_sys::interpolation::polcof` | `slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `POLFIT` | historically_user_callable_driver | `slatec_sys::approximation::polfit` | `slatec::interpolation::approximation::PolynomialFit::fit; slatec::interpolation::approximation::PolynomialFit::fit_weighted` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
| `POLINT` | historically_user_callable_driver | `slatec_sys::interpolation::polint` | `slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `POLYVL` | historically_user_callable_driver | `slatec_sys::interpolation::polyvl` | `slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate; slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate_with_derivatives` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `PPVAL` | historically_user_callable_driver | `slatec_sys::interpolation::ppval` | `slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate; slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into` | `interpolation-general` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `PVALUE` | historically_user_callable_driver | `slatec_sys::approximation::pvalue` | `slatec::interpolation::approximation::PolynomialFit::evaluate; slatec::interpolation::approximation::PolynomialFit::evaluate_into; slatec::interpolation::approximation::PolynomialFit::evaluate_with_derivatives` | `approximation-core` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `candidate-implemented-in-this-milestone` |
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
