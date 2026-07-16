# Dependency evidence sources

## Purpose

Dependency evidence is collected as independent observations tied to an exact source snapshot, parser/build run, and locator. No evidence class is authoritative for all questions.

## Source classes

### Prologue declarations

`ROUTINES CALLED`, `EXTERNALS`, callback descriptions, workspace notes, and error-system fields are documentary claims. Preserve raw spelling, order, dialect, exact span, parser version, and unrecognized text. A prologue entry never by itself proves an executable reference.

### Parsed source

Record `CALL`, function references, `EXTERNAL`, `INTRINSIC`, callback invocation and forwarding, `ENTRY`, alternate returns, `COMMON`, `SAVE`, `DATA`, `EQUIVALENCE`, and `BLOCK DATA`. Each observation identifies the enclosing program unit, statement span, condition or control context where known, and parser confidence.

### Netlib dependency products

Ingest `tree1`, `tree`, and plus-dependency retrievals as separate products. Record URL, retrieval time, bytes, checksum, requested root, returned files, parse assumptions, and unknown generating snapshot. `tree1` is treated as direct-product evidence; `tree` and plus-dependency products are closure/acquisition evidence, not executable-source truth.

### Object symbols

For every compiler profile collect defined, undefined, weak, local, common, and archive-member ownership information using compiler/linker-native tooling. Preserve original and normalized symbol names, object checksum, compiler fingerprint, source-to-object mapping, and command line.

### Linker experiments

Minimal link tests are observations about one exact build fingerprint. Record roots, supplied objects/archives, archive order, flags, selected providers, result, diagnostics checksum, resolved provider map where available, and accidental host/system resolutions.

### Runtime dependencies

Fortran runtime, math runtime, I/O, startup, and target-library dependencies are external-provider records. They must not be misclassified as missing SLATEC routines.

## Reliability boundaries

- Parsed executable references are strongest for source-level direct references when parsing coverage is complete.
- Object undefined symbols validate compiler-emitted linkage requirements but may include mangling, lowering, intrinsics, and runtime calls.
- Successful links prove only the tested configuration and may hide accidental system providers.
- Prologues and Netlib products are valuable independent checks and historical evidence.
- Shared state and callbacks require dedicated edges and cannot be inferred from an ordinary call list.
