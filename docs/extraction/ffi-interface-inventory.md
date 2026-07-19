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

## Explicit native archive and batch validation

`slatec-corpus build-native-ffi --offline` rebuilds the selected static archive
only after `generate-raw-ffi` has re-verified every selected source hash. The
archive, compiler logs, objects, authored ABI fixtures, and executable drivers
remain ignored below `evidence/raw-ffi/`; the compact build profile is recorded
in `generated/ffi-validation/native-build.json`.

`slatec-corpus validate-raw-ffi --offline` repeats that explicit verification,
then audits every emitted declaration against the generated interface inventory
and observed symbols. It also compiles and runs independently selectable
native-link batches. Use repeated `--batch` values to limit a local run, for
example:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-native-ffi --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-ffi --offline --batch batch_numeric_array_subroutines
```

The generated profile is explicitly `ffi-profile-gnu-mingw-x86_64`, for GNU
Fortran targeting `x86_64-w64-mingw32`. It is not portable ABI metadata. The
raw type aliases are available only with that profile feature. For an explicit
Cargo native-link test, set:

```text
SLATEC_NATIVE_LIB_DIR=<directory-containing-libslatec_selected.a>
SLATEC_GFORTRAN_RUNTIME_DIR=<optional-directory-containing-GNU-runtime-libraries>
```

and use the matching `x86_64-pc-windows-gnu` Rust target. When
`SLATEC_NATIVE_LIB_DIR` is set, `slatec-sys/build.rs` verifies both the feature
and target before emitting link directives; it never compiles Fortran. The
profile uses `i32` for `FortranInteger` and `FortranLogical`, `usize` for
trailing character lengths, observed GNU `nm` symbol spellings, and requires
the GNU `gfortran`, `quadmath`, and Windows C runtimes at link/runtime.

Feature groups are deliberately confidence-scoped:

- `raw-ffi-basic`: numeric scalar and numeric-array subroutines plus supported
  scalar functions;
- `raw-ffi-abi-sensitive`: complex arguments, logical values, and character
  arguments, after profile-specific validation;
- `raw-ffi-all-validated` (and the compatibility alias `raw-ffi`): both groups.

Callbacks, complex-returning functions, character-returning functions,
unresolved/conflicting interfaces, and selected infrastructure remain outside
every validated aggregate. Passing a batch is evidence for the exact GNU
MinGW profile only; it does not establish a safe wrapper, callback signature,
complex return convention, character result convention, thread safety, or
general ABI portability.

## Canonical raw API inventory (R1)

`generated/ffi/` answers a deliberately narrow question: what the selected
GNU Fortran toolchain could compile and describe. It is not a stable public
API inventory. R1 joins that evidence with the retained catalogue, selected
providers, source hashes, historical role, feature graph, safe-wrapper index,
and authored reviews into exactly one record per routine identity under
`generated/raw-api/`.

Regenerate and validate it without downloading source:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-api-inventory --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-api-inventory --offline
```

The result distinguishes generated candidates, ABI-validated generated
declarations, reviewed public drivers and subsidiaries, provider-backed
routines, link/runtime coverage, documentation coverage, safe wrappers, and
explicit exclusions. A declaration in `slatec_sys::generated` remains a
transitional ABI-shaped path until it has a reviewed canonical mathematical
path. Hash-guarded corrections in `metadata/raw-api-corrections.json` supply
only reviewed facts that cannot safely be inferred from executable source.
