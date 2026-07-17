# Safe function index

This index is generated from the reviewed safe-API inventories. The Rust surface is `no_std`; individual functions are classified as core-only or hosted `std`. The only validated native backend is GNU Fortran `x86_64-w64-mingw32`; this is not a bare-metal support claim.

## Alphabetical Rust API

| Rust API | Original Fortran | Domain | Precision | Purpose | Requirement | Feature | Example |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `slatec::blas::level1::dasum` | `DASUM` | BLAS | f64 | sum of absolute values | `core` | `blas-level1` | [absolute_sum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dasum_strided` | `DASUM` | BLAS | f64 | sum of absolute values | `core` | `blas-level1` | [absolute_sum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::daxpy` | `DAXPY` | BLAS | f64 | scaled vector addition | `core` | `blas-level1` | [axpy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::daxpy_strided` | `DAXPY` | BLAS | f64 | scaled vector addition | `core` | `blas-level1` | [axpy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dcopy` | `DCOPY` | BLAS | f64 | copy vector elements | `core` | `blas-level1` | [copy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dcopy_strided` | `DCOPY` | BLAS | f64 | copy vector elements | `core` | `blas-level1` | [copy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::ddot` | `DDOT` | BLAS | f64 | dot product | `core` | `blas-level1` | [dot](../../examples/blas/level1.rs) |
| `slatec::blas::level1::ddot_strided` | `DDOT` | BLAS | f64 | dot product | `core` | `blas-level1` | [dot](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dnrm2` | `DNRM2` | BLAS | f64 | Euclidean norm | `core` | `blas-level1` | [norm](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dnrm2_strided` | `DNRM2` | BLAS | f64 | Euclidean norm | `core` | `blas-level1` | [norm](../../examples/blas/level1.rs) |
| `slatec::blas::level1::drot` | `DROT` | BLAS | f64 | plane rotation | `core` | `blas-level1` | [rotation](../../examples/blas/level1.rs) |
| `slatec::blas::level1::drot_strided` | `DROT` | BLAS | f64 | plane rotation | `core` | `blas-level1` | [rotation](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dscal` | `DSCAL` | BLAS | f64 | scale a vector | `core` | `blas-level1` | [scale](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dscal_strided` | `DSCAL` | BLAS | f64 | scale a vector | `core` | `blas-level1` | [scale](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dswap` | `DSWAP` | BLAS | f64 | exchange vector elements | `core` | `blas-level1` | [swap](../../examples/blas/level1.rs) |
| `slatec::blas::level1::dswap_strided` | `DSWAP` | BLAS | f64 | exchange vector elements | `core` | `blas-level1` | [swap](../../examples/blas/level1.rs) |
| `slatec::blas::level1::idamax` | `IDAMAX` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level1` | [index_maximum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::idamax_strided` | `IDAMAX` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level1` | [index_maximum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::isamax` | `ISAMAX` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level1` | [index_maximum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::isamax_strided` | `ISAMAX` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level1` | [index_maximum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sasum` | `SASUM` | BLAS | f32 | sum of absolute values | `core` | `blas-level1` | [absolute_sum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sasum_strided` | `SASUM` | BLAS | f32 | sum of absolute values | `core` | `blas-level1` | [absolute_sum](../../examples/blas/level1.rs) |
| `slatec::blas::level1::saxpy` | `SAXPY` | BLAS | f32 | scaled vector addition | `core` | `blas-level1` | [axpy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::saxpy_strided` | `SAXPY` | BLAS | f32 | scaled vector addition | `core` | `blas-level1` | [axpy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::scopy` | `SCOPY` | BLAS | f32 | copy vector elements | `core` | `blas-level1` | [copy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::scopy_strided` | `SCOPY` | BLAS | f32 | copy vector elements | `core` | `blas-level1` | [copy](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sdot` | `SDOT` | BLAS | f32 | dot product | `core` | `blas-level1` | [dot](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sdot_strided` | `SDOT` | BLAS | f32 | dot product | `core` | `blas-level1` | [dot](../../examples/blas/level1.rs) |
| `slatec::blas::level1::snrm2` | `SNRM2` | BLAS | f32 | Euclidean norm | `core` | `blas-level1` | [norm](../../examples/blas/level1.rs) |
| `slatec::blas::level1::snrm2_strided` | `SNRM2` | BLAS | f32 | Euclidean norm | `core` | `blas-level1` | [norm](../../examples/blas/level1.rs) |
| `slatec::blas::level1::srot` | `SROT` | BLAS | f32 | plane rotation | `core` | `blas-level1` | [rotation](../../examples/blas/level1.rs) |
| `slatec::blas::level1::srot_strided` | `SROT` | BLAS | f32 | plane rotation | `core` | `blas-level1` | [rotation](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sscal` | `SSCAL` | BLAS | f32 | scale a vector | `core` | `blas-level1` | [scale](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sscal_strided` | `SSCAL` | BLAS | f32 | scale a vector | `core` | `blas-level1` | [scale](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sswap` | `SSWAP` | BLAS | f32 | exchange vector elements | `core` | `blas-level1` | [swap](../../examples/blas/level1.rs) |
| `slatec::blas::level1::sswap_strided` | `SSWAP` | BLAS | f32 | exchange vector elements | `core` | `blas-level1` | [swap](../../examples/blas/level1.rs) |
| `slatec::blas::level2::dgemv` | `DGEMV` | BLAS | f64 | general matrix-vector product | `core` | `blas-level2` | [gemv](../../examples/blas/level2.rs) |
| `slatec::blas::level2::dgemv_contiguous` | `DGEMV` | BLAS | f64 | general matrix-vector product | `core` | `blas-level2` | [gemv](../../examples/blas/level2.rs) |
| `slatec::blas::level2::dger` | `DGER` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level2` | [rank_one_update](../../examples/blas/level2.rs) |
| `slatec::blas::level2::dsymv` | `DSYMV` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level2` | [symmetric_matrix_vector](../../examples/blas/level2.rs) |
| `slatec::blas::level2::dtrmv` | `DTRMV` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level2` | [triangular_matrix_vector](../../examples/blas/level2.rs) |
| `slatec::blas::level2::dtrsv` | `DTRSV` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level2` | [triangular_solve](../../examples/blas/level2.rs) |
| `slatec::blas::level2::sgemv` | `SGEMV` | BLAS | f32 | general matrix-vector product | `core` | `blas-level2` | [gemv](../../examples/blas/level2.rs) |
| `slatec::blas::level2::sgemv_contiguous` | `SGEMV` | BLAS | f32 | general matrix-vector product | `core` | `blas-level2` | [gemv](../../examples/blas/level2.rs) |
| `slatec::blas::level2::sger` | `SGER` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level2` | [rank_one_update](../../examples/blas/level2.rs) |
| `slatec::blas::level2::ssymv` | `SSYMV` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level2` | [symmetric_matrix_vector](../../examples/blas/level2.rs) |
| `slatec::blas::level2::strmv` | `STRMV` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level2` | [triangular_matrix_vector](../../examples/blas/level2.rs) |
| `slatec::blas::level2::strsv` | `STRSV` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level2` | [triangular_solve](../../examples/blas/level2.rs) |
| `slatec::blas::level3::dgemm` | `DGEMM` | BLAS | f64 | general matrix-matrix product | `core` | `blas-level3` | [gemm](../../examples/blas/level3.rs) |
| `slatec::blas::level3::dgemm_contiguous` | `DGEMM` | BLAS | f64 | general matrix-matrix product | `core` | `blas-level3` | [gemm](../../examples/blas/level3.rs) |
| `slatec::blas::level3::dsyrk` | `DSYRK` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level3` | [symmetric_rank_k](../../examples/blas/level3.rs) |
| `slatec::blas::level3::dtrmm` | `DTRMM` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level3` | [triangular_matrix_multiply](../../examples/blas/level3.rs) |
| `slatec::blas::level3::dtrsm` | `DTRSM` | BLAS | f64 | validated scalar numerical function | `core` | `blas-level3` | [triangular_matrix_solve](../../examples/blas/level3.rs) |
| `slatec::blas::level3::sgemm` | `SGEMM` | BLAS | f32 | general matrix-matrix product | `core` | `blas-level3` | [gemm](../../examples/blas/level3.rs) |
| `slatec::blas::level3::sgemm_contiguous` | `SGEMM` | BLAS | f32 | general matrix-matrix product | `core` | `blas-level3` | [gemm](../../examples/blas/level3.rs) |
| `slatec::blas::level3::ssyrk` | `SSYRK` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level3` | [symmetric_rank_k](../../examples/blas/level3.rs) |
| `slatec::blas::level3::strmm` | `STRMM` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level3` | [triangular_matrix_multiply](../../examples/blas/level3.rs) |
| `slatec::blas::level3::strsm` | `STRSM` | BLAS | f32 | validated scalar numerical function | `core` | `blas-level3` | [triangular_matrix_solve](../../examples/blas/level3.rs) |
| `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares` | `DBOCLS` | linear least squares | f64 | bounded constrained linear least-squares fitting | `std` | `least-squares-linear-bounded-constrained` | [bounded linear least squares with bounded constraint expressions](../../examples/least_squares/bounded_constrained_fit.rs) |
| `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32` | `SBOCLS` | linear least squares | f32 | bounded constrained linear least-squares fitting | `std` | `least-squares-linear-bounded-constrained` | [bounded linear least squares with bounded constraint expressions](../../examples/least_squares/active_bound_and_constraint.rs) |
| `slatec::bounded_least_squares::solve_bounded_least_squares` | `DBOLS` | linear least squares | f64 | dense bounded linear least-squares fitting | `std` | `least-squares-linear-bounded` | [bounded linear least squares](../../examples/least_squares/bounded_fit.rs) |
| `slatec::bounded_least_squares::solve_bounded_least_squares_f32` | `SBOLS` | linear least squares | f32 | dense bounded linear least-squares fitting | `std` | `least-squares-linear-bounded` | [bounded linear least squares](../../examples/least_squares/mixed_bounds.rs) |
| `slatec::constrained_least_squares::solve_constrained_least_squares` | `DLSEI` | linear least squares | f64 | dense equality and inequality constrained linear least-squares fitting | `std` | `least-squares-linear-constrained` | [equality and inequality constrained linear least squares](../../examples/least_squares/equality_constrained_fit.rs) |
| `slatec::constrained_least_squares::solve_constrained_least_squares_f32` | `LSEI` | linear least squares | f32 | dense equality and inequality constrained linear least-squares fitting | `std` | `least-squares-linear-constrained` | [equality and inequality constrained linear least squares](../../examples/least_squares/inequality_constrained_fit.rs) |
| `slatec::least_squares::covariance_from_expert_fit` | `DCOV` | least squares | f64 | nonlinear least-squares covariance estimation | `std` | `least-squares-covariance + least-squares-nonlinear-expert` | [nonlinear least-squares covariance estimation](../../examples/least_squares/covariance_nonlinear_fit.rs) |
| `slatec::least_squares::covariance_from_expert_fit_f32` | `SCOV` | least squares | f32 | nonlinear least-squares covariance estimation | `std` | `least-squares-covariance + least-squares-nonlinear-expert` | [nonlinear least-squares covariance estimation](../../examples/least_squares/covariance_nonlinear_fit.rs) |
| `slatec::least_squares::estimate_covariance` | `DCOV` | least squares | f64 | nonlinear least-squares covariance estimation | `std` | `least-squares-covariance` | [nonlinear least-squares covariance estimation](../../examples/least_squares/covariance_nonlinear_fit.rs) |
| `slatec::least_squares::estimate_covariance_f32` | `SCOV` | least squares | f32 | nonlinear least-squares covariance estimation | `std` | `least-squares-covariance` | [nonlinear least-squares covariance estimation](../../examples/least_squares/covariance_nonlinear_fit.rs) |
| `slatec::least_squares::estimate_covariance_finite_difference` | `DCOV` | least squares | f64 | nonlinear least-squares covariance estimation | `std` | `least-squares-covariance` | [nonlinear least-squares covariance estimation](../../examples/least_squares/covariance_linear_fit.rs) |
| `slatec::least_squares::estimate_covariance_finite_difference_f32` | `SCOV` | least squares | f32 | nonlinear least-squares covariance estimation | `std` | `least-squares-covariance` | [nonlinear least-squares covariance estimation](../../examples/least_squares/covariance_linear_fit.rs) |
| `slatec::least_squares::least_squares` | `DNLS1E` | least squares | f64 | finite-difference nonlinear least-squares fitting | `std` | `least-squares-nonlinear-easy` | [finite-difference nonlinear least squares](../../examples/least_squares/linear_fit.rs) |
| `slatec::least_squares::least_squares_expert` | `DNLS1` | least squares | f64 | expert finite-difference nonlinear least-squares fitting | `std` | `least-squares-nonlinear-expert` | [expert finite-difference nonlinear least squares](../../examples/least_squares/expert_finite_difference.rs) |
| `slatec::least_squares::least_squares_expert_f32` | `SNLS1` | least squares | f32 | expert finite-difference nonlinear least-squares fitting | `std` | `least-squares-nonlinear-expert` | [expert finite-difference nonlinear least squares](../../examples/least_squares/expert_finite_difference.rs) |
| `slatec::least_squares::least_squares_f32` | `SNLS1E` | least squares | f32 | finite-difference nonlinear least-squares fitting | `std` | `least-squares-nonlinear-easy` | [finite-difference nonlinear least squares](../../examples/least_squares/linear_fit_f32.rs) |
| `slatec::least_squares::least_squares_with_jacobian` | `DNLS1` | least squares | f64 | expert analytic-Jacobian nonlinear least-squares fitting | `std` | `least-squares-nonlinear-expert` | [expert analytic-Jacobian nonlinear least squares](../../examples/least_squares/expert_analytic_jacobian.rs) |
| `slatec::least_squares::least_squares_with_jacobian_f32` | `SNLS1` | least squares | f32 | expert analytic-Jacobian nonlinear least-squares fitting | `std` | `least-squares-nonlinear-expert` | [expert analytic-Jacobian nonlinear least squares](../../examples/least_squares/expert_analytic_jacobian_f32.rs) |
| `slatec::linear_least_squares::solve_nonnegative_least_squares` | `DWNNLS` | linear least squares | f64 | equality-constrained nonnegative linear least-squares fitting | `std` | `least-squares-linear-nonnegative` | [equality-constrained nonnegative linear least squares](../../examples/least_squares/nonnegative_fit.rs) |
| `slatec::linear_least_squares::solve_nonnegative_least_squares_f32` | `WNNLS` | linear least squares | f32 | equality-constrained nonnegative linear least-squares fitting | `std` | `least-squares-linear-nonnegative` | [equality-constrained nonnegative linear least squares](../../examples/least_squares/mixed_nonnegative_fit.rs) |
| `slatec::nonlinear::check_jacobian` | `DCKDER` | nonlinear | f64 | componentwise Jacobian consistency checking | `alloc` | `nonlinear-jacobian-check` | [Jacobian consistency checking](../../examples/nonlinear/check_jacobian.rs) |
| `slatec::nonlinear::check_jacobian_f32` | `CHKDER` | nonlinear | f32 | componentwise Jacobian consistency checking | `alloc` | `nonlinear-jacobian-check` | [Jacobian consistency checking](../../examples/nonlinear/check_jacobian.rs) |
| `slatec::nonlinear::solve_system` | `DNSQE` | nonlinear | f64 | finite-difference nonlinear-system solving | `std` | `nonlinear-easy` | [finite-difference nonlinear system](../../examples/nonlinear/solve_system.rs) |
| `slatec::nonlinear::solve_system_expert` | `DNSQ` | nonlinear | f64 | expert finite-difference nonlinear-system solving | `std` | `nonlinear-expert` | [expert finite-difference nonlinear system](../../examples/nonlinear/solve_system_expert.rs) |
| `slatec::nonlinear::solve_system_expert_f32` | `SNSQ` | nonlinear | f32 | expert finite-difference nonlinear-system solving | `std` | `nonlinear-expert` | [expert finite-difference nonlinear system](../../examples/nonlinear/solve_system_expert.rs) |
| `slatec::nonlinear::solve_system_f32` | `SNSQE` | nonlinear | f32 | finite-difference nonlinear-system solving | `std` | `nonlinear-easy` | [finite-difference nonlinear system](../../examples/nonlinear/solve_system_f32.rs) |
| `slatec::nonlinear::solve_system_with_jacobian` | `DNSQ` | nonlinear | f64 | expert analytic-Jacobian nonlinear-system solving | `std` | `nonlinear-expert` | [expert analytic-Jacobian nonlinear system](../../examples/nonlinear/solve_system_with_jacobian.rs) |
| `slatec::nonlinear::solve_system_with_jacobian_f32` | `SNSQ` | nonlinear | f32 | expert analytic-Jacobian nonlinear-system solving | `std` | `nonlinear-expert` | [expert analytic-Jacobian nonlinear system](../../examples/nonlinear/solve_system_with_jacobian.rs) |
| `slatec::ode::OdeSession::<f32, F, E>::integrate_to` | `SDRIV3` | ordinary differential equations | f32 | explicit real initial-value problem y'=f(t,y) | `std` | `ode-sdrive-expert` | [explicit real initial-value problem y'=f(t,y)](../../examples/ode/harmonic_oscillator.rs) |
| `slatec::ode::OdeSession::<f64, F, E>::integrate_to` | `DDRIV3` | ordinary differential equations | f64 | explicit real initial-value problem y'=f(t,y) | `std` | `ode-sdrive-expert` | [explicit real initial-value problem y'=f(t,y)](../../examples/ode/exponential_decay.rs) |
| `slatec::polynomials::chebyshev::chebyshev_series` | `DCSEVL` | polynomials | f64 | validated scalar numerical function | `std` | `special-polynomials` | [polynomials](../../examples/special/functions.rs) |
| `slatec::polynomials::chebyshev::chebyshev_series_f32` | `CSEVL` | polynomials | f32 | validated scalar numerical function | `std` | `special-polynomials` | [polynomials](../../examples/special/functions.rs) |
| `slatec::quadrature::integrate` | `DQAG` | quadrature | f64 | adaptive finite-interval integration | `std` | `quadrature-basic` | [finite](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_f32` | `QAG` | quadrature | f32 | adaptive finite-interval integration | `std` | `quadrature-basic` | [finite](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_fourier_tail` | `DQAWF` | quadrature | f64 | validated scalar numerical function | `std` | `quadrature-fourier` | [infinite_fourier_tail](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_fourier_tail_f32` | `QAWF` | quadrature | f32 | validated scalar numerical function | `std` | `quadrature-fourier` | [infinite_fourier_tail](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_infinite` | `DQAGI` | quadrature | f64 | adaptive infinite-interval integration | `std` | `quadrature-basic` | [infinite](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_infinite_f32` | `QAGI` | quadrature | f32 | adaptive infinite-interval integration | `std` | `quadrature-basic` | [infinite](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_nc79` | `DQNC79` | quadrature | f64 | validated scalar numerical function | `std` | `quadrature-nonadaptive` | [finite_nc79](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_nc79_f32` | `QNC79` | quadrature | f32 | validated scalar numerical function | `std` | `quadrature-nonadaptive` | [finite_nc79](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_non_adaptive` | `DQNG` | quadrature | f64 | validated scalar numerical function | `std` | `quadrature-nonadaptive` | [finite_non_adaptive](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_non_adaptive_f32` | `QNG` | quadrature | f32 | validated scalar numerical function | `std` | `quadrature-nonadaptive` | [finite_non_adaptive](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_oscillatory` | `DQAWO` | quadrature | f64 | finite oscillatory integration | `std` | `quadrature-oscillatory` | [finite_oscillatory](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_oscillatory_f32` | `QAWO` | quadrature | f32 | finite oscillatory integration | `std` | `quadrature-oscillatory` | [finite_oscillatory](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_principal_value` | `DQAWC` | quadrature | f64 | validated scalar numerical function | `std` | `quadrature-basic` | [cauchy_principal_value](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_principal_value_f32` | `QAWC` | quadrature | f32 | validated scalar numerical function | `std` | `quadrature-basic` | [cauchy_principal_value](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_singular` | `DQAGS` | quadrature | f64 | validated scalar numerical function | `std` | `quadrature-basic` | [finite_endpoint_singularity](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_singular_f32` | `QAGS` | quadrature | f32 | validated scalar numerical function | `std` | `quadrature-basic` | [finite_endpoint_singularity](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_weighted_endpoints` | `DQAWS` | quadrature | f64 | validated scalar numerical function | `std` | `quadrature-weighted` | [finite_endpoint_weight](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_weighted_endpoints_f32` | `QAWS` | quadrature | f32 | validated scalar numerical function | `std` | `quadrature-weighted` | [finite_endpoint_weight](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_with_breakpoints` | `DQAGP` | quadrature | f64 | validated scalar numerical function | `std` | `quadrature-breakpoints` | [finite_breakpoints](../../examples/quadrature/families.rs) |
| `slatec::quadrature::integrate_with_breakpoints_f32` | `QAGP` | quadrature | f32 | validated scalar numerical function | `std` | `quadrature-breakpoints` | [finite_breakpoints](../../examples/quadrature/families.rs) |
| `slatec::roots::find_root` | `DFZERO` | roots | f64 | bracketed scalar root finding | `std` | `roots-scalar` | [bracketed scalar root](../../examples/roots/scalar.rs) |
| `slatec::roots::find_root_f32` | `FZERO` | roots | f32 | bracketed scalar root finding | `std` | `roots-scalar` | [bracketed scalar root](../../examples/roots/scalar.rs) |
| `slatec::special::airy::airy_ai` | `DAI` | special functions | f64 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::airy::airy_ai_f32` | `AI` | special functions | f32 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::airy::airy_ai_scaled` | `DAIE` | special functions | f64 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::airy::airy_ai_scaled_f32` | `AIE` | special functions | f32 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::airy::airy_bi` | `DBI` | special functions | f64 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::airy::airy_bi_f32` | `BI` | special functions | f32 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::airy::airy_bi_scaled` | `DBIE` | special functions | f64 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::airy::airy_bi_scaled_f32` | `BIE` | special functions | f32 | validated scalar numerical function | `std` | `special-airy` | [airy](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i0` | `DBESI0` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i0_f32` | `BESI0` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i0_scaled` | `DBSI0E` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i0_scaled_f32` | `BESI0E` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i1` | `DBESI1` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i1_f32` | `BESI1` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i1_scaled` | `DBSI1E` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_i1_scaled_f32` | `BESI1E` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_j0` | `DBESJ0` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_j0_f32` | `BESJ0` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_j1` | `DBESJ1` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_j1_f32` | `BESJ1` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k0` | `DBESK0` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k0_f32` | `BESK0` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k0_scaled` | `DBSK0E` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k0_scaled_f32` | `BESK0E` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k1` | `DBESK1` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k1_f32` | `BESK1` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k1_scaled` | `DBSK1E` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_k1_scaled_f32` | `BESK1E` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_y0` | `DBESY0` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_y0_f32` | `BESY0` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_y1` | `DBESY1` | special functions | f64 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::bessel::bessel_y1_f32` | `BESY1` | special functions | f32 | validated scalar numerical function | `std` | `special-bessel` | [bessel](../../examples/special/functions.rs) |
| `slatec::special::elementary::cbrt` | `DCBRT` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::cbrt_f32` | `CBRT` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::cos_degrees` | `DCOSDG` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::cos_degrees_f32` | `COSDG` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::dawson` | `DDAWS` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::dawson_f32` | `DAWS` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::exprel` | `DEXPRL` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::exprel_f32` | `EXPREL` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::log1p` | `DLNREL` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::log1p_f32` | `ALNREL` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::sin_degrees` | `DSINDG` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::elementary::sin_degrees_f32` | `SINDG` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::error_functions::erf` | `DERF` | special functions | f64 | validated scalar numerical function | `std` | `special-error` | [error_functions](../../examples/special/functions.rs) |
| `slatec::special::error_functions::erf_f32` | `ERF` | special functions | f32 | validated scalar numerical function | `std` | `special-error` | [error_functions](../../examples/special/functions.rs) |
| `slatec::special::error_functions::erfc` | `DERFC` | special functions | f64 | validated scalar numerical function | `std` | `special-error` | [error_functions](../../examples/special/functions.rs) |
| `slatec::special::error_functions::erfc_f32` | `ERFC` | special functions | f32 | validated scalar numerical function | `std` | `special-error` | [error_functions](../../examples/special/functions.rs) |
| `slatec::special::gamma::beta` | `DBETA` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::beta_f32` | `BETA` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::binomial_coefficient` | `DBINOM` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::gamma::binomial_coefficient_f32` | `BINOM` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::gamma::digamma` | `DPSI` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::digamma_f32` | `PSI` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::factorial` | `DFAC` | special functions | f64 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::gamma::factorial_f32` | `FAC` | special functions | f32 | validated scalar numerical function | `std` | `special-elementary` | [elementary](../../examples/special/functions.rs) |
| `slatec::special::gamma::gamma` | `DGAMMA` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::gamma_f32` | `GAMMA` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::incomplete_gamma_lower` | `DGAMI` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::incomplete_gamma_lower_f32` | `GAMI` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::incomplete_gamma_upper` | `DGAMIC` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::incomplete_gamma_upper_f32` | `GAMIC` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::log_beta` | `DLBETA` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::log_beta_f32` | `ALBETA` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::log_gamma` | `DLNGAM` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::log_gamma_f32` | `ALNGAM` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::reciprocal_gamma` | `DGAMR` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::reciprocal_gamma_f32` | `GAMR` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::regularized_beta` | `DBETAI` | special functions | f64 | validated scalar numerical function | `std` | `special` | [probability](../../examples/special/functions.rs) |
| `slatec::special::gamma::regularized_beta_f32` | `BETAI` | special functions | f32 | validated scalar numerical function | `std` | `special` | [probability](../../examples/special/functions.rs) |
| `slatec::special::gamma::tricomi_incomplete_gamma` | `DGAMIT` | special functions | f64 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::gamma::tricomi_incomplete_gamma_f32` | `GAMIT` | special functions | f32 | validated scalar numerical function | `std` | `special` | [gamma_beta](../../examples/special/functions.rs) |
| `slatec::special::integrals::exponential_integral_e1` | `DE1` | special functions | f64 | validated scalar numerical function | `std` | `special-integrals` | [integrals](../../examples/special/functions.rs) |
| `slatec::special::integrals::exponential_integral_e1_f32` | `E1` | special functions | f32 | validated scalar numerical function | `std` | `special-integrals` | [integrals](../../examples/special/functions.rs) |
| `slatec::special::integrals::exponential_integral_ei` | `DEI` | special functions | f64 | validated scalar numerical function | `std` | `special-integrals` | [integrals](../../examples/special/functions.rs) |
| `slatec::special::integrals::exponential_integral_ei_f32` | `EI` | special functions | f32 | validated scalar numerical function | `std` | `special-integrals` | [integrals](../../examples/special/functions.rs) |

## Original Fortran routine index

- `AI` -> `slatec::special::airy::airy_ai_f32`
- `AIE` -> `slatec::special::airy::airy_ai_scaled_f32`
- `ALBETA` -> `slatec::special::gamma::log_beta_f32`
- `ALNGAM` -> `slatec::special::gamma::log_gamma_f32`
- `ALNREL` -> `slatec::special::elementary::log1p_f32`
- `BESI0` -> `slatec::special::bessel::bessel_i0_f32`
- `BESI0E` -> `slatec::special::bessel::bessel_i0_scaled_f32`
- `BESI1` -> `slatec::special::bessel::bessel_i1_f32`
- `BESI1E` -> `slatec::special::bessel::bessel_i1_scaled_f32`
- `BESJ0` -> `slatec::special::bessel::bessel_j0_f32`
- `BESJ1` -> `slatec::special::bessel::bessel_j1_f32`
- `BESK0` -> `slatec::special::bessel::bessel_k0_f32`
- `BESK0E` -> `slatec::special::bessel::bessel_k0_scaled_f32`
- `BESK1` -> `slatec::special::bessel::bessel_k1_f32`
- `BESK1E` -> `slatec::special::bessel::bessel_k1_scaled_f32`
- `BESY0` -> `slatec::special::bessel::bessel_y0_f32`
- `BESY1` -> `slatec::special::bessel::bessel_y1_f32`
- `BETA` -> `slatec::special::gamma::beta_f32`
- `BETAI` -> `slatec::special::gamma::regularized_beta_f32`
- `BI` -> `slatec::special::airy::airy_bi_f32`
- `BIE` -> `slatec::special::airy::airy_bi_scaled_f32`
- `BINOM` -> `slatec::special::gamma::binomial_coefficient_f32`
- `CBRT` -> `slatec::special::elementary::cbrt_f32`
- `CHKDER` -> `slatec::nonlinear::check_jacobian_f32`
- `COSDG` -> `slatec::special::elementary::cos_degrees_f32`
- `CSEVL` -> `slatec::polynomials::chebyshev::chebyshev_series_f32`
- `DAI` -> `slatec::special::airy::airy_ai`
- `DAIE` -> `slatec::special::airy::airy_ai_scaled`
- `DASUM` -> `slatec::blas::level1::dasum`
- `DASUM` -> `slatec::blas::level1::dasum_strided`
- `DAWS` -> `slatec::special::elementary::dawson_f32`
- `DAXPY` -> `slatec::blas::level1::daxpy`
- `DAXPY` -> `slatec::blas::level1::daxpy_strided`
- `DBESI0` -> `slatec::special::bessel::bessel_i0`
- `DBESI1` -> `slatec::special::bessel::bessel_i1`
- `DBESJ0` -> `slatec::special::bessel::bessel_j0`
- `DBESJ1` -> `slatec::special::bessel::bessel_j1`
- `DBESK0` -> `slatec::special::bessel::bessel_k0`
- `DBESK1` -> `slatec::special::bessel::bessel_k1`
- `DBESY0` -> `slatec::special::bessel::bessel_y0`
- `DBESY1` -> `slatec::special::bessel::bessel_y1`
- `DBETA` -> `slatec::special::gamma::beta`
- `DBETAI` -> `slatec::special::gamma::regularized_beta`
- `DBI` -> `slatec::special::airy::airy_bi`
- `DBIE` -> `slatec::special::airy::airy_bi_scaled`
- `DBINOM` -> `slatec::special::gamma::binomial_coefficient`
- `DBOCLS` -> `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares`
- `DBOLS` -> `slatec::bounded_least_squares::solve_bounded_least_squares`
- `DBSI0E` -> `slatec::special::bessel::bessel_i0_scaled`
- `DBSI1E` -> `slatec::special::bessel::bessel_i1_scaled`
- `DBSK0E` -> `slatec::special::bessel::bessel_k0_scaled`
- `DBSK1E` -> `slatec::special::bessel::bessel_k1_scaled`
- `DCBRT` -> `slatec::special::elementary::cbrt`
- `DCKDER` -> `slatec::nonlinear::check_jacobian`
- `DCOPY` -> `slatec::blas::level1::dcopy`
- `DCOPY` -> `slatec::blas::level1::dcopy_strided`
- `DCOSDG` -> `slatec::special::elementary::cos_degrees`
- `DCOV` -> `slatec::least_squares::covariance_from_expert_fit`
- `DCOV` -> `slatec::least_squares::estimate_covariance`
- `DCOV` -> `slatec::least_squares::estimate_covariance_finite_difference`
- `DCSEVL` -> `slatec::polynomials::chebyshev::chebyshev_series`
- `DDAWS` -> `slatec::special::elementary::dawson`
- `DDOT` -> `slatec::blas::level1::ddot`
- `DDOT` -> `slatec::blas::level1::ddot_strided`
- `DDRIV3` -> `slatec::ode::OdeSession::<f64, F, E>::integrate_to`
- `DE1` -> `slatec::special::integrals::exponential_integral_e1`
- `DEI` -> `slatec::special::integrals::exponential_integral_ei`
- `DERF` -> `slatec::special::error_functions::erf`
- `DERFC` -> `slatec::special::error_functions::erfc`
- `DEXPRL` -> `slatec::special::elementary::exprel`
- `DFAC` -> `slatec::special::gamma::factorial`
- `DFZERO` -> `slatec::roots::find_root`
- `DGAMI` -> `slatec::special::gamma::incomplete_gamma_lower`
- `DGAMIC` -> `slatec::special::gamma::incomplete_gamma_upper`
- `DGAMIT` -> `slatec::special::gamma::tricomi_incomplete_gamma`
- `DGAMMA` -> `slatec::special::gamma::gamma`
- `DGAMR` -> `slatec::special::gamma::reciprocal_gamma`
- `DGEMM` -> `slatec::blas::level3::dgemm`
- `DGEMM` -> `slatec::blas::level3::dgemm_contiguous`
- `DGEMV` -> `slatec::blas::level2::dgemv`
- `DGEMV` -> `slatec::blas::level2::dgemv_contiguous`
- `DGER` -> `slatec::blas::level2::dger`
- `DLBETA` -> `slatec::special::gamma::log_beta`
- `DLNGAM` -> `slatec::special::gamma::log_gamma`
- `DLNREL` -> `slatec::special::elementary::log1p`
- `DLSEI` -> `slatec::constrained_least_squares::solve_constrained_least_squares`
- `DNLS1` -> `slatec::least_squares::least_squares_expert`
- `DNLS1` -> `slatec::least_squares::least_squares_with_jacobian`
- `DNLS1E` -> `slatec::least_squares::least_squares`
- `DNRM2` -> `slatec::blas::level1::dnrm2`
- `DNRM2` -> `slatec::blas::level1::dnrm2_strided`
- `DNSQ` -> `slatec::nonlinear::solve_system_expert`
- `DNSQ` -> `slatec::nonlinear::solve_system_with_jacobian`
- `DNSQE` -> `slatec::nonlinear::solve_system`
- `DPSI` -> `slatec::special::gamma::digamma`
- `DQAG` -> `slatec::quadrature::integrate`
- `DQAGI` -> `slatec::quadrature::integrate_infinite`
- `DQAGP` -> `slatec::quadrature::integrate_with_breakpoints`
- `DQAGS` -> `slatec::quadrature::integrate_singular`
- `DQAWC` -> `slatec::quadrature::integrate_principal_value`
- `DQAWF` -> `slatec::quadrature::integrate_fourier_tail`
- `DQAWO` -> `slatec::quadrature::integrate_oscillatory`
- `DQAWS` -> `slatec::quadrature::integrate_weighted_endpoints`
- `DQNC79` -> `slatec::quadrature::integrate_nc79`
- `DQNG` -> `slatec::quadrature::integrate_non_adaptive`
- `DROT` -> `slatec::blas::level1::drot`
- `DROT` -> `slatec::blas::level1::drot_strided`
- `DSCAL` -> `slatec::blas::level1::dscal`
- `DSCAL` -> `slatec::blas::level1::dscal_strided`
- `DSINDG` -> `slatec::special::elementary::sin_degrees`
- `DSWAP` -> `slatec::blas::level1::dswap`
- `DSWAP` -> `slatec::blas::level1::dswap_strided`
- `DSYMV` -> `slatec::blas::level2::dsymv`
- `DSYRK` -> `slatec::blas::level3::dsyrk`
- `DTRMM` -> `slatec::blas::level3::dtrmm`
- `DTRMV` -> `slatec::blas::level2::dtrmv`
- `DTRSM` -> `slatec::blas::level3::dtrsm`
- `DTRSV` -> `slatec::blas::level2::dtrsv`
- `DWNNLS` -> `slatec::linear_least_squares::solve_nonnegative_least_squares`
- `E1` -> `slatec::special::integrals::exponential_integral_e1_f32`
- `EI` -> `slatec::special::integrals::exponential_integral_ei_f32`
- `ERF` -> `slatec::special::error_functions::erf_f32`
- `ERFC` -> `slatec::special::error_functions::erfc_f32`
- `EXPREL` -> `slatec::special::elementary::exprel_f32`
- `FAC` -> `slatec::special::gamma::factorial_f32`
- `FZERO` -> `slatec::roots::find_root_f32`
- `GAMI` -> `slatec::special::gamma::incomplete_gamma_lower_f32`
- `GAMIC` -> `slatec::special::gamma::incomplete_gamma_upper_f32`
- `GAMIT` -> `slatec::special::gamma::tricomi_incomplete_gamma_f32`
- `GAMMA` -> `slatec::special::gamma::gamma_f32`
- `GAMR` -> `slatec::special::gamma::reciprocal_gamma_f32`
- `IDAMAX` -> `slatec::blas::level1::idamax`
- `IDAMAX` -> `slatec::blas::level1::idamax_strided`
- `ISAMAX` -> `slatec::blas::level1::isamax`
- `ISAMAX` -> `slatec::blas::level1::isamax_strided`
- `LSEI` -> `slatec::constrained_least_squares::solve_constrained_least_squares_f32`
- `PSI` -> `slatec::special::gamma::digamma_f32`
- `QAG` -> `slatec::quadrature::integrate_f32`
- `QAGI` -> `slatec::quadrature::integrate_infinite_f32`
- `QAGP` -> `slatec::quadrature::integrate_with_breakpoints_f32`
- `QAGS` -> `slatec::quadrature::integrate_singular_f32`
- `QAWC` -> `slatec::quadrature::integrate_principal_value_f32`
- `QAWF` -> `slatec::quadrature::integrate_fourier_tail_f32`
- `QAWO` -> `slatec::quadrature::integrate_oscillatory_f32`
- `QAWS` -> `slatec::quadrature::integrate_weighted_endpoints_f32`
- `QNC79` -> `slatec::quadrature::integrate_nc79_f32`
- `QNG` -> `slatec::quadrature::integrate_non_adaptive_f32`
- `SASUM` -> `slatec::blas::level1::sasum`
- `SASUM` -> `slatec::blas::level1::sasum_strided`
- `SAXPY` -> `slatec::blas::level1::saxpy`
- `SAXPY` -> `slatec::blas::level1::saxpy_strided`
- `SBOCLS` -> `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32`
- `SBOLS` -> `slatec::bounded_least_squares::solve_bounded_least_squares_f32`
- `SCOPY` -> `slatec::blas::level1::scopy`
- `SCOPY` -> `slatec::blas::level1::scopy_strided`
- `SCOV` -> `slatec::least_squares::covariance_from_expert_fit_f32`
- `SCOV` -> `slatec::least_squares::estimate_covariance_f32`
- `SCOV` -> `slatec::least_squares::estimate_covariance_finite_difference_f32`
- `SDOT` -> `slatec::blas::level1::sdot`
- `SDOT` -> `slatec::blas::level1::sdot_strided`
- `SDRIV3` -> `slatec::ode::OdeSession::<f32, F, E>::integrate_to`
- `SGEMM` -> `slatec::blas::level3::sgemm`
- `SGEMM` -> `slatec::blas::level3::sgemm_contiguous`
- `SGEMV` -> `slatec::blas::level2::sgemv`
- `SGEMV` -> `slatec::blas::level2::sgemv_contiguous`
- `SGER` -> `slatec::blas::level2::sger`
- `SINDG` -> `slatec::special::elementary::sin_degrees_f32`
- `SNLS1` -> `slatec::least_squares::least_squares_expert_f32`
- `SNLS1` -> `slatec::least_squares::least_squares_with_jacobian_f32`
- `SNLS1E` -> `slatec::least_squares::least_squares_f32`
- `SNRM2` -> `slatec::blas::level1::snrm2`
- `SNRM2` -> `slatec::blas::level1::snrm2_strided`
- `SNSQ` -> `slatec::nonlinear::solve_system_expert_f32`
- `SNSQ` -> `slatec::nonlinear::solve_system_with_jacobian_f32`
- `SNSQE` -> `slatec::nonlinear::solve_system_f32`
- `SROT` -> `slatec::blas::level1::srot`
- `SROT` -> `slatec::blas::level1::srot_strided`
- `SSCAL` -> `slatec::blas::level1::sscal`
- `SSCAL` -> `slatec::blas::level1::sscal_strided`
- `SSWAP` -> `slatec::blas::level1::sswap`
- `SSWAP` -> `slatec::blas::level1::sswap_strided`
- `SSYMV` -> `slatec::blas::level2::ssymv`
- `SSYRK` -> `slatec::blas::level3::ssyrk`
- `STRMM` -> `slatec::blas::level3::strmm`
- `STRMV` -> `slatec::blas::level2::strmv`
- `STRSM` -> `slatec::blas::level3::strsm`
- `STRSV` -> `slatec::blas::level2::strsv`
- `WNNLS` -> `slatec::linear_least_squares::solve_nonnegative_least_squares_f32`

## Domain index

### BLAS

- `slatec::blas::level1::dasum` — sum of absolute values
- `slatec::blas::level1::dasum_strided` — sum of absolute values
- `slatec::blas::level1::daxpy` — scaled vector addition
- `slatec::blas::level1::daxpy_strided` — scaled vector addition
- `slatec::blas::level1::dcopy` — copy vector elements
- `slatec::blas::level1::dcopy_strided` — copy vector elements
- `slatec::blas::level1::ddot` — dot product
- `slatec::blas::level1::ddot_strided` — dot product
- `slatec::blas::level1::dnrm2` — Euclidean norm
- `slatec::blas::level1::dnrm2_strided` — Euclidean norm
- `slatec::blas::level1::drot` — plane rotation
- `slatec::blas::level1::drot_strided` — plane rotation
- `slatec::blas::level1::dscal` — scale a vector
- `slatec::blas::level1::dscal_strided` — scale a vector
- `slatec::blas::level1::dswap` — exchange vector elements
- `slatec::blas::level1::dswap_strided` — exchange vector elements
- `slatec::blas::level1::idamax` — validated scalar numerical function
- `slatec::blas::level1::idamax_strided` — validated scalar numerical function
- `slatec::blas::level1::isamax` — validated scalar numerical function
- `slatec::blas::level1::isamax_strided` — validated scalar numerical function
- `slatec::blas::level1::sasum` — sum of absolute values
- `slatec::blas::level1::sasum_strided` — sum of absolute values
- `slatec::blas::level1::saxpy` — scaled vector addition
- `slatec::blas::level1::saxpy_strided` — scaled vector addition
- `slatec::blas::level1::scopy` — copy vector elements
- `slatec::blas::level1::scopy_strided` — copy vector elements
- `slatec::blas::level1::sdot` — dot product
- `slatec::blas::level1::sdot_strided` — dot product
- `slatec::blas::level1::snrm2` — Euclidean norm
- `slatec::blas::level1::snrm2_strided` — Euclidean norm
- `slatec::blas::level1::srot` — plane rotation
- `slatec::blas::level1::srot_strided` — plane rotation
- `slatec::blas::level1::sscal` — scale a vector
- `slatec::blas::level1::sscal_strided` — scale a vector
- `slatec::blas::level1::sswap` — exchange vector elements
- `slatec::blas::level1::sswap_strided` — exchange vector elements
- `slatec::blas::level2::dgemv` — general matrix-vector product
- `slatec::blas::level2::dgemv_contiguous` — general matrix-vector product
- `slatec::blas::level2::dger` — validated scalar numerical function
- `slatec::blas::level2::dsymv` — validated scalar numerical function
- `slatec::blas::level2::dtrmv` — validated scalar numerical function
- `slatec::blas::level2::dtrsv` — validated scalar numerical function
- `slatec::blas::level2::sgemv` — general matrix-vector product
- `slatec::blas::level2::sgemv_contiguous` — general matrix-vector product
- `slatec::blas::level2::sger` — validated scalar numerical function
- `slatec::blas::level2::ssymv` — validated scalar numerical function
- `slatec::blas::level2::strmv` — validated scalar numerical function
- `slatec::blas::level2::strsv` — validated scalar numerical function
- `slatec::blas::level3::dgemm` — general matrix-matrix product
- `slatec::blas::level3::dgemm_contiguous` — general matrix-matrix product
- `slatec::blas::level3::dsyrk` — validated scalar numerical function
- `slatec::blas::level3::dtrmm` — validated scalar numerical function
- `slatec::blas::level3::dtrsm` — validated scalar numerical function
- `slatec::blas::level3::sgemm` — general matrix-matrix product
- `slatec::blas::level3::sgemm_contiguous` — general matrix-matrix product
- `slatec::blas::level3::ssyrk` — validated scalar numerical function
- `slatec::blas::level3::strmm` — validated scalar numerical function
- `slatec::blas::level3::strsm` — validated scalar numerical function

### special functions

- `slatec::special::airy::airy_ai` — validated scalar numerical function
- `slatec::special::airy::airy_ai_f32` — validated scalar numerical function
- `slatec::special::airy::airy_ai_scaled` — validated scalar numerical function
- `slatec::special::airy::airy_ai_scaled_f32` — validated scalar numerical function
- `slatec::special::airy::airy_bi` — validated scalar numerical function
- `slatec::special::airy::airy_bi_f32` — validated scalar numerical function
- `slatec::special::airy::airy_bi_scaled` — validated scalar numerical function
- `slatec::special::airy::airy_bi_scaled_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_i0` — validated scalar numerical function
- `slatec::special::bessel::bessel_i0_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_i0_scaled` — validated scalar numerical function
- `slatec::special::bessel::bessel_i0_scaled_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_i1` — validated scalar numerical function
- `slatec::special::bessel::bessel_i1_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_i1_scaled` — validated scalar numerical function
- `slatec::special::bessel::bessel_i1_scaled_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_j0` — validated scalar numerical function
- `slatec::special::bessel::bessel_j0_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_j1` — validated scalar numerical function
- `slatec::special::bessel::bessel_j1_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_k0` — validated scalar numerical function
- `slatec::special::bessel::bessel_k0_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_k0_scaled` — validated scalar numerical function
- `slatec::special::bessel::bessel_k0_scaled_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_k1` — validated scalar numerical function
- `slatec::special::bessel::bessel_k1_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_k1_scaled` — validated scalar numerical function
- `slatec::special::bessel::bessel_k1_scaled_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_y0` — validated scalar numerical function
- `slatec::special::bessel::bessel_y0_f32` — validated scalar numerical function
- `slatec::special::bessel::bessel_y1` — validated scalar numerical function
- `slatec::special::bessel::bessel_y1_f32` — validated scalar numerical function
- `slatec::special::elementary::cbrt` — validated scalar numerical function
- `slatec::special::elementary::cbrt_f32` — validated scalar numerical function
- `slatec::special::elementary::cos_degrees` — validated scalar numerical function
- `slatec::special::elementary::cos_degrees_f32` — validated scalar numerical function
- `slatec::special::elementary::dawson` — validated scalar numerical function
- `slatec::special::elementary::dawson_f32` — validated scalar numerical function
- `slatec::special::elementary::exprel` — validated scalar numerical function
- `slatec::special::elementary::exprel_f32` — validated scalar numerical function
- `slatec::special::elementary::log1p` — validated scalar numerical function
- `slatec::special::elementary::log1p_f32` — validated scalar numerical function
- `slatec::special::elementary::sin_degrees` — validated scalar numerical function
- `slatec::special::elementary::sin_degrees_f32` — validated scalar numerical function
- `slatec::special::error_functions::erf` — validated scalar numerical function
- `slatec::special::error_functions::erf_f32` — validated scalar numerical function
- `slatec::special::error_functions::erfc` — validated scalar numerical function
- `slatec::special::error_functions::erfc_f32` — validated scalar numerical function
- `slatec::special::gamma::beta` — validated scalar numerical function
- `slatec::special::gamma::beta_f32` — validated scalar numerical function
- `slatec::special::gamma::binomial_coefficient` — validated scalar numerical function
- `slatec::special::gamma::binomial_coefficient_f32` — validated scalar numerical function
- `slatec::special::gamma::digamma` — validated scalar numerical function
- `slatec::special::gamma::digamma_f32` — validated scalar numerical function
- `slatec::special::gamma::factorial` — validated scalar numerical function
- `slatec::special::gamma::factorial_f32` — validated scalar numerical function
- `slatec::special::gamma::gamma` — validated scalar numerical function
- `slatec::special::gamma::gamma_f32` — validated scalar numerical function
- `slatec::special::gamma::incomplete_gamma_lower` — validated scalar numerical function
- `slatec::special::gamma::incomplete_gamma_lower_f32` — validated scalar numerical function
- `slatec::special::gamma::incomplete_gamma_upper` — validated scalar numerical function
- `slatec::special::gamma::incomplete_gamma_upper_f32` — validated scalar numerical function
- `slatec::special::gamma::log_beta` — validated scalar numerical function
- `slatec::special::gamma::log_beta_f32` — validated scalar numerical function
- `slatec::special::gamma::log_gamma` — validated scalar numerical function
- `slatec::special::gamma::log_gamma_f32` — validated scalar numerical function
- `slatec::special::gamma::reciprocal_gamma` — validated scalar numerical function
- `slatec::special::gamma::reciprocal_gamma_f32` — validated scalar numerical function
- `slatec::special::gamma::regularized_beta` — validated scalar numerical function
- `slatec::special::gamma::regularized_beta_f32` — validated scalar numerical function
- `slatec::special::gamma::tricomi_incomplete_gamma` — validated scalar numerical function
- `slatec::special::gamma::tricomi_incomplete_gamma_f32` — validated scalar numerical function
- `slatec::special::integrals::exponential_integral_e1` — validated scalar numerical function
- `slatec::special::integrals::exponential_integral_e1_f32` — validated scalar numerical function
- `slatec::special::integrals::exponential_integral_ei` — validated scalar numerical function
- `slatec::special::integrals::exponential_integral_ei_f32` — validated scalar numerical function

### polynomials

- `slatec::polynomials::chebyshev::chebyshev_series` — validated scalar numerical function
- `slatec::polynomials::chebyshev::chebyshev_series_f32` — validated scalar numerical function

### quadrature

- `slatec::quadrature::integrate` — adaptive finite-interval integration
- `slatec::quadrature::integrate_f32` — adaptive finite-interval integration
- `slatec::quadrature::integrate_fourier_tail` — validated scalar numerical function
- `slatec::quadrature::integrate_fourier_tail_f32` — validated scalar numerical function
- `slatec::quadrature::integrate_infinite` — adaptive infinite-interval integration
- `slatec::quadrature::integrate_infinite_f32` — adaptive infinite-interval integration
- `slatec::quadrature::integrate_nc79` — validated scalar numerical function
- `slatec::quadrature::integrate_nc79_f32` — validated scalar numerical function
- `slatec::quadrature::integrate_non_adaptive` — validated scalar numerical function
- `slatec::quadrature::integrate_non_adaptive_f32` — validated scalar numerical function
- `slatec::quadrature::integrate_oscillatory` — finite oscillatory integration
- `slatec::quadrature::integrate_oscillatory_f32` — finite oscillatory integration
- `slatec::quadrature::integrate_principal_value` — validated scalar numerical function
- `slatec::quadrature::integrate_principal_value_f32` — validated scalar numerical function
- `slatec::quadrature::integrate_singular` — validated scalar numerical function
- `slatec::quadrature::integrate_singular_f32` — validated scalar numerical function
- `slatec::quadrature::integrate_weighted_endpoints` — validated scalar numerical function
- `slatec::quadrature::integrate_weighted_endpoints_f32` — validated scalar numerical function
- `slatec::quadrature::integrate_with_breakpoints` — validated scalar numerical function
- `slatec::quadrature::integrate_with_breakpoints_f32` — validated scalar numerical function

### roots

- `slatec::roots::find_root` — bracketed scalar root finding
- `slatec::roots::find_root_f32` — bracketed scalar root finding

### nonlinear

- `slatec::nonlinear::check_jacobian` — componentwise Jacobian consistency checking
- `slatec::nonlinear::check_jacobian_f32` — componentwise Jacobian consistency checking
- `slatec::nonlinear::solve_system` — finite-difference nonlinear-system solving
- `slatec::nonlinear::solve_system_expert` — expert finite-difference nonlinear-system solving
- `slatec::nonlinear::solve_system_expert_f32` — expert finite-difference nonlinear-system solving
- `slatec::nonlinear::solve_system_f32` — finite-difference nonlinear-system solving
- `slatec::nonlinear::solve_system_with_jacobian` — expert analytic-Jacobian nonlinear-system solving
- `slatec::nonlinear::solve_system_with_jacobian_f32` — expert analytic-Jacobian nonlinear-system solving

### least squares

- `slatec::least_squares::covariance_from_expert_fit` — nonlinear least-squares covariance estimation
- `slatec::least_squares::covariance_from_expert_fit_f32` — nonlinear least-squares covariance estimation
- `slatec::least_squares::estimate_covariance` — nonlinear least-squares covariance estimation
- `slatec::least_squares::estimate_covariance_f32` — nonlinear least-squares covariance estimation
- `slatec::least_squares::estimate_covariance_finite_difference` — nonlinear least-squares covariance estimation
- `slatec::least_squares::estimate_covariance_finite_difference_f32` — nonlinear least-squares covariance estimation
- `slatec::least_squares::least_squares` — finite-difference nonlinear least-squares fitting
- `slatec::least_squares::least_squares_expert` — expert finite-difference nonlinear least-squares fitting
- `slatec::least_squares::least_squares_expert_f32` — expert finite-difference nonlinear least-squares fitting
- `slatec::least_squares::least_squares_f32` — finite-difference nonlinear least-squares fitting
- `slatec::least_squares::least_squares_with_jacobian` — expert analytic-Jacobian nonlinear least-squares fitting
- `slatec::least_squares::least_squares_with_jacobian_f32` — expert analytic-Jacobian nonlinear least-squares fitting

### linear least squares

- `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares` — bounded constrained linear least-squares fitting
- `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32` — bounded constrained linear least-squares fitting
- `slatec::bounded_least_squares::solve_bounded_least_squares` — dense bounded linear least-squares fitting
- `slatec::bounded_least_squares::solve_bounded_least_squares_f32` — dense bounded linear least-squares fitting
- `slatec::constrained_least_squares::solve_constrained_least_squares` — dense equality and inequality constrained linear least-squares fitting
- `slatec::constrained_least_squares::solve_constrained_least_squares_f32` — dense equality and inequality constrained linear least-squares fitting
- `slatec::linear_least_squares::solve_nonnegative_least_squares` — equality-constrained nonnegative linear least-squares fitting
- `slatec::linear_least_squares::solve_nonnegative_least_squares_f32` — equality-constrained nonnegative linear least-squares fitting

### ordinary differential equations

- `slatec::ode::OdeSession::<f32, F, E>::integrate_to` — explicit real initial-value problem y'=f(t,y)
- `slatec::ode::OdeSession::<f64, F, E>::integrate_to` — explicit real initial-value problem y'=f(t,y)

## Capability index

### Core only

- `slatec::blas::level1::dasum`
- `slatec::blas::level1::dasum_strided`
- `slatec::blas::level1::daxpy`
- `slatec::blas::level1::daxpy_strided`
- `slatec::blas::level1::dcopy`
- `slatec::blas::level1::dcopy_strided`
- `slatec::blas::level1::ddot`
- `slatec::blas::level1::ddot_strided`
- `slatec::blas::level1::dnrm2`
- `slatec::blas::level1::dnrm2_strided`
- `slatec::blas::level1::drot`
- `slatec::blas::level1::drot_strided`
- `slatec::blas::level1::dscal`
- `slatec::blas::level1::dscal_strided`
- `slatec::blas::level1::dswap`
- `slatec::blas::level1::dswap_strided`
- `slatec::blas::level1::idamax`
- `slatec::blas::level1::idamax_strided`
- `slatec::blas::level1::isamax`
- `slatec::blas::level1::isamax_strided`
- `slatec::blas::level1::sasum`
- `slatec::blas::level1::sasum_strided`
- `slatec::blas::level1::saxpy`
- `slatec::blas::level1::saxpy_strided`
- `slatec::blas::level1::scopy`
- `slatec::blas::level1::scopy_strided`
- `slatec::blas::level1::sdot`
- `slatec::blas::level1::sdot_strided`
- `slatec::blas::level1::snrm2`
- `slatec::blas::level1::snrm2_strided`
- `slatec::blas::level1::srot`
- `slatec::blas::level1::srot_strided`
- `slatec::blas::level1::sscal`
- `slatec::blas::level1::sscal_strided`
- `slatec::blas::level1::sswap`
- `slatec::blas::level1::sswap_strided`
- `slatec::blas::level2::dgemv`
- `slatec::blas::level2::dgemv_contiguous`
- `slatec::blas::level2::dger`
- `slatec::blas::level2::dsymv`
- `slatec::blas::level2::dtrmv`
- `slatec::blas::level2::dtrsv`
- `slatec::blas::level2::sgemv`
- `slatec::blas::level2::sgemv_contiguous`
- `slatec::blas::level2::sger`
- `slatec::blas::level2::ssymv`
- `slatec::blas::level2::strmv`
- `slatec::blas::level2::strsv`
- `slatec::blas::level3::dgemm`
- `slatec::blas::level3::dgemm_contiguous`
- `slatec::blas::level3::dsyrk`
- `slatec::blas::level3::dtrmm`
- `slatec::blas::level3::dtrsm`
- `slatec::blas::level3::sgemm`
- `slatec::blas::level3::sgemm_contiguous`
- `slatec::blas::level3::ssyrk`
- `slatec::blas::level3::strmm`
- `slatec::blas::level3::strsm`

### Requires `alloc`

- `slatec::nonlinear::check_jacobian`
- `slatec::nonlinear::check_jacobian_f32`

### Requires `std`

- `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares`
- `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32`
- `slatec::bounded_least_squares::solve_bounded_least_squares`
- `slatec::bounded_least_squares::solve_bounded_least_squares_f32`
- `slatec::constrained_least_squares::solve_constrained_least_squares`
- `slatec::constrained_least_squares::solve_constrained_least_squares_f32`
- `slatec::least_squares::covariance_from_expert_fit`
- `slatec::least_squares::covariance_from_expert_fit_f32`
- `slatec::least_squares::estimate_covariance`
- `slatec::least_squares::estimate_covariance_f32`
- `slatec::least_squares::estimate_covariance_finite_difference`
- `slatec::least_squares::estimate_covariance_finite_difference_f32`
- `slatec::least_squares::least_squares`
- `slatec::least_squares::least_squares_expert`
- `slatec::least_squares::least_squares_expert_f32`
- `slatec::least_squares::least_squares_f32`
- `slatec::least_squares::least_squares_with_jacobian`
- `slatec::least_squares::least_squares_with_jacobian_f32`
- `slatec::linear_least_squares::solve_nonnegative_least_squares`
- `slatec::linear_least_squares::solve_nonnegative_least_squares_f32`
- `slatec::nonlinear::solve_system`
- `slatec::nonlinear::solve_system_expert`
- `slatec::nonlinear::solve_system_expert_f32`
- `slatec::nonlinear::solve_system_f32`
- `slatec::nonlinear::solve_system_with_jacobian`
- `slatec::nonlinear::solve_system_with_jacobian_f32`
- `slatec::ode::OdeSession::<f32, F, E>::integrate_to`
- `slatec::ode::OdeSession::<f64, F, E>::integrate_to`
- `slatec::polynomials::chebyshev::chebyshev_series`
- `slatec::polynomials::chebyshev::chebyshev_series_f32`
- `slatec::quadrature::integrate`
- `slatec::quadrature::integrate_f32`
- `slatec::quadrature::integrate_fourier_tail`
- `slatec::quadrature::integrate_fourier_tail_f32`
- `slatec::quadrature::integrate_infinite`
- `slatec::quadrature::integrate_infinite_f32`
- `slatec::quadrature::integrate_nc79`
- `slatec::quadrature::integrate_nc79_f32`
- `slatec::quadrature::integrate_non_adaptive`
- `slatec::quadrature::integrate_non_adaptive_f32`
- `slatec::quadrature::integrate_oscillatory`
- `slatec::quadrature::integrate_oscillatory_f32`
- `slatec::quadrature::integrate_principal_value`
- `slatec::quadrature::integrate_principal_value_f32`
- `slatec::quadrature::integrate_singular`
- `slatec::quadrature::integrate_singular_f32`
- `slatec::quadrature::integrate_weighted_endpoints`
- `slatec::quadrature::integrate_weighted_endpoints_f32`
- `slatec::quadrature::integrate_with_breakpoints`
- `slatec::quadrature::integrate_with_breakpoints_f32`
- `slatec::roots::find_root`
- `slatec::roots::find_root_f32`
- `slatec::special::airy::airy_ai`
- `slatec::special::airy::airy_ai_f32`
- `slatec::special::airy::airy_ai_scaled`
- `slatec::special::airy::airy_ai_scaled_f32`
- `slatec::special::airy::airy_bi`
- `slatec::special::airy::airy_bi_f32`
- `slatec::special::airy::airy_bi_scaled`
- `slatec::special::airy::airy_bi_scaled_f32`
- `slatec::special::bessel::bessel_i0`
- `slatec::special::bessel::bessel_i0_f32`
- `slatec::special::bessel::bessel_i0_scaled`
- `slatec::special::bessel::bessel_i0_scaled_f32`
- `slatec::special::bessel::bessel_i1`
- `slatec::special::bessel::bessel_i1_f32`
- `slatec::special::bessel::bessel_i1_scaled`
- `slatec::special::bessel::bessel_i1_scaled_f32`
- `slatec::special::bessel::bessel_j0`
- `slatec::special::bessel::bessel_j0_f32`
- `slatec::special::bessel::bessel_j1`
- `slatec::special::bessel::bessel_j1_f32`
- `slatec::special::bessel::bessel_k0`
- `slatec::special::bessel::bessel_k0_f32`
- `slatec::special::bessel::bessel_k0_scaled`
- `slatec::special::bessel::bessel_k0_scaled_f32`
- `slatec::special::bessel::bessel_k1`
- `slatec::special::bessel::bessel_k1_f32`
- `slatec::special::bessel::bessel_k1_scaled`
- `slatec::special::bessel::bessel_k1_scaled_f32`
- `slatec::special::bessel::bessel_y0`
- `slatec::special::bessel::bessel_y0_f32`
- `slatec::special::bessel::bessel_y1`
- `slatec::special::bessel::bessel_y1_f32`
- `slatec::special::elementary::cbrt`
- `slatec::special::elementary::cbrt_f32`
- `slatec::special::elementary::cos_degrees`
- `slatec::special::elementary::cos_degrees_f32`
- `slatec::special::elementary::dawson`
- `slatec::special::elementary::dawson_f32`
- `slatec::special::elementary::exprel`
- `slatec::special::elementary::exprel_f32`
- `slatec::special::elementary::log1p`
- `slatec::special::elementary::log1p_f32`
- `slatec::special::elementary::sin_degrees`
- `slatec::special::elementary::sin_degrees_f32`
- `slatec::special::error_functions::erf`
- `slatec::special::error_functions::erf_f32`
- `slatec::special::error_functions::erfc`
- `slatec::special::error_functions::erfc_f32`
- `slatec::special::gamma::beta`
- `slatec::special::gamma::beta_f32`
- `slatec::special::gamma::binomial_coefficient`
- `slatec::special::gamma::binomial_coefficient_f32`
- `slatec::special::gamma::digamma`
- `slatec::special::gamma::digamma_f32`
- `slatec::special::gamma::factorial`
- `slatec::special::gamma::factorial_f32`
- `slatec::special::gamma::gamma`
- `slatec::special::gamma::gamma_f32`
- `slatec::special::gamma::incomplete_gamma_lower`
- `slatec::special::gamma::incomplete_gamma_lower_f32`
- `slatec::special::gamma::incomplete_gamma_upper`
- `slatec::special::gamma::incomplete_gamma_upper_f32`
- `slatec::special::gamma::log_beta`
- `slatec::special::gamma::log_beta_f32`
- `slatec::special::gamma::log_gamma`
- `slatec::special::gamma::log_gamma_f32`
- `slatec::special::gamma::reciprocal_gamma`
- `slatec::special::gamma::reciprocal_gamma_f32`
- `slatec::special::gamma::regularized_beta`
- `slatec::special::gamma::regularized_beta_f32`
- `slatec::special::gamma::tricomi_incomplete_gamma`
- `slatec::special::gamma::tricomi_incomplete_gamma_f32`
- `slatec::special::integrals::exponential_integral_e1`
- `slatec::special::integrals::exponential_integral_e1_f32`
- `slatec::special::integrals::exponential_integral_ei`
- `slatec::special::integrals::exponential_integral_ei_f32`
