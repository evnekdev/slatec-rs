# Runtime-validated safe special functions

`slatec::special` is an opt-in, scalar safe facade over selected original
SLATEC Fortran routines. The numerical implementations remain Fortran; these
wrappers only validate inputs, coordinate the validated runtime profile, and
confine the raw ABI calls to small unsafe blocks.

## Supported profile and setup

This API is supported only with GNU Fortran on `x86_64-w64-mingw32` and Rust's
`x86_64-pc-windows-gnu` target. It requires the
`ffi-profile-gnu-mingw-x86_64` raw-FFI profile. It is not a portability claim
for other compilers or targets.

Enable the family features deliberately:

```toml
slatec = { version = "0.1", features = ["special-functions", "special-functions-f32", "special-functions-polynomials"] }
```

Native applications use the default `bundled` provider, which acquires
checksum-pinned sources, applies the validated profile support, and builds the
selected family closure automatically. Maintainers can still validate the
complete runtime profile explicitly:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-native-ffi --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-runtime-profile --offline
cargo test -p slatec --features special-functions-native-tests --target x86_64-pc-windows-gnu
```

Wrapper generation refuses a runtime profile unless its ABI, machine constants,
legacy-error behaviour, and FNLIB initialization are all validated for the
same selected-corpus and raw-FFI snapshots.

## Runtime and error boundary

An earlier special-function attempt was stopped because historical
machine-constant templates produced zero-filled values and terminated FNLIB
initialization. The explicit GNU MinGW profile now supplies separate,
profile-specific compatibility providers during native archive construction.
They do not change the archived selected source evidence or translate a
numerical algorithm.

The exposed FNLIB calls use an internal lock because saved initialization state
and legacy error configuration are process-global. This is a conservative
serialization policy for this module only; it does not make raw SLATEC or the
whole library thread-safe. A poisoned lock is recovered for later callers.

`SpecialFunctionError` reports ordinary rejected inputs and checked integer
conversion failures. It does not pretend every legacy XERROR path provides a
structured Rust error. Known fatal inputs are rejected before entering
Fortran; potentially fatal native probes run in child processes and remain
outside the public API contract.

## Included surface

The compact generated inventory records each raw symbol, domain policy, and
runtime state policy. This initial runtime-validated batch contains f64 and
f32 pairs for:

- stable elementary helpers: `log1p`, `exprel`, `cbrt`, degree sine/cosine,
  and Dawson's integral;
- gamma and beta helpers: gamma, reciprocal gamma, log gamma, beta, log beta,
  regularized beta, incomplete-gamma forms, digamma, factorial, and binomial;
- error functions: `erf` and `erfc`;
- Airy functions: `Ai`, `Bi`, and their scaled forms;
- scalar real Bessel functions: J, Y, I, K orders zero and one, including the
  selected scaled I/K forms;
- exponential integrals: `e1` and `ei`; and
- a coefficient-slice Chebyshev series evaluator under `slatec::polynomials`.

The f64 function names are unqualified; f32 variants have an `_f32` suffix.
The exact inclusion/deferment inventory lives in
[`generated/safe-api/special-function-wrapper-index.json`](../../generated/safe-api/special-function-wrapper-index.json)
and [`generated/safe-api/special-function-deferred.json`](../../generated/safe-api/special-function-deferred.json).
It intentionally excludes callbacks, dynamic workspaces, sequences, complex
and character returns, unresolved interfaces, and routine contracts that are
not yet suitable for a public safe call.

## Domains and examples

The wrappers deliberately use conservative validated domains. For example,
`e1` rejects non-positive input, `log1p` rejects values at or below `-1`, and
the exposed K Bessel forms require a positive argument. Other valid-domain
restrictions are recorded per wrapper in the generated inventory. Scaled forms
are named explicitly; they must not be confused with their unscaled analogue.

```rust,no_run
use slatec::special::{airy, bessel, elementary, error_functions, gamma};

assert!((elementary::sin_degrees(30.0)? - 0.5).abs() < 1.0e-12);
assert!((gamma::gamma(0.5)? - std::f64::consts::PI.sqrt()).abs() < 1.0e-12);
assert_eq!(error_functions::erf(0.0)?, 0.0);
assert_eq!(bessel::bessel_j0(0.0)?, 1.0);
assert!(airy::airy_ai(0.0)?.is_finite());
# Ok::<(), slatec::special::SpecialFunctionError>(())
```

No wrapper here replaces the original algorithm, configures the public legacy
error system, or guarantees behaviour outside the supported native profile.
