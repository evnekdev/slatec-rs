# Launch Prompt for Codex

Use this after E08 has been merged and a specific implementation work package has been selected.

```markdown
Use the repository `evnekdev/slatec-rs` and begin from the latest `master` branch.

Read these files before making changes:

1. `slatec-evidence-prompts/README.md`
2. `slatec-evidence-prompts/SHARED_CONTEXT.md`
3. `slatec-evidence-prompts/prompts/E09_codex_implementation.md`
4. `docs/implementation/README.md`
5. `docs/implementation/codex-roadmap.md`
6. `<SELECTED_WORK_PACKAGE_FILE>`
7. every prerequisite named by the selected work package

Implement exactly one work package. Create a dedicated branch and do not combine unrelated milestones.

Requirements:

- preserve original source artifacts unchanged;
- make generated outputs deterministic;
- add and run the required tests;
- preserve unknown parser fields and raw evidence;
- do not infer undocumented ABI properties;
- do not create safe wrappers;
- do not split native crates unless the selected work package explicitly and evidentially authorizes it;
- record commands, test results, and unresolved cases.

At completion:

1. summarize the implementation;
2. list all changed files;
3. report commands and tests run;
4. report unresolved parser or source cases;
5. commit and push the branch;
6. open a draft pull request when connector permissions permit.
```
