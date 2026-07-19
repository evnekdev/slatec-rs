# SLATEC piecewise-polynomial audit

The reviewed real PP roots in the authoritative selected source snapshot are
`PPVAL`/`DPPVAL` (scalar value or derivative evaluation),
`PPQAD`/`DPPQAD` (definite integration), and `BSPPP`/`DBSPPP` (B-spline to PP
conversion). The safe facade is the hosted `piecewise-polynomial` feature;
its complete machine-readable inventory, source closure, storage contract,
domain rules, conversion decisions, and concurrency classification are in
`generated/safe-api/piecewise-polynomial-*.json`.

PP form uses strictly increasing `XI` breakpoints and a `C(LDC,LXI)` matrix.
For piece `j`, `C(i,j)` is the `(i-1)`th right derivative at `XI(j)`, so the
polynomial divides Taylor terms by factorials. `PPVAL` selects the right
piece at an interior breakpoint and the final piece at the final endpoint.
`PPQAD` integrates the stored pieces exactly and accepts signed/reversed
limits. Both native routines can extrapolate, but the safe API makes the
policy explicit and defaults to rejecting out-of-domain calls.

`BSPPP`/`DBSPPP` are included because their B-representation input and PP
output contracts are complete. Given `N` coefficients and order `K`, the
facade preallocates the reviewed capacities: `K*(N+3)` work values,
`K*(N-K+1)` PP coefficients, and `N-K+2` breakpoints. The driver writes a
strict PP breakpoint sequence even where the source B-spline had repeated
knots; output is validated before it becomes a `PiecewisePolynomial`.

The PP numeric closure includes `INTRV`/`DINTRV` and, for conversion,
`BSPDR`/`DBSPDR`, `BSPEV`/`DBSPEV`, and `BSPVN`/`DBSPVN`, plus the checked
XERROR support closure. Focused source and object review found no numerical
callback or Fortran-unit I/O protocol and no mutable PP numerical COMMON,
SAVE, or DATA state. XERROR remains process-global, so safe PP calls retain
the global runtime lock; independent owned vectors alone do not establish
native reentrancy.

PCHIP conversion is explicitly deferred. PCHIP's safe type stores knots,
values, and Hermite derivatives, whereas this PP representation stores
right-Taylor derivative columns. Reconstructing PP coefficients in Rust would
be a new translated conversion rather than a reviewed selected native call.
The inverse PP-to-B-spline conversion, construction/fitting, multidimensional
forms, arbitrary strides, complex data, and ecosystem adapters are deferred
for the same reason: they need their own storage and source-contract audit.
