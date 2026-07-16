# SLATEC evidence model

This directory defines the normalized evidence vocabulary used by source-corpus, routine, dependency, package, domain, rights, build, and generated-output records.

The model deliberately keeps independent dimensions separate. A field can be directly verified from a retrieved source while its package provenance is uncertain, its relationship to another provider is unresolved, and redistribution remains blocked.

## Files

- [`evidence-model.md`](evidence-model.md) — common record and field-evidence model.
- [`status-vocabularies.md`](status-vocabularies.md) — canonical enums, meanings, and permitted transitions.
- [`source-locators.md`](source-locators.md) — artifact, path, span, parser, and extraction provenance.
- [`conflict-representation.md`](conflict-representation.md) — competing claims and provider conflicts.
- [`schema-versioning.md`](schema-versioning.md) — schema evolution, migrations, and compatibility.
- [`../../metadata/schema/evidence-schema-spec.md`](../../metadata/schema/evidence-schema-spec.md) — machine-validation specification.

## Governing rules

1. A status value is valid only in its declared dimension.
2. Missing, unknown, absent, empty, and not applicable are distinct states.
3. Inference never becomes verification without new evidence.
4. Generated FFI may use only fields that pass the field-specific evidence gate.
5. Conflicting evidence is preserved as multiple claims plus an explicit resolution record.
6. Every derived value identifies its inputs, algorithm, parser version, and extraction run.
7. Rights status is a packaging gate, not a numerical or factual confidence score.
