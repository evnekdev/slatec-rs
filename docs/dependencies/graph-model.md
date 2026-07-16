# Dependency graph model

## Layered graph

The model stores evidence claims first and materializes named graph views. It does not maintain one universal graph.

Required views:

1. `documented-direct` — prologue and `tree1` claims;
2. `parsed-direct` — executable source references;
3. `callback-interface` — required, invoked, and forwarded callbacks;
4. `shared-state` — common blocks, saved state, block-data initialization, equivalence overlap;
5. `object-link` — defined/undefined/common symbol relations;
6. `linker-observed` — providers actually selected by experiments;
7. `selected-native` — policy-selected providers and component ownership;
8. named transitive closures derived from explicitly listed edge kinds.

## Node kinds

- `program_unit`
- `entry_point`
- `statement_function`
- `callback_interface`
- `common_block`
- `saved_state`
- `block_data`
- `source_file`
- `object_file`
- `archive_member`
- `native_archive`
- `external_provider`
- `runtime_service`
- `symbol`
- `provider_variant`
- `native_component`

Program units and entry points have distinct callable IDs but share source and object ownership.

## Edge kinds

### Procedure/reference

- `calls`
- `references_function`
- `declares_external`
- `passes_external`
- `requires_callback`
- `invokes_callback`
- `forwards_callback`
- `alternate_entry_of`
- `alias_of`

### State

- `reads_common`
- `writes_common`
- `declares_common`
- `initializes_common`
- `has_saved_state`
- `equivalences_storage`
- `initialized_by_block_data`

### Build and symbols

- `defines_symbol`
- `requires_symbol`
- `provides_symbol`
- `selected_provider`
- `compiled_from`
- `member_of_archive`
- `requires_runtime`
- `requires_component`

### Reconciliation/documentation

- `documented_dependency`
- `reported_direct_by_tree1`
- `reported_closure_by_tree`
- `included_by_plus_dependencies`
- `alternate_implementation`
- `corrected_version_of`
- `possible_duplicate_of`

## Edge attributes

Every edge has an ID, endpoints, kind, graph layer, condition, link relevance, closure relevance, fact status, review state, evidence IDs, source snapshot, and optional conflict group. Conditional edges preserve predicates or a raw condition locator; unknown predicates remain explicit.

## SCCs and closures

SCCs are computed only from a named edge profile. The default native SCC profile includes selected direct executable calls and link-required provider edges, but excludes documentary, precision-peer, ownership, and ordinary state edges. Callback and state SCCs are separate analyses.

Closures carry the direct-graph semantic hash, selected provider policy, edge profile, root, maximum depth, and sorted reachable IDs.
