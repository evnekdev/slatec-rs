# Safe SDRIVE expert ODE sessions

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Wrappers: `OdeSession<f32, _, _>` over `SDRIV3` and `OdeSession<f64, _, _>` over `DDRIV3`.
- Scope: real explicit `y'(t) = f(t, y)` only; roots, Jacobians, mass matrices, DAEs, and interpolation are not exposed.
- Callback: a panic, error, malformed native callback request (wrong `N`, null, non-finite time, or aliased buffers), or non-finite derivative sets native `N = 0`; no Rust panic crosses Fortran.
- State: session-owned `WORK`, `IWORK`, `NSTATE`, time, and state support same-direction continuation only.
- Runtime: native calls are globally serialized and XERROR control is restored by scope. `SDSTP` and `DDSTP` each declare initialized `IER` local storage (`DATA IER /.FALSE./`); it is treated as potentially persistent and protected by the same lock pending object-level inspection.
- Workspace: `WORK = (MXORD + 4) * N + 250`, `IWORK = 50` for `NROOT=0`, `MITER=0`, and `IMPL=0`.
- Native execution validation: pending. The available offline source cache lacks `src/ddcor.f`; a composed temporary cache then lacked `lin/dgbfa.f`. No source was downloaded or committed, no archive was rebuilt, and no native ODE test, link probe, symbol inspection, or concurrent execution result is claimed.
