# Batch B callback-bearing public raw interfaces

Batch B promotes a conservative slice of callback-bearing SLATEC raw
interfaces after source-level callback ABI reconstruction. It is a generated
public raw tier: every promoted routine has a selected source hash, observed
native symbol, outer ABI fingerprint, callback ABI fingerprint, canonical
mathematical path, declaration feature, provider feature, compile probe, and
native link probe.

The generated authoritative reports are under
[`generated/raw-api/`](../../generated/raw-api/):

- `callback-classification.json` classifies every retained identity for
  callback-bearing Batch B eligibility;
- `callback-shapes.json` records each distinct callback ABI fingerprint and
  the routines that use it;
- `callback-forwarding.json` records callback actuals forwarded through
  subsidiary calls before invocation;
- `batch-b-candidates.json` records the 47 newly promoted routines;
- `batch-b-exclusions.json` gives every deferred identity an explicit reason;
- `batch-b-summary.md` gives reproducible counts; and
- `batch-b-manifest.json` records the deterministic semantic hash.

## Scope and boundary

Batch B accepts historically public callback-bearing drivers and expert
primitives whose outer ABI is otherwise in the supported numeric/logical GNU
Fortran profile and whose callback call shape can be reconstructed from the
selected fixed-form source. It covers QUADPACK-style function callbacks,
ODE function callbacks, and sparse iterative solver matrix/preconditioner
subroutine callbacks.

It deliberately does not add a safe callback abstraction. Raw callers pass
native Fortran-compatible function pointers directly. The native code may call
them synchronously, repeatedly, and through subsidiary routines. Rust callbacks
must not unwind across the FFI boundary, must honor the exact pointer
read/write contract, and must arrange any user state through process-global or
otherwise FFI-safe storage outside these declarations.

Batch B excludes unresolved callback signatures, non-public subsidiaries,
unsupported outer ABIs, missing or conflicting symbols, and callback-bearing
interfaces that were not in the compiler-observed callback-review batch. An
exclusion is a deferment decision, not a guessed declaration.

## Paths and features

Promoted canonical paths are grouped under explicit callback namespaces:

```rust
slatec_sys::quadrature::callbacks::qk15
slatec_sys::quadrature::callbacks::dqagse
slatec_sys::linear_algebra::sparse::callbacks::scg
slatec_sys::ode::callbacks::derkf
```

Enable `slatec-sys` declaration features such as `batch-b-quadrature`,
`batch-b-linear-algebra`, or `batch-b-ode`, or enable the corresponding broad
mathematical family alias. For the verified source provider, select the
matching `slatec-src` provider feature: `quadrature`, `linear-algebra`, or
`ode`. A direct raw application may instead provide compatible external
symbols.

Existing reviewed or compatibility raw paths remain available. Batch B does
not duplicate independent `extern` declarations for routines already promoted
by an earlier reviewed family.

## Validation and regeneration

The maintained offline workflow is:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-batch-b --offline --source-cache-dir target/slatec-source-cache
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-batch-b --offline --source-cache-dir target/slatec-source-cache
```

The generator emits deterministic canonical compile probes in
`crates/slatec-sys/tests/batch_b_compile_01.rs` and native link probes in
`crates/slatec/tests/raw_batch_b_link_01.rs`. Representative native smoke
tests in `crates/slatec/tests/raw_batch_b_native.rs` verify that selected
single-precision, double-precision, and sparse-solver callbacks are actually
invoked on the supported GNU MinGW source profile.

Batch B remains below the reviewed semantic-documentation tier. It stabilizes
the raw declaration path and callback ABI evidence for its candidates, but it
does not claim full numerical validation, safe panic containment, per-routine
semantic examples, or a user-data/lifetime-safe Rust callback wrapper.
