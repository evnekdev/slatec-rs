# E07 — Define Dependency Evidence Ingestion and Reconciliation

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`. Read E01–E06, Prompt 08 outputs, and the final audit.

## Objective

Define how dependency and shared-state evidence is gathered, preserved, compared, and reconciled. Do not implement the graph extractor.

## Evidence sources

Include:

- prologue `ROUTINES CALLED`;
- parsed `CALL` statements and function references;
- `EXTERNAL` declarations and callback arguments;
- `COMMON`, `BLOCK DATA`, `SAVE`, and `ENTRY`;
- aliases and renamed units;
- Netlib `tree`, `tree1`, and plus-dependency products;
- object defined, undefined, and common symbols;
- Fortran runtime dependencies;
- actual linker tests.

## Required design

Define node and edge types, evidence provenance, confidence and conflict states, conditional and callback edges, global-state edges, alternate providers, duplicate symbols, SCCs, source ownership, and native-component candidates.

State the minimum proof required before assigning a unit to a component, separating an archive, creating an independent `*-sys` crate, or claiming thread safety.

## Required outputs

```text
docs/dependencies/
    evidence-sources.md
    graph-model.md
    reconciliation-policy.md
    symbol-validation-policy.md
    linker-test-policy.md
    native-component-proof.md
metadata/schema/
    dependency-evidence-spec.md
    symbol-record-spec.md
    component-record-spec.md
```

## Completion criteria

The future implementation reports disagreements without silently choosing one source.
