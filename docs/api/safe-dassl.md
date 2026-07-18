# Safe residual-only DASSL sessions

Enable the hosted API with an explicit backend:

```text
slatec = { version = "0.1", default-features = false, features = ["std", "source-build", "dassl"] }
```

`source-build` consumes only the separately acquired, SHA-256-verified cache;
it never downloads source during a Cargo build. `DaeSession<f32, _>` calls
SLATEC `SDASSL`; `DaeSession<f64, _>` calls `DDASSL`.

## Scope

The API represents real first-order index-1 initial-value problems
`G(t, y, y') = 0`. The caller supplies `t0`, `y(t0)`, and `y'(t0)`, which
must be sufficiently consistent with the residual. Construction validates
dimensions, finiteness, tolerances, controls, and checked workspace arithmetic,
but cannot prove DAE consistency or determine the DAE index.

DASSL uses its dense, internally finite-differenced iteration matrix. A
residual can therefore be evaluated more than once per step and can see
temporary trial values of `y` and `y'`. The owned, non-cloneable session
retains time, state, derivative, scalar or component-wise tolerance storage,
and opaque `RWORK`/`IWORK` continuation history. Exact restricted formulas are
`RWORK = 40 + (MAXORD + 4) * NEQ + NEQ^2` and `IWORK = 20 + NEQ`, with
`MAXORD` in `1..=5`. Advance diagnostics expose the documented cumulative
`IWORK(11..15)` step, residual, iteration-matrix, error-test, and convergence
failure counters with their Rust zero-based positions recorded in metadata.

## Residual and recovery

The Rust callback is `FnMut(t, &[y], &[y_prime], &mut [residual])` and returns
`Result<ResidualAction, E>`. `Continue` maps to `IRES=0` and
`RecoverableFailure` to `IRES=-1`; `FatalFailure`, a Rust error, panic,
non-finite output, or malformed native request stops native execution through
`IRES=-2`. Panics are caught and never unwind through Fortran. Callback
registration exists only for the native call and nested callback-based DASSL
use is rejected.

`advance_to` provides requested-output advancement. `IDID=3` is
`ReachedTarget`; the distinct native `IDID=2` status is retained defensively
but is unreachable because this restricted API fixes `INFO(4)=0`. Excessive work,
tolerance adjustment, and error-weight failure retain only the recovery state
documented by DASSL. Repeated error tests, convergence failures, singular
iteration matrices, repeated residual failures, fatal callback termination,
and native-contract violations poison the history; create a fresh session or
use `restart_from` after supplying a new initial pair.

## Deliberate exclusions and runtime

This initial feature has no user Jacobian, banded or sparse storage, events,
mass-matrix convenience API, consistent-initial-condition solver, complex
systems, sensitivity equations, dense output, or ecosystem adapters. It
neither creates files nor exposes Fortran units.

Calls are process-globally serialized for every backend. The reviewed closure
contains saved DATA state in DASSL subsidiaries and reaches process-global
XERROR; callback dispatch is additionally scoped by the same lock. `Send` for
a session is only automatic when its residual permits it; it does not mean
that two native calls execute concurrently.
