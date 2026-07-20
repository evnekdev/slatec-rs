# Native link granularity

This document describes the guarantees measured for the supported GNU MinGW
source backend. It distinguishes Rust declaration availability from native
implementation inclusion: an `extern "C"` declaration neither compiles nor
links the corresponding Fortran routine. A native implementation enters a
final executable only when the linker resolves a referenced symbol from the
provider archive.

## Measured source backend pipeline

`slatec-src` reads its reviewed source closure, verifies every cached source
against its SHA-256, and invokes:

```text
gfortran -x f77 -std=legacy -ffixed-line-length-none -c SOURCE -o OBJECT
ar cr ARCHIVE OBJECT...
ar q ARCHIVE OBJECT...       # later deterministic chunks
ar s ARCHIVE
```

There is one object for every selected physical source (plus the three
reviewed machine-profile sources when required). There is no generated
amalgamated source, `ld -r`, partial link, `--whole-archive`, `/WHOLEARCHIVE`,
or linker group directive in this source backend. The build script emits an
ordinary static `slatec_family` link directive and static `libgfortran` and
`libquadmath` directives. `slatec-sys/build.rs` intentionally emits no native
link directive.

An archive can be large without making every consumer binary large. The linker
extracts an archive member only for an unresolved symbol, then follows that
member's genuine undefined-symbol closure. A source that historically contains
several program units remains one object; the inventory identifies those cases
so their retention can be reviewed instead of inferred from feature names.

## Evidence and regression checks

Run, with the verified source cache and supported compiler configured:

```text
cargo run -p slatec-tools -- generate-native-link-audit
cargo run -p slatec-tools -- validate-native-link-audit
```

The deterministic reports in [`generated/native-link/`](../../generated/native-link/)
contain one archive-member row, symbol dependency edges, release-probe symbols
and section sizes, and an explicit pipeline manifest. Binaries, linker maps,
objects, archives, compiler diagnostics, and elapsed-time measurements remain
under ignored `target/native-link/`.

The probes include raw SAXPY, DDOT, SAXPY+DDOT, DGAMMA, callback-bearing FZERO
(function address retained without guessing a callback invocation ABI), FISHPACK
HWSCRT and POIS3D addresses, safe BLAS and gamma wrappers, and actual `full`
source-provider builds with `all` declarations. The `all_no_call` assertion
requires no SLATEC implementation symbols. SAXPY and DGAMMA probes reject
unrelated public BLAS, special, roots, and FISHPACK drivers. Link maps show the
archive members selected for the direct raw probes.

Release probes explicitly request GNU linker section garbage collection. This
is measurement and regression evidence, not a global workspace release-profile
change. The manifest records current compiler commands; it does not claim that
debug, user-specific RUSTFLAGS, LTO, stripping, panic, or linker settings have
identical size behavior. The separate safe Cargo probes show the facade's
actual feature/provider route.

Safe-facade closure is measured separately from direct raw closure. The former
over-retention was caused by broad Rust codegen objects, not native archive
coalescing. Safe wrapper isolation, its supported release-profile partitioning,
and the required safe/raw probe pairs are documented in
[safe-facade link granularity](safe-facade-link-granularity.md).

## Runtime and provider limits

The reports list final executable sections and imported DLLs separately from
the static archive sizes. `libgfortran`, `libquadmath`, platform runtime,
unwind/debug metadata, and Rust standard-library code can dominate a small
SLATEC closure; archive byte size is not a size contribution. The project
retains static GNU runtimes for the supported source backend and does not
change that deployment policy merely to optimize a probe.

The guarantee is limited to this source backend. A user-supplied shared library
has independent on-disk size; a supplied static archive may be granular or
monolithic; a provider that uses one object or whole-archive linking can retain
everything. `external-backend` deliberately leaves native linking to the final
application, so consumers should inspect their own binary with `nm`, `size`,
`objdump -p`, and a linker map.

## Scope and follow-up

The audit reports source-build compilation/object/archive counts for the full
closure and records family membership per object. It does not rewrite or split
historical Fortran. A future split of an unrelated multi-unit source needs an
exact-source, hash-traceable build-output transformation and dedicated review
of `ENTRY`, `COMMON`, `BLOCK DATA`, and `DATA` semantics.
