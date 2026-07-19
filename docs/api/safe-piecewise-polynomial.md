# Safe piecewise-polynomial interpolation

The hosted `piecewise-polynomial` feature exposes a narrow owned real
univariate SLATEC PP-form representation at
`slatec::interpolation::piecewise_polynomial`. It requires `std`, an explicit
native backend, and the reviewed GNU MinGW ABI profile:

```text
cargo add slatec --features std,external-backend,piecewise-polynomial
```

`PiecewisePolynomial::<f32>` calls `PPVAL` and `PPQAD`; the `f64` form calls
`DPPVAL` and `DPPQAD`. The owned breakpoints `XI` must be finite and strictly
increasing. For `LXI = breakpoints.len() - 1` pieces and PP order `K`, the
flat coefficient vector has exactly `K * LXI` values in the original
column-major `C(K,LXI)` layout. The `K` values for a piece are right Taylor
derivatives, not ordinary polynomial coefficients:

```text
p_j(x) = sum(i = 0 .. K-1) C[i,j] * (x - XI[j])^i / i!
```

Thus `coefficients_for_piece(j)[0]` is the value at the piece's left
breakpoint and each later element is the corresponding derivative there.
Inputs are kept unchanged: the API never sorts or merges breakpoints, scales
coefficients, materializes a dense matrix, or performs fitting.

`evaluate`, `derivative`, and `evaluate_into` use `PPVAL`/`DPPVAL` with a
fresh internal interval-search state. At an interior breakpoint they select
the right piece; at the final breakpoint they select the final piece. Batch
evaluation preserves point ordering and writes directly into the caller's
exact-length output slice. A derivative order must be below `order()`.

`integrate` calls `PPQAD`/`DPPQAD` for exact integration of the stored PP
pieces. Equal bounds return zero and reversed bounds preserve the native
negative sign. Native PP routines can extrapolate, but this safe API rejects
out-of-domain points and bounds by default. `Extrapolation::Clamp` is an
explicit Rust-side policy that clamps a coordinate to the nearest endpoint;
it never asks SLATEC to extrapolate.

When `bspline` is also enabled, `BSpline::to_piecewise_polynomial` performs
the exact native `BSPPP`/`DBSPPP` conversion. It allocates checked private
storage of `K * (N + 3)` work values, `K * (N - K + 1)` PP coefficients, and
`N - K + 2` candidate breakpoints, then validates the returned PP form before
exposing it. This is representation conversion, not interpolation or a
translated coefficient calculation.

Every native entry is globally serialized while the XERROR state is scoped
and restored. The reviewed PP numeric path has no user callback or external
file protocol, but XERROR and provider/runtime qualification still prevent a
parallel-native-execution claim. The API owns all native-mutable storage and
does not accept caller work arrays.

This first PP feature deliberately excludes PP-to-B-spline conversion, PCHIP
conversion, construction/fitting from data, multidimensional PP forms,
arbitrary strides, external array adapters, and translated algorithms. PCHIP
continues to own its distinct knots/values/derivatives representation; no
native selected root provides an exact PCHIP-to-PP container conversion.

Examples: `examples/piecewise_polynomial/from_pieces.rs`,
`evaluate_derivatives.rs`, `integrate.rs`, and `from_bspline.rs`.
