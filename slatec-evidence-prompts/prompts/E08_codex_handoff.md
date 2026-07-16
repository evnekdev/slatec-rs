# E08 — Prepare the Codex Implementation Handoff

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`. Read E01–E07 and current Rust architecture documents.

## Objective

Convert the research into a bounded, executable Codex programme. Do not implement the parser in this task.

## Work packages

Define separate milestones for:

1. source acquisition and checksumming;
2. normalization;
3. fixed-form lexical parsing;
4. program-unit extraction;
5. prologue parsing;
6. semantic extraction;
7. schema validation;
8. Markdown/TOML generation;
9. dependency extraction;
10. object compilation and symbol inspection;
11. linker validation;
12. audit and reproducibility.

For each package specify prerequisites, exact inputs and outputs, commands, tests, acceptance criteria, failure modes, protected files, and PR boundaries.

Recommend implementation language and crate structure. Rust is preferred unless another language is strongly justified for a narrow preprocessing task.

## Required outputs

```text
docs/implementation/
    README.md
    codex-roadmap.md
    work-package-01-source-acquisition.md
    work-package-02-normalization.md
    work-package-03-parser-core.md
    work-package-04-prologue-parser.md
    work-package-05-metadata-generation.md
    work-package-06-dependency-graph.md
    work-package-07-symbol-validation.md
    work-package-08-ci-and-reproducibility.md
    acceptance-test-matrix.md
metadata/
    implementation-roadmap.toml
```

## Completion criteria

Each work package can be started in a fresh Codex session with minimal interpretation.
