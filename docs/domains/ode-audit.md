# ODE-family audit

This is a corpus and safety audit, not a public ODE API. The compact,
deterministic records in `generated/safe-api/ode-*.{json,md}` cover selected
SLATEC ODE drivers, their public interpolation support, callback contracts,
continuation state, global state, tolerances, Jacobians, events, workspaces,
and source closures.

## Discovered families

DEPAC provides `DERKF`/`DDERKF` (embedded Runge--Kutta--Fehlberg 4(5)),
`DEABM`/`DDEABM` (Adams--Bashforth--Moulton orders 1--12), and
`DEBDF`/`DDEBDF` (stiff BDF orders 1--5). `SDRIV1`/`DDRIV1`,
`SDRIV2`/`DDRIV2`, and `SDRIV3`/`DDRIV3` are the real SDRIVE family;
the last is its full expert interface. `SDASSL`/`DDASSL` solve implicit
DAEs, `G(t, y, y') = 0`, with BDF methods. `CDRIV*` is recorded but deferred
as a complex-number API outside the first real `f32`/`f64` scope.

## Recommendation

The recommended next family is **`SDRIV3`/`DDRIV3`**, restricted initially to
real explicit IVPs, `y' = f(t, y)`, with an RHS-only interface. The native RHS
callback receives a mutable local `N`; setting it to zero causes documented
`NSTATE = 6` termination. That provides a native termination path for a Rust
trampoline to contain panics and user errors without unwinding through Fortran.

The future API should be an owned, non-cloneable `OdeSession`, not a stateless
solve function. It should own mutable native work arrays, keep calls under the
native runtime lock with scoped XERROR restoration, and expose documented
start/continue transitions. Its first scope should defer roots, Jacobians,
mass matrices, and interpolation views even though SDRIVE can support them.

## Deferred families

`DERKF`/`DDERKF` and `DEABM`/`DDEABM` have compact workspaces and no driver
COMMON state, but their RHS callback has no documented abort signal. `DEBDF`,
`DDEBDF`, `INTYD`, and `DINTYD` retain integration history in process-global
COMMON blocks, so serialization cannot make independent sessions safe.
`SDASSL`/`DDASSL` do not retain COMMON state in the reviewed driver, but their
residual/Jacobian contracts, consistent initial conditions, and DAE failure
semantics warrant a separate milestone.

`SDRIV3` supports sign-change roots through `G`, but offers neither direction
filtering nor terminal/nonterminal selection, and locates at most one root per
equation per internal step. Event APIs are therefore deferred, not simulated.

## Storage policy

A later wrapper must use `&[T]`, `&mut [T]`, and lightweight checked matrix
views; allocate native mutable storage internally; and keep any `nalgebra`,
`ndarray`, or `faer` adapters optional and separate. It must not create a new
owned vector or matrix ecosystem. This audit adds no features, raw FFI,
provider closure, source artifact, native binary, network behavior, or
translated algorithm.
