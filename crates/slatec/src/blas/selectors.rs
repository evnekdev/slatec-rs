//! Safe selector values for the BLAS routines that otherwise take characters.

use core::ffi::c_char;

use super::BlasError;

/// Matrix operation used by BLAS routines with a transpose selector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transpose {
    /// Use the stored matrix without transposition.
    None,
    /// Use the ordinary transpose.
    Transpose,
    /// Use the conjugate transpose where a complex safe API supports it.
    ConjugateTranspose,
}

#[allow(dead_code)]
impl Transpose {
    pub(crate) fn real_character(self, operation: &'static str) -> Result<c_char, BlasError> {
        match self {
            Self::None => Ok(b'N' as c_char),
            Self::Transpose => Ok(b'T' as c_char),
            Self::ConjugateTranspose => Err(BlasError::UnsupportedTranspose {
                operation,
                transpose: self,
            }),
        }
    }

    pub(crate) fn is_transposed(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The stored triangle of a triangular or symmetric matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Triangle {
    /// The upper triangle is stored.
    Upper,
    /// The lower triangle is stored.
    Lower,
}

#[allow(dead_code)]
impl Triangle {
    pub(crate) fn character(self) -> c_char {
        match self {
            Self::Upper => b'U' as c_char,
            Self::Lower => b'L' as c_char,
        }
    }
}

/// Whether a triangular diagonal is implicit unit values or stored values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Diagonal {
    /// The diagonal is implicitly one and need not be read from the matrix.
    Unit,
    /// The diagonal is read from the matrix.
    NonUnit,
}

#[allow(dead_code)]
impl Diagonal {
    pub(crate) fn character(self) -> c_char {
        match self {
            Self::Unit => b'U' as c_char,
            Self::NonUnit => b'N' as c_char,
        }
    }
}

/// Which side contains a triangular or symmetric matrix operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
    /// The matrix operand acts on the left.
    Left,
    /// The matrix operand acts on the right.
    Right,
}

#[allow(dead_code)]
impl Side {
    pub(crate) fn character(self) -> c_char {
        match self {
            Self::Left => b'L' as c_char,
            Self::Right => b'R' as c_char,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagonal, Side, Transpose, Triangle};
    use crate::blas::BlasError;

    #[test]
    fn real_selectors_reject_conjugate_transpose() {
        assert_eq!(Transpose::None.real_character("DGEMM"), Ok(b'N' as i8));
        assert_eq!(Transpose::Transpose.real_character("DGEMM"), Ok(b'T' as i8));
        assert!(matches!(
            Transpose::ConjugateTranspose.real_character("DGEMM"),
            Err(BlasError::UnsupportedTranspose { .. })
        ));
        assert_eq!(Triangle::Upper.character(), b'U' as i8);
        assert_eq!(Diagonal::Unit.character(), b'U' as i8);
        assert_eq!(Side::Right.character(), b'R' as i8);
    }
}
