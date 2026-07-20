# `slatec-tools` instructions

Scope: `crates/slatec-tools/**`.

## Responsibility

This crate is the authoritative home for source-corpus processing, inventories, generators, audits, validation, and deterministic evidence production.

## Rules

- Generators must be deterministic for identical inputs.
- Separate authored inputs, cached external evidence, generated Rust, generated reports, and ignored build artifacts.
- Every generated format needs a clear schema/version or stable semantic contract.
- Validation commands must fail on drift; generators must not silently normalize away material discrepancies.
- Prefer one authoritative record per routine/source/symbol and derive secondary reports from it.
- Keep classifications explicit: supported, unsupported, missing provider, unresolved callback, catalogue-only, excluded artefact, and so on.
- Source/ABI corrections must identify evidence and expected source hash.
- Offline mode must not access the network.
- Clean-root regeneration must not depend on undeclared local state.
- Do not embed broad runtime function-pointer registries into published numerical crates merely to support tooling.
- Large raw logs, binaries, maps, archives, and source caches belong under ignored `target/` or cache locations; commit compact deterministic summaries only.
- When adding a command, update command dispatch, usage/help, validation, documentation, and deterministic manifests together.

## Generator workflow

Before changing output:

1. identify the authoritative input and generator;
2. change authored code/data, not the output;
3. generate into the expected output directory;
4. run the corresponding validator;
5. rerun and compare output for determinism;
6. inspect the diff for unrelated churn;
7. perform clean-root regeneration where the workflow supports it.
