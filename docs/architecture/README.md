# Architecture index

This index is the starting point for maintainers and coding agents. Keep it short and update links when architecture documents move.

## Layering

- `../../README.md` — project overview and current capabilities.
- `slatec-sys-public-raw-api.md` — raw API architecture, canonical paths, compatibility policy, documentation, and generated-layer transition.
- `../api/family-features-and-backends.md` — public family features and provider relationships.
- `native-link-granularity.md` — native source-to-object-to-archive pipeline and final-link guarantees.

## Repository map

- `../agent/REPOSITORY-MAP.md` — ownership, authoritative inputs, generated outputs, and common change paths.
- `../agent/TASK-CHECKLIST.md` — reconnaissance, implementation, validation, and completion checklist for Codex tasks.

## Core invariants

1. `slatec-sys` declares ABIs and does not select providers.
2. `slatec-src` owns provider/source closure and native archive construction.
3. `slatec-core` remains provider-neutral shared infrastructure.
4. `slatec` owns checked safe wrappers.
5. `slatec-tools` owns generators and audits.
6. `generated/` is reproducible output, never an authored implementation layer.
7. `slatec-sys/all` includes every public function-family aggregate but selects no provider.
8. Feature breadth does not imply final binary breadth.
9. Numerical source and ABI changes require explicit evidence and regression coverage.
10. New cross-cutting rules require mechanical validation.

## Maintenance rule

When a PR changes cross-crate ownership, generation flow, feature semantics, canonical API paths, source-provider guarantees, or link-granularity guarantees, update this index and the relevant detailed document in the same PR.
