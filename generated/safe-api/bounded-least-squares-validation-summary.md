# Safe bounded linear least squares

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Reviewed public wrappers: 2 (`DBOLS`, `SBOLS`).
- Contract: minimize `||A x-b||₂` subject to closed per-variable bounds; native `IND` codes are hidden behind `VariableBounds`.
- Storage: caller matrix and right-hand side are copied into owned column-major `W(M,N+1)` because the native driver overwrites it.
- Options: typed scaling and positive iteration limit only; raw option-array language is deferred.
- Runtime: calls serialize saved DBOLS/SBOLS and legacy XERROR state; only recoverable level-one iteration-limit behavior is scoped.
- Rank: the driver uses a private numerical dependence test and exposes no stable rank output; success does not claim a unique minimizer.
- Deferred: combined equality-bound, general inequality, and linear-programming drivers.
