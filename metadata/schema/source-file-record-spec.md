# Source-file record specification

Schema ID: `slatec-rs/source-file`  
Initial version: `1.0.0`

## Identity and provenance

Required fields:

- `id` derived from source-set and original member path;
- `artifact_id`, artifact SHA-256;
- original archive member path and normalized comparison path;
- member SHA-256 and byte length;
- selected/canonical state and selection-policy reference;
- rights record IDs;
- extraction run ID and raw content locator.

## Text observations

Record detected byte encoding, BOM state, line-ending counts, final-newline state, NUL presence, maximum physical line length, fixed-form profile, and any decoding uncertainty. Raw bytes remain authoritative.

## Parser view

Record parser-view hash, permitted transformations, source-map hash, parser configuration, and relationship to raw bytes. A parser view is not a patched source file.

## Contents

Reference program-unit IDs, file-level comment/prologue blocks, unassigned spans, diagnostics, conflicts, and review items. Collections use explicit E05 states.

## Coverage and gates

Record byte/token/statement/prologue coverage, unsupported syntax spans, inventory/dependency eligibility, and reasons a file is excluded from generated FFI.

## Duplicate and collision data

Record exact-path, case-fold, declared-unit, raw-hash, normalized-hash, and executable-token comparison groups. Provider selection remains a policy resolution, not an extraction side effect.

## Invariants

- member hash agrees with artifact manifest;
- raw locator resolves exactly the declared bytes;
- parser-view transformation is allowed by normalization policy;
- source map covers the complete parser view and maps to raw source;
- selected state agrees with canonical-corpus policy;
- a source file with unsafe or unresolved extraction provenance cannot pass generation gates.
