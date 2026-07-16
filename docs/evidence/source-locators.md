# Source locators and extraction provenance

## Locator purpose

A locator must identify the exact evidence used for one claim. A broad package index is insufficient for a declaration, coefficient, argument intent, or executable dependency when an exact file is available.

## Locator structure

```text
artifact_id
artifact_sha256
member_path
member_sha256
representation
span
field
context_sha256
```

### Artifact and member identity

- `artifact_id` references the candidate/canonical artifact register.
- `artifact_sha256` is mandatory for immutable downloaded evidence.
- `member_path` preserves the original archive path.
- `member_sha256` identifies the raw extracted member.
- live resources additionally record requested URL, final URL, retrieval timestamp, and response metadata.

### Representation

Allowed representations include:

- `raw_bytes`
- `decoded_text`
- `normalized_parser_view`
- `fortran_ast`
- `object_symbol_table`
- `linker_diagnostic`
- `runtime_probe`
- `manual_page`
- `index_record`

A locator into a normalized or parsed representation must also identify the raw member from which it was derived.

### Span forms

A locator uses the narrowest stable form available:

- byte range: `bytes:start-end`;
- line range: `lines:start-end` plus a context hash;
- prologue field: `prologue:PURPOSE`, `prologue:ARGUMENTS`, or similar;
- AST path: program unit, declaration, statement, expression, or call site;
- symbol-table entry: object, section, symbol, binding, and type;
- experiment step and assertion ID;
- archive member or index row.

Line numbers alone are not stable enough for long-lived generated evidence. Include a content/context hash.

## Extraction record

Every parser or analyzer run records:

```text
run_id
tool_name
tool_version
parser_name
parser_version
schema_version
started_at
completed_at
input_manifest_sha256
configuration_sha256
output_manifest_sha256
warnings[]
unparsed_regions[]
```

`parser_version` must change whenever parsing semantics can change, even if command-line compatibility remains intact.

## Field-level evidence

Each evidence-dependent field references one or more claim IDs. A claim may cite multiple locators when a value requires reconciliation, such as:

- declaration plus executable use;
- prologue dependency list plus parsed call site;
- archive change log plus old and new source members;
- source declaration plus compiler ABI probe;
- rights notice plus authoritative package statement.

## Locator integrity

Validation must ensure:

1. artifact and member IDs exist;
2. hashes use lowercase hexadecimal SHA-256;
3. paths do not escape the artifact extraction root;
4. spans resolve against the declared representation;
5. normalized representations link to raw preimages;
6. derived locators identify the producing run;
7. no field claims evidence from a later snapshot without an explicit cross-snapshot relationship.
