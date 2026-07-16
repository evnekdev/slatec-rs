# Routine record specification

Schema ID: `slatec-rs/routine`  
Initial version: `1.0.0`

## Identity

Required fields:

- `id`: stable source-qualified routine or entry ID;
- `program_unit_id`;
- `callable_kind`: `subroutine`, `function`, `entry`, or `program` where catalogued;
- `name_original`, `name_normalized`;
- `selected_provider`;
- `source_file_id`;
- `header_locator_id`.

## Signature

- ordered `arguments` with explicit `position`;
- `alternate_return_positions`;
- function result record when applicable;
- language-level type, rank, dimensions, and character length claims;
- declaration and prologue signature claims kept separately;
- signature conflict IDs.

Each argument records stable ID, original/normalized name, type claims, declaration locators, procedure-role claims, documented role, inferred intent claims, collection states for dimensions/workspace facts, review state, and FFI gate result.

## Documentation and classification

Fields may include purpose, library/family, GAMS codes, authors, references, revision entries, related routines, user-callable classification, and domain proposals. Every field is represented by claims and locators; project proposals use `proposal` statement kind.

## Dependencies and state

Reference IDs for direct call/function/callback candidates, common blocks, saved state, data initialization, error behavior, machine hooks, unresolved references, and dependency conflicts.

## ABI section

Language-level source facts are separated from target/compiler ABI claims. ABI claims require a build profile and may include symbol spelling, character lengths, complex return, logical representation, calling convention, alternate returns, and common layout.

## Generation gates

A routine contains inventory, dependency, raw-FFI, and safe-API gate results. Gate results reference blocking claims, conflicts, review items, and probe requirements.

## Invariants

- argument positions are unique and contiguous excluding explicit alternate-return syntax positions;
- every verified field has a resolving locator;
- an entry shares implementation owner with its parent program unit;
- no open signature/provider conflict may pass raw FFI;
- unknown safety facts block safe API;
- generated Rust names are projections, not routine identity.
