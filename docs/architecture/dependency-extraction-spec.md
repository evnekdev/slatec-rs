# Dependency extraction and validation specification

## Scope

This specification defines how `slatec-rs` should extract, reconcile and validate dependency information for the SLATEC Common Mathematical Library. It covers source-level calls, prologue declarations, Netlib dependency products, callbacks, common blocks, external libraries, compiler-generated references and native linker behaviour.

The output is intended to support:

- reproducible routine catalogues;
- complete native link closures;
- identification of global state and callback requirements;
- evidence-based raw-crate boundaries;
- prevention of duplicate source and symbol ownership;
- generated human-readable dependency summaries.

This is a project specification, not a claim that the historical SLATEC distribution itself used this data model.

## Governing principles

1. Pin every input to a source snapshot and checksum.
2. Preserve each evidence source independently.
3. Distinguish program units, source files, external symbols and build-selected implementations.
4. Do not treat caller callbacks, intrinsics or compiler runtime symbols as ordinary SLATEC routine dependencies.
5. Never infer source ownership from GAMS classification alone.
6. Validate source-derived graphs against compiled objects and minimal links.
7. Preserve unresolved and conflicting evidence rather than silently choosing one interpretation.
8. Assign each source file and each exported implementation symbol to at most one owning native build unit.

