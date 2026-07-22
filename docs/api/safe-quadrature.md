# Safe adaptive quadrature

The `slatec::quadrature` module is a safe, closure-based facade over the
original SLATEC/QUADPACK drivers in single and double precision. No
integration algorithm is implemented in Rust.

This API is available only for the validated `ffi-profile-gnu-mingw-x86_64`
profile: GNU Fortran targeting `x86_64-w64-mingw32`. The runtime-profile
machine constants, legacy errors, and FNLIB initialization must match the
selected-corpus snapshot. The offline `source-build` provider uses an explicit
verified cache; ordinary Cargo builds do not acquire source.

## API families

- `integrate` and `integrate_f32`: finite intervals with a selectable
  Gauss-Kronrod rule (`DQAG`/`QAG`).
- `integrate_singular` and `integrate_singular_f32`: finite intervals where an
  integrable endpoint singularity benefits from extrapolation (`DQAGS`/`QAGS`).
- `integrate_infinite` and `integrate_infinite_f32`: upper, lower, or two-sided
  infinite intervals (`DQAGI`/`QAGI`).
- `integrate_principal_value` and its single-precision counterpart: Cauchy
  principal values with the singular point strictly inside the interval
  (`DQAWC`/`QAWC`).
- `integrate_with_breakpoints` and `integrate_with_breakpoints_f32`: finite
  intervals split at caller-supplied interior points (`DQAGP`/`QAGP`).
- `integrate_weighted_endpoints` and its single-precision counterpart:
  algebraic and logarithmic endpoint weights (`DQAWS`/`QAWS`).
- `integrate_oscillatory` and its single-precision counterpart: finite sine or
  cosine weighted intervals (`DQAWO`/`QAWO`).
- `integrate_fourier_tail` and its single-precision counterpart: semi-infinite
  sine or cosine Fourier tails (`DQAWF`/`QAWF`).
- `integrate_non_adaptive` and its single-precision counterpart: QNG's fixed
  nested-rule progression, without adaptive subdivision (`DQNG`/`QNG`).
- `integrate_nc79` and its single-precision counterpart: the bounded,
  adaptive Newton-Cotes 7/9 driver (`DQNC79`/`QNC79`).
- `integrate_tabulated` and `integrate_tabulated_f32`: arbitrary-spacing,
  finite tabulated samples integrated by overlapping parabolas
  (`DAVINT`/`AVINT`). These use checked owned `TabulatedData` values and do
  not install a callback or expose a workspace.

The low-level `QK*` and `QC25*` kernels and public expert `*E` workspace
drivers remain deferred. The safe layer never exposes raw work arrays.

```rust,no_run
use slatec::quadrature::{integrate, IntegrationOptions};

let result = integrate(
    |x| x * x,
    0.0,
    1.0,
    IntegrationOptions::default(),
)?;
assert!((result.value - 1.0 / 3.0).abs() < 1.0e-10);
# Ok::<(), slatec::quadrature::IntegrationError>(())
```

## Callback and concurrency policy

The selected Fortran interfaces accept a function pointer but no user-data
pointer. Rust therefore installs a scoped, thread-local callback context while
holding a shared native-runtime lock. The trampoline uses `catch_unwind`; a
panic never crosses Fortran frames. A panic or non-finite callback result is
recorded, subsequent callback evaluations return a finite sentinel, and the
safe call reports `CallbackPanicked` or `CallbackFailed` after Fortran returns.

The context is cleared by an RAII guard on every Rust return path. Parallel
callers are serialized because the selected native runtime has process-global
error state. Nested callback-based integration is rejected before taking the
lock, avoiding deadlock. The integrand may call runtime-validated non-callback
SLATEC functions because the native lock is reentrant on its owning thread.

The sentinel strategy cannot force QUADPACK to return immediately. A failed
callback may therefore incur additional native evaluations before the Rust
error is returned, but its result is never presented as a successful integral.

## Extended driver inputs and workspaces

`integrate_with_breakpoints` accepts points in any order, copies and sorts
them, and rejects non-finite, duplicate, endpoint-equal, or exterior values.
For `n` supplied points it passes `NPTS2 = n + 2` and allocates
`LENIW = 2 * LIMIT + NPTS2`, `LENW = 2 * LENIW - NPTS2`; `LIMIT` must be at
least `n + 1`.

