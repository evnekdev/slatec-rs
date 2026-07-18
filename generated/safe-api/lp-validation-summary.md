# Safe in-memory SLATEC linear programming

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Wrappers: `SPLP` (`f32`) and `DSPLP` (`f64`).
- Model: minimize `c^T x` subject to `A x = w`, with typed lower, upper, ranged/fixed, or free bounds on both variables and row activities.
- Sparse protocol: owned validated CSC is delivered through the one-based `USRMAT`/`DUSRMT` callback protocol without densification or reordering.
- In-memory contract: printing, continuation, save/restore, and option key 54 are disabled; capacity is checked before FFI. The source profile contains no paging or file-I/O implementation. ABI-compatible project traps do no I/O and turn any unexpected paging entry into a contract violation.
- Workspace: `LAMAT=max(N+7,N+NNZ+6)`, `LBM=8*M`, `WORK=LAMAT+LBM+4*N+8*M`, and `IWORK=LAMAT+2*LBM+N+11*M`, all calculated with checked arithmetic.
- Runtime: the complete callback/XERROR/native/status scope is process serialized. XERROR control flag and output units are restored. Avoiding paging does not make LP reentrant.
- Validation: both precisions cover optimal, equality/fixed-bound, infeasible, and no-finite-solution cases; malformed sparse inputs, capacity rejection, callback protocol errors, and callback panic containment are covered. No native or source artifact is committed.
