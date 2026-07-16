# E10 — Audit Generated Source and Routine Metadata

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`. Read E01–E08, generated extraction outputs, parser tests, implementation reports, and relevant source artifacts.

## Objective

Independently assess whether generated metadata is trustworthy enough for dependency analysis and FFI planning. Do not modify parser code.

## Required audit

Review:

- deterministic regeneration;
- source checksums;
- parser-version recording;
- field-level source locators;
- raw unparsed-field preservation;
- program-unit counts and identity;
- argument extraction;
- callbacks and workspaces;
- errors and GAMS classification;
- provenance;
- direct calls and global state;
- Markdown/TOML parity;
- duplicate IDs;
- unresolved references;
- fixture coverage;
- false positives and false negatives.

Use a stratified manual sample across core support, BLAS/LINPACK, QUADPACK, FFTPACK, PCHIP, DASSL, special functions, SLAP, and legacy prologues.

## Required outputs

```text
docs/audits/
    extraction-audit.md
    sampled-routine-review.md
    parser-gap-register.md
    readiness-for-dependency-analysis.md
metadata/
    extraction-audit.toml
```

Classify readiness as one of:

- `not-ready`;
- `pilot-ready`;
- `dependency-analysis-ready`;
- `ffi-planning-ready`.

Do not approve automatic FFI generation unless signatures and ABI-relevant fields have adequate evidence.

## Completion criteria

The project has an evidence-backed decision to proceed to full dependency analysis or return to parser development.
