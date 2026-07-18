//! Safe in-memory-only sparse linear programming over SLATEC `SPLP`/`DSPLP`.
//!
//! The solved model is `minimize c^T x` subject to `A x = w`, with typed
//! bounds on both `x` and row activities `w`. Matrix entries are borrowed from
//! validated owned sparse columns during one globally serialized callback
//! sequence. Native paging, save/restore, and printing are disabled; a problem
//! whose requested matrix capacity is too small is rejected before FFI.

use alloc::vec::Vec;
use core::cell::Cell;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{Add, Mul, Sub};
use std::cell::Cell as ThreadCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::thread_local;

use slatec_sys::FortranInteger;

use crate::runtime::{lock_native, permit_lp_native_statuses};

thread_local! {
    static ACTIVE_CONTEXT: ThreadCell<*mut c_void> = const { ThreadCell::new(core::ptr::null_mut()) };
}

/// A typed lower/upper-bound category accepted by `SPLP` and `DSPLP`.
///
/// The native `IND` codes are private: lower, upper, two-sided (including
/// equal endpoints), and free map to codes 1, 2, 3, and 4 respectively.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LpBound<T> {
    /// No lower or upper bound.
    Free,
    /// A finite inclusive lower bound.
    Lower(T),
    /// A finite inclusive upper bound.
    Upper(T),
    /// Finite inclusive lower and upper bounds.
    Range {
        /// Inclusive lower endpoint.
        lower: T,
        /// Inclusive upper endpoint.
        upper: T,
    },
    /// A fixed value, encoded as equal native lower and upper bounds.
    Fixed(T),
}

/// Validated owned compressed sparse columns used by the LP callback.
///
/// `column_offsets` has `columns + 1` entries, starts at zero, ends at the
/// number of values, and is monotone. Row indices are zero-based in Rust and
/// must be strictly increasing within each column. Duplicate indices, stored
/// zeros, and non-finite values are rejected. The callback converts indices to
/// one-based Fortran coordinates without reordering or densifying.
#[derive(Clone, Debug, PartialEq)]
pub struct SparseColumns<T> {
    rows: usize,
    columns: usize,
    column_offsets: Vec<usize>,
    row_indices: Vec<usize>,
    values: Vec<T>,
}

impl<T> SparseColumns<T> {
    /// Number of matrix rows.
    #[must_use]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of matrix columns.
    #[must_use]
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Number of explicitly stored nonzero entries.
    #[must_use]
    pub fn nonzeros(&self) -> usize {
        self.values.len()
    }

    /// Zero-based compressed-column offsets.
    #[must_use]
    pub fn column_offsets(&self) -> &[usize] {
        &self.column_offsets
    }

    /// Zero-based row indices, strictly increasing within each column.
    #[must_use]
    pub fn row_indices(&self) -> &[usize] {
        &self.row_indices
    }

    /// Stored nonzero coefficient values.
    #[must_use]
    pub fn values(&self) -> &[T] {
        &self.values
    }
}

/// One owned sparse LP in the native `A x = w` form.
///
/// `objective` and variable bounds have one entry per matrix column; row
/// bounds have one entry per matrix row. Inputs remain owned and unchanged by
/// a solve because all native-mutable vectors and work arrays are private
/// copies.
#[derive(Clone, Debug, PartialEq)]
pub struct LinearProgram<T> {
    objective: Vec<T>,
    matrix: SparseColumns<T>,
    row_bounds: Vec<LpBound<T>>,
    variable_bounds: Vec<LpBound<T>>,
}

impl<T> LinearProgram<T> {
    /// Linear objective coefficients `c` in `minimize c^T x`.
    #[must_use]
    pub fn objective(&self) -> &[T] {
        &self.objective
    }

    /// Validated compressed sparse-column matrix.
    #[must_use]
    pub fn matrix(&self) -> &SparseColumns<T> {
        &self.matrix
    }

    /// Bounds on row activities `w = A x`.
    #[must_use]
    pub fn row_bounds(&self) -> &[LpBound<T>] {
        &self.row_bounds
    }

    /// Bounds on decision variables `x`.
    #[must_use]
    pub fn variable_bounds(&self) -> &[LpBound<T>] {
        &self.variable_bounds
    }
}

/// Controls that do not expose the native option-array language.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LpOptions {
    /// Optional maximum number of matrix coefficients kept in native
    /// high-speed storage. `None` allocates the exact required capacity.
    /// A value below the matrix nonzero count returns [`LpError::PagingRequired`].
    pub maximum_matrix_entries: Option<usize>,
}

/// Rust-side validation failure detected before native execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LpInputError {
    /// The matrix had zero rows or zero columns.
    EmptyDimension,
    /// The offset vector length was not `columns + 1`.
    OffsetLength {
        /// Required length.
        expected: usize,
        /// Supplied length.
        actual: usize,
    },
    /// The first compressed-column offset was not zero.
    FirstOffsetNotZero,
    /// The final offset did not equal the coefficient count.
    FinalOffsetMismatch,
    /// Compressed-column offsets decreased.
    NonMonotonicOffsets,
    /// Value and row-index lengths differed.
    ValueIndexLengthMismatch,
    /// A row index was outside the matrix.
    RowIndexOutOfRange {
        /// Invalid zero-based index.
        index: usize,
    },
    /// Row indices in a column were not strictly increasing.
    UnsortedOrDuplicateRowIndex {
        /// Zero-based column containing the violation.
        column: usize,
    },
    /// A matrix coefficient was zero; structural zeros must be omitted.
    ExplicitZero,
    /// A coefficient, objective entry, or bound endpoint was non-finite.
    NonFiniteValue,
    /// A lower endpoint exceeded its upper endpoint.
    InvalidBoundRange,
    /// The objective length did not equal the matrix column count.
    ObjectiveLength,
    /// The row-bound length did not equal the matrix row count.
    RowBoundLength,
    /// The variable-bound length did not equal the matrix column count.
    VariableBoundLength,
    /// A dimension, option word, or native loop expression cannot fit the ABI.
    DimensionOverflow,
}

