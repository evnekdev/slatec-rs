# Program-unit record specification

Schema ID: `slatec-rs/program-unit`  
Initial version: `1.0.0`

## Required fields

- `id`;
- `source_file_id`;
- `kind`: `program`, `subroutine`, `function`, `block_data`, or `implicit_main`;
- original and normalized name, with explicit absent-name state where permitted;
- start/end locators;
- selected-provider state;
- parser run ID;
- syntax coverage;
- semantic coverage;
- diagnostics and review item IDs.

## Syntax inventory

Record ordered statement IDs, declaration/executable boundary, labels, entries, statement functions, format statements, and preserved unknown statements. Each statement has exact physical and raw spans and a syntax kind or unknown rule ID.

## Scope inventory

Reference entity records for dummy arguments, locals, arrays, parameters, externals, intrinsics, common blocks, saved data, equivalence groups, labels, and implicit typing rules.

## Callable identities

A program unit owns zero or more routine records: its primary callable identity where applicable plus entries. `BLOCK DATA` has no ordinary FFI routine but participates in initialization/provider metadata.

## State and dependencies

Reference extracted dependency candidates, common/state nodes, block-data relationships, unresolved names, and conflicts. Dependency selection is not embedded as an unqualified list.

## Invariants

- program-unit spans do not overlap except through explicitly represented malformed/recovery regions;
- every statement belongs to exactly one program unit or file-level unassigned region;
- entries lie within the parent span;
- all entity locators resolve within the source member;
- selected duplicate program-unit names require explicit provider resolution;
- complete coverage requires no unparsed nontrivia spans.
