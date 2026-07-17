# Safe bounded constrained linear least squares

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Reviewed wrappers: `DBOCLS` (f64) and `SBOCLS` (f32).
- Contract: minimize `||E x-f||â‚‚` while both original variables `x` and constraint expressions `y=Cx` have closed bounds. A fixed `y` bound represents an equality; intervals represent constraint ranges.
- Storage: `C` rows precede objective rows in owned column-major `W`; auxiliary `y` columns are native-owned.
- Workspace: all source formulas, including 17 writable `IOPT` entries, use checked arithmetic.
- Runtime: saved native state and documented level-one XERROR behavior are serialized and restored.
- Deferred: raw option-array language, active-set/dual diagnostics, constrained covariance, sparse/warm-start work, and linear programming.
