# Reviewed but deferred SLATEC linear programming

`SPLP` and `DSPLP` are **not supported public APIs** in this crate. There is
no `optimization` Cargo feature, raw declaration, provider source closure,
safe wrapper, or example for them. This document records the completed source
audit and the reason for deferral.

## Mathematical model

The original drivers solve

```text
minimize c^T x
subject to A x = w.
```

`A` has `MRELAS` rows and `NVARS` columns. `COSTS` supplies `c`; there is no
objective constant. Minimization is the native default and option key 50 can
select maximization.

Bounds apply independently to the variables `x` and to the row activities
`w = A x`. Their `IND` codes are identical: 1 is a lower bound, 2 is an upper
bound, 3 is a closed interval (including equal endpoints for an equality), and
4 is free. Thus equality, one-sided, two-sided, and free rows are expressed as
bounds on `w`, not as a separate modern constraint-array interface.

## Sparse callback protocol

The default `USRMT`/`DUSRMT` external procedure reads a serialized matrix
stream. Initialization uses `IFLAG(1)=1`; later calls yield one `(I, J, AIJ,
INDCAT)` entry while `IFLAG(1)=2`; `IFLAG(1)=3` ends the stream. `I` and `J`
are one-based. `INDCAT=0` assigns an entry and `INDCAT=1` accumulates it. The
provided stream callbacks assign entries; they do not establish a safe Rust
policy for duplicate entries.

## Why the drivers remain deferred

The sparse path is not wholly memory resident. The f64 path reaches
`DPRWVR -> SOPENM/SCLOSM`; the f32 path reaches `PRWVIR -> SOPENM/SCLOSM`.
`SOPENM` opens a direct-access external Fortran unit (default unit 1), and
`SCLOSM` closes it with `STATUS='KEEP'`. The driver offers only a unit number:
it offers no filename ownership, scratch-file policy, deterministic deletion,
or reliable propagation of paging I/O failure into `INFO`.

A process-global runtime lock would serialize calls but cannot make this
external file lifecycle recoverable or artifact-free. Replacing the paging
subsidiary with an I/O shim would alter the reviewed native contract, so this
milestone intentionally does not do so.

The deterministic audit records are in
`generated/safe-api/linear-programming-*.json` and
`generated/safe-api/linear-programming-validation-summary.md`.
