# Architecture index

This index is the starting point for maintainers and coding agents. Keep it short and update links when architecture documents move.

## Layering

- [Project overview](../../README.md) — current capabilities.
- [Raw API architecture](slatec-sys-public-raw-api.md) — canonical paths, private declaration ownership, documentation, and generated-layer policy.
- [Final raw API coverage](../api/raw-api-final-coverage.md) — Batch A-D closure, permanent dispositions, exceptional interfaces, and reopen policy.
- [Family features and backends](../api/family-features-and-backends.md) — public family features and provider relationships.
- [Bundled provider and provenance gate](bundled-provider-and-provenance.md) — target carrier design, redistribution evidence, and the no-fallback release gate.
- [Native link granularity](native-link-granularity.md) — native source-to-object-to-archive pipeline and final-link guarantees.
- [Safe-facade link granularity](safe-facade-link-granularity.md) — safe-wrapper operation closure policy and safe-versus-raw link evidence.

## Repository map

- [Repository map](../agent/REPOSITORY-MAP.md) — ownership, authoritative inputs, generated outputs, and common change paths.
- [Task checklist](../agent/TASK-CHECKLIST.md) — reconnaissance, implementation, validation, and completion checklist for Codex tasks.
- [Using Codex](../agent/USING-CODEX.md) — how hierarchical repository guidance applies to Codex work.

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
