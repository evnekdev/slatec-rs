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
The wrapper-specific retained source and object projections are recorded in
[`per-wrapper-native-state.json`](../../generated/safe-api/per-wrapper-native-state.json);
unrelated provider objects are not projected onto every wrapper.

## Concurrency classes

| Class | Current meaning |
| --- | --- |
| `SerializedGlobal` | The safe entry point requires `std` and enters the process-wide native-runtime guard. It may not run its native segment concurrently with another such call. |
| `BackendDependent` | The API preserves an existing `no_std` or `alloc` capability and therefore cannot acquire the hosted lock. It makes no Rust thread-safety claim; provider source provenance and parallel stress tests are required before narrowing the classification. |

There is currently no `ParallelSafe` safe wrapper. A future relaxation needs a
complete transitive source audit, compiler-local-storage evidence, a reviewed
XERROR/error path, writable-object-symbol inspection, COMMON ownership,
provider/runtime evidence, and targeted concurrent stress tests. Successful
tests alone do not override an identified mutable global.

The lock is reentrant only on its owning Rust thread. This lets a non-callback
safe helper run from an active hosted callback without deadlock. A
callback-based nested native operation remains rejected, because its scoped
thread-local callback context would otherwise be ambiguous.

## Mutable storage evidence

The audit treats `COMMON`, `SAVE`, `DATA`, initialized locals, `BLOCK DATA`,
`EQUIVALENCE`, compiler/runtime state, XERROR, connected Fortran units, and
linked-provider state as potentially relevant. `DATA` initialization gives a
local saved lifetime; an explicit `SAVE` does too. The absence of `SAVE` is not
proof that ordinary locals are automatic: compiler flags can change that
storage model. This build records the exact GNU MinGW 14.2.0 command and its
storage probe in
[`fortran-storage-model.json`](../../generated/safe-api/fortran-storage-model.json).
With `-x f77 -std=legacy -ffixed-line-length-none -c`, ordinary probe locals
are automatic, while `DATA` locals emit writable static `.bss` storage.

Two concrete SDRIVE findings are `DATA IER /.FALSE./` in `SDSTP` and `DDSTP`.
Each object emits a local-linkage writable `ier.0` in `.bss`; local linkage is
still one shared instance in the loaded process, not thread-local storage.
`SDNTL`/`DDNTL` reset the value and set it on callback, factorization,
singular-diagonal, and related failures; `SDPST`/`DDPST` also reset and set it
on their factorization or user-solver failure paths. Resetting normal paths
does not make this saved mutable state reentrant. The sessions consequently
remain `SerializedGlobal`. Their Rust callback context is also scoped
thread-local mutable state, while XERROR uses saved `J4SAVE` and `XERSVE`
process state restored by RAII.

The reviewed acquisition process now hash-verifies all 330 selected provider
sources, including `src/ddcor.f` and `lin/dgbfa.f`, from their canonical
Netlib origins. It also compiles and inspects every selected object plus the
three project machine-constant profile objects. The resulting source,
writable-symbol, COMMON, and XERROR records are in
[`native-source-mutable-state-index.json`](../../generated/safe-api/native-source-mutable-state-index.json),
[`native-writable-symbol-index.json`](../../generated/safe-api/native-writable-symbol-index.json),
[`common-block-index.json`](../../generated/safe-api/common-block-index.json),
and [`xerror-state-index.json`](../../generated/safe-api/xerror-state-index.json).
Object inspection supplements source analysis; it does not prove reentrancy.

The source-state scanner now consumes complete fixed-form logical statements
from the shared quote- and Hollerith-aware lexer. Its fixture suite covers
named and blank COMMON, multiple blocks per statement, continuations, legal
spacing and case, labels, comments and literals, SAVE (including saved COMMON
blocks), DATA, initialized declarations, BLOCK DATA, EQUIVALENCE, ENTRY,
multiple program units, continued dimensions, and INCLUDE recognition. GNU
Fortran `-fdump-parse-tree -fsyntax-only` is an independent oracle for every
fixture and every one of the 333 selected/profile sources. The selected scan
and compiler oracle agree with no discrepancies; the bidirectional source and
COFF writable-symbol cross-check is recorded in
[`native-state-crosscheck.json`](../../generated/safe-api/native-state-crosscheck.json).

The empty selected COMMON result is therefore scoped evidence, not an
assumption. None of the exact 333 currently compiled selected/profile units
declares COMMON, as recorded in
[`selected-common-block-index.json`](../../generated/safe-api/selected-common-block-index.json).
The separate 1,452-file reviewed-corpus scan finds 172 COMMON declarations in
deferred or otherwise unselected files and preserves them in
[`generated/corpus/common-block-index.json`](../../generated/corpus/common-block-index.json).
Those future-family findings do not contaminate current closures, but they
cannot be lost when a later family is considered.

A global lock prevents races in the safe wrapper, but does not make the native
algorithm reentrant. Similarly, a session being `Send` when its callback and
error are `Send` does not permit simultaneous native execution. No current
wrapper is advertised as natively parallel-safe; external BLAS and other
provider/runtime implementations remain `BackendDependent` unless their own
state and threading contracts are proven.

Concurrency is recorded as three separate questions: Rust ownership and
`Send`/`Sync`, reentrancy of the exact retained SLATEC object closure, and the
thread-safety contract of the compiler runtime and linked provider. The
roadmap in
[`concurrency-relaxation-roadmap.md`](../../generated/safe-api/concurrency-relaxation-roadmap.md)
does not change behavior. Test-only instrumentation measures active hosted
lock scopes, the maximum simultaneous count, current ownership, and nested
same-thread entry; cross-family native tests must continue to observe a
maximum of one. Independent buffers remain storage evidence only, never proof
of native reentrancy.

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
