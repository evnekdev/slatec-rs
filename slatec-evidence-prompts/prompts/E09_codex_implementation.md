# E09 — Implement One Extraction-Tooling Work Package

This prompt is for Codex.

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`, `docs/implementation/README.md`, `docs/implementation/codex-roadmap.md`, and one selected work-package document.

## Objective

Implement exactly one work package at a time.

## Rules

- Create a dedicated branch.
- Do not combine unrelated work packages.
- Preserve original source artifacts unchanged.
- Make generated output deterministic.
- Add tests before or alongside implementation.
- Run every required check.
- Record commands and results.
- Preserve unknown fields and raw evidence.
- Do not infer undocumented ABI properties.
- Do not generate safe Rust wrappers.
- Do not split native crates unless the selected package explicitly authorizes and validates it.

## Completion deliverables

- implementation summary;
- changed-file manifest;
- commands and tests run;
- unresolved cases;
- committed and pushed branch;
- draft pull request when permissions permit.
