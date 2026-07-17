//! Checked column-major Jacobian access used by nonlinear callbacks.

use core::fmt;

/// An out-of-range logical Jacobian index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JacobianIndexError {
    /// Requested zero-based row.
    pub row: usize,
    /// Requested zero-based column.
    pub column: usize,
    /// Number of logical rows.
    pub rows: usize,
    /// Number of logical columns.
    pub columns: usize,
}

impl fmt::Display for JacobianIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Jacobian index ({}, {}) is outside {} by {}",
            self.row, self.column, self.rows, self.columns
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for JacobianIndexError {}

/// Mutable checked view of a column-major Jacobian matrix.
///
/// SLATEC stores an entry at `row + column * leading_dimension`. Logical
/// indexing is limited to `rows × columns`; any physical padding implied by a
/// larger leading dimension is not returned by [`Self::get`] or
/// [`Self::get_mut`]. The type itself uses only `core` and does not allocate.
pub struct JacobianMut<'a, T> {
    data: &'a mut [T],
    rows: usize,
    columns: usize,
    leading_dimension: usize,
}

impl<'a, T> JacobianMut<'a, T> {
    pub(crate) fn new(
        data: &'a mut [T],
        rows: usize,
        columns: usize,
        leading_dimension: usize,
    ) -> Option<Self> {
        if leading_dimension < rows || leading_dimension.checked_mul(columns)? > data.len() {
            return None;
        }
        Some(Self {
            data,
            rows,
            columns,
            leading_dimension,
        })
    }

    /// Returns the logical row count.
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the logical column count.
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the logical column count.
    ///
    /// This is a short alias for [`Self::columns`]. A nonlinear least-squares
    /// Jacobian has `M` residual rows and `N` parameter columns, so it is
    /// generally rectangular.
    pub const fn cols(&self) -> usize {
        self.columns
    }

    /// Returns the physical column stride corresponding to Fortran `LDFJAC`.
    pub const fn leading_dimension(&self) -> usize {
        self.leading_dimension
    }

    /// Returns a shared reference to a logical matrix entry.
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.index(row, column).map(|index| &self.data[index])
    }

    /// Returns a mutable reference to a logical matrix entry.
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.index(row, column).map(|index| &mut self.data[index])
    }

    /// Writes one logical matrix entry.
    ///
    /// # Errors
    ///
    /// Returns [`JacobianIndexError`] when either zero-based index is outside
    /// the logical matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "nonlinear-expert")]
    /// # {
    /// # use slatec::nonlinear::JacobianMut;
    /// # fn fill(mut jacobian: JacobianMut<'_, f64>) {
    /// jacobian.set(0, 0, 2.0).unwrap();
    /// assert_eq!(jacobian.get(0, 0), Some(&2.0));
    /// # }
    /// # }
    /// ```
    pub fn set(&mut self, row: usize, column: usize, value: T) -> Result<(), JacobianIndexError> {
        let index = self.index(row, column).ok_or(JacobianIndexError {
            row,
            column,
            rows: self.rows,
            columns: self.columns,
        })?;
        self.data[index] = value;
        Ok(())
    }

    /// Returns the physical column-major storage, including any padding.
    ///
    /// Logical entries still use `row + column * leading_dimension`. Callers
    /// that use this escape hatch must not interpret padding as matrix entries.
    pub fn as_column_major_slice_mut(&mut self) -> &mut [T] {
        self.data
    }

    fn index(&self, row: usize, column: usize) -> Option<usize> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        column
            .checked_mul(self.leading_dimension)
            .and_then(|base| base.checked_add(row))
    }
}

#[cfg(test)]
mod tests {
    use super::JacobianMut;

    #[test]
    fn indexing_is_column_major_and_padding_is_not_logical() {
        let mut data = [0_i32; 8];
        let mut view = JacobianMut::new(&mut data, 2, 2, 4).unwrap();
        view.set(1, 1, 7).unwrap();
        assert_eq!(view.get(1, 1), Some(&7));
        assert!(view.get(2, 0).is_none());
        assert_eq!(view.as_column_major_slice_mut()[5], 7);
    }
}
