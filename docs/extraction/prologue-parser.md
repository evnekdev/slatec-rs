# SLATEC prologue parser

`slatec-corpus scan-prologues` is the documentary extraction stage after canonical corpus preparation and fixed-form program-unit scanning. It consumes the committed corpus manifests and `generated/program-units/program-units.json`; it does not discover source files independently and does not download anything.

Run it locally after preparing and scanning the pinned archive:

```text
cargo run -p slatec-tools --bin slatec-corpus -- scan-prologues --offline
```

The command accepts `--evidence-dir`, `--manifest-dir`, `--program-unit-dir`, `--output-dir`, and `--offline`. `--manifest-dir` points to the corpus manifests, defaulting to `generated/corpus`; `--program-unit-dir` defaults to `generated/program-units`; and committed output defaults to `generated/prologues`.

## Evidence Boundary

The parser verifies snapshot IDs, program-unit locators, source-member paths, and raw source hashes before reading a file. It associates nearby comment blocks with each program unit, classifies the detected prologue dialect, extracts recognized headings, preserves unknown headings, and records diagnostics for ambiguous or incomplete cases.

Raw prologue text is written only to ignored local evidence:

```text
evidence/prologues/<snapshot-id>/raw-prologues.json
```

The committed `generated/prologues/` directory contains compact indexes only:

- `manifest.json`
- `prologue-index.json`
- `field-index.json`
- `dialect-summary.json`
- `diagnostics.json`
- `extraction-summary.md`

The indexes use deterministic columnar JSON for large repeated records. They include IDs, dialects, line ranges, field names, content hashes, collection states, diagnostic IDs, and summary counts. They do not include full descriptions, usage text, argument prose, references, or raw comment blocks.

## Supported Detection

The first parser version recognizes final SLATEC `C***BEGIN PROLOGUE` / `C***END PROLOGUE` blocks, legacy date/category markers, QUADPACK-like legacy headers, and package-style markers such as BLAS, LINPACK, EISPACK, FFTPACK, FISHPACK, and PCHIP when those markers are present in associated comment blocks.

Dialect IDs include:

- `slatec-final`
- `slatec-legacy`
- `quadpack`
- `package-legacy`
- `unrecognized`
- `absent`
- `ambiguous`

Unknown or malformed headings are preserved as structural records with review diagnostics rather than discarded.

## Evidence Semantics

All extracted prologue fields are documentation claims with `statement_kind = "reported_external_claim"`. They do not verify executable declarations, argument type or intent, dependencies, workspace requirements, package membership, ABI properties, thread safety, or redistribution rights.

Collection fields distinguish explicit empty evidence from absence. For example, an explicit `NONE` under `ROUTINES CALLED` is represented as `known_empty`, while a missing `ROUTINES CALLED` field remains `unknown`.

## What This Stage Does Not Prove

Successful prologue extraction proves deterministic documentary extraction against the pinned snapshot and parser version. It does not prove that prologue claims agree with executable source, object symbols, Netlib dependency products, or future ABI probes.

Deferred stages must reconcile prologue claims against parsed source and build evidence before using them for dependency graphs, raw FFI generation, or safe Rust APIs.
