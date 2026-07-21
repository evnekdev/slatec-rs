# Safe tabulated-data operations

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- The `tabulated-data` feature provides one checked, owned sample representation for finite strictly increasing real abscissas and matching finite values. It exposes eight reviewed SLATEC roots through six public safe operations: Newton interpolation construction, value evaluation, derivative evaluation, Taylor coefficients, and f32/f64 arbitrary-spacing integration.
- `POLINT`/`DPLINT` store the native Newton representation privately. `POLYVL`/`DPOLVL` and `POLCOF`/`DPOLCF` operate only on owned buffers; `AVINT`/`DAVINT` reads checked samples.
- No callback, raw pointer, caller workspace, sorting, extrapolation, or retained native pointer is exposed. Every native call is serialized through the existing process-global runtime lock because XERROR and provider/runtime state remain reachable.
- `BINT4`/`DBINT4`, raw B-spline subsidiaries, `GAUS8`/`DGAUS8`, and ODE/DAE callback expansions were considered and deliberately deferred for the recorded, different-contract reasons.