/// Reviewed native failure groups for documented `INFO=-4..-29` returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LpNativeFailure {
    /// Native WORK/IWORK length rejection (`INFO=-4`).
    WorkspaceContract,
    /// Native dimension rejection (`INFO=-5` or `-6`).
    DimensionContract,
    /// Native sparse callback rejection (`INFO=-7`, `-8`, or `-9`).
    SparseCallbackContract,
    /// Native bound rejection (`INFO=-10..=-13`).
    BoundContract,
    /// Native option rejection (`INFO=-14..=-22`).
    OptionContract,
    /// Unsupported user-basis failure (`INFO=-23` or `-24`).
    InitialBasis,
    /// Native iteration moved to a singular point (`INFO=-26`).
    SingularPoint,
    /// LA05 basis factorization failed (`INFO=-27`).
    BasisFactorization,
    /// Documented nominal LA05 basis storage was exhausted (`INFO=-28`).
    InsufficientBasisWorkspace,
    /// Unsupported dense-matrix callback option failed (`INFO=-29`).
    DenseMatrixOption,
}

/// Failure from validation, callback containment, workspace, or native code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LpError {
    /// Invalid Rust input rejected before FFI.
    InvalidInput(LpInputError),
    /// The requested high-speed matrix capacity would activate native paging.
    PagingRequired {
        /// Matrix coefficients that must remain resident.
        required_matrix_storage: usize,
        /// Caller-selected resident coefficient capacity.
        available_matrix_storage: usize,
    },
    /// An internal allocation failed without entering native code.
    AllocationFailed,
    /// The contained sparse callback panicked.
    CallbackPanicked,
    /// Native code made a malformed sparse callback request.
    CallbackProtocolViolation,
    /// A paging, open, or close routine was unexpectedly entered.
    ForbiddenPagingEntry {
        /// Paging-routine entry count.
        paging_entries: usize,
        /// File-open routine entry count.
        open_entries: usize,
        /// File-close routine entry count.
        close_entries: usize,
    },
    /// Another callback-based LP invocation was active on this thread.
    ReentrantCall,
    /// A documented native failure that is not an optimization outcome.
    NativeFailure(LpNativeFailure),
    /// Native output violated the reviewed contract.
    NativeContractViolation,
}

impl core::fmt::Display for LpError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInput(error) => write!(formatter, "invalid linear program: {error:?}"),
            Self::PagingRequired {
                required_matrix_storage,
                available_matrix_storage,
            } => write!(
                formatter,
                "linear program needs {required_matrix_storage} resident matrix entries but only {available_matrix_storage} were allowed"
            ),
            Self::AllocationFailed => formatter.write_str("linear-programming allocation failed"),
            Self::CallbackPanicked => formatter.write_str("sparse matrix callback panicked"),
            Self::CallbackProtocolViolation => {
                formatter.write_str("native sparse callback protocol was violated")
            }
            Self::ForbiddenPagingEntry { .. } => {
                formatter.write_str("forbidden native paging or file routine was entered")
            }
            Self::ReentrantCall => formatter.write_str("nested LP callback dispatch is rejected"),
            Self::NativeFailure(failure) => {
                write!(formatter, "native SPLP/DSPLP failure: {failure:?}")
            }
            Self::NativeContractViolation => {
                formatter.write_str("native SPLP/DSPLP output violated its reviewed contract")
            }
        }
    }
}

impl std::error::Error for LpError {}

/// Legitimate optimization termination status from `SPLP` or `DSPLP`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LpStatus {
    /// An optimum and meaningful primal solution were returned (`INFO=1`).
    Optimal,
    /// No `x,w` satisfy the equality relation and bounds (`INFO=-1`).
    Infeasible,
    /// The objective has no finite optimum under the native diagnosis (`INFO=-2`).
    NoFiniteSolution,
    /// Both native infeasibility and no-finite-solution diagnoses occurred (`INFO=-3`).
    InfeasibleAndNoFiniteSolution,
    /// The native default `3*(rows+columns)` simplex-step limit was reached (`INFO=-25`).
    IterationLimit,
}

/// Independently recomputed meaningful optimum data.
#[derive(Clone, Debug, PartialEq)]
pub struct LpSolution<T> {
    /// Optimal decision variables `x`.
    pub variables: Vec<T>,
    /// Independently recomputed activities `A x`.
    pub row_activities: Vec<T>,
    /// Independently recomputed objective `c^T x`.
    pub objective_value: T,
}

/// One optimization termination, with a solution only when status is optimal.
#[derive(Clone, Debug, PartialEq)]
pub struct LpResult<T> {
    /// Exact mapped native termination status.
    pub status: LpStatus,
    /// Meaningful optimum data, present only for [`LpStatus::Optimal`].
    pub solution: Option<LpSolution<T>>,
}

