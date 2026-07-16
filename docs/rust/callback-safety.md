# Callback and global-state safety

## Status

This document proposes a safety model for SLATEC routines that invoke caller-supplied procedures or depend on shared mutable native state.

## Why callbacks are a separate boundary

Representative SLATEC interfaces include:

- scalar integrands such as `DQAG`;
- scalar root functions such as `DFZERO`;
- residual and optional Jacobian callbacks in `DNSQ` and `DNLS1`;
- residual/Jacobian callbacks plus persistent state in `DDASSL`;
- sparse operator and preconditioner callbacks in SLAP;
- matrix-access callbacks in sparse linear programming.

These interfaces differ in signature, control flags, reentrancy behavior and error transport. A single untyped global callback slot is not an acceptable general solution ([quadrature survey](../domains/quadrature.md); [nonlinear equations](../domains/nonlinear-equations.md); [differential equations](../domains/differential-equations.md); [sparse methods](../domains/sparse-methods.md)).

## Safety invariants

A safe callback adapter must ensure:

1. Rust panic never unwinds through Fortran or C frames;
2. captured Rust values outlive every possible native callback;
3. no native callback observes a moved or invalid context;
4. callback errors are transported through a documented native control path or reported after conservative termination;
5. nested calls do not resolve to the wrong context;
6. concurrent calls do not share mutable callback state accidentally;
7. cleanup occurs after normal, error and panic paths;
8. no aliasing violation is introduced by constructing Rust slices from native pointers.

## Proposed trampoline model

### Per-signature trampoline

Generate one trampoline family for each verified callback signature, for example:

```text
ScalarFnF64
ResidualFnF64
JacobianFnF64
DaeResidualFnF64
SparseMatVecF64
PreconditionerF64
```

Each trampoline:

- receives ABI-level arguments;
- resolves a scoped context;
- validates dimensions known at that point;
- invokes the Rust callback inside `catch_unwind`;
- records panic/error state;
- writes only documented outputs;
- requests native termination when the interface supports it.

### Scoped context

Preferred order of implementation strategies:

1. explicit user-data pointer, if a project-owned wrapper can add one;
2. thread-local stack of scoped contexts;
3. serialized process-global registry as a temporary fallback.

Many legacy Fortran signatures lack a user-data pointer. A generated C/Fortran wrapper cannot invent one unless it owns the complete callback dispatch path. For unmodified routines, a thread-local stack is the likely first prototype.

## Nested calls

A stack, not one slot, is required if a callback invokes another SLATEC wrapper. Each entry records:

- callback type ID;
- context pointer;
- active-thread identity;
- panic/error storage;
- cancellation state;
- generation token.

Nested use is enabled only after tests verify that native routines do not store callback pointers for later use.

## Concurrency

### Thread-local callback context

Thread-local context permits parallel calls on different threads only if:

- the Fortran routine and dependencies do not use mutable global state;
- the error subsystem is thread-safe or isolated;
- callbacks remain on the invoking thread;
- compiler runtime behavior is compatible.

### Global-state serialization

If `XERROR`/`XERMSG`, common blocks, saved variables or callback dispatch are process-global, a native-call mutex may be required.

Serialization classes may be:

```text
no_global_lock
error_subsystem_lock
family_lock
process_native_lock
```

The routine catalogue should record the narrowest verified class. Unknown defaults to conservative locking.

## Panic containment

The trampoline catches panics and stores a payload or normalized message. It must not resume unwinding until all native frames have returned.

Possible outcomes:

- use a documented negative callback flag to terminate;
- return a benign placeholder value and allow the driver to terminate through an evaluation limit;
- invoke a reviewed nonfatal cancellation shim;
- mark the routine unsupported for safe callbacks if no safe termination path exists.

The wrapper must document when callback errors cannot stop immediately.

## Callback error model

A safe callback may return:

```text
Result<Output, CallbackError>
```

The outer solver result distinguishes:

- native convergence/failure;
- user callback error;
- callback panic;
- cancellation;
- native fatal error.

Do not encode all callback failures as an arbitrary function value such as NaN unless the routine explicitly documents that behavior.

## Cancellation and observers

Some routines use mutable integer flags for cancellation, Jacobian mode or printing/monitoring. Safe wrappers may expose:

```text
ControlFlow::Continue
ControlFlow::Cancel
```

or observer hooks only when a verified flag meaning exists.

Printing callbacks should normally become observer callbacks rather than native formatted output. They may be omitted from the first safe release.

## Sparse operator callbacks

SLAP callbacks can require matrix-vector, transpose and preconditioner actions with package-specific work arrays and integer parameters. Proposed safe traits should be capability-specific:

```text
LinearOperator
TransposeLinearOperator
Preconditioner
```

The trampoline must not construct overlapping mutable and immutable slices from aliased Fortran buffers. Exact alias contracts must be verified from source.

## DAE/ODE callbacks

Stateful solver sessions must keep callback contexts alive across every continuation call. They must still unregister the active context after each native entry, because the legacy routine is not assumed to retain a callback pointer between calls.

A session owns:

- problem callback objects;
- parameter storage;
- persistent work arrays;
- solver mode;
- last native status;
- callback failure state.

## Global error subsystem

Callback safety cannot be separated from SLATEC error handling. A callback may trigger a routine that calls `XERMSG`, and the driver may also report an explicit status.

Proposed policy:

- exactly one selected error subsystem;
- configure nonfatal behavior where supported;
- intercept `XERHLT`/site hooks with reviewed shims;
- capture diagnostics into scoped storage when feasible;
- preserve repetition and severity data;
- serialize calls if error configuration is mutable global state.

## Unsafe escape hatch

Raw crates expose the original external-procedure ABI for expert users. Safe crates should not accept arbitrary raw function pointers unless the API is explicitly `unsafe` and documents context, lifetime, unwinding and concurrency requirements.

## Validation matrix

For each callback family test:

- normal callback;
- callback returning an application error;
- callback panic;
- cancellation;
- nested SLATEC call;
- two concurrent calls;
- callback with captured mutable state;
- invalid output or NaN;
- early native failure before first callback;
- repeated session continuation;
- cleanup after every exit path.

Tests must run under debug/release and supported compiler/platform combinations.

## Publication gates

A callback-based safe wrapper is publishable only when:

- callback signature is source-verified;
- all argument lengths/mutability are understood;
- panic containment is tested;
- cancellation/error behavior is defined;
- nested/concurrent policy is documented;
- global-state classification is known or conservatively locked;
- native quick checks and Rust integration tests pass.

## Open questions

- Do any routines invoke callbacks from a different thread? Historically unlikely, but it must be tested rather than assumed.
- Which callback APIs lack a prompt termination path?
- Can the error subsystem be made thread-local through wrappers?
- Are callback pointers ever retained in common or saved storage?
- Can a single thread-local context stack support mixed compiler ABIs?
- How should callback panics be represented without retaining non-`Send` payloads across threads?
- Which solver observers are useful enough for the first safe release?

## Sources

- [Routine prologues](../architecture/routine-prologues.md)
- [Error handling](../architecture/error-handling.md)
- [Quadrature](../domains/quadrature.md)
- [Nonlinear equations](../domains/nonlinear-equations.md)
- [Optimization](../domains/optimization.md)
- [Sparse methods](../domains/sparse-methods.md)
- [Differential equations](../domains/differential-equations.md)
- [Native build strategy](native-build-strategy.md)
