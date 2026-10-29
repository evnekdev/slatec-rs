# Codex repository guide

<!-- agent-rule: sys-provider-neutral -->
<!-- agent-rule: all-declaration-only -->
<!-- agent-rule: generated-no-hand-edit -->
<!-- agent-rule: link-granularity -->
<!-- agent-rule: honest-native-validation -->

This file is a map, not the complete specification. Read the relevant linked documents before editing. More-specific `AGENTS.md` files under a directory override this file for that subtree.

Every workspace crate needs a crate-local `AGENTS.md`; an exceptional path must be documented here with an `<!-- agent-guidance-exemption: crates/path -->` marker.

## Repository purpose

`slatec-rs` provides layered Rust access to selected SLATEC numerical routines:

- `crates/slatec-sys`: unsafe raw declarations and ABI types. It does not select or build a provider.
- `crates/slatec-src`: source/system/external provider selection and native source closures.
- `crates/slatec-core`: shared provider-neutral types and infrastructure.
- `crates/slatec`: safe Rust facade.
- `crates/slatec-tools`: authoritative generators, audits, validation, and source-corpus tooling.
- `generated`: committed deterministic evidence and generated outputs. Never hand-edit these files.
- `docs`: architecture, API policy, validation evidence, and user documentation.

Start with the [architecture index](docs/architecture/README.md), [repository map](docs/agent/REPOSITORY-MAP.md), and [task checklist](docs/agent/TASK-CHECKLIST.md).
For how these documents apply to Codex tasks, read [Using Codex](docs/agent/USING-CODEX.md).

## Non-negotiable architecture rules

1. Preserve the layer boundary: safe facade -> raw declarations -> provider. Do not make `slatec-sys` choose or build a provider.
2. `slatec-sys/all` includes every public mathematical function-family aggregate. It is declaration-only and must not select a provider.
3. Enabling a feature must not by itself retain numerical implementations in a final binary. Only referenced routines and their legitimate transitive closure should link for the validated source backend.
4. Public routines have one canonical mathematical-family path. ABI-shaped generated namespaces are transitional/internal, not the desired public organization.
5. Do not modify numerical Fortran sources or source semantics unless the task explicitly requires it.
6. Source corrections and ABI corrections must be evidence-based, source-hash guarded, deterministic, and documented.
7. Do not introduce one Cargo feature per routine unless the task explicitly establishes that no better design exists.
8. Preserve `no_std` and provider-neutral boundaries where already supported.
9. Do not weaken tests, audits, deterministic generation, or safety checks to make a change pass.
10. Never claim a native command passed unless it actually ran in the current environment.

## Mandatory issue-first change history

**Before changing ANY file in this repository, open or identify a sufficiently detailed GitHub issue in `evnekdev/slatec-rs`.** This applies to every crate in the SLATEC family (`slatec`, `slatec-sys`, `slatec-src`, `slatec-core`, `slatec-tools`, bundled-provider crates), as well as documentation, tests, generators, source metadata, native build integration, manifests, CI and generated records. This includes one-line edits, urgent fixes and planned refactors. Do not implement first and file an issue later.

An issue must identify **why** the change is necessary, concrete proposed work and owning paths/crates, evidence and expected outcome, affected public APIs/features/providers/ABIs/source closures (or explicitly state none), compatibility and native-link risks, bounded non-goals, acceptance criteria and validation commands. An existing issue counts only if its description actually covers the task; add missing details **before** editing. Use one issue per cohesive change, not per file.

Record meaningful decisions and update the issue **before** any material expansion in scope. Create a separate issue for unrelated work. Every cross-repository change requires a related issue in each affected repository, linked before modifications. Reference the issue from commits and PRs, then record exact implemented work, PASS/FAIL/NOT RUN/BLOCKED verification, remaining limitations and closure reason. Tracking-only issue administration (creating/editing issues, comments and reviews) does not itself require another issue.

