# Reviewed but deferred SLATEC linear programming

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Candidates: `SPLP` (f32) and `DSPLP` (f64); neither has a raw declaration, provider feature, source closure, safe wrapper, or example.
- Model: minimize `c^T x` subject to `A x = w`, with `IND`-encoded lower, upper, two-sided/fixed, or free bounds on both `x` and the row activities `w`.
- Sparse protocol: `USRMT`/`DUSRMT` is a one-based external callback protocol, not a modern owned sparse-matrix ABI.
- Conditional paging: the single path uses `PRWVIR` and the double path uses `DPRWVR`. Their `SOPENM` call occurs when the first page is written; the package documents activation only for save/restore or when the matrix exceeds high-speed storage. The caller is not required to pre-open unit 1. Option 54 chooses its global Fortran unit; `SCLOSM` is option-controlled and closes with `STATUS='KEEP'`.
- Decision: a runtime lock can serialize access, but cannot provide a safe unit lifecycle, deterministic deletion, or a reliable post-I/O-failure native state. No native I/O shim is substituted.
