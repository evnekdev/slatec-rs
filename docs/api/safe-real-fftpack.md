# Safe real FFTPACK plans

Enable the hosted plan API with an explicit backend:

```text
slatec = { version = "0.1", default-features = false, features = ["std", "source-build", "fftpack-real"] }
```

`source-build` is offline-only and requires a separately acquired,
hash-verified source cache. `external-backend` is available for applications
that link their own reviewed provider. The selected SLATEC snapshot exposes
these real FFTPACK routines only in single precision; this API deliberately
has no `f64` plan types.

## Plan ownership and calls

`RealFftPlan`, `EasyRealFftPlan`, `SineTransformPlan`,
`CosineTransformPlan`, `QuarterWaveSinePlan`, and
`QuarterWaveCosinePlan` each own an initialized native work array. They are
not cloneable and have no public raw-workspace access. In-place transform
methods take a contiguous `&mut [f32]` with the exact plan length and pass
that original allocation to SLATEC; they neither allocate nor copy data.

`EZFFTF` is different: it reads `&[f32]` and returns explicit owned mean,
cosine, and sine arrays. `EZFFTB` writes a caller-provided `&mut [f32]` from
those exact coefficient arrays. There is no complex-number dependency and no
automatic conversion to a complex vector.

All plan construction and transforms currently hold the existing process-wide
native runtime lock. The closure has no reachable XERROR, callback, COMMON, or
Fortran-I/O path, but `RFFTI1` and `EZFFT1` contain read-only DATA/SAVE factor
tables and the first safe facade does not relax the hosted runtime policy.

## Periodic packed real spectrum

After `RealFftPlan::forward`, a length-`N` buffer contains the exact `RFFTF`
layout:

```text
R[0]                  = sum_j x[j]
R[2k - 1]             = sum_j x[j] cos(2*pi*k*j/N)
R[2k]                 = -sum_j x[j] sin(2*pi*k*j/N)
                       for 1 <= k <= floor((N-1)/2)
R[N - 1]              = sum_j (-1)^j x[j], when N is even
```

`RealSpectrumRef` exposes this without allocating. In particular,
`harmonic(k)` returns `(cosine, negative_sine)`, preserving the native minus
sign rather than pretending the packed data is an ordinary complex array.
`RFFTB(RFFTF(x)) = N*x`; the API never silently normalizes.

## Exact sine and cosine formulas

The historical FFTPACK names are primary. For a vector with one-based
mathematical indices, `SINT` computes

```text
Y[k] = 2 * sum(j = 1..N, X[j] * sin(pi*j*k/(N + 1))),  k = 1..N.
```

`COST` uses zero-based endpoints and computes

```text
Y[k] = X[0] + (-1)^k * X[N - 1]
       + 2 * sum(j = 1..N - 2, X[j] * cos(pi*j*k/(N - 1))),  k = 0..N - 1.
```

For the quarter-wave cosine forward transform, with `k = 1..N`,

```text
COSQF(X)[k - 1] = X[0] + 2 * sum(j = 1..N - 1,
    X[j] * cos(pi*(2*k - 1)*j/(2*N))).
```

The executable `COSQB` inverse is

```text
COSQB(Y)[j] = 4 * sum(k = 1..N,
    Y[k - 1] * cos(pi*(2*k - 1)*j/(2*N))),  j = 0..N - 1.
```

For quarter-wave sine transforms,

```text
SINQF(X)[k - 1] = (-1)^(k - 1) * X[N - 1]
  + 2 * sum(j = 1..N - 1, X[j - 1] * sin(pi*(2*k - 1)*j/(2*N))),
SINQB(Y)[j - 1] = 4 * sum(k = 1..N,
    Y[k - 1] * sin(pi*(2*k - 1)*j/(2*N))),  j = 1..N.
```

`EZFFTF` returns the mean and Fourier-series cosine/sine coefficients rather
than the `RFFTF` packed layout. For even `N`, the final sine coefficient is
zero and its final cosine coefficient is the Nyquist term. `EZFFTB` evaluates
that normalized real Fourier series directly, so it reconstructs the original
values without an additional scale factor.

## Other native conventions

| Plan | Native routines | Exact composition |
| --- | --- | --- |
| `EasyRealFftPlan` | `EZFFTI/EZFFTF/EZFFTB` | synthesis reconstructs the normalized Fourier series |
| `SineTransformPlan` | `SINTI/SINT` | `SINT(SINT(x)) = 2*(N+1)*x` |
| `CosineTransformPlan` | `COSTI/COST` | `COST(COST(x)) = 2*(N-1)*x` |
| `QuarterWaveSinePlan` | `SINQI/SINQF/SINQB` | either composition is `4*N*x` |
| `QuarterWaveCosinePlan` | `COSQI/COSQF/COSQB` | either composition is `4*N*x` |

The verified minimum lengths are one for all listed plans except `COSTI/COST`,
which require `N >= 2`. Workspaces are `2*N+15` for periodic real FFT,
`3*N+15` for easy/cosine/quarter-wave plans, and `floor(7*N/2)+16` for full
sine. Arithmetic and native integer conversion are checked before allocation
or FFI.

The legacy `COSQB` prologue displays an inverse formula with factor 2, but its
executable source—including its small-size paths—uses factor 4. The API and
direct-oracle tests follow the executable behavior and the documented `4*N`
composition relation.

Complex FFTPACK, multidimensional FFTs, FISHPACK, arbitrary strides, external
array adapters, and translated algorithms are intentionally out of scope.