This is a maintainer-required workflow, not permission to bypass existing native source, generator, ABI, feature/provider, scientific or release validation. See [task checklist](docs/agent/TASK-CHECKLIST.md) and shared [engineering-architecture issue-first guidance](https://github.com/evnekdev/engineering-architecture/blob/main/policies/agent-workflow.md).

## Before editing

For any non-trivial task, first identify and report:

1. authoritative authored inputs;
2. generated outputs;
3. affected crates and features;
4. canonical public paths and compatibility constraints;
5. native-provider implications;
6. required validation commands;
7. files or areas deliberately out of scope.

Search authored registries and generator code before searching large generated reports. Treat generated reports as evidence, not as the primary implementation source.

## Generated-file policy

- Do not manually patch files under `generated/`.
- Do not manually patch Rust files marked `@generated` or `Do not edit`.
- Find and change the authoritative registry, manifest, template, or generator.
- Regenerate through the documented command.
- Run both validation and deterministic-regeneration checks where available.
- Keep native objects, archives, executables, maps, caches, and raw tool logs under ignored `target/` paths.

## Change discipline

- Keep each PR focused on one milestone.
- Avoid combining family promotion, architecture redesign, feature changes, binary-size optimization, and publishing in one change.
- Preserve existing public paths and behaviour unless the task explicitly authorizes a breaking change.
- Prefer small independently testable modules and deterministic data transformations.
- Add a mechanical invariant when introducing a new architectural rule.
- Update architecture documentation when changing cross-crate responsibilities or generation flow.
- Review the final diff for unrelated generated churn.

## Repository engineering workflow

Use `rg` for text and symbol searches. For non-trivial Rust analysis, use
rust-analyzer when an available interface supports the required semantic
operation. Use `sg` (ast-grep) for repeated structural transformations: first
preview matches, then apply the narrowest syntax-aware rewrite. Do not use a
regular-expression rewrite where it could alter serialized names, FFI symbols,
or public compatibility.

Use the following tools when they are relevant to the task:

- `cargo check` for compile validation; `cargo test` or `cargo nextest run` for behavioural validation;
- `cargo clippy`, `cargo fmt`, and compiler-suggested `cargo fix` for Rust hygiene;
- `cargo expand` when investigating macro-generated code;
- `cargo semver-checks` and `cargo public-api` when evaluating published API compatibility;
- `cargo machete` when reviewing likely unused dependencies; and
- `git diff` plus `git diff --check` for the final change review.

For every non-trivial refactoring:

1. Inspect `git status` before editing, then identify definitions and references with `rg`.
2. Preserve serialized names, FFI symbols, and public compatibility unless the task explicitly authorizes a breaking change.
3. Change authored inputs rather than generated output; regenerate affected output through its owner.
4. Inspect the complete diff after automated changes and before completion.
5. Run formatting, compilation, tests, Clippy, and the relevant generator or native validation before handoff.

For ordinary Rust changes, unless a user or task-specific instruction requires a
narrower scope, run this final matrix in addition to task-specific validation:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check
```

## Baseline validation

Run the narrowest relevant checks while iterating, then before completion run as much of this matrix as the task and environment permit:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo test --doc --workspace
```

Also run every generator validation, source-cache verification, native regression, feature/provider reconciliation, deterministic-regeneration, and package-content audit affected by the change.

Run `cargo run -p slatec-tools --bin slatec-corpus -- validate-agent-guidance` whenever
`AGENTS.md`, workspace membership, architecture navigation, or `docs/agent` files change.

If a command cannot run because of unavailable source cache, compiler, target, credentials, network, or unpublished workspace dependencies, report the exact limitation and do not mark it passed.

## Completion report

State:

- starting origin SHA and final branch/commit;
- files and subsystems changed;
- authoritative inputs changed;
- generated outputs regenerated;
- public API/feature/provider impact;
- commands actually run and their results;
- commands not run and why;
- remaining risks or follow-up work;
- confirmation that no unrelated generated or native artifacts were committed.