`EndpointWeight` represents exactly the four QAWS modes: `Algebraic`,
`AlgebraicLogLower`, `AlgebraicLogUpper`, and `AlgebraicLogBoth`. These calls
require a strictly increasing finite interval and finite `alpha` and `beta`
strictly greater than `-1`.

`OscillatoryWeight::{Sine, Cosine}` replaces QAWO/QAWF's raw selector. Finite
oscillatory calls use `OscillatoryOptions`; their internal buffers have
`LENIW = 2 * LIMIT` and `LENW = 2 * LENIW + 25 * MAXIMUM_MOMENTS`. Fourier-tail
calls use `FourierOptions`, require a positive absolute tolerance and at least
three cycles, and allocate `LENIW = CYCLE_LIMIT + 2 * LIMIT` and
`LENW = 2 * LENIW + 25 * MAXIMUM_MOMENTS`. Moment and cycle buffers are always
owned by the call and discarded afterward. A zero-frequency sine Fourier tail
returns exact zero without calling the callback; zero-frequency cosine follows
the selected driver's documented ordinary infinite-integral path.

`NonAdaptiveOptions` contains only tolerances because QNG has no caller
workspace or subdivision limit. `Nc79Options` contains the relative tolerance
used by QNC79, whose result deliberately reports evaluations but not an
invented error estimate. These two drivers are distinct from `IntegrationOptions`.

```rust,no_run
use slatec::quadrature::{
    integrate_fourier_tail, integrate_nc79, integrate_non_adaptive,
    integrate_oscillatory, integrate_weighted_endpoints, integrate_with_breakpoints,
    EndpointWeight, FourierOptions, IntegrationOptions, Nc79Options,
    NonAdaptiveOptions, OscillatoryOptions, OscillatoryWeight,
};

let kink = integrate_with_breakpoints(|x| x.abs(), -1.0, 1.0, &[0.0], IntegrationOptions::default())?;
let endpoint = integrate_weighted_endpoints(|_| 1.0, 0.0, 1.0, -0.5, 0.0, EndpointWeight::Algebraic, IntegrationOptions::default())?;
let finite_wave = integrate_oscillatory(|_| 1.0, 0.0, core::f64::consts::PI, 1.0, OscillatoryWeight::Sine, OscillatoryOptions::default())?;
let tail = integrate_fourier_tail(|x| (-x).exp(), 0.0, 1.0, OscillatoryWeight::Cosine, FourierOptions::default())?;
let fixed_rules = integrate_non_adaptive(|x| x * x, 0.0, 1.0, NonAdaptiveOptions::default())?;
let nc79 = integrate_nc79(|x| x * x, 0.0, 1.0, Nc79Options::default())?;
# let _ = (kink, endpoint, finite_wave, tail, fixed_rules, nc79);
# Ok::<(), slatec::quadrature::IntegrationError>(())
```

## Validation and errors

The wrapper rejects non-finite finite bounds, invalid tolerances, a zero or
unrepresentable subdivision limit, and a principal-value point outside the
open interval. Reversed finite bounds are supported. Workspace allocation uses
checked `4 * limit` arithmetic and checked conversion to the profile's Fortran
`INTEGER`.

The extended drivers use their documented formulas above with checked addition,
multiplication, allocation, and `FortranInteger` conversion. They reject
invalid breakpoint, weight, frequency, cycle, moment, and tolerance inputs
before calling Fortran. QAWF cycle-limit and bad-cycle statuses, QNG's
fixed-rule exhaustion, and QNC79's degenerate-interval status are mapped to
specific `IntegrationError` variants.

Native statuses are mapped without claiming more detail than QUADPACK reports:
maximum subdivisions, roundoff, bad integrand behavior, non-convergence, and
probable divergence or slow convergence. Native status 6 represents invalid
input; ordinary causes are prevented by the Rust checks. Callback NaN and
infinity are errors rather than integrand values.

`IntegrationOptions::default()` targets double precision. Single-precision
calls should use `IntegrationOptions::single_precision()` or request another
realistic tolerance; the safe layer rejects tolerances below the QAG-family
machine-precision rule.

## Native setup

The default provider builds and links the selected family closure when the
native tests run for the GNU target:

```text
cargo test -p slatec --features quadrature-native-tests --target x86_64-pc-windows-gnu --test quadrature_native
```

Source-only builds and tests can select `external-backend` and require neither
a Fortran compiler nor a native archive. Objects, archives, executables, and
detailed logs remain ignored.
