# Next safe expansion selection

Selected batch: **checked weighted polynomial least-squares fitting**. A single immutable checked representation covers weighted f32/f64 polynomial fitting, source-defined model selection, evaluation, derivatives, and origin coefficients without duplicating global interpolation or exposing native workspace arrays.

## Rejected alternatives

- `FC/DFC and EFC/DEFC`: constrained and incremental B-spline fitting retains persistent workspace, optional variance state, and caller-described constraints; no coherent owned workflow is proven (deferred stateful workflow)
- `P1VLU`: not a retained selected-corpus identity; POLFIT's single-precision evaluator is PVALUE (corrected source taxonomy)
- `BINT4/DBINT4`: promoted to the typed BSpline::interpolate_cubic workflow; its source boundary and knot policies are no longer an unclassified candidate (implemented in current milestone)
- `BSPEV/DBSPEV and associated spline primitives`: already represented by owned B-spline and piecewise-polynomial safe methods; raw-array duplication would regress storage safety (covered by higher-level API)
- `GAUS8/DGAUS8`: callback-bearing routine would duplicate established QUADPACK options-based integration and callback containment (raw only)
- `SDRIV3/DDRIV3 events and analytic Jacobians`: requires a dedicated continuation, callback, and event-lifecycle audit (deferred)
- `DASSL Jacobian or mass-matrix callbacks`: matrix layout and callback aliasing remain outside the residual-only session contract (deferred)
- `dense, banded, packed, sparse, and eigenproblem algorithms`: safe facades are intentionally excluded in favour of mature Rust numerical ecosystem crates (external ecosystem)
