# Metadata schema programme

This directory contains specifications for machine validation of SLATEC evidence and generated metadata.

E05 defines the semantic model but does not implement validators. The recommended implementation is:

1. **JSON Schema Draft 2020-12** as the normative structural and enum schema after TOML/YAML parsing;
2. a project validator for cross-file references, hashes, state transitions, conflict gates, and deterministic ordering;
3. optional Taplo configuration for editor feedback on TOML source files.

JSON Schema is preferred over TOML-specific validation as the normative layer because the same semantic records may be represented in TOML, YAML, or JSON. CUE may be evaluated later for generation and policy constraints, but introducing it is not required for the first deterministic extraction toolchain.

See [`evidence-schema-spec.md`](evidence-schema-spec.md).
