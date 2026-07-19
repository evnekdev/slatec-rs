# Complex FFTPACK audit

The selected `fishfft` source snapshot contains the single-precision complex
FFTPACK roots `CFFTI`, `CFFTF`, and `CFFTB`, plus the standard real-array
implementation routines `CFFTI1`, `CFFTF1`, and `CFFTB1`. The legacy roots use
Fortran `COMPLEX` dummy arguments and call their `*1` subsidiaries through
historical non-standard argument association. They are marked subsidiary in
the package audit and are deliberately not bound by the safe API.

The legacy roots each return immediately for `N = 1`; the standard `*1`
initializer does not have that identity-length return. To avoid binding the
non-standard legacy complex ABI, the safe plan deliberately accepts `N >= 2`.

The exposed closure uses `CFFTI1/CFFTF1/CFFTB1` with the `PASSF*` and `PASSB*`
radix helpers. Its native data interface is explicit: both transform sequence
arrays are `2*N` interleaved real words, and no Fortran complex value ABI
crosses the Rust boundary. `CFFTI1` initializes `WA(2*N)` and `IFAC(15)`;
forward and backward also use a private `CH(2*N)` scratch array.

`CFFTF1` has the negative-exponent DFT sign and `CFFTB1` has the
positive-exponent sign. Both are unnormalized, so their composition has scale
`N`. Direct test-only O(N^2) complex DFT oracles cover impulse, constant,
complex exponential, odd/even, prime, composite, and round-trip cases.

The complete selected closure has no XERROR, callbacks, COMMON declarations,
or Fortran I/O. `CFFTI1` does contain `SAVE`/`DATA NTRYH /3,4,2,5/`, a static
factor-trial table emitted in writable object storage. The table is not written
by the source closure, but project policy conservatively retains global native
serialization; no parallel-execution claim is made. The selected snapshot has
no double-precision complex FFTPACK root, so `Complex64`, multidimensional
transforms, and external-complex-ABI support remain deferred.
