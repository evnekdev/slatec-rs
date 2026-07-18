# Safe real FFTPACK plans

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Reviewed native entry points: 16 single-precision routines in six real transform families. No double-precision real FFTPACK counterparts occur in this selected source snapshot.
- Plans own initialized workspaces, use exact native contiguous buffers, preserve native normalization, and do not copy in-place transform data.
- `RFFTF` uses a documented packed spectrum; `EZFFTF` exposes explicit mean/cosine/sine arrays.
- The closure has no COMMON, XERROR, callback, or Fortran-I/O path. `RFFTI1` and `EZFFT1` retain compile-time DATA/SAVE factor tables that are read-only in the audited source. This first facade stays `SerializedGlobal` under the existing hosted runtime policy.
- The executable `COSQB` path uses a factor of 4, despite an older prologue formula that displays 2; native small-N direct-oracle tests and the stated `4*N` composition relation confirm the source behavior.
- Complex FFTPACK, multidimensional transforms, FISHPACK, adapters, and translated algorithms remain deferred.
