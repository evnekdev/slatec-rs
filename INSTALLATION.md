# Installation

Copy the contents of this package into the root of the `slatec-rs` repository, preserving paths.

The package adds:

```text
AGENTS.md
crates/slatec-sys/AGENTS.md
crates/slatec-src/AGENTS.md
crates/slatec-core/AGENTS.md
crates/slatec/AGENTS.md
crates/slatec-tools/AGENTS.md
generated/AGENTS.md
docs/architecture/README.md
docs/agent/REPOSITORY-MAP.md
docs/agent/TASK-CHECKLIST.md
```

Review any existing file at the same path before overwriting it. The package was designed against the repository structure containing the five workspace crates listed above.

After copying:

```bash
git status --short
git diff --check
git add AGENTS.md crates/*/AGENTS.md generated/AGENTS.md docs/architecture/README.md docs/agent
git commit -m "Add Codex repository guidance"
```

No code generation or Rust build is required merely to add these documentation files, though normal repository policy may still require documentation checks.