trait Scalar:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + core::fmt::Debug
    + 'static
{
    const ZERO: Self;
    const ONE: Self;
    fn is_finite(self) -> bool;
    fn abs(self) -> Self;
    fn epsilon() -> Self;
    fn option_integer(value: FortranInteger) -> Option<Self>;
    unsafe fn call_native(call: &mut NativeCall<'_, Self>);
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn epsilon() -> Self {
        Self::EPSILON
    }
    fn option_integer(value: FortranInteger) -> Option<Self> {
        let converted = value as Self;
        ((converted as FortranInteger) == value).then_some(converted)
    }
    unsafe fn call_native(call: &mut NativeCall<'_, Self>) {
        // SAFETY: NativeCall owns correctly sized writable arrays and the
        // caller keeps its callback context installed for the complete call.
        unsafe {
            slatec_sys::linear_programming::splp(
                matrix_callback_f32,
                &mut call.rows,
                &mut call.columns,
                call.costs.as_mut_ptr(),
                call.options.as_mut_ptr(),
                call.callback_data.as_mut_ptr(),
                call.lower.as_mut_ptr(),
                call.upper.as_mut_ptr(),
                call.bound_kinds.as_mut_ptr(),
                &mut call.info,
                call.primal.as_mut_ptr(),
                call.duals.as_mut_ptr(),
                call.basis.as_mut_ptr(),
                call.workspace.as_mut_ptr(),
                &mut call.workspace_len,
                call.integer_workspace.as_mut_ptr(),
                &mut call.integer_workspace_len,
            );
        }
    }
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn epsilon() -> Self {
        Self::EPSILON
    }
    fn option_integer(value: FortranInteger) -> Option<Self> {
        Some(value as Self)
    }
    unsafe fn call_native(call: &mut NativeCall<'_, Self>) {
        // SAFETY: see the f32 dispatch; this is the matching reviewed ABI.
        unsafe {
            slatec_sys::linear_programming::dsplp(
                matrix_callback_f64,
                &mut call.rows,
                &mut call.columns,
                call.costs.as_mut_ptr(),
                call.options.as_mut_ptr(),
                call.callback_data.as_mut_ptr(),
                call.lower.as_mut_ptr(),
                call.upper.as_mut_ptr(),
                call.bound_kinds.as_mut_ptr(),
                &mut call.info,
                call.primal.as_mut_ptr(),
                call.duals.as_mut_ptr(),
                call.basis.as_mut_ptr(),
                call.workspace.as_mut_ptr(),
                &mut call.workspace_len,
                call.integer_workspace.as_mut_ptr(),
                &mut call.integer_workspace_len,
            );
        }
    }
}

macro_rules! impl_public_precision {
    ($scalar:ty) => {
        impl SparseColumns<$scalar> {
            /// Constructs validated owned compressed sparse columns.
            pub fn new(
                rows: usize,
                columns: usize,
                column_offsets: Vec<usize>,
                row_indices: Vec<usize>,
                values: Vec<$scalar>,
            ) -> Result<Self, LpError> {
                validate_sparse(rows, columns, &column_offsets, &row_indices, &values)?;
                Ok(Self {
                    rows,
                    columns,
                    column_offsets,
                    row_indices,
                    values,
                })
            }
        }

        impl LinearProgram<$scalar> {
            /// Constructs a validated owned LP in `minimize c^T x`, `A x = w` form.
            pub fn new(
                objective: Vec<$scalar>,
                matrix: SparseColumns<$scalar>,
                row_bounds: Vec<LpBound<$scalar>>,
                variable_bounds: Vec<LpBound<$scalar>>,
            ) -> Result<Self, LpError> {
                validate_problem(&objective, &matrix, &row_bounds, &variable_bounds)?;
                Ok(Self {
                    objective,
                    matrix,
                    row_bounds,
                    variable_bounds,
                })
            }

            /// Solves with exact resident matrix capacity and documented nominal LA05 workspace.
            ///
            /// Requires `std`, an explicit native backend, and
            /// `optimization-linear-programming-in-memory`. Calls are process-serialized,
            /// XERROR state is restored, and no paging file is opened.
            pub fn solve(&self) -> Result<LpResult<$scalar>, LpError> {
                solve_impl(self, LpOptions::default())
            }

            /// Solves with a validated resident matrix-entry limit.
            ///
            /// A limit below `matrix().nonzeros()` returns `PagingRequired` before FFI;
            /// native option key 54 and every save/file option remain unavailable.
            pub fn solve_with_options(
                &self,
                options: LpOptions,
            ) -> Result<LpResult<$scalar>, LpError> {
                solve_impl(self, options)
            }
        }
    };
}

impl_public_precision!(f32);
impl_public_precision!(f64);

fn validate_sparse<T: Scalar>(
    rows: usize,
    columns: usize,
    offsets: &[usize],
    indices: &[usize],
    values: &[T],
) -> Result<(), LpError> {
    if rows == 0 || columns == 0 {
        return Err(LpError::InvalidInput(LpInputError::EmptyDimension));
    }
    let expected = columns
        .checked_add(1)
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    if offsets.len() != expected {
        return Err(LpError::InvalidInput(LpInputError::OffsetLength {
            expected,
            actual: offsets.len(),
        }));
    }
    if indices.len() != values.len() {
        return Err(LpError::InvalidInput(
            LpInputError::ValueIndexLengthMismatch,
        ));
    }
    if offsets.first().copied() != Some(0) {
        return Err(LpError::InvalidInput(LpInputError::FirstOffsetNotZero));
    }
    if offsets.last().copied() != Some(values.len()) {
        return Err(LpError::InvalidInput(LpInputError::FinalOffsetMismatch));
    }
    if offsets.windows(2).any(|pair| pair[0] > pair[1]) {
        return Err(LpError::InvalidInput(LpInputError::NonMonotonicOffsets));
    }
    for column in 0..columns {
        let start = offsets[column];
        let end = offsets[column + 1];
        let mut previous = None;
        for (&row, &value) in indices[start..end].iter().zip(&values[start..end]) {
            if row >= rows {
                return Err(LpError::InvalidInput(LpInputError::RowIndexOutOfRange {
                    index: row,
                }));
            }
            if previous.is_some_and(|prior| prior >= row) {
                return Err(LpError::InvalidInput(
                    LpInputError::UnsortedOrDuplicateRowIndex { column },
                ));
            }
            if !value.is_finite() {
                return Err(LpError::InvalidInput(LpInputError::NonFiniteValue));
            }
            if value == T::ZERO {
                return Err(LpError::InvalidInput(LpInputError::ExplicitZero));
            }
            previous = Some(row);
        }
    }
    Ok(())
}

fn validate_problem<T: Scalar>(
    objective: &[T],
    matrix: &SparseColumns<T>,
    row_bounds: &[LpBound<T>],
    variable_bounds: &[LpBound<T>],
) -> Result<(), LpError> {
    if objective.len() != matrix.columns {
        return Err(LpError::InvalidInput(LpInputError::ObjectiveLength));
    }
    if row_bounds.len() != matrix.rows {
        return Err(LpError::InvalidInput(LpInputError::RowBoundLength));
    }
    if variable_bounds.len() != matrix.columns {
        return Err(LpError::InvalidInput(LpInputError::VariableBoundLength));
    }
    if objective.iter().any(|value| !value.is_finite()) {
        return Err(LpError::InvalidInput(LpInputError::NonFiniteValue));
    }
    for bound in row_bounds.iter().chain(variable_bounds) {
        validate_bound(*bound)?;
    }
    Ok(())
}

fn validate_bound<T: Scalar>(bound: LpBound<T>) -> Result<(), LpError> {
    match bound {
        LpBound::Free => Ok(()),
        LpBound::Lower(value) | LpBound::Upper(value) | LpBound::Fixed(value) => {
            if value.is_finite() {
                Ok(())
            } else {
                Err(LpError::InvalidInput(LpInputError::NonFiniteValue))
            }
        }
        LpBound::Range { lower, upper } => {
            if !lower.is_finite() || !upper.is_finite() {
                Err(LpError::InvalidInput(LpInputError::NonFiniteValue))
            } else if lower > upper {
                Err(LpError::InvalidInput(LpInputError::InvalidBoundRange))
            } else {
                Ok(())
            }
        }
    }
}

struct NativeCall<'a, T> {
    rows: FortranInteger,
    columns: FortranInteger,
    costs: Vec<T>,
    options: Vec<T>,
    callback_data: Vec<T>,
    lower: Vec<T>,
    upper: Vec<T>,
    bound_kinds: Vec<FortranInteger>,
    info: FortranInteger,
    primal: Vec<T>,
    duals: Vec<T>,
    basis: Vec<FortranInteger>,
    workspace: Vec<T>,
    workspace_len: FortranInteger,
    integer_workspace: Vec<FortranInteger>,
    integer_workspace_len: FortranInteger,
    _borrow: PhantomData<&'a ()>,
}

