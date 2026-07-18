# Safe BLAS Level 1 API

`slatec::blas::level1` is an opt-in safe facade over the original selected
SLATEC BLAS Level 1 Fortran routines. It does not reimplement any numerical
algorithm in Rust.

Enable it only with the compiler and target profile that was validated for the
raw bindings:

```toml
slatec = { version = "0.1", default-features = false, features = ["std", "source-build", "blas-level1"] }
```

The supported profile is GNU Fortran on `x86_64-w64-mingw32`, linked from Rust
with target `x86_64-pc-windows-gnu`. Acquire the verified cache explicitly,
then select the offline `source-build` provider:

```text
cargo run -p slatec-tools --bin slatec-corpus -- acquire-provider-sources --output-dir evidence/provider-sources
$env:SLATEC_SOURCE_CACHE = "evidence/provider-sources"
cargo test -p slatec --features blas-level1-native-tests --target x86_64-pc-windows-gnu
```

`prebuilt` is blocked pending rights clearance; `system` and
`external-backend` provide explicit alternatives.
Applications that need a raw declaration can depend on `slatec-sys` directly;
the safe facade deliberately does not duplicate it.

## Supported routines

The initial interface provides f32 and f64 wrappers for `COPY`, `SWAP`, `SCAL`,
`AXPY`, `DOT`, `NRM2`, `ASUM`, `I?AMAX`, and `ROT`. Each has a contiguous form
and an explicit `_strided` form. Complex operations, mixed-precision dot and
scale operations, callbacks, complex-returning functions, character routines,
and all unresolved raw interfaces remain out of this safe API.

Contiguous calls use unit stride and validate matching lengths where needed.
Strided calls validate `n`, nonzero increments, Fortran `INTEGER` conversion,
and required backing storage with checked arithmetic. A strided slice is the
BLAS backing store. With a negative increment, the Fortran BLAS routine derives
the logical first position from that backing-store base; callers must therefore
provide the complete storage span, not a pre-offset subslice.

Zero-length operations return without invoking Fortran. `I?AMAX` returns
`Result<Option<usize>, BlasError>`: `None` for an empty vector and a Rust
zero-based index otherwise.

```rust,no_run
use slatec::blas::level1::{daxpy, ddot, dnrm2, idamax};

let x = [1.0, 2.0, 3.0];
let mut y = [4.0, 5.0, 6.0];
daxpy(2.0, &x, &mut y)?;
assert_eq!(y, [6.0, 9.0, 12.0]);
assert_eq!(ddot(&x, &y)?, 60.0);
assert_eq!(dnrm2(&[3.0, 4.0])?, 5.0);
assert_eq!(idamax(&[1.0, -5.0, 2.0])?, Some(1));
# Ok::<(), slatec::blas::BlasError>(())
```

`BlasError` reports validation failures rather than panicking for normal input
errors. Raw Fortran pointers remain confined to small, documented unsafe blocks
inside the wrappers; immutable source slices and mutable destination slices
express the safe API's aliasing contract.

The safe complex API is intentionally deferred. `slatec-sys` layout records
are not exposed as preferred public complex types, and no complex return ABI is
assumed by this module.

## Backend-qualified concurrency

Concurrency is an implementation-specific claim, not a consequence of the
BLAS name. The exact hash-verified GNU MinGW `source-build` objects for
`ASUM`, `AXPY`, `COPY`, `DOT`, `SCAL`, `SWAP`, and `I?AMAX` have single-object
closures with no writable static symbols, transitive calls, COMMON, XERROR,
Fortran I/O, or callbacks. Under the recorded production compiler flags,
ordinary locals are automatic. High-contention tests observe simultaneous
entry into those exact native routines. Therefore their contiguous and
strided wrappers are classified `ParallelSafe` for that backend only, with
this qualification:

> Independent calls with non-overlapping mutable storage may execute
> concurrently.

Read-only inputs may be shared. Safe Rust borrowing prevents one call from
constructing overlapping mutable slices; unsafe aliases created elsewhere are
outside this contract. Positive and negative nonzero increments are supported,
zero increments are rejected, and zero-length operations return before FFI.

`SNRM2`/`DNRM2` and `SROT`/`DROT` remain excluded because their selected
objects contain saved writable state. The `system` and `external-backend`
profiles cannot identify or version the user-supplied provider, so they remain
`BackendDependent`; the same applies to unqualified OpenBLAS, MKL, BLIS,
Accelerate, separately built Netlib BLAS, and unknown providers. The reviewed
Netlib/SLATEC loops are single-threaded, while external providers may use
worker threads or process-global thread controls. `slatec-rs` does not change
provider thread counts, and `ParallelSafe` is not a performance or
oversubscription recommendation.

The conservative provider records link to the vendors' own threading material:
[OpenBLAS controls](https://github.com/OpenMathLib/OpenBLAS#setting-the-number-of-threads-using-environment-variables),
[oneMKL threading control](https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/2024-2/threading-control.html),
[BLIS multithreading](https://github.com/flame/blis/blob/master/docs/Multithreading.md),
and [Accelerate BLAS threading](https://developer.apple.com/documentation/accelerate/blas_threading).
These links explain why provider-internal behavior varies; they do not qualify
any provider under the project's unidentified external profiles.

Production dispatch is unchanged. These `core`-capable BLAS wrappers continue
to call their selected provider directly. ODE, callback, XERROR, and solver
families retain the process-wide native runtime lock.
