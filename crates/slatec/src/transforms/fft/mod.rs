//! Fourier transforms.
//!
//! # Status: Partial

/// An error detected before an FFTPACK plan enters native code.
///
/// The reviewed FFTPACK routines have no status return. Constructors therefore
/// validate documented dimensions and checked workspace arithmetic, while
/// transform methods require the exact plan length.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FftError {
    /// The requested transform length is outside the routine's documented range.
    InvalidLength {
        /// The requested length.
        length: usize,
        /// The smallest accepted length for this plan family.
        minimum: usize,
    },
    /// A transform slice does not match the plan length.
    LengthMismatch {
        /// Plan length established by its constructor.
        expected: usize,
        /// Slice length passed to the transform method.
        actual: usize,
    },
    /// A length or workspace formula cannot be represented by the native ABI.
    DimensionOverflow,
    /// Allocation of private native plan storage failed.
    AllocationFailed,
}

/// Complex FFTPACK transforms.
pub mod complex;
/// Multidimensional transforms.
pub mod multidimensional;
/// Real FFTPACK transform plans.
pub mod real;
