# Safe SDRIVE expert ODE sessions

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Wrappers: `OdeSession<f32, _, _>` over `SDRIV3` and `OdeSession<f64, _, _>` over `DDRIV3`.
- Scope: real explicit `y'(t) = f(t, y)` only; roots, Jacobians, mass matrices, DAEs, and interpolation are not exposed.
- Callback: a panic, error, malformed native callback request (wrong `N`, null, non-finite time, or aliased buffers), or non-finite derivative sets native `N = 0`; no Rust panic crosses Fortran.
- State: session-owned `WORK`, `IWORK`, `NSTATE`, time, and state support same-direction continuation only.
- Runtime: native calls are globally serialized and XERROR control is restored by scope. `SDSTP` and `DDSTP` each declare initialized `IER` local storage (`DATA IER /.FALSE./`), which GNU MinGW emits as local writable `ier.0` `.bss`; reset paths do not make that state reentrant.
- I/O: the selected drivers have formatted `WRITE` diagnostics on error paths but no `OPEN`, `CLOSE`, or retained external-file protocol. Typed input validation and scoped XERROR handling avoid those paths for valid safe calls; serialization remains mandatory.
- Workspace: `WORK = (MXORD + 4) * N + 250`, `IWORK = 50` for `NROOT=0`, `MITER=0`, and `IMPL=0`.
- Native execution validation: complete against the explicit hash-verified cache, including `DDCOR` and `DGBFA`; analytic, continuation, callback failure/panic/non-finite, tolerance, excess-work, nested-call, serialization, XERROR-restoration, and cross-session contamination tests pass. The `link_ode_sdrive` probe retains `ddriv3_` and no DASSL, DEPAC, or LP root symbol. No source or native artifact is committed.
