# Real scalar special-function audit

This audit begins from the generated scalar-function inventory rather than
routine names. It keeps only real scalar functions whose ABI, domain, result,
and native error path are reviewed.

The `special-scalar-expanded` feature adds `ALI`/`DLI`, `SPENC`/`DSPENC`, and
Carlson `RC`/`DRC`, `RF`/`DRF`, `RD`/`DRD`, and `RJ`/`DRJ`. The first four are
one-real-argument Fortran functions. The Carlson routines return one real
value and write `IER`; the safe layer maps `IER = 0` to success and preserves
1 (invalid signs), 2 (too-small argument combination), and 3 (too-large
argument) as a typed native-status error.

`ACOSH`/`DACOSH`, `ASINH`/`DASINH`, and `ATANH`/`DATANH` are reviewed as
ordinary real operations already supplied by Rust's `f32`/`f64` methods. They
remain available through the reviewed raw ABI but are deliberately not wrapped
for name parity: this project makes no SLATEC-specific accuracy advantage claim
for them.

`CHU`/`DCHU` are deferred because their prologues identify no-convergence
paths and a small-argument parameter region where the algorithm is bad.
`POCH`/`DPOCH` and `POCH1`/`DPOCH1` are deferred because their own prologues
describe preliminary or incomplete wrong-argument handling. `COT`/`DCOT` are
deferred because the near-pole precision-warning path is fatal and cannot be
preflighted exactly from the published contract. Complex routines, sequence
generators, workspace APIs, random routines, and FNLIB implementation helpers
remain outside this scalar API.

All selected additions use saved initialization and/or the global XERROR
subsystem. They are `SerializedGlobal` on the reviewed GNU MinGW source
backend. External and system providers remain `BackendDependent`; no wrapper
claims parallel native execution. The API owns no work arrays and adds no
array, complex-number, matrix, or ecosystem dependency.
