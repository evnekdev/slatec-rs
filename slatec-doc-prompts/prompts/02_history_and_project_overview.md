# Prompt 02 — SLATEC history and project overview

This is a standalone research task for `slatec-rs`, a future Rust interface to the SLATEC Common Mathematical Library. The present phase is documentation only; do not write Rust code.

## Task

Produce a carefully sourced historical and conceptual overview of SLATEC.

Research:

- why the library was created;
- the institutions and individuals involved;
- the historical computing environment that motivated a common library;
- release chronology and major version changes;
- relationship to national laboratories and standards organizations;
- how SLATEC integrated existing packages versus adding its own routines;
- historical distribution and maintenance model;
- current preservation status and continuing influence;
- what is known versus uncertain about present maintenance.

Do not repeat folklore without verification. Separate:

1. sourced history;
2. modern interpretation;
3. implications for `slatec-rs`.

## Required files

```text
docs/history/overview.md
docs/history/chronology.md
docs/history/participating-institutions.md
docs/history/releases-and-changes.md
docs/README.md
```

The root documentation README should explain the purpose and navigation of the knowledge base, not the Rust implementation.

## Required structure

Each historical document should include:

- scope;
- main narrative or table;
- unresolved questions;
- sources.

The chronology should use exact dates or years only when supported. Mark approximate dates explicitly.

## Source rules

Use official SLATEC/Netlib material first, then original reports and reputable historical sources. Cite each nontrivial factual paragraph.

## Deliverable

Return a downloadable ZIP containing only the requested files with repository-relative paths. Do not modify GitHub directly.
