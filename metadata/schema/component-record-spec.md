# Native component record specification

## Component identity

Each component has:

- stable ID, kind, status and policy version;
- selected source files, program units, objects and archive members;
- provided symbols and required external symbols;
- component dependencies and external providers;
- SCC IDs and reason codes;
- callback interfaces and shared-state resources;
- block-data retention policy;
- supported build fingerprints;
- rights/release gate references;
- proof record and review state.

## Proof record

The proof records the source-manifest hash, selected-provider hash, direct-graph hash, SCC-analysis hash, unresolved-reference inventory, duplicate-resolution inventory, symbol-manifest hash, linker-test IDs, runtime-test IDs, reviewer decisions, and invalidation inputs.

## Invariants

- every selected source has exactly one native owner;
- every selected external definition has exactly one provider;
- no object is present in multiple selected components;
- component dependencies are acyclic after native SCC collapse;
- all cross-component required symbols map to declared dependencies/providers;
- open link, state, ownership, ABI, or rights blockers prevent `approved`;
- a component change invalidates dependent linker and FFI proofs.
