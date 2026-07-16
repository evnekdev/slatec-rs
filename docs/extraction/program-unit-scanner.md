# Fixed-form program-unit scanner

`slatec-corpus scan-program-units` is the lexical inventory stage after canonical corpus preparation. It reads only verified, selected raw files named by `generated/corpus/source-files.json` and `generated/corpus/selected-providers.json`; it never discovers files independently or downloads an archive.

Run it locally after preparing the pinned archive:

```text
cargo run -p slatec-tools --bin slatec-corpus -- scan-program-units --offline
```

The command accepts `--evidence-dir`, `--manifest-dir`, `--output-dir`, and `--offline`. `--offline` is required: acquisition belongs to the previous corpus stage. It verifies snapshot identity, selected-provider state, and every raw extracted file hash before scanning.

## Outputs and evidence boundary

The committed `generated/program-units/` directory contains the compact source-scan index, program units, entry points, duplicate-provider report, diagnostics, manifest, and summary. It deliberately does not contain complete physical-line or logical-statement records.

Detailed structural inventories are written to the ignored local path `evidence/scans/<snapshot-id>/` as `physical-lines.json` and `logical-statements.json`. They contain locators, classifications, and hashes but no copied Fortran statements. `generated/program-units/manifest.json` records their hashes and relative evidence path, so a local reviewer can verify the committed index against the ignored evidence. This keeps large mechanical detail out of Git and out of normal review output.

Outputs are deterministic: fixed audit timestamps are separated from semantic inputs, and IDs derive from source hashes and raw spans. Re-running against the same evidence, manifests, and scanner version must produce identical generated and local-evidence files. Existing different output directories are not overwritten.

## Recognized fixed-form inventory

The scanner preserves raw bytes and locators, recognizes fixed-form labels, column-six continuation, columns 7--72 statement text, trailing columns, `C`/`c`/`*`/`!` comment lines, quoted strings, doubled quotes, inline comments, Hollerith-aware comment handling, and semicolon-separated statements. Tabs, non-ASCII bytes, malformed labels, uncertain boundaries, and other unsupported constructs become deterministic review items rather than being silently normalized.

It inventories `SUBROUTINE`, typed and untyped `FUNCTION`, `PROGRAM`, named and unnamed `BLOCK DATA`, `ENTRY`, and bare or typed `END` boundaries. Identifier comparison is case-insensitive while original spelling and source locators are retained. A source file may provide zero, one, or several top-level units.

The duplicate report distinguishes duplicate top-level units, multiple entries, and program-unit/entry collisions. It does not use filename, archive order, or directory order to resolve a collision. A clean result is scoped only to the pinned snapshot, selected source files, and scanner version.

## What it does not prove

This stage is not a Fortran semantic parser. It does not extract dependencies, declarations beyond program-unit prefixes, ABI signatures, `COMMON`/`SAVE`/`DATA` semantics, FFI, native builds, or safe Rust APIs. A successful scan proves mechanical reproducibility and the scoped provider-name result only; it does not establish historical originality, redistribution rights, ABI safety, dependency completeness, or valid native crate splitting.
