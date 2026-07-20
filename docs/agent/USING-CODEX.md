# Using Codex in this repository

Codex reads `AGENTS.md` hierarchically. The root file applies across the repository; a deeper `AGENTS.md` adds rules for its own subtree. Prompt instructions take precedence over repository guidance.

Launch Codex from the repository root. For a substantial task, ask it to perform reconnaissance first: fetch and verify `origin/master`, read every applicable guidance file and the relevant architecture documents, identify authoritative inputs and generated outputs, then state the plan and non-goals before editing.

Run the offline guidance check whenever guidance, workspace membership, architecture navigation, or `docs/agent` files change:

```bash
cargo run -p slatec-tools --bin slatec-corpus -- validate-agent-guidance
```

Update `AGENTS.md` when a concise, subtree-specific rule or ownership boundary changes. Put detailed design, evidence, and long-lived policy in the architecture documents instead; avoid duplicating those manuals in guidance files.
