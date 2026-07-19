# Safe Cartesian FISHPACK M1

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Selected driver: `HWSCRT` from `fishfft/hwscrt.f` (9bcd5a3be9e6d63e7dcc33637eb37ef07ba10b727b74859d08cb4daa7f813202), not the materially different standalone FISHPACK revision.
- Closure: 11 source units, including the existing canonical `SCOPY`; no FFTPACK transform root, XERROR, callback, or Fortran-I/O dependency is reachable.
- Native ABI: `hwscrt_`, single precision and 32-bit GNU Fortran `INTEGER`; the safe facade owns all edge buffers, RHS/solution storage, and workspace.
- Validation: asymmetric Dirichlet Poisson, negative-coefficient Helmholtz, codes 2/4 mixed boundaries, periodic code 0, singular Neumann code 3 with `PERTRB`, positive-coefficient `IERROR=6`, layout, and concurrent-call serialization all execute in the source-backend native test.
- Runtime: calls remain process-globally serialized. The focused state audit found no mutable FISHPACK source storage, but no parallel-native-execution claim is made.
