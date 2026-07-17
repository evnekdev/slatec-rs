# Safe equality and inequality constrained linear least squares

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Reviewed wrappers: `DLSEI` (f64) and `LSEI` (f32).
- Contract: minimize `||A x-b||₂` subject to `E x=f` and `G x≥h`; native rows are copied in that order.
- Status: MODE 0 and 1 yield defined results; MODE 2 and 3 are structured infeasibility errors; MODE 4 is a contained contract violation.
- Ranks: `IP(1)` is equality rank and `IP(2)` is reduced-objective rank.
- Deferred: covariance option, combined bounds, duals, active-set state, linear programming, sparse/warm-start APIs.