#[derive(Clone, Copy)]
enum CallbackFault {
    Protocol,
}

struct CallbackContext<'a, T> {
    matrix: &'a SparseColumns<T>,
    cursor: usize,
    fault: Option<CallbackFault>,
    panicked: bool,
    active: Cell<bool>,
    #[cfg(test)]
    panic_on_entry: bool,
}

struct ContextGuard;

impl ContextGuard {
    fn install(pointer: *mut c_void) -> Result<Self, LpError> {
        ACTIVE_CONTEXT.with(|slot| {
            if slot.get().is_null() {
                slot.set(pointer);
                Ok(Self)
            } else {
                Err(LpError::ReentrantCall)
            }
        })
    }
}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        ACTIVE_CONTEXT.with(|slot| slot.set(core::ptr::null_mut()));
    }
}

unsafe extern "C" fn matrix_callback_f32(
    row: *mut FortranInteger,
    column: *mut FortranInteger,
    value: *mut f32,
    category: *mut FortranInteger,
    options: *mut f32,
    data: *mut f32,
    flags: *mut FortranInteger,
) {
    matrix_callback(row, column, value, category, options, data, flags);
}

unsafe extern "C" fn matrix_callback_f64(
    row: *mut FortranInteger,
    column: *mut FortranInteger,
    value: *mut f64,
    category: *mut FortranInteger,
    options: *mut f64,
    data: *mut f64,
    flags: *mut FortranInteger,
) {
    matrix_callback(row, column, value, category, options, data, flags);
}

