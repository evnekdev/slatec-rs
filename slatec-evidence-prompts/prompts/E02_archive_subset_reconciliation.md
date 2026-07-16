# E02 — Reconcile Archives, Live Trees, and Relocated Subsets

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md` and read all E01 outputs.

## Objective

Determine how candidate SLATEC artifacts relate at the file and program-unit level. Identify duplicates, differences, relocated files, alternate implementations, and potentially newer or corrected copies.

Do not make the final canonical-corpus decision unless the evidence is conclusive.

## Investigation

Compare, as public access permits:

- the main SLATEC archive;
- the live `slatec/src` tree;
- relocated Netlib subsets;
- standalone upstream packages;
- quick-check and installation bundles;
- machine-specific alternatives.

Investigate:

- filename and case-only matches;
- byte-identical and normalized-text-identical files;
- comment-only, prologue-only, and executable-code differences;
- renamed program units;
- multiple program units per file;
- duplicate definitions;
- revision markers;
- missing and added files;
- alternate machine constants and error systems.

When direct comparison cannot be completed, specify exactly what must be downloaded and compared locally. Never invent checksums or diff results.

## Required outputs

```text
docs/source-corpus/
    reconciliation-method.md
    archive-vs-live-tree.md
    relocated-subsets.md
    upstream-package-comparison.md
    duplicate-and-conflict-register.md
metadata/
    source-relationships.toml
```

Use statuses such as:

- `verified-identical`;
- `verified-different`;
- `likely-identical`;
- `possible-duplicate`;
- `alternate-implementation`;
- `unresolved`.

## Completion criteria

- Comparison methodology is reproducible.
- Every known overlapping source set has a recorded relationship.
- Unresolved comparisons are itemized.
- No duplicate provider is silently selected.
