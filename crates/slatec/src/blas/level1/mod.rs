//! Checked safe wrappers for the initial real-valued BLAS Level 1 surface.
//!
//! A strided slice is a backing store, not a pre-adjusted pointer. For a
//! negative increment the original BLAS routine derives the logical starting
//! position from the base address; every operation validates that position
//! before passing the base pointer to Fortran.
//!
//! Each operation class has its own Rust module. This is intentional: the
//! GNU linker selects native archive members before it can discard dormant
//! sections in a broad Rust object. Keeping raw references in small wrapper
//! modules lets a call retain its operation closure rather than its family.

mod dasum;
mod daxpy;
mod dcopy;
mod ddot;
mod dnrm2;
mod drot;
mod dscal;
mod dswap;
mod idamax;
mod isamax;
mod sasum;
mod saxpy;
mod scopy;
mod sdot;
mod snrm2;
mod srot;
mod sscal;
mod sswap;

#[cfg(test)]
use super::{BlasError, validation};

pub use dasum::{dasum, dasum_strided};
pub use daxpy::{daxpy, daxpy_strided};
pub use dcopy::{dcopy, dcopy_strided};
pub use ddot::{ddot, ddot_strided};
pub use dnrm2::{dnrm2, dnrm2_strided};
pub use drot::{drot, drot_strided};
pub use dscal::{dscal, dscal_strided};
pub use dswap::{dswap, dswap_strided};
pub use idamax::{idamax, idamax_strided};
pub use isamax::{isamax, isamax_strided};
pub use sasum::{sasum, sasum_strided};
pub use saxpy::{saxpy, saxpy_strided};
pub use scopy::{scopy, scopy_strided};
pub use sdot::{sdot, sdot_strided};
pub use snrm2::{snrm2, snrm2_strided};
pub use srot::{srot, srot_strided};
pub use sscal::{sscal, sscal_strided};
pub use sswap::{sswap, sswap_strided};

#[inline]
pub(super) fn audited_candidate_call<R>(call: impl FnOnce() -> R) -> R {
    #[cfg(feature = "blas-level1-concurrency-native-tests")]
    let _audit = crate::runtime::Blas1NativeCallAudit::enter();
    call()
}

#[cfg(test)]
mod tests {
    use super::{BlasError, daxpy, daxpy_strided, validation};

    #[test]
    fn storage_calculation_handles_negative_increment() {
        assert_eq!(validation::required_storage(0, -1), Ok(0));
        assert_eq!(validation::required_storage(3, 2), Ok(5));
        assert_eq!(validation::required_storage(3, -2), Ok(5));
    }

    #[test]
    fn storage_calculation_rejects_zero_and_minimum_increment() {
        assert!(matches!(
            validation::required_storage(1, 0),
            Err(BlasError::InvalidIncrement { .. })
        ));
        assert_eq!(
            validation::required_storage(2, isize::MIN),
            Err(BlasError::ArithmeticOverflow)
        );
    }

    #[test]
    fn mismatched_contiguous_lengths_fail_before_native_call() {
        let mut y = [0.0];
        assert_eq!(
            daxpy(1.0, &[1.0, 2.0], &mut y),
            Err(BlasError::LengthMismatch { x: 2, y: 1 })
        );
    }

    #[test]
    fn invalid_strided_storage_fails_before_native_call() {
        let mut y = [0.0; 2];
        assert!(matches!(
            daxpy_strided(2, 1.0, &[1.0], 2, &mut y, 1),
            Err(BlasError::InsufficientStorage { argument: "x", .. })
        ));
    }
}