fn matrix_callback<T: Scalar>(
    row: *mut FortranInteger,
    column: *mut FortranInteger,
    value: *mut T,
    category: *mut FortranInteger,
    options: *mut T,
    data: *mut T,
    flags: *mut FortranInteger,
) {
    let context_pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if context_pointer.is_null() {
        finish_callback(row, column, flags);
        return;
    }
    // SAFETY: ContextGuard installed a matching context for this monomorphized
    // trampoline and holds it alive until the foreign call returns.
    let context = unsafe { &mut *(context_pointer.cast::<CallbackContext<'_, T>>()) };
    if context.active.replace(true) {
        context.fault = Some(CallbackFault::Protocol);
        finish_callback(row, column, flags);
        return;
    }
    let arguments = CallbackArguments {
        row,
        column,
        value,
        category,
        options,
        data,
        flags,
    };
    let outcome = catch_unwind(AssertUnwindSafe(|| callback_body(context, arguments)));
    context.active.set(false);
    if outcome.is_err() {
        context.panicked = true;
        finish_callback(row, column, flags);
    }
}

struct CallbackArguments<T> {
    row: *mut FortranInteger,
    column: *mut FortranInteger,
    value: *mut T,
    category: *mut FortranInteger,
    options: *mut T,
    data: *mut T,
    flags: *mut FortranInteger,
}

fn callback_body<T: Scalar>(context: &mut CallbackContext<'_, T>, arguments: CallbackArguments<T>) {
    let CallbackArguments {
        row,
        column,
        value,
        category,
        options,
        data,
        flags,
    } = arguments;
    #[cfg(test)]
    if context.panic_on_entry {
        panic!("injected sparse callback panic");
    }
    if row.is_null()
        || column.is_null()
        || value.is_null()
        || category.is_null()
        || options.is_null()
        || data.is_null()
        || flags.is_null()
    {
        context.fault = Some(CallbackFault::Protocol);
        finish_callback(row, column, flags);
        return;
    }
    // SAFETY: non-null pointers are supplied by the reviewed Fortran caller.
    let flag = unsafe { *flags };
    if flag == 1 {
        context.cursor = 0;
        return;
    }
    if flag != 2 {
        context.fault = Some(CallbackFault::Protocol);
        finish_callback(row, column, flags);
        return;
    }
    if context.cursor == context.matrix.values.len() {
        finish_callback(row, column, flags);
        return;
    }
    let index = context.cursor;
    let rust_column = context
        .matrix
        .column_offsets
        .partition_point(|&offset| offset <= index)
        - 1;
    let rust_row = context.matrix.row_indices[index];
    let native_row = FortranInteger::try_from(rust_row + 1).ok();
    let native_column = FortranInteger::try_from(rust_column + 1).ok();
    let (Some(native_row), Some(native_column)) = (native_row, native_column) else {
        context.fault = Some(CallbackFault::Protocol);
        finish_callback(row, column, flags);
        return;
    };
    // SAFETY: all output pointers were checked and name independent scalars.
    unsafe {
        *row = native_row;
        *column = native_column;
        *value = context.matrix.values[index];
        *category = 0;
    }
    context.cursor += 1;
    if context.cursor == context.matrix.values.len() {
        // SAFETY: flags is non-null above. The final coefficient remains valid
        // while IFLAG=3 tells DPLPUP to stop after consuming it.
        unsafe { *flags = 3 };
    }
}

fn finish_callback(
    row: *mut FortranInteger,
    column: *mut FortranInteger,
    flags: *mut FortranInteger,
) {
    // SAFETY: each pointer is written only when independently non-null.
    unsafe {
        if let Some(row) = row.as_mut() {
            *row = 0;
        }
        if let Some(column) = column.as_mut() {
            *column = 0;
        }
        if let Some(flags) = flags.as_mut() {
            *flags = 3;
        }
    }
}

fn solve_impl<T: Scalar>(
    problem: &LinearProgram<T>,
    options: LpOptions,
) -> Result<LpResult<T>, LpError> {
    let rows = problem.matrix.rows;
    let columns = problem.matrix.columns;
    let nonzeros = problem.matrix.values.len();
    let available = options.maximum_matrix_entries.unwrap_or(nonzeros);
    if available < nonzeros {
        return Err(LpError::PagingRequired {
            required_matrix_storage: nonzeros,
            available_matrix_storage: available,
        });
    }
    let rows_native = native_integer(rows)?;
    let columns_native = native_integer(columns)?;
    let matrix_words = columns
        .checked_add(nonzeros)
        .and_then(|value| value.checked_add(6))
        .map(|value| value.max(columns + 7))
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let basis_words = rows
        .checked_mul(8)
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let matrix_native = native_integer(matrix_words)?;
    let basis_native = native_integer(basis_words)?;
    let _callback_iteration_guard = rows
        .checked_mul(columns)
        .and_then(|value| value.checked_mul(2))
        .and_then(|value| value.checked_add(1))
        .and_then(|value| FortranInteger::try_from(value).ok())
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let _default_iteration_guard = rows
        .checked_add(columns)
        .and_then(|value| value.checked_mul(3))
        .and_then(|value| FortranInteger::try_from(value).ok())
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let real_len = matrix_words
        .checked_add(basis_words)
        .and_then(|value| value.checked_add(columns.checked_mul(4)?))
        .and_then(|value| value.checked_add(rows.checked_mul(8)?))
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let integer_len = matrix_words
        .checked_add(
            basis_words
                .checked_mul(2)
                .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?,
        )
        .and_then(|value| value.checked_add(columns))
        .and_then(|value| value.checked_add(rows.checked_mul(11)?))
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let total = rows
        .checked_add(columns)
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let real_len_native = native_integer(real_len)?;
    let integer_len_native = native_integer(integer_len)?;
    let matrix_option = T::option_integer(matrix_native)
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;
    let basis_option = T::option_integer(basis_native)
        .ok_or(LpError::InvalidInput(LpInputError::DimensionOverflow))?;

    let mut lower = try_zeroed(total, T::ZERO)?;
    let mut upper = try_zeroed(total, T::ZERO)?;
    let mut bound_kinds = try_zeroed(total, 0)?;
    for (index, bound) in problem
        .variable_bounds
        .iter()
        .chain(&problem.row_bounds)
        .copied()
        .enumerate()
    {
        encode_bound(
            bound,
            &mut lower[index],
            &mut upper[index],
            &mut bound_kinds[index],
        );
    }
    let options = vec_from_slice(&[
        T::option_integer(4).unwrap(),
        T::option_integer(51).unwrap(),
        T::ZERO,
        T::option_integer(9).unwrap(),
        T::option_integer(53).unwrap(),
        T::ONE,
        matrix_option,
        basis_option,
        T::option_integer(12).unwrap(),
        T::option_integer(55).unwrap(),
        T::ZERO,
        T::option_integer(15).unwrap(),
        T::option_integer(57).unwrap(),
        T::ZERO,
        T::ONE,
    ])?;
    let mut call = NativeCall {
        rows: rows_native,
        columns: columns_native,
        costs: vec_from_slice(&problem.objective)?,
        options,
        callback_data: try_zeroed(1, T::ZERO)?,
        lower,
        upper,
        bound_kinds,
        info: 0,
        primal: try_zeroed(total, T::ZERO)?,
        duals: try_zeroed(total, T::ZERO)?,
        basis: try_zeroed(total, 0)?,
        workspace: try_zeroed(real_len, T::ZERO)?,
        workspace_len: real_len_native,
        integer_workspace: try_zeroed(integer_len, 0)?,
        integer_workspace_len: integer_len_native,
        _borrow: PhantomData,
    };
    let mut context = CallbackContext {
        matrix: &problem.matrix,
        cursor: 0,
        fault: None,
        panicked: false,
        active: Cell::new(false),
        #[cfg(test)]
        panic_on_entry: false,
    };
    let _native = lock_native();
    slatec_src::reset_lp_forbidden_io_entries();
    let _xerror = permit_lp_native_statuses();
    let _context_guard =
        ContextGuard::install((&mut context as *mut CallbackContext<'_, T>).cast())?;
    // SAFETY: all arguments, callback lifetime, workspace formulas, option
    // words, and process-global scopes were established above.
    unsafe { T::call_native(&mut call) };
    let forbidden = slatec_src::lp_forbidden_io_entries();
    if forbidden != (0, 0, 0) {
        return Err(LpError::ForbiddenPagingEntry {
            paging_entries: forbidden.0,
            open_entries: forbidden.1,
            close_entries: forbidden.2,
        });
    }
    if context.panicked {
        return Err(LpError::CallbackPanicked);
    }
    if context.fault.is_some() || context.cursor != nonzeros {
        return Err(LpError::CallbackProtocolViolation);
    }
    map_result(problem, call.info, &call.primal)
}

fn native_integer(value: usize) -> Result<FortranInteger, LpError> {
    FortranInteger::try_from(value)
        .map_err(|_| LpError::InvalidInput(LpInputError::DimensionOverflow))
}

fn try_zeroed<T: Copy>(length: usize, value: T) -> Result<Vec<T>, LpError> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(length)
        .map_err(|_| LpError::AllocationFailed)?;
    output.resize(length, value);
    Ok(output)
}

fn vec_from_slice<T: Copy>(input: &[T]) -> Result<Vec<T>, LpError> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(input.len())
        .map_err(|_| LpError::AllocationFailed)?;
    output.extend_from_slice(input);
    Ok(output)
}

fn encode_bound<T: Scalar>(
    bound: LpBound<T>,
    lower: &mut T,
    upper: &mut T,
    kind: &mut FortranInteger,
) {
    match bound {
        LpBound::Free => *kind = 4,
        LpBound::Lower(value) => {
            *lower = value;
            *kind = 1;
        }
        LpBound::Upper(value) => {
            *upper = value;
            *kind = 2;
        }
        LpBound::Range {
            lower: low,
            upper: high,
        } => {
            *lower = low;
            *upper = high;
            *kind = 3;
        }
        LpBound::Fixed(value) => {
            *lower = value;
            *upper = value;
            *kind = 3;
        }
    }
}

fn map_result<T: Scalar>(
    problem: &LinearProgram<T>,
    info: FortranInteger,
    primal: &[T],
) -> Result<LpResult<T>, LpError> {
    let status = match info {
        1 => LpStatus::Optimal,
        -1 => LpStatus::Infeasible,
        -2 => LpStatus::NoFiniteSolution,
        -3 => LpStatus::InfeasibleAndNoFiniteSolution,
        -25 => LpStatus::IterationLimit,
        -4 => return Err(LpError::NativeFailure(LpNativeFailure::WorkspaceContract)),
        -5 | -6 => return Err(LpError::NativeFailure(LpNativeFailure::DimensionContract)),
        -9..=-7 => {
            return Err(LpError::NativeFailure(
                LpNativeFailure::SparseCallbackContract,
            ));
        }
        -13..=-10 => return Err(LpError::NativeFailure(LpNativeFailure::BoundContract)),
        -22..=-14 => return Err(LpError::NativeFailure(LpNativeFailure::OptionContract)),
        -24 | -23 => return Err(LpError::NativeFailure(LpNativeFailure::InitialBasis)),
        -26 => return Err(LpError::NativeFailure(LpNativeFailure::SingularPoint)),
        -27 => return Err(LpError::NativeFailure(LpNativeFailure::BasisFactorization)),
        -28 => {
            return Err(LpError::NativeFailure(
                LpNativeFailure::InsufficientBasisWorkspace,
            ));
        }
        -29 => return Err(LpError::NativeFailure(LpNativeFailure::DenseMatrixOption)),
        _ => return Err(LpError::NativeContractViolation),
    };
    if status != LpStatus::Optimal {
        return Ok(LpResult {
            status,
            solution: None,
        });
    }
    let columns = problem.matrix.columns;
    let variables = vec_from_slice(
        primal
            .get(..columns)
            .ok_or(LpError::NativeContractViolation)?,
    )?;
    if variables.iter().any(|value| !value.is_finite()) {
        return Err(LpError::NativeContractViolation);
    }
    let mut activities = try_zeroed(problem.matrix.rows, T::ZERO)?;
    for (column, &variable) in variables.iter().enumerate().take(columns) {
        for index in
            problem.matrix.column_offsets[column]..problem.matrix.column_offsets[column + 1]
        {
            let row = problem.matrix.row_indices[index];
            activities[row] = activities[row] + problem.matrix.values[index] * variable;
        }
    }
    if activities.iter().any(|value| !value.is_finite()) {
        return Err(LpError::NativeContractViolation);
    }
    let native_activities = primal
        .get(columns..columns + problem.matrix.rows)
        .ok_or(LpError::NativeContractViolation)?;
    if activities
        .iter()
        .zip(native_activities)
        .any(|(&left, &right)| !approximately_equal(left, right))
    {
        return Err(LpError::NativeContractViolation);
    }
    if variables
        .iter()
        .zip(&problem.variable_bounds)
        .any(|(&value, &bound)| !bound_contains(bound, value))
        || activities
            .iter()
            .zip(&problem.row_bounds)
            .any(|(&value, &bound)| !bound_contains(bound, value))
    {
        return Err(LpError::NativeContractViolation);
    }
    let objective_value = problem
        .objective
        .iter()
        .zip(&variables)
        .fold(T::ZERO, |sum, (&cost, &value)| sum + cost * value);
    if !objective_value.is_finite() {
        return Err(LpError::NativeContractViolation);
    }
    Ok(LpResult {
        status,
        solution: Some(LpSolution {
            variables,
            row_activities: activities,
            objective_value,
        }),
    })
}

fn approximately_equal<T: Scalar>(left: T, right: T) -> bool {
    let factor = T::option_integer(4096).unwrap();
    let tolerance = T::epsilon() * factor * (T::ONE + left.abs() + right.abs());
    (left - right).abs() <= tolerance
}

fn bound_contains<T: Scalar>(bound: LpBound<T>, value: T) -> bool {
    let factor = T::option_integer(4096).unwrap();
    let tolerance = T::epsilon() * factor * (T::ONE + value.abs());
    match bound {
        LpBound::Free => true,
        LpBound::Lower(lower) => value + tolerance >= lower,
        LpBound::Upper(upper) => value - tolerance <= upper,
        LpBound::Range { lower, upper } => value + tolerance >= lower && value - tolerance <= upper,
        LpBound::Fixed(fixed) => (value - fixed).abs() <= tolerance * (T::ONE + fixed.abs()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn sparse_validation_rejects_malformed_layouts() {
        assert!(matches!(
            SparseColumns::<f64>::new(2, 1, vec![0, 2], vec![1, 0], vec![1.0, 2.0]),
            Err(LpError::InvalidInput(
                LpInputError::UnsortedOrDuplicateRowIndex { .. }
            ))
        ));
        assert!(matches!(
            SparseColumns::<f32>::new(2, 1, vec![0, 1], vec![2], vec![1.0]),
            Err(LpError::InvalidInput(
                LpInputError::RowIndexOutOfRange { .. }
            ))
        ));
        assert!(matches!(
            SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![f64::NAN]),
            Err(LpError::InvalidInput(LpInputError::NonFiniteValue))
        ));
    }

    #[test]
    fn every_bound_category_encodes() {
        let bounds = [
            LpBound::Free,
            LpBound::Lower(1.0),
            LpBound::Upper(2.0),
            LpBound::Range {
                lower: 1.0,
                upper: 2.0,
            },
            LpBound::Fixed(3.0),
        ];
        let expected = [4, 1, 2, 3, 3];
        for (bound, expected) in bounds.into_iter().zip(expected) {
            let (mut lower, mut upper, mut kind) = (0.0, 0.0, 0);
            encode_bound(bound, &mut lower, &mut upper, &mut kind);
            assert_eq!(kind, expected);
        }
    }

    #[test]
    fn problem_validation_rejects_bad_bounds_lengths_and_native_overflow() {
        let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
        assert!(matches!(
            LinearProgram::<f64>::new(
                vec![1.0],
                matrix.clone(),
                vec![LpBound::Range {
                    lower: 2.0,
                    upper: 1.0,
                }],
                vec![LpBound::Free],
            ),
            Err(LpError::InvalidInput(LpInputError::InvalidBoundRange))
        ));
        assert!(matches!(
            LinearProgram::<f64>::new(vec![], matrix, vec![LpBound::Free], vec![LpBound::Free]),
            Err(LpError::InvalidInput(LpInputError::ObjectiveLength))
        ));
        assert!(matches!(
            native_integer(usize::MAX),
            Err(LpError::InvalidInput(LpInputError::DimensionOverflow))
        ));
    }

    #[test]
    fn capacity_limit_rejects_before_ffi() {
        let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
        let problem = LinearProgram::<f64>::new(
            vec![1.0],
            matrix,
            vec![LpBound::Lower(0.0)],
            vec![LpBound::Free],
        )
        .unwrap();
        assert_eq!(
            problem.solve_with_options(LpOptions {
                maximum_matrix_entries: Some(0)
            }),
            Err(LpError::PagingRequired {
                required_matrix_storage: 1,
                available_matrix_storage: 0
            })
        );
    }

    #[test]
    fn malformed_callback_request_is_contained() {
        let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
        let mut context = CallbackContext {
            matrix: &matrix,
            cursor: 0,
            fault: None,
            panicked: false,
            active: Cell::new(false),
            panic_on_entry: false,
        };
        let _guard =
            ContextGuard::install((&mut context as *mut CallbackContext<'_, f64>).cast()).unwrap();
        let (mut row, mut column, mut value, mut category, mut option, mut data, mut flag) =
            (0, 0, 0.0, 0, 0.0, 0.0, 99);
        // SAFETY: every callback scalar pointer remains valid for the call.
        unsafe {
            matrix_callback_f64(
                &mut row,
                &mut column,
                &mut value,
                &mut category,
                &mut option,
                &mut data,
                &mut flag,
            );
        }
        assert!(context.fault.is_some());
        assert_eq!(flag, 3);
    }

    #[test]
    fn callback_panic_is_caught_before_ffi_boundary() {
        let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
        let mut context = CallbackContext {
            matrix: &matrix,
            cursor: 0,
            fault: None,
            panicked: false,
            active: Cell::new(false),
            panic_on_entry: true,
        };
        let _guard =
            ContextGuard::install((&mut context as *mut CallbackContext<'_, f64>).cast()).unwrap();
        let (mut row, mut column, mut value, mut category, mut option, mut data, mut flag) =
            (0, 0, 0.0, 0, 0.0, 0.0, 1);
        // SAFETY: every callback scalar pointer remains valid for the call.
        unsafe {
            matrix_callback_f64(
                &mut row,
                &mut column,
                &mut value,
                &mut category,
                &mut option,
                &mut data,
                &mut flag,
            );
        }
        assert!(context.panicked);
        assert_eq!(flag, 3);
    }

    #[cfg(feature = "optimization-linear-programming-in-memory-native-tests")]
    fn bounded_problem_f64() -> LinearProgram<f64> {
        let matrix =
            SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
        LinearProgram::<f64>::new(
            vec![1.0, 2.0],
            matrix,
            vec![LpBound::Lower(1.0)],
            vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
        )
        .unwrap()
    }

    #[test]
    #[cfg(feature = "optimization-linear-programming-in-memory-native-tests")]
    fn dsplp_solves_small_bounded_problem_without_paging() {
        let result = bounded_problem_f64().solve().unwrap();
        assert_eq!(result.status, LpStatus::Optimal);
        let solution = result.solution.unwrap();
        assert!((solution.variables[0] - 1.0).abs() < 1e-9);
        assert!(solution.variables[1].abs() < 1e-9);
        assert!((solution.objective_value - 1.0).abs() < 1e-9);
        assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
    }

    #[test]
    #[cfg(feature = "optimization-linear-programming-in-memory-native-tests")]
    fn splp_solves_small_bounded_problem_without_paging() {
        let matrix =
            SparseColumns::<f32>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
        let problem = LinearProgram::<f32>::new(
            vec![1.0, 2.0],
            matrix,
            vec![LpBound::Lower(1.0)],
            vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
        )
        .unwrap();
        let result = problem.solve().unwrap();
        assert_eq!(result.status, LpStatus::Optimal);
        let solution = result.solution.unwrap();
        assert!((solution.objective_value - 1.0).abs() < 1e-4);
        assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
    }

    #[test]
    #[cfg(feature = "optimization-linear-programming-in-memory-native-tests")]
    fn native_statuses_distinguish_infeasible_and_unbounded() {
        let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
        let infeasible = LinearProgram::<f64>::new(
            vec![1.0],
            matrix,
            vec![LpBound::Lower(1.0)],
            vec![LpBound::Fixed(0.0)],
        )
        .unwrap()
        .solve()
        .unwrap();
        assert_eq!(infeasible.status, LpStatus::Infeasible);
        assert!(infeasible.solution.is_none());

        let matrix = SparseColumns::<f64>::new(1, 2, vec![0, 0, 1], vec![0], vec![1.0]).unwrap();
        let unbounded = LinearProgram::<f64>::new(
            vec![-1.0, 0.0],
            matrix,
            vec![LpBound::Fixed(0.0)],
            vec![LpBound::Lower(0.0), LpBound::Fixed(0.0)],
        )
        .unwrap()
        .solve()
        .unwrap();
        assert_eq!(unbounded.status, LpStatus::NoFiniteSolution);
        assert!(unbounded.solution.is_none());
        assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
    }

    #[test]
    #[cfg(feature = "optimization-linear-programming-in-memory-native-tests")]
    fn equality_row_and_fixed_variable_are_honored() {
        let matrix =
            SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
        let result = LinearProgram::<f64>::new(
            vec![1.0, 3.0],
            matrix,
            vec![LpBound::Fixed(3.0)],
            vec![
                LpBound::Range {
                    lower: 0.0,
                    upper: 5.0,
                },
                LpBound::Fixed(1.0),
            ],
        )
        .unwrap()
        .solve()
        .unwrap();
        let solution = result.solution.unwrap();
        assert!((solution.variables[0] - 2.0).abs() < 1e-9);
        assert!((solution.variables[1] - 1.0).abs() < 1e-9);
        assert!((solution.row_activities[0] - 3.0).abs() < 1e-9);
    }

    #[test]
    #[cfg(feature = "optimization-linear-programming-in-memory-native-tests")]
    fn xerror_flag_and_output_units_are_restored() {
        let _lock = crate::runtime::lock_native();
        let mut before_flag = 0;
        let mut before_units = [0; 5];
        let mut before_count = 0;
        // SAFETY: the process lock is held and the documented five-element
        // output-unit buffer remains valid for the calls.
        unsafe {
            slatec_sys::legacy_error::xgetf(&mut before_flag);
            slatec_sys::legacy_error::xgetua(before_units.as_mut_ptr(), &mut before_count);
        }
        assert_eq!(
            bounded_problem_f64().solve().unwrap().status,
            LpStatus::Optimal
        );
        let mut after_flag = 0;
        let mut after_units = [0; 5];
        let mut after_count = 0;
        // SAFETY: the process lock is still held and buffers remain valid.
        unsafe {
            slatec_sys::legacy_error::xgetf(&mut after_flag);
            slatec_sys::legacy_error::xgetua(after_units.as_mut_ptr(), &mut after_count);
        }
        assert_eq!(after_flag, before_flag);
        assert_eq!(after_count, before_count);
        assert_eq!(after_units, before_units);
    }
}
