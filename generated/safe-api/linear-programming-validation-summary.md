# Reviewed but deferred SLATEC linear programming

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Candidates: `SPLP` (f32) and `DSPLP` (f64); neither has a raw declaration, provider feature, source closure, safe wrapper, or example.
- Model: minimize `c^T x` subject to `A x = w`, with `IND`-encoded lower, upper, two-sided/fixed, or free bounds on both `x` and the row activities `w`.
- Sparse protocol: `USRMT`/`DUSRMT` is a one-based external callback protocol, not a modern owned sparse-matrix ABI.
- Blocking dependency: the single path uses `PRWVIR -> SOPENM/SCLOSM`; the double path uses `DPRWVR -> SOPENM/SCLOSM`. They open a process-global direct-access Fortran unit and close it with `STATUS='KEEP'`.
- Decision: a runtime lock can serialize access, but cannot give Rust filename ownership, deterministic cleanup, or a reliable post-I/O-failure native state. No native I/O shim is substituted.
