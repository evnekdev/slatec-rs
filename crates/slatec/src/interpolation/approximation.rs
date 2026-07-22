//! Checked weighted polynomial least-squares fitting.
//!
//! `PolynomialFit` owns the orthogonal-polynomial representation produced
//! by the reviewed SLATEC `POLFIT`/`DPOLFT` pair. It is deliberately distinct
//! from the `interpolation::tabulated::InterpolatingPolynomial` type: fitting accepts
//! unordered and repeated abscissas and minimizes a weighted residual, while
//! interpolation constructs one global polynomial through distinct samples.
//! Caller slices and every native work array remain private, and all native
//! calls are serialized through the process-wide runtime lock.

#[cfg(feature = "approximation-polynomial-fitting")]
mod enabled {
    use alloc::vec::Vec;

    use slatec_core::to_fortran_integer;
    use slatec_sys::FortranInteger;

    use crate::runtime::lock_native;

    /// The source-defined statistical significance level for automatic degree
    /// selection.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum StatisticalSignificance {
        /// Use the source-defined 1% F-test threshold.
        OnePercent,
        /// Use the source-defined 5% F-test threshold.
        FivePercent,
        /// Use the source-defined 10% F-test threshold.
        TenPercent,
    }

    /// Degree-selection policy passed to `POLFIT`/`DPOLFT`.
    #[derive(Clone, Debug, PartialEq)]
    pub enum PolynomialDegreeSelection<T> {
        /// Compute every degree from zero through `max_degree` and retain the
        /// highest-degree fit.
        AllDegrees,
        /// Increase degree until the source-reported RMS residual is no
        /// greater than this positive finite tolerance, or reach `max_degree`.
        RmsTolerance(T),
        /// Select the degree with the source's F test.
        StatisticalFTest(StatisticalSignificance),
    }

    /// Controls for a checked polynomial least-squares fit.
    #[derive(Clone, Debug, PartialEq)]
    pub struct PolynomialFitOptions<T> {
        max_degree: usize,
        selection: PolynomialDegreeSelection<T>,
    }

    impl<T> PolynomialFitOptions<T> {
        /// Requests every polynomial degree through `max_degree`.
        #[must_use]
        pub const fn all_degrees(max_degree: usize) -> Self {
            Self {
                max_degree,
                selection: PolynomialDegreeSelection::AllDegrees,
            }
        }

        /// Requests the first degree whose RMS residual meets `tolerance`.
        #[must_use]
        pub const fn rms_tolerance(max_degree: usize, tolerance: T) -> Self {
            Self {
                max_degree,
                selection: PolynomialDegreeSelection::RmsTolerance(tolerance),
            }
        }

        /// Requests source-defined F-test degree selection.
        #[must_use]
        pub const fn statistical_test(
            max_degree: usize,
            significance: StatisticalSignificance,
        ) -> Self {
            Self {
                max_degree,
                selection: PolynomialDegreeSelection::StatisticalFTest(significance),
            }
        }

        /// Returns the maximum permitted polynomial degree.
        #[must_use]
        pub const fn max_degree(&self) -> usize {
            self.max_degree
        }

        /// Returns the selected degree policy.
        #[must_use]
        pub const fn selection(&self) -> &PolynomialDegreeSelection<T> {
            &self.selection
        }
    }

    /// Completion state returned by the native fit driver.
    #[derive(Clone, Debug, PartialEq)]
    pub enum PolynomialFitStatus<T> {
        /// The requested degree-selection rule completed normally.
        Complete,
        /// No degree through `max_degree` met the requested RMS tolerance; the
        /// retained model is the documented best fit at `max_degree`.
        ToleranceNotReached {
            /// The requested RMS tolerance.
            requested_rms_error: T,
            /// The source-reported RMS residual of the retained model.
            achieved_rms_error: T,
        },
        /// The F test did not pass with this maximum degree; the retained
        /// model is the source's statistically best candidate.
        StatisticalTestInconclusive {
            /// The source-reported RMS residual of the retained model.
            achieved_rms_error: T,
        },
    }

    /// A polynomial value together with requested first derivatives.
    #[derive(Clone, Debug, PartialEq)]
    pub struct PolynomialFitEvaluation<T> {
        /// Value of the fitted polynomial at the query point.
        pub value: T,
        /// First through requested-order derivatives in ascending order.
        pub derivatives: Vec<T>,
    }

    /// Failure while validating or executing a polynomial fit.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum PolynomialFitError {
        /// No samples were supplied.
        TooFewSamples,
        /// Abscissa and ordinate slices have unequal lengths.
        LengthMismatch {
            /// Number of abscissas supplied.
            abscissas: usize,
            /// Number of ordinates supplied.
            values: usize,
        },
        /// A supplied weight slice has the wrong length.
        WeightLengthMismatch {
            /// Number of samples supplied.
            samples: usize,
            /// Number of weights supplied.
            weights: usize,
        },
        /// An abscissa is NaN or infinite.
        NonFiniteAbscissa {
            /// Zero-based input index.
            index: usize,
        },
        /// An ordinate is NaN or infinite.
        NonFiniteValue {
            /// Zero-based input index.
            index: usize,
        },
        /// A supplied weight is NaN, infinite, zero, or negative.
        InvalidWeight {
            /// Zero-based input index.
            index: usize,
        },
        /// The requested RMS tolerance is not positive and finite.
        InvalidRmsTolerance,
        /// `max_degree` violates the source's degree restriction for the
        /// chosen model-selection rule.
        InvalidMaximumDegree {
            /// Number of supplied samples.
            samples: usize,
            /// Requested maximum degree.
            max_degree: usize,
            /// Whether the statistical F-test rule was selected.
            statistical_test: bool,
        },
        /// A count or exact native workspace formula overflows Rust or the
        /// reviewed SLATEC `INTEGER` ABI.
        DimensionOverflow,
        /// A private native buffer could not be allocated.
        AllocationFailed,
        /// A query point is NaN or infinite.
        NonFiniteQuery,
        /// A Taylor-expansion origin is NaN or infinite.
        NonFiniteExpansionOrigin,
        /// A batch output slice does not have one slot per query point.
        OutputLengthMismatch {
            /// Number of query points supplied.
            points: usize,
            /// Number of output slots supplied.
            output: usize,
        },
        /// A preflighted input error or an undocumented native state was
        /// observed from a reviewed driver.
        NativeContractViolation {
            /// Original SLATEC routine that contradicted its contract.
            routine: &'static str,
            /// Native status, when the routine returns one.
            status: FortranInteger,
        },
    }

    impl core::fmt::Display for PolynomialFitError {
        fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::TooFewSamples => write!(formatter, "at least one sample is required"),
                Self::LengthMismatch { abscissas, values } => write!(
                    formatter,
                    "abscissa count {abscissas} does not match value count {values}"
                ),
                Self::WeightLengthMismatch { samples, weights } => write!(
                    formatter,
                    "sample count {samples} does not match weight count {weights}"
                ),
                Self::NonFiniteAbscissa { index } => {
                    write!(formatter, "abscissa at index {index} is not finite")
                }
                Self::NonFiniteValue { index } => {
                    write!(formatter, "value at index {index} is not finite")
                }
                Self::InvalidWeight { index } => {
                    write!(
                        formatter,
                        "weight at index {index} must be positive and finite"
                    )
                }
                Self::InvalidRmsTolerance => {
                    write!(formatter, "RMS tolerance must be positive and finite")
                }
                Self::InvalidMaximumDegree {
                    samples,
                    max_degree,
                    statistical_test,
                } => {
                    let maximum = if *statistical_test {
                        samples.saturating_sub(2)
                    } else {
                        samples.saturating_sub(1)
                    };
                    write!(
                        formatter,
                        "maximum degree {max_degree} exceeds source limit {maximum} for {samples} samples"
                    )
                }
                Self::DimensionOverflow => write!(
                    formatter,
                    "sample count, derivative count, or native workspace exceeds the supported range"
                ),
                Self::AllocationFailed => {
                    write!(
                        formatter,
                        "could not allocate private polynomial-fit storage"
                    )
                }
                Self::NonFiniteQuery => write!(formatter, "polynomial query must be finite"),
                Self::NonFiniteExpansionOrigin => {
                    write!(formatter, "Taylor-expansion origin must be finite")
                }
                Self::OutputLengthMismatch { points, output } => write!(
                    formatter,
                    "output length {output} does not match query count {points}"
                ),
                Self::NativeContractViolation { routine, status } => write!(
                    formatter,
                    "{routine} returned unexpected native status {status}"
                ),
            }
        }
    }

    impl std::error::Error for PolynomialFitError {}

    /// An immutable, owned weighted polynomial least-squares model.
    ///
    /// The representation is orthogonal-polynomial data private to
    /// `POLFIT`/`DPOLFT`; callers use checked value, derivative, and
    /// power-coefficient operations rather than workspace arrays.
    #[derive(Clone, Debug, PartialEq)]
    pub struct PolynomialFit<T> {
        degree: usize,
        selection: PolynomialDegreeSelection<T>,
        status: PolynomialFitStatus<T>,
        rms_error: T,
        weighted_residual_sum_squares: T,
        fitted_values: Vec<T>,
        representation: Vec<T>,
    }

    impl<T> PolynomialFit<T> {
        /// Returns the degree selected by the original driver.
        #[must_use]
        pub const fn degree(&self) -> usize {
            self.degree
        }

        /// Returns the requested degree-selection policy.
        #[must_use]
        pub const fn selection(&self) -> &PolynomialDegreeSelection<T> {
            &self.selection
        }

        /// Returns the source-defined completion state.
        #[must_use]
        pub const fn status(&self) -> &PolynomialFitStatus<T> {
            &self.status
        }

        /// Returns the source-reported RMS residual for the retained model.
        #[must_use]
        pub const fn rms_error(&self) -> T
        where
            T: Copy,
        {
            self.rms_error
        }

        /// Returns the weighted sum of squared residuals recomputed from the
        /// source-returned fit values and the checked input data.
        #[must_use]
        pub const fn weighted_residual_sum_squares(&self) -> T
        where
            T: Copy,
        {
            self.weighted_residual_sum_squares
        }

        /// Returns the fitted values at the original input samples.
        #[must_use]
        pub fn fitted_values(&self) -> &[T] {
            &self.fitted_values
        }
    }

    fn native_integer(value: usize) -> Result<FortranInteger, PolynomialFitError> {
        to_fortran_integer(value).map_err(|_| PolynomialFitError::DimensionOverflow)
    }

    fn zeroed<T: Default>(length: usize) -> Result<Vec<T>, PolynomialFitError> {
        let mut values = Vec::new();
        values
            .try_reserve_exact(length)
            .map_err(|_| PolynomialFitError::AllocationFailed)?;
        values.resize_with(length, T::default);
        Ok(values)
    }

    fn copied<T: Copy + Default>(values: &[T]) -> Result<Vec<T>, PolynomialFitError> {
        let mut output = zeroed(values.len())?;
        output.copy_from_slice(values);
        Ok(output)
    }

    fn representation_length(
        sample_count: usize,
        max_degree: usize,
    ) -> Result<usize, PolynomialFitError> {
        sample_count
            .checked_mul(3)
            .and_then(|value| {
                max_degree
                    .checked_mul(3)
                    .and_then(|degree| value.checked_add(degree))
            })
            .and_then(|value| value.checked_add(3))
            .ok_or(PolynomialFitError::DimensionOverflow)
    }

    macro_rules! impl_polynomial_fit_precision {
        ($scalar:ty, $fit:path, $evaluate:path, $coefficients:path) => {
            impl PolynomialFit<$scalar> {
                /// Fits finite samples with unit weights.
                ///
                /// The input abscissas may be unordered or repeated. This is
                /// a weighted least-squares fit, not interpolation; no input
                /// sample storage is retained after construction.
                pub fn fit(
                    abscissas: &[$scalar],
                    values: &[$scalar],
                    options: PolynomialFitOptions<$scalar>,
                ) -> Result<Self, PolynomialFitError> {
                    Self::fit_with_weights(abscissas, values, None, options)
                }

                /// Fits finite samples with one positive finite weight per
                /// sample.
                ///
                /// Weights are copied to private native storage. To request
                /// unit weights without allocating a slice, use [`Self::fit`].
                pub fn fit_weighted(
                    abscissas: &[$scalar],
                    values: &[$scalar],
                    weights: &[$scalar],
                    options: PolynomialFitOptions<$scalar>,
                ) -> Result<Self, PolynomialFitError> {
                    Self::fit_with_weights(abscissas, values, Some(weights), options)
                }

                fn fit_with_weights(
                    abscissas: &[$scalar],
                    values: &[$scalar],
                    weights: Option<&[$scalar]>,
                    options: PolynomialFitOptions<$scalar>,
                ) -> Result<Self, PolynomialFitError> {
                    if abscissas.is_empty() {
                        return Err(PolynomialFitError::TooFewSamples);
                    }
                    if abscissas.len() != values.len() {
                        return Err(PolynomialFitError::LengthMismatch {
                            abscissas: abscissas.len(),
                            values: values.len(),
                        });
                    }
                    for (index, value) in abscissas.iter().enumerate() {
                        if !value.is_finite() {
                            return Err(PolynomialFitError::NonFiniteAbscissa { index });
                        }
                    }
                    for (index, value) in values.iter().enumerate() {
                        if !value.is_finite() {
                            return Err(PolynomialFitError::NonFiniteValue { index });
                        }
                    }
                    if let Some(weights) = weights {
                        if weights.len() != abscissas.len() {
                            return Err(PolynomialFitError::WeightLengthMismatch {
                                samples: abscissas.len(),
                                weights: weights.len(),
                            });
                        }
                        for (index, weight) in weights.iter().enumerate() {
                            if !weight.is_finite() || *weight <= 0.0 {
                                return Err(PolynomialFitError::InvalidWeight { index });
                            }
                        }
                    }

                    let statistical_test = matches!(
                        &options.selection,
                        PolynomialDegreeSelection::StatisticalFTest(_)
                    );
                    let maximum = if statistical_test {
                        abscissas.len().checked_sub(2)
                    } else {
                        abscissas.len().checked_sub(1)
                    };
                    if maximum.is_none_or(|maximum| options.max_degree > maximum) {
                        return Err(PolynomialFitError::InvalidMaximumDegree {
                            samples: abscissas.len(),
                            max_degree: options.max_degree,
                            statistical_test,
                        });
                    }
                    let mut eps = match &options.selection {
                        PolynomialDegreeSelection::AllDegrees => 0.0,
                        PolynomialDegreeSelection::RmsTolerance(tolerance) => {
                            if !tolerance.is_finite() || *tolerance <= 0.0 {
                                return Err(PolynomialFitError::InvalidRmsTolerance);
                            }
                            *tolerance
                        }
                        PolynomialDegreeSelection::StatisticalFTest(significance) => {
                            match significance {
                                StatisticalSignificance::OnePercent => -0.01,
                                StatisticalSignificance::FivePercent => -0.05,
                                StatisticalSignificance::TenPercent => -0.10,
                            }
                        }
                    };

                    let sample_count = abscissas.len();
                    let mut native_sample_count = native_integer(sample_count)?;
                    let mut native_max_degree = native_integer(options.max_degree)?;
                    let mut selected_degree = 0;
                    let mut native_abscissas = copied(abscissas)?;
                    let mut native_values = copied(values)?;
                    let mut native_weights = match weights {
                        Some(weights) => copied(weights)?,
                        None => {
                            let mut unit_weights = zeroed(sample_count)?;
                            unit_weights[0] = -1.0;
                            unit_weights
                        }
                    };
                    let mut fitted_values = zeroed(sample_count)?;
                    let mut native_status = 0;
                    let mut representation =
                        zeroed(representation_length(sample_count, options.max_degree)?)?;
                    {
                        let _native = lock_native();
                        // SAFETY: all source restrictions are preflighted:
                        // N>=1, finite copied arrays have exact N elements,
                        // positive explicit weights (or the documented W(1)<0
                        // unit-weight mode), MAXDEG obeys the selected EPS
                        // rule, R has N elements, and A has 3*N+3*MAXDEG+3
                        // elements. Every mutable pointer targets private
                        // storage that remains live for the native call.
                        unsafe {
                            $fit(
                                &mut native_sample_count,
                                native_abscissas.as_mut_ptr(),
                                native_values.as_mut_ptr(),
                                native_weights.as_mut_ptr(),
                                &mut native_max_degree,
                                &mut selected_degree,
                                &mut eps,
                                fitted_values.as_mut_ptr(),
                                &mut native_status,
                                representation.as_mut_ptr(),
                            );
                        }
                    }
                    if !(1..=4).contains(&native_status)
                        || selected_degree < 0
                        || usize::try_from(selected_degree)
                            .ok()
                            .is_none_or(|degree| degree > options.max_degree)
                    {
                        return Err(PolynomialFitError::NativeContractViolation {
                            routine: if core::mem::size_of::<$scalar>() == 4 {
                                "POLFIT"
                            } else {
                                "DPOLFT"
                            },
                            status: native_status,
                        });
                    }
                    if native_status == 2 || !eps.is_finite() {
                        return Err(PolynomialFitError::NativeContractViolation {
                            routine: if core::mem::size_of::<$scalar>() == 4 {
                                "POLFIT"
                            } else {
                                "DPOLFT"
                            },
                            status: native_status,
                        });
                    }

                    let weighted_residual_sum_squares = values
                        .iter()
                        .zip(&fitted_values)
                        .zip(&native_weights)
                        .fold(0.0, |sum, ((value, fitted), weight)| {
                            let residual = *value - *fitted;
                            sum + *weight * residual * residual
                        });
                    let status = match (&options.selection, native_status) {
                        (PolynomialDegreeSelection::RmsTolerance(requested), 3) => {
                            PolynomialFitStatus::ToleranceNotReached {
                                requested_rms_error: *requested,
                                achieved_rms_error: eps,
                            }
                        }
                        (PolynomialDegreeSelection::StatisticalFTest(_), 4) => {
                            PolynomialFitStatus::StatisticalTestInconclusive {
                                achieved_rms_error: eps,
                            }
                        }
                        (_, 1) => PolynomialFitStatus::Complete,
                        _ => {
                            return Err(PolynomialFitError::NativeContractViolation {
                                routine: if core::mem::size_of::<$scalar>() == 4 {
                                    "POLFIT"
                                } else {
                                    "DPOLFT"
                                },
                                status: native_status,
                            });
                        }
                    };
                    Ok(Self {
                        degree: usize::try_from(selected_degree)
                            .map_err(|_| PolynomialFitError::DimensionOverflow)?,
                        selection: options.selection,
                        status,
                        rms_error: eps,
                        weighted_residual_sum_squares,
                        fitted_values,
                        representation,
                    })
                }

                /// Evaluates the selected polynomial at a finite point.
                pub fn evaluate(&self, point: $scalar) -> Result<$scalar, PolynomialFitError> {
                    Ok(self.evaluate_with_derivatives(point, 0)?.value)
                }

                /// Evaluates the selected polynomial and its first
                /// `derivative_count` derivatives at a finite point.
                pub fn evaluate_with_derivatives(
                    &self,
                    point: $scalar,
                    derivative_count: usize,
                ) -> Result<PolynomialFitEvaluation<$scalar>, PolynomialFitError> {
                    if !point.is_finite() {
                        return Err(PolynomialFitError::NonFiniteQuery);
                    }
                    let _native = lock_native();
                    self.evaluate_locked(point, derivative_count)
                }

                /// Evaluates this immutable fit at each finite query point.
                ///
                /// `output` must have exactly one slot per point. The native
                /// lock is acquired once for the complete batch.
                pub fn evaluate_into(
                    &self,
                    points: &[$scalar],
                    output: &mut [$scalar],
                ) -> Result<(), PolynomialFitError> {
                    if points.len() != output.len() {
                        return Err(PolynomialFitError::OutputLengthMismatch {
                            points: points.len(),
                            output: output.len(),
                        });
                    }
                    if points.iter().any(|point| !point.is_finite()) {
                        return Err(PolynomialFitError::NonFiniteQuery);
                    }
                    let _native = lock_native();
                    for (point, value) in points.iter().zip(output) {
                        *value = self.evaluate_locked(*point, 0)?.value;
                    }
                    Ok(())
                }

                fn evaluate_locked(
                    &self,
                    point: $scalar,
                    derivative_count: usize,
                ) -> Result<PolynomialFitEvaluation<$scalar>, PolynomialFitError> {
                    let mut degree = native_integer(self.degree)?;
                    let mut derivatives_native = native_integer(derivative_count)?;
                    let mut point = point;
                    let mut value = 0.0;
                    let mut derivatives = zeroed(derivative_count.max(1))?;
                    let mut representation = copied(&self.representation)?;
                    // SAFETY: L is in the validated fitted range, NDER fits
                    // the reviewed INTEGER ABI, YP has max(1, NDER) elements,
                    // and A is a private exact copy of the representation
                    // returned by the matching fit driver. The source states
                    // A is input here, so cloning preserves this type's
                    // immutable representation even if a provider violates it.
                    unsafe {
                        $evaluate(
                            &mut degree,
                            &mut derivatives_native,
                            &mut point,
                            &mut value,
                            derivatives.as_mut_ptr(),
                            representation.as_mut_ptr(),
                        );
                    }
                    derivatives.truncate(derivative_count);
                    Ok(PolynomialFitEvaluation { value, derivatives })
                }

                /// Converts the selected fit to coefficients in ascending
                /// powers of `x` about the origin.
                pub fn power_coefficients(&self) -> Result<Vec<$scalar>, PolynomialFitError> {
                    self.power_coefficients_at(0.0)
                }

                /// Converts the selected fit to Taylor coefficients in
                /// ascending powers of `(x - origin)`.
                ///
                /// A nonzero origin may provide a better-scaled local
                /// representation than ordinary powers about zero. The
                /// returned vector has `degree() + 1` entries, so the fitted
                /// polynomial is `coefficients[0] + coefficients[1] *
                /// (x - origin) + ...`.
                pub fn power_coefficients_at(
                    &self,
                    origin: $scalar,
                ) -> Result<Vec<$scalar>, PolynomialFitError> {
                    if !origin.is_finite() {
                        return Err(PolynomialFitError::NonFiniteExpansionOrigin);
                    }
                    let mut degree = native_integer(self.degree)?;
                    let mut origin = origin;
                    let mut coefficients = zeroed(
                        self.degree
                            .checked_add(1)
                            .ok_or(PolynomialFitError::DimensionOverflow)?,
                    )?;
                    let mut representation = copied(&self.representation)?;
                    let _native = lock_native();
                    // SAFETY: TC has L+1 slots and A is a private copy of the
                    // matching POLFIT/DPOLFT representation. The positive L
                    // form returns ordinary ascending Taylor coefficients.
                    unsafe {
                        $coefficients(
                            &mut degree,
                            &mut origin,
                            coefficients.as_mut_ptr(),
                            representation.as_mut_ptr(),
                        );
                    }
                    Ok(coefficients)
                }
            }
        };
    }

    impl_polynomial_fit_precision!(
        f32,
        slatec_sys::approximation::polfit,
        slatec_sys::approximation::pvalue,
        slatec_sys::approximation::pcoef
    );
    impl_polynomial_fit_precision!(
        f64,
        slatec_sys::approximation::dpolft,
        slatec_sys::approximation::dp1vlu,
        slatec_sys::approximation::dpcoef
    );
}

#[cfg(feature = "approximation-polynomial-fitting")]
pub use enabled::*;

#[cfg(not(feature = "approximation-polynomial-fitting"))]
/// The checked polynomial-fitting API is available with the
/// `approximation-polynomial-fitting` feature.
pub struct ApproximationFeatureRequired;
