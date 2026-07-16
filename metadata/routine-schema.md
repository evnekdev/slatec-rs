# Routine metadata schema

## Purpose

This schema records routine-level facts without conflating source evidence, inference, and future `slatec-rs` proposals. TOML records use `[[routines]]`; Markdown pages mirror the same core fields in YAML front matter.

## Evidence rules

Every record has `source_access` and may attach `evidence_status` to nested facts. Allowed values are `verified`, `derived`, `inferred`, `unverified`, and `conflicting`, following the repository citation policy. Empty arrays mean “none found in the inspected source” only when the associated status is verified; otherwise they mean “not yet extracted”.

## Core fields

| Field | Type | Meaning |
|---|---|---|
| `id` | string | stable lowercase routine ID |
| `name` | string | canonical Fortran routine name |
| `source_file`, `source_url` | string | exact official source location |
| `source_access` | enum | whether content was opened or only located |
| `kind` | enum | `subroutine` or `function` |
| `precision` | string | historical type/precision family, not a Rust generic promise |
| `purpose` | string | concise paraphrase of official purpose |
| `gams_codes` | string array | verbatim historical classifications |
| `user_callable` | bool | based on prologue/table-of-contents evidence |
| `origin_package` | string | stable package/family ID or `individual`/`unknown` |
| `call_sequence` | string | documented Fortran call/reference form |
| `candidate_domain` | string | project proposal matching `metadata/domains.toml` |

## Arguments

Each `[[routines.arguments]]` has `name`, `fortran_type`, `intent`, and `evidence_status`. `intent` is one of `in`, `out`, `inout`, `work`, `ext`, or `unknown`. Intent is copied only when documented; otherwise it is marked inferred or unknown.

## Callbacks and workspaces

`callbacks` stores documented calling conventions or semantic roles. `work_arrays` stores minimum lengths, persistence, partitioning, or output significance. These fields are arrays because a routine may use several distinct callbacks or storage regions.

## Errors, dependencies, and provenance

- `documented_errors`: routine status/error behavior.
- `direct_dependencies`: direct routine names from the prologue or verified source inspection.
- `related_routines`: `SEE ALSO` or tightly related family members.
- `authors`, `revision_summary`: source provenance.
- `references`: bibliographic/source references, not copied prose.
- `abi_questions`: unresolved compiler, layout, callback, state, or linking issues.

## Generated catalogue requirements

Future generators should retain source checksums, prologue dialect, extraction timestamp, parser version, raw unrecognized prologue fields, and field-level locators. A parser must never silently turn missing information into `false`, `none`, or an inferred intent.

## Validation invariants

- IDs and names are unique.
- Every source URL is official and directly identifies the artifact used.
- Every candidate domain exists in `metadata/domains.toml` or is explicitly provisional.
- Argument names are unique within a routine.
- `user_callable = false` is not inferred merely from a routine being low-level.
- TOML and YAML representations must agree on identity, source, type, GAMS, package and candidate domain.
