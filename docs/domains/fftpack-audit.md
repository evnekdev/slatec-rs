# Real FFTPACK audit

The authoritative selected `fishfft` snapshot contains 16 reviewed public
single-precision real FFTPACK entry points: `RFFTI/RFFTF/RFFTB`,
`EZFFTI/EZFFTF/EZFFTB`, `SINTI/SINT`, `COSTI/COST`,
`SINQI/SINQF/SINQB`, and `COSQI/COSQF/COSQB`. It contains no corresponding
double-precision real entry points. The safe API therefore exposes only the
audited `f32` family; no names or precision pairs were inferred mechanically.

The narrow closure comprises 32 hash-pinned `fishfft` source files: the six
family front ends, FFT factor initialization, real radix kernels, and the
full/quarter-wave helpers. It does not include complex FFTPACK, multidimensional
transforms, FISHPACK, BLAS, XERROR, callbacks, or Fortran I/O.

Source scanning found two compile-time initialized saved factor tables:
`NTRYH` in `RFFTI1` and `EZFFT1`. The closure has no writers for either table.
They are nevertheless captured in generated native-state metadata. Until
object-level evidence and an explicit runtime-gate review support a narrower
claim, all safe plan calls retain process-global serialization. This is a
conservative implementation decision, not a claim that the numerical work
arrays are global: each plan owns a distinct initialized workspace.

Direct O(N^2) test-only oracles validate periodic packed coefficients, easy
Fourier coefficients and synthesis, full sine/cosine transforms, and
quarter-wave forward/backward formulas over lengths 1–10 (or 2–10 for COST),
including odd, even, prime, and composite sizes. The `COSQB` source/prologue
factor discrepancy is recorded explicitly and tests use source behavior.
