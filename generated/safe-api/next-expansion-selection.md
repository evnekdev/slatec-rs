# Next safe expansion selection

Selected batch: **checked tabulated data and global polynomial operations**. A single checked data type supports two distinctive SLATEC workflows—global polynomial interpolation and arbitrary-spacing tabulated integration—without duplicating existing callback APIs or exposing workspace arrays.

## Rejected alternatives

- `BINT4/DBINT4`: requires an explicit cubic B-spline boundary and knot policy distinct from the existing checked B-spline representation (future candidate)
- `BSPEV/DBSPEV and associated spline primitives`: already represented by owned B-spline and piecewise-polynomial safe methods; raw-array duplication would regress storage safety (covered by higher-level API)
- `GAUS8/DGAUS8`: callback-bearing routine would duplicate established QUADPACK options-based integration and callback containment (raw only)
- `SDRIV3/DDRIV3 events and analytic Jacobians`: requires a dedicated continuation, callback, and event-lifecycle audit (deferred)
- `DASSL Jacobian or mass-matrix callbacks`: matrix layout and callback aliasing remain outside the residual-only session contract (deferred)
- `dense, banded, packed, sparse, and eigenproblem algorithms`: safe facades are intentionally excluded in favour of mature Rust numerical ecosystem crates (external ecosystem)
