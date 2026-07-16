# E03 — Select and Document the Canonical Corpus Policy

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md` and read all E01 and E02 outputs.

## Objective

Define the reproducible source-selection policy used by future parsing, compilation, and metadata generation.

The result may select one artifact, one live tree, or a reconciled corpus. Every choice must be evidence-based.

## Required decisions

Determine:

- the primary baseline artifact;
- whether it is immutable;
- whether relocated subsets override, supplement, or only document baseline files;
- how corrections and alternate implementations are handled;
- how duplicate providers are resolved;
- how machine-specific files are selected;
- how project patches and future upstream changes are recorded;
- which files are excluded and why;
- whether the original corpus is preserved unchanged;
- whether a normalized working corpus is generated separately.

Keep these layers separate:

1. preserved original artifact;
2. selected canonical source;
3. normalization-only transformations;
4. project patches;
5. generated outputs.

## Required outputs

```text
docs/source-corpus/
    canonical-corpus-policy.md
    source-selection-precedence.md
    normalization-policy.md
    patch-policy.md
    reproducibility-policy.md
metadata/
    canonical-corpus.toml
```

The TOML must include policy version, baseline and selected source IDs, exclusions, override rules, normalization rules, patch policy, checksum requirements, and unresolved selections.

If evidence remains insufficient, produce a conditional policy with explicit blockers.

## Completion criteria

A future deterministic tool can determine which files enter the working corpus, which provider wins, which transformations are permitted, and how the result is reproduced.
