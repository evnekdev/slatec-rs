# Probability and statistics

## Scope

SLATEC’s explicit GAMS `L` holdings are smaller than its linear algebra or special-function domains. They include selected distribution functions, pseudo-random generators, covariance/variance support and regression/fitting routines. Many statistical capabilities are cross-listed under special functions (`C`), approximation (`K`), optimization (`G`) or linear algebra ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Principal families

| Family | Representative routines | Role |
|---|---|---|
| Distribution-related functions | `ERF/DERF`, `ERFC/DERFC` and functions based on incomplete gamma/beta | normal-tail and related calculations |
| Uniform random generation | `RAND`, `RUNIF` | pseudo-random uniform values |
| Gaussian generation | `RGAUSS` | normally distributed pseudo-random values |
| Curve variance/covariance | `CV/DCV`, `SCOV/DCOV` | variance of spline fit or covariance after nonlinear least squares |
| Regression and fitting | `FC/DFC`, `EFC/DEFC`, polynomial and least-squares families | weighted/constrained regression |
| Time-series/transform overlap | FFT routines cross-referenced from GAMS `L10` | spectral/time-series calculations |

The table of contents does not present a broad standalone statistics package comparable to modern statistical environments.

## Provenance

- distribution calculations often come from FNLIB or other special-function families;
- covariance after nonlinear least squares belongs operationally to the `NLS1`/MINPACK-derived fitting workflow;
- spline regression belongs to interpolation/approximation families;
- random generators may be individually contributed utilities rather than one package.

**Project policy:** preserve functional cross-listing. Do not force all routines classified under `L` into a single historical package.

## Precision variants and state

Distribution and fitting routines commonly have S/D pairs. Random generators may expose single-precision results and internal integer state or saved state. Exact seed/state control must be verified from source before wrapping.

Random-number routines are particularly sensitive to:

- hidden global state;
- reproducibility across compilers;
- integer overflow assumptions;
- seed initialization;
- thread safety.

They should not be exposed as global free functions until state behavior is known.

## Dependencies

Statistics depends heavily on:

- gamma, incomplete gamma, beta and error functions;
- linear least squares and matrix factorization;
- nonlinear least squares;
- interpolation and spline representation;
- transforms for spectral methods;
- machine constants and error handling.

## Numerical limitations

- tail probabilities can lose relative accuracy through subtraction;
- covariance estimates rely on rank/model assumptions;
- historical random generators may have short periods or poor modern statistical quality;
- floating-point and integer overflow behavior can affect reproducibility;
- regression routines inherit conditioning and scaling issues from linear algebra;
- the domain lacks comprehensive descriptive statistics, modern distributions and robust inference.

## Modern equivalents

Modern statistical libraries provide much broader distributions, RNG algorithms, tests and models. Rust’s `rand` ecosystem and dedicated statistics crates should normally be preferred for new stochastic applications. SLATEC routines are most valuable for legacy reproducibility and algorithms embedded in other SLATEC workflows.

## FFI and safe-wrapper considerations

| Risk | Mitigation proposal |
|---|---|
| hidden RNG state | inspect source; expose an owned generator only if state can be isolated |
| platform-dependent integer arithmetic | reproducibility tests on each target |
| tail probability cancellation | expose direct complementary functions where available |
| covariance validity | return rank/status metadata |
| fitting workspaces | reuse interpolation/optimization safe layers |
| cross-domain ownership | re-export rather than duplicate implementations |

## Project proposals

- Put probability-distribution special functions in `slatec-special`, with convenience re-exports from a statistics facade.
- Put fitting and covariance methods with their owning solver/interpolant.
- Create `slatec-statistics` only if a coherent public surface remains after provenance-based ownership.
- Avoid advertising historical RNGs as cryptographic or modern simulation-quality generators.

Candidate result objects should preserve solver status and rank assumptions.

## Tentative crate ownership

- Raw routines remain with their provenance packages/families.
- Safe facade: optional `slatec-statistics`.
- RNGs may belong in `slatec-utilities` or a dedicated compatibility module after state analysis.

## Open questions

- What are the algorithms and periods of `RAND`, `RUNIF` and `RGAUSS`?
- Is RNG state explicit, common-block based or saved locally?
- Which probability distribution functions are directly represented beyond error/incomplete gamma/beta functions?
- Which covariance routines require preceding solver work arrays?
- Is there enough coherent functionality to justify a separate safe crate?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-fnlib`](https://www.netlib.org/slatec/fnlib/)
