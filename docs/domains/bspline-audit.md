# SLATEC B-spline audit

The selected SLATEC snapshot contains a broader real B-spline package than is
appropriate for one first safe wrapper. This milestone exposes only the
validated scalar B-representation operations: `BVALU`/`DBVALU` value and
derivative evaluation and `BSQAD`/`DBSQAD` definite integration. The safe
surface is [`BSpline`](../api/safe-bspline.md), available at
`slatec::interpolation::bspline` behind the hosted `bspline` feature.

`BVALU` and `DBVALU` consume a knot sequence `T` of length `N + K`, `N`
coefficients `A`, an order `K`, derivative order `IDERIV`, a point, a mutable
interval hint, and `3*K` work values. The safe type retains only `T`, `A`, and
`K`; each call owns a fresh interval hint and exact private work buffer.
`BSQAD` and `DBSQAD` use the same representation with two domain endpoints
and `3*K` work values. Their reviewed quadrature tables limit the initial safe
integration method to `K <= 20`.

The supported basic domain is `T[K-1]..=T[N]` in Rust indexing. Knots are
finite and nondecreasing, knot runs do not exceed `K`, and the basic domain
must have positive width. The public API does not sort knots, merge
duplicates, insert knots, alter coefficients, perform fitting, or request
native extrapolation. Evaluation outside the domain is rejected unless the
caller explicitly selects endpoint clamping.

The source closure is deliberately narrow: the four public roots,
`INTRV`/`DINTRV`, and the reviewed XERROR support closure. Evaluation has no
callback or file protocol. `BSQAD`/`DBSQAD` retain initialized Gauss tables,
and the reachable XERROR storage is process-global, so every native B-spline
call is `SerializedGlobal` under the established runtime lock. Avoiding
caller-managed workspace does not establish native reentrancy.

Deferred work includes `BINTK`/`DBINTK` and `BINT4`/`DBINT4` interpolation
construction; `BSPEV`/`DBSPEV` plus `BSPDR`/`DBSPDR` multi-derivative
evaluation, whose checked derivative-table and interval-cache representation
deserves a dedicated API; `BSPVN`/`DBSPVN`, `BSPVD`/`DBSPVD`, `BSPLVN`, and
`BSPLVD` basis APIs; and `BFQAD`/`DBFQAD` callback integration. Exact
B-to-piecewise-polynomial conversion is now owned by the separate
`piecewise-polynomial` feature using `BSPPP`/`DBSPPP`. `BSPDOC` is a non-executable
package documentation unit, not a numerical entry point. Tensor-product
splines, smoothing/fitting, NURBS, complex routines, and matrix or ecosystem
adapters remain deferred as well. These cases require their own
representation, workspace, callback, or native-state reviews.
