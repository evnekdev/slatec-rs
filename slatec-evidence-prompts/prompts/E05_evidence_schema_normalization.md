# E05 — Normalize Evidence and Metadata Vocabularies

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`. Read the routine, package, domain, dependency, and source metadata plus E01–E04.

## Objective

Define one coherent evidence model while keeping distinct dimensions separate:

- fact verification status;
- source retrieval status;
- provenance confidence;
- relationship confidence;
- documentation coverage;
- unresolved conflict state.

## Required work

Define:

- canonical vocabulary names and values;
- exact meaning and state transitions;
- which fields may be inferred;
- which evidence can drive generated FFI;
- human-review requirements;
- conflict representation;
- field-level source locators;
- parser versions and extraction timestamps;
- schema versioning and migration;
- empty-array, unknown, absent, and not-applicable semantics.

Recommend a machine-validation system such as JSON Schema, CUE, Taplo, or a justified alternative. Do not implement it here.

## Required outputs

```text
docs/evidence/
    README.md
    evidence-model.md
    status-vocabularies.md
    source-locators.md
    conflict-representation.md
    schema-versioning.md
metadata/schema/
    README.md
    evidence-schema-spec.md
```

## Completion criteria

Every vocabulary has one purpose, dimensions cannot be accidentally interchanged, insufficient evidence cannot drive bindings, and migration is defined before bulk extraction.
