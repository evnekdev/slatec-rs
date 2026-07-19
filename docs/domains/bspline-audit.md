# SLATEC B-spline audit

The selected SLATEC snapshot contains a broader real B-spline package than is
appropriate for one coherent safe wrapper. This milestone exposes general
order exact interpolation with caller-provided complete knots through
`BINTK`/`DBINTK`, plus `BVALU`/`DBVALU` value and derivative evaluation and
`BSQAD`/`DBSQAD` definite integration. The safe surface is
[`BSpline`](../api/safe-bspline.md), available at
`slatec::interpolation::bspline` behind the hosted `bspline` feature.

`BVALU` and `DBVALU` consume a knot sequence `T` of length `N + K`, `N`
coefficients `A`, an order `K`, derivative order `IDERIV`, a point, a mutable
interval hint, and `3*K` work values. The safe type retains only `T`, `A`, and
`K`; each call owns a fresh interval hint and exact private work buffer.
`BSQAD` and `DBSQAD` use the same representation with two domain endpoints
and `3*K` work values. Their reviewed quadrature tables limit the initial safe
integration method to `K <= 20`.

`BINTK` and `DBINTK` solve the general-order collocation problem using a
complete input knot vector `T`, `N` strictly increasing input nodes `X`, and
`N` ordinates `Y`. They write `N` coefficients to `BCOEF`, use exact private
factorization storage `Q=(2*K-1)*N`, and use `WORK=2*K`. They enforce
`s(X(i))=Y(i)` without a regular native status argument. The safe constructor
therefore checks the documented endpoint placement and
Schoenberg--Whitney support conditions before entry and independently checks
node reproduction after return. It does not generate knots, retain the
factorization, or expose the BNFAC/BNSLV implementation details.

The supported basic domain is `T[K-1]..=T[N]` in Rust indexing. Knots are
finite and nondecreasing, knot runs do not exceed `K`, and the basic domain
must have positive width. The public API does not sort knots, merge
duplicates, insert knots, alter coefficients, perform fitting, or request
native extrapolation. Evaluation outside the domain is rejected unless the
caller explicitly selects endpoint clamping.

The source closure is deliberately narrow: the six public roots,
`INTRV`/`DINTRV`, `BNFAC`/`DBNFAC`, `BNSLV`/`DBNSLV`, `BSPVN`/`DBSPVN`, and
the reviewed XERROR support closure. Construction has no callback or file
protocol and no numeric `COMMON`, `SAVE`, `DATA`, or `BLOCK DATA` state in the
constructor-specific closure. `BSQAD`/`DBSQAD` retain initialized Gauss
tables, and reachable XERROR storage is process-global, so every native
B-spline call is `SerializedGlobal` under the established runtime lock.
Avoiding caller-managed workspace does not establish native reentrancy.

Deferred work includes `BINT4`/`DBINT4` fixed cubic interpolation
construction, whose endpoint-condition switches and knot-placement policy are
materially distinct from the selected `BINTK`/`DBINTK` API; `BSPEV`/`DBSPEV`
plus `BSPDR`/`DBSPDR` multi-derivative
evaluation, whose checked derivative-table and interval-cache representation
deserves a dedicated API; `BSPVN`/`DBSPVN`, `BSPVD`/`DBSPVD`, `BSPLVN`, and
`BSPLVD` basis APIs; and `BFQAD`/`DBFQAD` callback integration. Exact
B-to-piecewise-polynomial conversion is now owned by the separate
`piecewise-polynomial` feature using `BSPPP`/`DBSPPP`. `BSPDOC` is a non-executable
package documentation unit, not a numerical entry point. Tensor-product
splines, smoothing/fitting, NURBS, complex routines, and matrix or ecosystem
adapters remain deferred as well. These cases require their own
representation, workspace, callback, or native-state reviews.
