# SLATEC documentation collection prompt pack

This pack contains standalone prompts for building the initial `slatec-rs` knowledge base in separate ChatGPT conversations.

## Recommended execution model

Use a fresh conversation for each numbered prompt. Upload this repository skeleton or, preferably, the latest documentation ZIP produced by the preceding prompt when a task depends on earlier work.

Do **not** ask each conversation to edit GitHub through the connector. For this documentation phase, the more reliable workflow is:

1. Run one prompt in a standalone ChatGPT conversation.
2. Ask the model to produce the requested files as a downloadable ZIP.
3. Download and inspect the ZIP locally.
4. Merge it into your local `slatec-rs` checkout.
5. Commit and push with ordinary Git tooling.
6. Upload the updated documentation ZIP to the next conversation when prior context is required.

This avoids connector write friction, gives you a reviewable artifact, and prevents independent conversations from overwriting one another. The GitHub connector remains useful for read-only inspection and later PR review.

## Suggested model tier

- Prompts 01-05: highest available reasoning/research tier, such as GPT-5.6 Thinking or equivalent.
- Prompt 06: highest reasoning tier; the work is taxonomy-heavy.
- Prompts 07-08: Codex High is preferable if processing many source files or generating metadata programmatically. ChatGPT can perform a small pilot.
- Prompts 09-10: highest reasoning tier for synthesis and quality review.

## Order

Run prompts in numeric order. Prompts 03-05 may run in parallel after Prompt 02, but merge conflicts must be resolved locally.

## Shared rules

Every prompt instructs the model to:

- browse current authoritative sources;
- prioritize official Netlib/SLATEC material and original package manuals;
- distinguish sourced facts, modern interpretation, and `slatec-rs` proposals;
- paraphrase rather than reproduce long copyrighted passages;
- cite every factual assertion that is not obvious;
- record uncertainty instead of guessing;
- create Markdown files rather than only answering in chat;
- return a ZIP containing only the requested repository-relative files;
- not modify GitHub directly.

## Expected repository area

```text
docs/
  README.md
  history/
  architecture/
  domains/
  packages/
  taxonomy/
  routines/
  rust/
  sources/
metadata/
```

## Merge discipline

Before merging a ZIP:

- verify that all paths are repository-relative;
- inspect citations and source URLs;
- reject unattributed claims;
- check that files do not silently replace newer work;
- run a Markdown link checker when available;
- commit each prompt result separately.
