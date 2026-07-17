# SLATEC optimization-family audit

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Discovery found no standalone scalar-minimization, vector-bound nonlinear-minimization, or nonlinear-programming driver in the selected corpus.
- Existing safe families cover nonlinear least squares, covariance, nonlinear equation support, derivative checking, and constrained linear least squares.
- The only remaining public general optimizer is `SPLP`/`DSPLP`; both remain deferred because their mandatory paging path retains a process-global direct-access external file.
- Recommendation: **no optimization family yet**. A future wrapper must retain panic-contained callbacks, scoped XERROR control, serialized native calls, and slices/lightweight views with privately owned mutable native storage; optional nalgebra/ndarray/faer adapters must remain separate.
- No public optimization feature, raw declaration, provider closure, native source, or translated algorithm is added.
