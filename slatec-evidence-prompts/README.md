# SLATEC-RS Evidence Extraction Prompt Pack

This pack contains autonomous prompts E01–E10 for the **Source Corpus Freezing and Evidence Extraction** phase of `slatec-rs`.

## Recommended repository location

Copy this directory into:

```text
slatec-evidence-prompts/
```

at the root of `evnekdev/slatec-rs`.

## Execution order

| Prompt | Purpose | Environment |
|---|---|---|
| E01 | Candidate source corpus inventory | ChatGPT, GPT-5.6 Thinking |
| E02 | Archive and subset reconciliation | ChatGPT, GPT-5.6 Thinking |
| E03 | Canonical corpus policy | ChatGPT, GPT-5.6 Thinking |
| E04 | Rights and provenance register | ChatGPT, GPT-5.6 Thinking |
| E05 | Evidence model normalization | ChatGPT, GPT-5.6 Thinking |
| E06 | Extraction toolchain specification | ChatGPT, GPT-5.6 Thinking |
| E07 | Dependency evidence reconciliation | ChatGPT, GPT-5.6 Thinking |
| E08 | Codex implementation handoff | ChatGPT, GPT-5.6 Thinking |
| E09 | Implement one work package | Codex, High reasoning |
| E10 | Independent extraction audit | ChatGPT, GPT-5.6 Thinking |

Run E01–E08 sequentially. Merge and push each result before starting the next prompt. E09 is a reusable Codex launch prompt for one implementation work package at a time. Run E10 after generated extraction outputs exist.

## Files

- `SHARED_CONTEXT.md` — rules common to E01–E10.
- `LAUNCH_CHATGPT.md` — copyable launcher for E01–E08 or E10.
- `LAUNCH_CODEX.md` — copyable launcher for E09.
- `prompts/E01_...md` through `prompts/E10_...md` — task specifications.

## Output workflow

For ChatGPT tasks, request a downloadable ZIP rather than direct GitHub writes. Review, merge, commit, and push locally. For Codex implementation tasks, use a branch and draft pull request.
