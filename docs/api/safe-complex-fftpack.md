# Safe complex FFTPACK plans

Enable the hosted single-precision complex plan API with an explicit native
backend:

```text
slatec = { version = "0.1", default-features = false, features = ["std", "source-build", "fftpack-complex"] }
```

`source-build` is offline-only and uses the separately acquired, hash-verified
source cache. Applications that link their own reviewed provider can use
`external-backend` instead. The selected SLATEC snapshot has the reviewed
single-precision roots `CFFTI1`, `CFFTF1`, and `CFFTB1`; it has no reviewed
double-precision complex roots. Consequently this API exposes
`ComplexFftPlan32` over `num_complex::Complex32` and intentionally has no
`Complex64` plan.

## Plan and storage contract

`ComplexFftPlan32::new(n)` accepts `n >= 2`, owns its initialized workspaces,
and is not cloneable. The exact native allocations are:

```text
CH   = 2*n f32 words
WA   = 2*n f32 words
IFAC = 15 Fortran integer words
```

`forward` and `backward` take exactly `&mut [Complex32]` of length `n`. They
pass that original allocation directly to the standard FFTPACK `*1` interface:
`CFFTF1` and `CFFTB1` declare the sequence as a real `C(*)` array, with the
two words for each element ordered `re, im`. `num-complex` 0.4 documents
`Complex32` as `repr(C)` with precisely that field order and a layout compatible
with `[f32; 2]`; the wrapper validates the checked `2*n` conversion and passes
only a `*mut f32` at FFI. It does not rely on a Fortran `COMPLEX` value ABI.

No transform allocates, copies, transposes, repacks, or normalizes its input.
NaN and infinity are passed through to FFTPACK because this family has no
finite-value status return.

## Direction and normalization

For zero-based indices, native forward and backward transforms are exactly:

```text
forward(X)[j]  = sum(k = 0..n-1, X[k] * exp(-i*2*pi*j*k/n))
backward(X)[j] = sum(k = 0..n-1, X[k] * exp(+i*2*pi*j*k/n))
```

Neither call divides by `n`; therefore
`backward(forward(X)) = n * X`. Apply any desired scale explicitly in caller
code. The two directions are deliberately separate methods rather than a
generic normalization setting.

## Runtime and scope

Construction and transforms retain the existing process-wide native runtime
lock. The exact closure has no XERROR, callback, COMMON, or Fortran-I/O path,
but `CFFTI1` contains the source-read-only `SAVE`/`DATA` factor-trial table
`NTRYH`; object evidence records writable static storage. The safe API is
therefore `SerializedGlobal` for the reviewed source backend, and external or
system providers remain `BackendDependent`. A plan may move if its fields allow
it, but a transform needs `&mut self`, so one plan cannot be transformed by two
safe callers simultaneously.

This milestone is limited to one-dimensional `Complex32` transforms. Complex
`f64`, multidimensional FFTPACK, generic complex traits, arbitrary strides,
ecosystem adapters, external FFT implementations, and translated algorithms
remain deferred.