The final SLATEC guide defines `ROUTINES CALLED` as SLATEC routines directly referenced or declared external and passed onward, excluding intrinsics and formal caller-supplied external procedures ([`slatec-guide`](https://www.netlib.org/slatec/guide)). Netlib separately advertises direct and transitive dependency trees, while package pages expose generated “plus dependencies” retrievals ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

## Input inventory

Every extraction run must begin with a manifest containing:

- canonical source archive URL;
- retrieval timestamp;
- archive checksum;
- unpacked file inventory and file checksums;
- selected moved package directories;
- machine/error compatibility sources included or excluded;
- parser name and version;
- compiler and flags used for validation;
- target triple and native integer/default-real model;
- Netlib `tree1`, `tree` and plus-dependency retrieval metadata when available.

A graph without a pinned source inventory is not reproducible.

## Node model

Canonical node kinds are:

| Kind | Meaning |
|---|---|
| `program_unit` | Fortran subroutine, function, main program, alternate entry or block-data unit |
| `source_file` | Physical source artifact containing one or more program units |
| `callback_interface` | Caller-supplied external procedure contract |
| `common_block` | Named or blank common storage |
| `block_data` | Initializer program unit for common storage |
| `external_symbol` | Linker-visible symbol not owned by the selected SLATEC source set |
| `intrinsic` | Fortran intrinsic or compiler-recognized intrinsic operation |
| `runtime` | Compiler, math, I/O or platform runtime dependency |
| `implementation_choice` | Build policy selecting one of multiple implementations |
| `native_component` | Proposed native archive/object group |
| `rust_crate` | Proposed Rust owning or consuming crate |

Program-unit identity must not be collapsed into source-file identity. One file can contain multiple program units, `ENTRY` points or a block-data unit; conversely, duplicate implementations of one symbol may occur in separate files.

## Edge model

Canonical edge kinds are:

| Edge | Source | Target | Meaning |
|---|---|---|---|
| `calls` | program unit | program unit/external symbol | Executable direct invocation |
| `references_function` | program unit | program unit/external symbol | Function used in an expression |
| `passes_external` | program unit | program unit | Named library routine passed onward |
| `requires_callback` | program unit | callback interface | Formal caller-supplied external |
| `uses_intrinsic` | program unit | intrinsic | Intrinsic operation |
| `reads_common` | program unit | common block | Reads shared state |
| `writes_common` | program unit | common block | Writes shared state |
| `declares_common` | program unit | common block | Declares layout; read/write may still be unresolved |
| `initialized_by` | common block | block-data unit | Initialization relationship |
| `requires_runtime` | program unit | runtime | Compiler/platform service |
| `alternate_entry` | program unit | program unit | `ENTRY` alias relationship |
| `alias_of` | external symbol | program unit | ABI or build alias |
| `precision_peer` | program unit | program unit | Documented type/precision analogue, not a call edge |
| `alternative_implementation` | implementation choice | program unit | Mutually exclusive implementation candidate |
| `defined_in` | program unit | source file | Physical source ownership |
| `observed_undefined` | object/source file | external symbol | Undefined object-file symbol |
| `observed_defined` | object/source file | external symbol | Defined object-file symbol |
| `owned_by` | source/symbol/component | native component/Rust crate | Candidate ownership mapping |

Only call-like edges participate in ordinary routine transitive closure. Precision peers, aliases, common-state edges and ownership relations are separate graph dimensions.

## Evidence classes and precedence

Every asserted node or edge carries one or more evidence records:

1. `source_ast` — parsed executable source or declarations;
2. `source_text` — conservative lexical extraction where an AST parser cannot parse legacy source;
3. `prologue` — `ROUTINES CALLED`, `COMMON BLOCKS`, callback description or related fields;
4. `netlib_tree1` — official direct-reference product;
5. `netlib_tree` — official direct-or-indirect product;
6. `netlib_plus_dependencies` — generated bundle inventory;
7. `object_defined` / `object_undefined` — compiler output inspected with `nm`, `objdump`, `dumpbin` or equivalent;
8. `link_test` — successful or failed link of a controlled root and selected closure;
9. `runtime_test` — execution of a quick check or smoke test;
10. `manual_review` — documented expert resolution.

Executable source is the primary authority for source-level calls in the selected snapshot. Object symbols are the primary authority for what that compiler emitted. A successful link is the primary authority for sufficiency under the exact tested build configuration. These authorities answer different questions and must not overwrite each other.

## Source parsing

### Fixed-form normalization

The parser must retain original bytes and produce a normalized analysis view that handles:

- fixed-form columns and continuation rules;
- comment cards and tab-form extensions;
- Hollerith constants;
- statement labels;
- case-insensitive identifiers;
- multiple program units per file;
- `INCLUDE` and preprocessing only when actually used;
- old-style character constants and arithmetic `IF`;
- `ENTRY`, `EXTERNAL`, `INTRINSIC`, `COMMON`, `EQUIVALENCE`, `SAVE` and `BLOCK DATA`.

Normalization must never be used as a replacement source artifact.

### Direct calls

Extract `CALL name(...)` as `calls` unless `name` is a formal external callback or unresolved procedure variable. Function references require symbol-table analysis because `F(X)` can be an array reference, statement function, intrinsic, external function or library function.

Resolution order for an apparent function reference:

1. local statement function;
2. local array/variable declaration;
3. explicit `INTRINSIC`;
4. explicit/formal `EXTERNAL` callback;
5. program unit defined in selected source set;
6. known compiler intrinsic without explicit declaration;
7. known external library/runtime symbol;
8. unresolved reference.

### Externals

A formal external procedure supplied by the application becomes a `callback_interface`, not an unresolved library reference. A named SLATEC routine declared `EXTERNAL` and passed to another routine becomes `passes_external` plus the relevant source-level symbol relationship.

### Intrinsics

Intrinsics are recorded for audit and portability analysis but excluded from native routine closure. Compiler lowering may emit runtime symbols such as power, complex arithmetic or I/O helpers; those become runtime edges only after object inspection.

### Conditional references

Fortran 77 calls are usually statically visible even when reached only under an input flag. Record the edge with `condition_text` or a normalized condition when derivable. Conditionality affects minimal runtime paths but not conservative static link closure.

## Prologue extraction

Parse both final and legacy prologue dialects. Preserve:

- raw `ROUTINES CALLED` tokens;
- `COMMON BLOCKS` entries;
- callbacks documented in argument descriptions;
- type/precision peers;
- `SEE ALSO` relationships;
- unrecognized fields and original locators.

Prologue dependencies are evidence, not automatically canonical truth. Compare them with executable references and report:

- declared but not observed;
- observed but not declared;
- ambiguous names;
- stale package or renamed references.

## Netlib dependency products

### `tree1`

Ingest as direct-reference evidence only after recording format, retrieval date and source snapshot assumptions. Do not assume it covers moved package subsets or compatibility sources until verified.

### `tree`

Ingest as a transitive-closure assertion. Recompute closure from the project’s resolved direct graph and compare root-by-root.

### Plus dependencies

For each selected root record:

- root path;
- exact generated URL;
- retrieval timestamp;
- returned files and checksums;
- duplicate filenames/symbols;
- files present but not reachable in the project graph;
- project-reachable files missing from the bundle.

A plus-dependency bundle is an independent cross-check, not a build manifest.

## Precision pairs and generated variants

Precision/type analogues are non-call relationships. Record:

- documented `TYPE` family;
- naming transformation when regular;
- source similarity or generation evidence;
- missing peers;
- differences in dependencies, workspaces or error behaviour.

Never generate a dependency edge from a single-precision routine to its double-precision peer merely because the names correspond.

## Alternate entries and aliases

For every `ENTRY` statement:

- create a distinct program-unit/symbol record;
- link it to the containing implementation with `alternate_entry`;
- record its distinct argument list;
- treat all entry symbols as defined by the same object/source owner;
- prevent separate crate ownership of entry points from one source object.

Compiler aliases, wrapper shims and name-mangling aliases must be represented separately from Fortran `ENTRY`.

## Common blocks, saved state and initialization

Parse each common declaration into an ordered layout record containing variable names, declared types, dimensions and source location. Compare every declaration of the same common block for compatible total layout and ordering.

Record whether each program unit reads or writes common members when data-flow analysis can establish it; otherwise use `declares_common` with unresolved access mode.

`BLOCK DATA` must be linked through `initialized_by`. Validation must test whether ordinary archive extraction retains the initializer; if not, the build strategy must force inclusion or replace initialization through a verified mechanism.

Mutable `SAVE` variables should be recorded as program-unit-local global state even though they are not common blocks.

## Missing, conditional and external routines

Every unresolved symbol must be classified as one of:

- expected caller callback;
- Fortran intrinsic;
- compiler/runtime symbol;
- system math or I/O symbol;
- external BLAS/package dependency supplied by build policy;
- optional/site hook;
- source missing from selected snapshot;
- name-resolution ambiguity;
- probable parser defect;
- genuinely unknown.

No unresolved item may disappear merely because a linker happened to find a symbol in the host environment.

## Duplicate symbols

Detect duplicates at three levels:

1. same Fortran program-unit name in multiple source files;
2. same mangled external symbol in multiple objects/archives;
3. same semantic routine supplied both internally and by an external implementation.

Each duplicate group must specify one resolution policy:

- select exactly one source implementation;
- rename/wrap one implementation;
- delegate to an external provider;
- fail the build as ambiguous.

Archive order must never be the intentional selection mechanism.

## Cycles and strongly connected components

Compute SCCs over resolved call-like edges. For each SCC report:

- member program units;
- member source files;
- packages/domains represented;
- common-state nodes touched;
- incoming/outgoing edges;
- whether the cycle is source-real, callback-mediated or introduced by aliases;
- recommended indivisible native component.

A cycle can constrain native archive grouping without requiring the same public Rust module or safe crate.

## Canonical graph products

The complete extractor should emit deterministic, sorted files such as:

```text
metadata/dependencies/nodes.toml
metadata/dependencies/direct-edges.toml
metadata/dependencies/transitive-closure.toml
metadata/dependencies/unresolved.toml
metadata/dependencies/sccs.toml
metadata/dependencies/duplicates.toml
metadata/dependencies/ownership.toml
metadata/dependencies/build-validation.toml
docs/generated/dependencies/index.md
docs/generated/dependencies/<root>.md
```

Large closures may later use JSON Lines, SQLite or a compact binary cache internally, but TOML remains the reviewable canonical export for the initial project.

## Worked example: `DQAG`

The official `DQAG` source documents and executes calls to `DQAGE` and `XERROR`. It accepts caller callback `F`, which is passed to `DQAGE`; `F` is therefore a callback requirement, not a missing SLATEC routine. `DQAG` also partitions its `WORK` array into four regions before forwarding them to `DQAGE` ([official `DQAG` source](https://www.netlib.org/quadpack/dqag.f)).

Verified direct records for the root are:

```text
DQAG --calls--> DQAGE
DQAG --calls--> XERROR
DQAG --requires_callback--> callback:f64_to_f64
DQAG --passes_external--> F (callback instance passed to DQAGE)
```

`DQAG` selects a Gauss–Kronrod pair through `KEY`. The official local-rule sources `DQK15`, `DQK21`, `DQK31`, `DQK41`, `DQK51` and `DQK61` each document a direct dependency on `D1MACH` and a callback requirement for the integrand. Their inclusion under `DQAGE` must be verified from the opened `DQAGE` source or source parser before asserting those edges as verified. The source URLs exist in the canonical QUADPACK collection; an inaccessible or unparsed `DQAGE` file must produce an explicit verification gap, not a guessed edge.

The conservative candidate native closure for validation is therefore:

```text
DQAG
DQAGE
DQK15 DQK21 DQK31 DQK41 DQK51 DQK61   # conditional candidates pending DQAGE verification
DQPSRT                                  # candidate adaptive ordering helper pending source verification
D1MACH                                  # verified dependency of the local rules
XERROR and its selected error closure   # verified direct dependency from DQAG
```

Validation procedure:

1. compile each candidate source separately;
2. inspect defined and undefined symbols;
3. confirm which local rules and ordering helpers are undefined from `DQAGE`;
4. link `DQAG` with only the computed closure and selected error/machine support;
5. run a finite-interval smoke test for every `KEY` class;
6. compare the computed closure with Netlib plus-dependencies and `tree1/tree` when ingested;
7. record all discrepancies.

This example deliberately distinguishes verified edges from conditional candidates.

## Validation stages

### Static consistency

- every edge endpoint resolves or has an unresolved classification;
- every program unit has exactly one selected source implementation;
- every source file has at most one owning native component;
- every candidate domain/package ID exists;
- direct closure recomputation is deterministic;
- transitive closure excludes non-call edge classes;
- SCC membership is stable under deterministic ordering.

### Compilation

Compile one object per source file where possible. Record compiler diagnostics, defined/undefined symbols and name mangling. Repeat with intended optimization and default-kind flags because emitted runtime references can differ.

### Link tests

For representative roots:

- link exactly the computed source/object closure;
- link against each supported external BLAS policy;
- test archive order independence or use linker grouping explicitly;
- force/verify block-data retention;
- test deliberate omission of each dependency to confirm expected failures;
- reject accidental resolution from unrelated system libraries.

### Runtime tests

Run quick checks or minimal numerical smoke tests. A successful link does not prove correct common-block layout, initialization, callback ABI or algorithm behaviour.

## ChatGPT versus Codex recommendation

### Suitable for ChatGPT

- refining schemas and evidence terminology;
- reviewing representative prologues and edge discrepancies;
- explaining SCCs, ownership conflicts and build-policy options;
- drafting human-readable summaries;
- investigating a small number of ambiguous routines with authoritative sources.

### Requires Codex High

The complete extraction should be performed with Codex High because it requires repository-wide parsing, scripts, deterministic file generation, compilation across many sources, object-symbol inspection, repeated linker experiments, validation reports and regeneration after corrections.

Codex should implement tests before relying on generated metadata. ChatGPT should review the resulting schemas, representative records and unresolved conflicts rather than manually transcribing the complete graph.

## Unresolved questions

- What source snapshot generated Netlib `tree1` and `tree`?
- Do those products include relocated package directories?
- Which legacy dialects require lexical fallback rather than AST parsing?
- Which compiler/runtime symbols vary by optimization level?
- Which duplicate historical error or machine-support symbols exist?
- Can a single native core archive safely serve several raw Rust crates?
- Which callback and common-state edges require runtime serialization?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`netlib-plus-dependencies`](../sources/source-register.md#core-slatec-sources)
- [Official `DQAG` source](https://www.netlib.org/quadpack/dqag.f)
- [Official `DQK15` source](https://www.netlib.org/quadpack/dqk15.f)
- [Official `DQK21` source](https://www.netlib.org/quadpack/dqk21.f)
