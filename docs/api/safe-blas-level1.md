# Safe BLAS Level 1 API

`slatec::blas::level1` is an opt-in safe facade over the original selected
SLATEC BLAS Level 1 Fortran routines. It does not reimplement any numerical
algorithm in Rust.

Enable it only with the compiler and target profile that was validated for the
raw bindings:

```toml
slatec = { version = "0.1", features = ["blas-level1"] }
```

The supported profile is GNU Fortran on `x86_64-w64-mingw32`, linked from Rust
with target `x86_64-pc-windows-gnu`. The default `bundled` provider downloads
checksum-pinned source files and builds the selected family automatically:

```text
cargo test -p slatec --features blas-level1-native-tests --target x86_64-pc-windows-gnu
```

`source-build`, `system`, and `external-backend` provide explicit alternatives.
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
