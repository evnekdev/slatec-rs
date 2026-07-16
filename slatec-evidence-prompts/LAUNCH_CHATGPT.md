# Launch Prompt for ChatGPT

Copy this into a new ChatGPT conversation and replace `<TASK_FILE>` with the selected prompt path, for example `slatec-evidence-prompts/prompts/E01_candidate_source_inventory.md`.

```markdown
Use the connected GitHub repository `evnekdev/slatec-rs`.

Work from the current `master` branch.

Before starting, inspect the current repository structure and read these files in order:

1. `slatec-evidence-prompts/README.md`
2. `slatec-evidence-prompts/SHARED_CONTEXT.md`
3. `<TASK_FILE>`

Then read every prerequisite referenced by the selected task file, including relevant outputs from earlier E-series tasks already merged into `master`.

Treat `SHARED_CONTEXT.md` as the programme-wide rules and `<TASK_FILE>` as the authoritative task specification. Where they differ, follow the more task-specific instruction unless it explicitly conflicts with repository policy.

Execute the selected task completely. Browse authoritative public sources where required. Do not modify GitHub through the connector.

Return a downloadable ZIP containing only the requested new or modified files, preserving repository-relative paths. Also provide:

- a file manifest;
- a source report;
- open questions;
- the recommended next action.

Do not include unrelated repository files in the ZIP.
```
