# Runtime concurrency and storage policy

## Decision

Native SLATEC calls are not advertised as thread-safe by default. Hosted safe
wrappers that enter the reviewed legacy runtime use the reentrant-per-thread,
process-wide `crate::runtime::lock_native` guard. The lock covers the complete
native call, including scoped callback registration and any reviewed XERROR
control change. It is a serialization policy, not proof that the underlying
Fortran is reentrant.

The complete per-function decision is generated in
[`generated/safe-api/concurrency-index.json`](../../generated/safe-api/concurrency-index.json).
Each record links to the corresponding findings in
[`generated/safe-api/mutable-global-state-index.json`](../../generated/safe-api/mutable-global-state-index.json).

## Concurrency classes

| Class | Current meaning |
| --- | --- |
| `SerializedGlobal` | The safe entry point requires `std` and enters the process-wide native-runtime guard. It may not run its native segment concurrently with another such call. |
| `BackendDependent` | The API preserves an existing `no_std` or `alloc` capability and therefore cannot acquire the hosted lock. It makes no Rust thread-safety claim; provider source provenance and parallel stress tests are required before narrowing the classification. |

There is currently no `ParallelSafe` safe wrapper. A future relaxation needs a
complete transitive source audit, a reviewed XERROR/error path, object-symbol
inspection when applicable, and targeted concurrent stress tests. Successful
tests alone do not override an identified mutable global.

The lock is reentrant only on its owning Rust thread. This lets a non-callback
safe helper run from an active hosted callback without deadlock. A
callback-based nested native operation remains rejected, because its scoped
thread-local callback context would otherwise be ambiguous.

## Mutable storage evidence

The audit treats `COMMON`, `SAVE`, `DATA`, initialized locals, `BLOCK DATA`,
`EQUIVALENCE`, compiler/runtime state, XERROR, connected Fortran units, and
linked-provider state as potentially relevant. It also records explicit
unknowns; absence of a `SAVE` statement is not evidence that storage is
automatic.

Two concrete SDRIVE findings are `DATA IER /.FALSE./` in `SDSTP` and `DDSTP`.
`IER` is subsequently passed to routines that can modify it, so the initialized
local is treated as potentially persistent until GNU object-symbol inspection
proves otherwise. The ODE sessions consequently remain `SerializedGlobal`.
Their Rust callback context is also scoped thread-local mutable state, and
XERROR control is process-global mutable state restored by RAII.

The available local offline cache is not a complete selected closure: it is
missing `src/ddcor.f`, and a composed temporary cache also lacks `lin/dgbfa.f`.
No native archive, object file, or binary inspection was produced for this
milestone. The generated state index marks the remaining selected source units
as unresolved rather than claiming an absence of mutable static storage.

## Storage and interoperability

`storage-contract-index.json` is the per-native-argument contract. The core
API remains slices, `Vec`, and existing checked views. Fortran matrices are
column-major with their reviewed leading dimensions; vectors use their
documented contiguous or incremented representation. Native work arrays are
internal opaque storage.

The following conversions are prohibited unless a later routine-specific
adapter explicitly documents and validates them:

- transpose or row-major reinterpretation;
- dense/packed expansion or packing;
- sparse densification or sparse-layout conversion;
- materializing arbitrary-stride views;
- any hidden data layout conversion.

An owned preservation copy is allowed only for reviewed native mutation or
workspace requirements, and must preserve the original logical Fortran order.
The core crate does not accept `matrixpacked`, `nalgebra`, `ndarray`, or `faer`
objects directly. Optional adapters remain possible, but must be separate
features and check scalar support, layout, contiguity/stride, mutability, and
leading dimensions. Packed triangular storage is never interchangeable with a
full rectangular Fortran matrix merely because both use column-oriented data.

## Deferred LP paging

`SPLP`/`DSPLP` remain reviewed-but-deferred. Their model is
`minimize c^T x` subject to `A x = w`, with typed bounds on both `x` and `w`.
Their one-based `USRMT`/`DUSRMT` sparse callback and conditional paging protocol
remain unsuitable for a safe public wrapper. Paging is documented as needed
for save/restore or when matrix storage exceeds high-speed memory; it does not
require the caller to pre-open unit 1. When activated, `PRWVIR`/`DPRWVR` uses
the selected global Fortran unit (option 54) and can call `SOPENM`; closing is
option-controlled and uses `STATUS='KEEP'`. Serialization does not provide a
safe unit lifecycle, deterministic deletion, or reliable recovery after an
I/O failure. No FFI, provider feature, source closure, wrapper, or I/O shim is
introduced.

See [`lp-paging-policy.json`](../../generated/safe-api/lp-paging-policy.json)
for the source-grounded record.
