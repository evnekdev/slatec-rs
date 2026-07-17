# Safe API capability and native support

The `slatec` and `slatec-core` crates are `#![no_std]`. Their Cargo features describe capabilities rather than making an embedded-platform promise:

| Capability | Feature configuration | Current safe APIs |
| --- | --- | --- |
| Core only | `--no-default-features` plus a BLAS feature | Checked integer conversion and slice-based BLAS Levels 1–3 |
| Allocation available | `--no-default-features --features alloc` | Capability layer only; no current public function requires allocation without `std` |
| Hosted runtime | default `std`, plus a numerical-family feature | Runtime-serialized special functions; allocation- and callback-managed quadrature and roots |

The `alloc` feature links Rust's standalone `alloc` crate and does **not** enable or require `std`; a downstream `no_std` target supplies its allocator. The `std` feature depends on `alloc` in the other direction. Callback-bearing APIs require `std` because they use panic containment, thread-local callback state, and a process-wide native runtime lock. Special functions also require `std` because the validated FNLIB and legacy error state are process-global. Those requirements are explicit in the feature graph.

## Native backend levels

The project reports support in separate layers:

- API `no_std` compatible: **yes**;
- allocation-free API available: **yes**, for the checked infrastructure and BLAS slices;
- native backend available: **yes**, for the explicit GNU MinGW profile;
- native runtime validated: **yes**, for `x86_64-w64-mingw32` with the recorded GNU Fortran profile;
- bare-metal execution validated: **no**.

The current profile is an OS-hosted GNU Fortran runtime. A successful `no_std` Rust check does not show that the native library or its runtime can execute on bare metal.

## Feature examples

Portable crate shell without a numerical family:

```text
cargo check -p slatec --no-default-features
```

Allocation-free BLAS validation and wrappers:

```text
cargo check -p slatec --no-default-features --features blas-level1
```

Hosted callback and FNLIB APIs remain opt-in through `quadrature`, `roots`, and `special-functions`. Cargo does not download or compile Fortran. Native linking continues to use the explicitly prepared archive and the `SLATEC_NATIVE_LIB_DIR` and `SLATEC_GFORTRAN_RUNTIME_DIR` environment variables.

## Documentation authority

[The generated function index](function-index.md) gives alphabetical Rust, original Fortran, domain, and capability views. Compact generated records provide the same information in `generated/safe-api/function-index.json`, the original argument mapping in `fortran-argument-map.json`, and example coverage in `example-coverage.json`.
