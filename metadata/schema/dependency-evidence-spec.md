# Dependency evidence record specification

## Identity

Each `dependency_evidence` record has:

- `id`, `schema_version`, `snapshot_id`;
- `from_id`, `to_ref`, optional resolved `to_id`;
- `edge_kind`, `evidence_class`, `graph_layer`;
- `condition`, `link_relevant`, `closure_relevant`;
- `fact_status`, `review_state`, `conflict_id`;
- `locator_ids`, `extraction_run_id`, `build_fingerprint_id`;
- raw and normalized names;
- notes and structured diagnostics.

## Evidence classes

`prologue`, `parsed_call`, `parsed_function_reference`, `external_declaration`, `callback_invocation`, `callback_forwarding`, `common_declaration`, `state_analysis`, `tree1`, `tree`, `plus_dependencies`, `object_defined`, `object_undefined`, `object_common`, `linker_resolution`, `runtime_observation`, `manual_review`.

## Conditions

Conditions are `unconditional`, `source_predicate`, `build_predicate`, `callback_dependent`, `provider_dependent`, or `unknown`. Preserve raw predicate locators and avoid symbolic simplification unless performed by a versioned analyzer.

## Validation

- endpoint IDs resolve when `to_id` is present;
- verified records have resolving locators;
- object/linker records have exact build fingerprints;
- closure products cannot masquerade as direct parsed edges;
- callbacks and runtimes use their dedicated classifications;
- conflicting records reference a conflict group;
- selected link-required edges have no unresolved provider ambiguity.
