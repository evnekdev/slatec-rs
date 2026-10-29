# Codex task checklist

Use this for changes spanning more than one file or crate. **The issue-first requirement below applies to every repository-content change, even a one-line edit.**

## 0. Required issue BEFORE editing

- [ ] Create or identify an existing GitHub issue **in this repository** that accurately covers the intended change; do so before editing any file, including documentation or generated records.
- [ ] Record problem/rationale, proposed files/crates, public API/features/providers/ABI/source-closure impact, supporting evidence, acceptance criteria, exact validation plan, explicit non-goals and stop conditions.
- [ ] For work spanning repositories, ensure each changed repository has its own cross-linked issue before any corresponding edit.
- [ ] Add an issue update **before** expanding scope; split unrelated work into a separate issue.
- [ ] Link commits/PRs to the issue and report exact results/limitations there before closure.

## 1. Reconnaissance

- [ ] Fetch and record current `origin/master` SHA.
- [ ] Read every applicable `AGENTS.md` from repository root to the target path.
- [ ] Read `docs/architecture/README.md` and the relevant detailed architecture documents.
- [ ] Run `cargo run -p slatec-tools --bin slatec-corpus -- validate-agent-guidance` when `AGENTS.md`, workspace membership, architecture navigation, or `docs/agent` files change.
- [ ] Identify authoritative authored inputs.
- [ ] Identify generated outputs and their generator commands.
- [ ] Identify affected crates, public paths, Cargo features, providers, and source closures.
- [ ] Identify compatibility and `no_std` constraints.
- [ ] Identify required native toolchain/cache/environment variables.
- [ ] State explicit non-goals before editing.

## 2. Plan

- [ ] Keep one primary objective.
- [ ] List the smallest coherent files/modules to change.
- [ ] Define acceptance criteria in observable terms.
- [ ] Prefer symbol/behaviour/invariant checks over subjective claims.
- [ ] Decide how deterministic regeneration will be verified.
- [ ] Decide what evidence belongs in committed summaries versus ignored `target/` artifacts.

## 3. Implementation

- [ ] Edit authored sources, not generated outputs.
- [ ] Preserve crate-layer responsibilities.
- [ ] Preserve public compatibility unless explicitly authorized otherwise.
- [ ] Avoid introducing broad production reachability or provider coupling.
- [ ] Add or update mechanical checks for new invariants.
- [ ] Add focused tests while implementing.
- [ ] Keep unrelated cleanup out of the patch.

## 4. Validation

- [ ] Run focused crate/unit tests first.
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo check --workspace`.
- [ ] Run strict workspace Clippy.
- [ ] Run workspace tests.
- [ ] Run strict rustdoc and doctests.
- [ ] Run affected reduced-feature and `no_std` checks.
- [ ] Run affected raw-FFI, source-cache, feature/provider, symbol, link, and native regressions.
- [ ] Run generator validation.
- [ ] Run the guidance validator when this PR changes its inputs.
- [ ] Run same-root deterministic regeneration.
- [ ] Run clean-root deterministic regeneration where supported.
- [ ] Run package-content audit.
- [ ] Inspect `git diff --check`, `git status`, and the complete diff.
- [ ] Verify no native/cache/temp artifacts are tracked.

## 5. Completion report

- [ ] Starting SHA, branch, final commit, and PR.
- [ ] Root cause or architectural rationale.
- [ ] Authored inputs changed.
- [ ] Generated outputs regenerated.
- [ ] Public API, feature, provider, and binary-link impact.
- [ ] Tests and commands actually run.
- [ ] Commands not run and exact reason.
- [ ] Remaining limitations/follow-ups.
- [ ] Confirmation of deterministic output and clean package contents.

## Prompt preamble template

Use this at the start of substantial Codex prompts:

```text
Before editing:
0. open or identify the GitHub issue that explains this work and its boundaries; update the issue if incomplete;
1. fetch and verify origin/master;
2. read all applicable AGENTS.md files;
3. read docs/architecture/README.md and relevant linked documents;
4. identify authoritative inputs, generated outputs, affected crates/features/providers, compatibility constraints, and validation commands;
5. report the implementation plan and explicit non-goals;
6. then implement without asking for confirmation unless blocked by missing credentials or inaccessible required inputs.
```
