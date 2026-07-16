# Conservative FFI inventory and raw-binding generation

`slatec-corpus scan-ffi-inventory --offline` reads only the providers selected
by `generated/selected-corpus/selected-providers.json`. It verifies each raw
source hash before applying the existing fixed-form logical-statement layer.

The inventory deliberately extracts only a possible raw-FFI boundary:

- program-unit name and kind;
- formal argument order;
- explicit or mechanically derived implicit scalar type;
- scalar versus array, rank, and raw dimension spelling;
- character length spelling;
- `EXTERNAL` and `INTRINSIC` declarations for formal arguments;
- function result type; and
- exact source locators, conflicts, and review diagnostics.

It is not a general Fortran parser. It neither evaluates expressions nor parses
executable statements, control flow, storage, dependencies, callbacks,
workspaces, or routine algorithms. Unsupported specification forms remain
review items; the tool does not invent an interface from them.

The generated `generated/ffi-inventory/` records are compact, columnar factual
indexes. `source-locator-index.json` owns exact byte and physical-line spans;
other records refer to those stable IDs. `selected-source-files.json` is the
ordered list of all source files represented by the complete selected-provider
manifest. The committed output has a 4 MB guard. No Fortran source text or raw
statement text is committed.

The compact prologue indexes intentionally do not carry documented argument
spelling. Consequently `source-prologue-name-order.json` reports unavailable
comparisons until a separately reviewed, ignored-evidence pass is introduced.
Documentation cannot override executable-source facts.

## Historical native pilot

`slatec-corpus probe-native-ffi --offline` is a separate, explicit compiler
experiment. It first verifies the selected raw source hashes and then compiles
these unchanged representative providers with an explicit legacy fixed-form
profile:

- `DASUM` — scalar function;
- `D1MACH` — machine constant;
- `DAXPY` — simple array subroutine;
- `XERMSG` — character arguments and error handling;
- `CABS` — complex-valued function; and
- `DFZERO` — external callback argument.

The probe records compiler target/version, object hashes, compiler-log hashes,
defined raw symbols, and a narrowly scoped raw Rust smoke test for `DASUM`.
On the observed MinGW profile that smoke test uses the observed `dasum_` symbol,
`DOUBLE PRECISION` return, default-`INTEGER` scalar pointers, and a contiguous
`DOUBLE PRECISION` array. Its success applies only to that exact compiler and
target profile.

Compilation and one scalar raw-call observation do not establish dependency
closure, character hidden-length convention, complex return ABI, callback ABI,
integer width, `COMMON` layout, error behaviour, thread safety, a native
component, generated production FFI, or a safe Rust wrapper. Those gates remain
separate work.

All compiler objects, authored probe source, binaries, and logs live under the
ignored `evidence/native-probe/` tree. Normal CI runs synthetic Rust tests only
and never downloads or compiles the SLATEC corpus.

## Corpus-wide raw FFI generation

`slatec-corpus generate-raw-ffi --offline` uses
`generated/selected-corpus/selected-providers.json` as its only corpus boundary.
It hash-verifies every selected physical source, compiles all 1,442 selected
source files with one explicitly supported GNU Fortran legacy fixed-form
profile, observes each object’s defined symbols, and builds an ignored static
archive where compilation permits it.

It emits a raw declaration only when the provider compiled, exactly one matching
raw symbol was observed, its minimal source interface has no unresolved type or
conflict, and the applicable ABI category passes the authored GNU Fortran ABI
probe. The probe checks default `INTEGER`, `REAL`, `DOUBLE PRECISION`,
`LOGICAL`, `COMPLEX`, trailing `CHARACTER` lengths, scalar functions, numeric
arrays, and an `EXTERNAL` callback call shape. Additional smoke tests call
selected original numeric/complex routines and exercise the machine and error
subsystems through an authored Fortran driver.
That driver establishes invocation only: the historical `I1MACH`/`D1MACH`
tables are not treated as validated representations of a modern host.

Generated declarations are split into feature-gated modules for numeric scalar
subroutines, numeric array subroutines, scalar functions, complex arguments,
logical values, and character values. Callbacks, complex- or
character-returning functions, unresolved/conflicting interfaces,
missing/ambiguous symbols, and selected infrastructure are kept in the compact
review queue rather than guessed. Callback and infrastructure modules have no
callable declarations.

`crates/slatec-sys/build.rs` remains a no-op: Cargo never downloads or compiles
Fortran implicitly. Objects, archives, authored probes, executables, and
compiler logs stay under ignored `evidence/raw-ffi/`; compact compiler,
symbol, confidence, review, and binding metadata is committed under
`generated/ffi/`.

The successful profile establishes evidence only for that GNU Fortran target.
It does not prove safe Rust use, `COMMON` layout, individual callback
signatures, complex/character return ABI, error semantics, thread safety,
component boundaries, or general ABI correctness.

## Local commands

With the complete selected corpus and its ignored evidence already present:

```text
cargo run -p slatec-tools --bin slatec-corpus -- scan-ffi-inventory --offline
cargo run -p slatec-tools --bin slatec-corpus -- probe-native-ffi --offline
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-ffi --offline
```

The native probe defaults to `x86_64-w64-mingw32-gfortran`; set
`SLATEC_FORTRAN_COMPILER` to use another explicitly reviewed compiler. Its Rust
smoke test requires the matching `x86_64-pc-windows-gnu` standard-library target
to be installed locally. No source download occurs in either command.

The corpus-wide generator uses the same compiler default. Set
`SLATEC_FORTRAN_COMPILER` only to a GNU Fortran executable that will be
independently validated; no source download occurs.
