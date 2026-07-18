# Safe PCHIP interpolation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- The selected PCHIP package contains 41 program units. The safe facade exposes the twelve reviewed f32/f64 roots for monotone, controlled-monotone, spline-Hermite construction, evaluation, first-derivative evaluation, and definite integration.
- Curves own contiguous knot/value/derivative vectors, pass `INCFD=1`, and never sort or merge data. PCHIC scratch is `2*(N-1)`; PCHSP scratch is `2*N`; both use checked arithmetic and fallible allocation.
- Native endpoint-cubic extrapolation is explicit: safe methods reject it by default, while an allow policy returns a warning report.
- PCHIP numerical units retain DATA/SAVE constants and every error path reaches process-global XERROR state. Calls remain globally serialized; no PCHIP native parallel-safety claim is made.
- B-splines, smoothing, multidimensional interpolation, stride support, adapters, and translated algorithms remain deferred.
