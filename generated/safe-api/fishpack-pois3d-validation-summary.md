# Safe structured FISHPACK POIS3D M2

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Selected driver: `POIS3D` from `fishfft/pois3d.f` (9daf0cd2c9eab106f9b508f426003a98719d21ed29b4fde0f224300d8e88da78); the current direct Netlib provider matched this selected body at acquisition.
- Exact closure: 32 source units: `POIS3D`, `POS3D1`, `TRIDQ`, `PIMACH`, and 28 reused real-FFTPACK units.
- Contract: an owned, checked structured block-tridiagonal system, not arbitrary six-face 3D boundary data.
- Validation: independent dense oracle, manufactured cyclic and noncyclic systems, all transverse ghost rules, raw `IERROR=9/10`, x-fast layout, and concurrent native-call serialization.
- Runtime: no persistent FISHPACK state was found, but native calls remain process-globally serialized.
