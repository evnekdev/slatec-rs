# E06 — Specify the Deterministic Extraction Toolchain

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`. Read E01–E05 and existing dependency specifications.

## Objective

Produce an implementation-ready specification for parsing the canonical SLATEC corpus and generating routine metadata. Do not implement code.

## Required parser capabilities

Cover fixed-form Fortran:

- `SUBROUTINE`, `FUNCTION`, `PROGRAM`, `BLOCK DATA`;
- continuation and comments;
- Hollerith constants where relevant;
- `ENTRY`, `EXTERNAL`, `INTRINSIC`;
- `COMMON`, `SAVE`, `DATA`, `EQUIVALENCE`;
- alternate returns;
- character arguments and lengths;
- complex functions and logical arguments;
- assigned/computed `GOTO` where relevant;
- statement functions;
- direct calls, function references, and callback forwarding.

Cover final SLATEC, legacy, QUADPACK, and package-specific prologue dialects. Preserve raw unrecognized fields and exact source spans.

Define fixtures covering modern and legacy SLATEC, QUADPACK, BLAS, LINPACK, FFTPACK, PCHIP, DASSL, special functions, SLAP, error handling, machine constants, multiple program units, `BLOCK DATA`, callbacks, complex returns, and character ABI cases.

## Required outputs

```text
docs/extraction/
    toolchain-architecture.md
    fixed-form-parser-spec.md
    prologue-parser-spec.md
    semantic-extraction-spec.md
    fixture-corpus.md
    generated-output-contract.md
    deterministic-build-requirements.md
    manual-review-workflow.md
metadata/schema/
    routine-record-spec.md
    program-unit-record-spec.md
    source-file-record-spec.md
```

## Completion criteria

Codex can implement the parser without independently making major architectural decisions.
