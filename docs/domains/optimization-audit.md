# Optimization-family audit

This is a historical audit record, not the current public API reference. It
inventories optimization-adjacent program units in the selected SLATEC corpus.
The later in-memory-only `SPLP`/`DSPLP` feature is documented separately in
the [safe linear-programming guide](../api/safe-linear-programming-in-memory.md).

The deterministic machine-readable records are in
`generated/safe-api/optimization-*.{json,md}`.  They cross-check the selected
corpus and raw-FFI inventories, so they do not require a native build or a
network request.

## Families found

The corpus contains the already wrapped nonlinear least-squares family
(`SNLS1`/`DNLS1`, easy variants, and `SCOV`/`DCOV`), nonlinear-equation and
Jacobian-checking drivers (`SNSQ`/`DNSQ`, easy variants, and
`CHKDER`/`DCKDER`), and the already wrapped constrained linear least-squares
drivers (`WNNLS`/`DWNNLS`, `SBOLS`/`DBOLS`, `LSEI`/`DLSEI`, and
`SBOCLS`/`DBOCLS`).  Trust-region, QR, finite-difference, norm, and PDE
normalization units are subsidiaries rather than independent optimization
drivers.

No standalone scalar-minimization, bounded multivariate-minimization, or
general nonlinear-programming driver is present in the selected corpus.  The
audit therefore does not reinterpret nonlinear equation solving or
least-squares fitting as a new optimization family.

## Original SPLP and DSPLP paging audit

`SPLP` and `DSPLP` are the only remaining public general optimization drivers.
Their mathematical form is

```text
minimize c^T x
subject to A x = w,
with typed lower/upper bounds on x and on row activities w.
```

They obtain sparse matrix data through the one-based `USRMT`/`DUSRMT` callback:
the `IFLAG` protocol initializes, yields entries, and terminates a column;
`INDCAT` chooses assignment or accumulation. Their paging path is conditional,
not a requirement that the caller pre-open a file: it is used for save/restore
or when matrix storage exceeds high-speed memory. `PRWVIR`/`DPRWVR` uses option
54's process-global Fortran unit and may call `SOPENM` on the first page write;
`SCLOSM` is option-controlled and closes with `STATUS='KEEP'`. This still
supplies neither a safe Rust-owned unit lifecycle nor deterministic deletion or
reliable I/O-failure propagation.

The existing native runtime lock can serialize calls, but it cannot make an
activated unit recoverable after a callback failure, native error, or panic,
and cannot meet the repository's cleanup guarantees. The current wrapper
therefore admits only resident problems, disables these options, and installs
forbidden-entry traps. It still does not add a native I/O shim or any out-of-
core paging feature.

## Future-wrapper policy

If a later selected corpus supplies a safe driver, its wrapper should use
slices or lightweight checked views for numeric inputs, allocate any mutated
native storage privately, and contain callbacks and XERROR state for the full
serialized native call.  Solver-specific problem, options, and result types
are appropriate; a new owned vector or matrix ecosystem is not.  Optional
`nalgebra`, `ndarray`, or `faer` adapters must remain separate features and
zero-copy is appropriate only when both native layout and mutation rules allow
it.
