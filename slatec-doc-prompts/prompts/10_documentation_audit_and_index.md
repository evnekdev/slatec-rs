# Prompt 10 — Documentation audit, cross-linking and final collection-stage index

This is a standalone quality-review task for `slatec-rs`. The user will upload the merged documentation ZIP produced by previous prompts.

## Task

Audit the documentation collection for completeness, consistency, citation quality and internal contradictions.

Perform:

- inventory of all Markdown and metadata files;
- broken-link and missing-target review;
- terminology consistency review;
- source-ID consistency review;
- duplicate or contradictory claim detection;
- distinction between fact, inference and proposal;
- missing citations;
- unsupported dates or quantities;
- domain overlap review;
- metadata-schema consistency;
- gaps that must be resolved before bulk routine extraction;
- prioritised issue list.

Do not silently rewrite uncertain technical claims. Mark them for review.

## Required files

```text
docs/index.md
docs/collection-status.md
docs/open-research-questions.md
docs/audit-report.md
metadata/coverage.toml
```

You may also correct existing documentation files when the correction is clearly supported. Include corrected files in the ZIP and list every change in `docs/audit-report.md`.

## Completion criteria

The collection stage is ready to advance only if:

- primary sources are registered;
- architecture and conventions are documented;
- mathematical domains and imported packages are mapped;
- the routine metadata pilot is coherent;
- dependency extraction is specified;
- Rust mapping is explicitly labelled as proposed;
- unresolved issues are recorded.

## Deliverable

Return a downloadable ZIP containing the requested files plus any explicitly corrected existing files. Preserve repository-relative paths. Do not modify GitHub directly.
