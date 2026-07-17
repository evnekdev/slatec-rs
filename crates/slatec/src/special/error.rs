//! Errors reported before a safe special-function call reaches Fortran.

use core::fmt;

/// Failure to meet a validated SLATEC special-function contract.
#[derive(Clone, Debug, PartialEq)]
pub enum SpecialFunctionError {
    /// An input is outside the conservative domain validated by this facade.
    Domain {
        /// Safe Rust function name.
        function: &'static str,
        /// Rust argument name.
        argument: &'static str,
        /// Rejected value, widened to `f64` for diagnostics.
        value: f64,
    },
    /// An integer input cannot be represented by the selected Fortran ABI.
    IntegerOverflow {
        /// Safe Rust function name.
        function: &'static str,
        /// Rust argument name.
        argument: &'static str,
    },
    /// The underlying routine reported its documented integer error state.
    NativeError {
        /// Safe Rust function name.
        function: &'static str,
        /// SLATEC error number.
        error_number: i32,
        /// SLATEC legacy error level.
        level: i32,
    },
    /// The process-global SLATEC runtime state could not be used safely.
    RuntimeStateUnavailable {
        /// Safe Rust function name.
        function: &'static str,
    },
    /// A native result violated a checked wrapper invariant.
    NativeContractViolation {
        /// Safe Rust function name.
        function: &'static str,
        /// Stable explanation of the violated postcondition.
        detail: &'static str,
    },
}

impl fmt::Display for SpecialFunctionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain {
                function,
                argument,
                value,
            } => write!(
                formatter,
                "{function}: {argument}={value} is outside the supported domain"
            ),
            Self::IntegerOverflow { function, argument } => {
                write!(
                    formatter,
                    "{function}: {argument} does not fit the selected Fortran INTEGER"
                )
            }
            Self::NativeError {
                function,
                error_number,
                level,
            } => write!(
                formatter,
                "{function}: SLATEC reported error {error_number} at level {level}"
            ),
            Self::RuntimeStateUnavailable { function } => {
                write!(formatter, "{function}: SLATEC runtime state is unavailable")
            }
            Self::NativeContractViolation { function, detail } => {
                write!(formatter, "{function}: native contract violation: {detail}")
            }
        }
    }
}

impl std::error::Error for SpecialFunctionError {}
