# Fourier and related transforms

## Scope

SLATEC classifies integral transforms under GAMS `J`. The principal identifiable package is FFTPACK, relocated with FISHPACK into `slatec/fishfft`. It provides complex, real, sine, cosine and quarter-wave transforms, usually with separate initialization routines ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`netlib-fftpack`](https://www.netlib.org/fftpack/)).

## Principal families

| Family | Initialization | Forward/backward routines | Notes |
|---|---|---|---|
| Complex periodic FFT | `CFFTI` | `CFFTF`, `CFFTB` | interleaved complex storage in historical Fortran representation |
| Real periodic FFT | `RFFTI` | `RFFTF`, `RFFTB` | packed real coefficient convention |
| Simplified real FFT | `EZFFTI` | `EZFFTF`, `EZFFTB` | simplified coefficient interface |
| Cosine transform | `COSTI` or family-specific init | `COST` | self-inverse up to normalization conventions |
| Quarter-wave cosine | `COSQI` | `COSQF`, `COSQB` | specialized symmetry |
| Sine transform | `SINTI` | `SINT` | real sine transform |
| Quarter-wave sine | `SINQI` | `SINQF`, `SINQB` | specialized symmetry |
| Internal kernels | shared radix passes | `*PASSF*`, `*PASSB*`, factorization helpers | subsidiary |

The SLATEC table of contents includes internal `RFFTF1/CFFTF1`-style routines and initialization helpers, but users normally call the package-level drivers.

## Initialization and workspace model

FFTPACK separates plan-like initialization from execution. Initialization factors the transform length and stores trigonometric tables in a work array. Reusing the initialized work array avoids repeated setup.

**Safe-wrapper implication:** represent initialized transforms as owned plan objects rather than ask users to size and retain anonymous work arrays.

## Normalization and representation

Forward and backward transforms are generally not both normalized. The exact scale factor depends on the transform family and must be documented per routine. Real transforms use packed coefficient layouts that differ from modern complex-array APIs.

A safe wrapper may offer:

- a faithful packed form;
- a convenience conversion to standard complex coefficients;
- explicit normalization options applied outside the raw call.

It must not silently change historical normalization without documenting it.

## Precision variants

The historical FFTPACK subset is predominantly single precision in its original package organization, though SLATEC may contain generated or adapted variants. Complex and real transforms are distinguished by routine family rather than a uniform S/D/C matrix. Exact double-precision coverage must be derived from the source inventory, not assumed.

## Dependencies

FFTPACK is mostly self-contained, using:

- factorization of transform length;
- trigonometric table generation;
- radix-specific pass routines;
- real work arrays and sometimes integer factors.

FISHPACK routines depend on FFTPACK for some separable PDE solves, creating a direction from PDE support to transforms.

## Numerical limitations

- performance depends on factorization of transform length;
- normalization is nonstandard relative to some modern libraries;
- packed real layouts are easy to misuse;
- work-array corruption invalidates later transforms;
- transforms are in-place or overwrite input in many routines;
- historical implementations are not SIMD/thread optimized;
- large-size integer overflow and workspace formulas require checks.

## Modern equivalents

FFTW and contemporary Rust FFT libraries provide optimized planning, multiple dimensions, SIMD and threading. FFTPACK remains useful for reproducibility, simple self-contained builds and historical compatibility, not as a default high-performance FFT engine.

## FFI considerations

| Risk | Proposed mitigation |
|---|---|
| reusable mutable work array | owned plan with controlled mutable execution |
| in-place aliasing | distinct in-place/out-of-place safe methods with copies when required |
| packed real coefficients | typed packed representation and conversion helpers |
| complex ABI | use verified compiler representation or raw real-pair storage |
| normalization ambiguity | method-specific documentation and explicit normalization enum |
| transform-length edge cases | checked constructors |
| plan thread safety | clone per thread or guard mutation until tested |

## Project proposals

```text
ComplexFftPlan
RealFftPlan
CosineTransformPlan
SineTransformPlan
QuarterWavePlan
```

Each plan records length, precision, workspace and normalization behavior. Public methods should name forward/backward direction and whether input is overwritten.

## Tentative crate ownership

- Raw: `slatec-fftpack-sys`.
- Safe: `slatec-transforms`.
- FISHPACK remains a PDE package depending on FFTPACK; the shared Netlib directory is not a safe-crate boundary.

## Open questions

- Which double-precision FFTPACK variants are actually present?
- Is the initialized workspace mutated during execution?
- Can one plan be used concurrently?
- What exact real coefficient layout does each driver expose?
- Which source revision is incorporated into SLATEC 4.1?
- Are symbol names likely to collide with other FFTPACK distributions?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-fishfft`](https://www.netlib.org/slatec/fishfft/)
- [`netlib-fftpack`](https://www.netlib.org/fftpack/)
- [`fftpack-user-guide`](https://www.netlib.org/fftpack/doc)
