# Safe complex FFTPACK plans

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- The reviewed standard CFFTI1/CFFTF1/CFFTB1 interface exposes `ComplexFftPlan32` over `num_complex::Complex32`; the selected snapshot contains no double-precision complex roots.
- The standard interface is valid for `N >= 2`. Legacy CFFTI/CFFTF/CFFTB wrappers have an `N = 1` identity early return, but are deferred because they use non-standard complex-dummy argument association.
- `Complex32` is passed as its documented real/imaginary `f32` words to the native real-array interface, so no Fortran COMPLEX value ABI crosses FFI.
- Forward uses the negative exponent, backward uses the positive exponent, and compositions scale by `N`; no automatic normalization occurs.
- Plans own exact `CH=2*N`, `WA=2*N`, and `IFAC=15` workspace. CFFTI1's source-read-only SAVE+DATA NTRYH table and the existing hosted policy keep calls `SerializedGlobal`.
- Deprecated non-standard CFFTI/CFFTF/CFFTB wrappers, Complex64, multidimensional transforms, adapters, and translated algorithms remain deferred.
