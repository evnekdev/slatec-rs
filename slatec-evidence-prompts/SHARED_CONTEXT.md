# Shared Context — SLATEC-RS Source Corpus and Evidence Extraction

Use the connected GitHub repository:

`evnekdev/slatec-rs`

Work from the current `master` branch unless the task explicitly requests a development branch.

## Required reading before every task

Inspect the repository structure, then read these files when present:

- `README.md`
- `slatec-doc-prompts/REPORT.md`
- `docs/audit-report.md`
- `docs/collection-status.md`
- `docs/open-research-questions.md`
- `docs/sources/source-register.md`
- `docs/sources/citation-policy.md`
- `metadata/sources.toml`

Also read every prerequisite named in the selected task prompt.

## Project objective

The current phase must freeze a reproducible SLATEC source corpus and build an evidence model suitable for deterministic parsing, dependency analysis, compilation, symbol validation, and later FFI planning.

The current documentation is broad but provisional. In particular, do not assume that:

- one source archive is already canonical;
- Netlib directory co-location proves package ownership;
- prologue `ROUTINES CALLED` fields exactly match executable dependencies;
- all Netlib-hosted material has one licence;
- routines are thread-safe;
- compiler ABI details are uniform;
- historical package boundaries equal Rust crate boundaries.

## Evidence rules

Always distinguish:

1. retrieved artifact facts;
2. facts directly derived from inspected source content;
3. mechanically derived results;
4. informed interpretations;
5. project design decisions;
6. unresolved or conflicting evidence.

Never silently replace one evidence source with another. Preserve disagreement between prologues, parsed source, Netlib dependency products, object symbols, and linker behaviour.

Record exact URLs, retrieval dates, archive names, file names, checksums, source spans, and parser versions whenever available. Never invent a checksum, source comparison, or file inspection that was not actually performed.

## General constraints

- Do not modify GitHub through the connector in ChatGPT documentation/research tasks.
- Do not implement Rust unless the selected prompt explicitly requests implementation.
- Prefer official Netlib, SLATEC, NIST/GAMS, and original package documentation.
- Use secondary sources only for context and label them accordingly.
- Do not reproduce large copyrighted passages; distil and cite.
- Preserve repository-relative paths in all returned artifacts.
- Do not include unrelated repository files in output ZIPs.

## Standard ChatGPT deliverables

Return:

1. a downloadable ZIP containing only created or modified files;
2. a manifest of included files;
3. a source report;
4. an `Open Questions` section;
5. a `Recommended Next Action` section.
