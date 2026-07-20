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
