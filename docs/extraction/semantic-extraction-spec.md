# Semantic extraction specification

## Output philosophy

Semantic extraction emits claims with evidence, not unqualified values. Directly parsed facts may be `verified`; deterministic computations from verified syntax may be `derived`; interpretations such as argument intent are `inferred` unless explicitly documented and reviewed.

## Scope and symbol tables

Build one scope per program unit plus entry-point views. Record:

- implicit typing map;
- declared entities and attributes;
- dummy arguments in source order;
- local variables, parameters, arrays, common members, externals, intrinsics, statement functions, labels, and entries;
- function result identity and type;
- collisions and unresolved names.

Do not invent declarations to make parsing succeed. Implicitly typed entities are marked as such.

## Routine and argument extraction

For every callable program unit and `ENTRY`, extract:

- stable source-qualified identity;
- original and normalized name;
- kind;
- ordered dummy arguments;
- alternate-return positions;
- declared or implicit type, kind/length expression, rank, dimensions, and character length;
- whether an argument is declared `EXTERNAL` or used as a procedure;
- source locators for header and declarations;
- prologue argument documentation as separate claims.

Argument `intent` is not a FORTRAN 77 declaration. It may be inferred from reads/writes only as a provisional claim and cannot pass the safe-API gate without review or stronger evidence.

## Dependencies

Extract separate edge candidates for:

- direct `CALL` targets;
- function references;
- alternate-entry calls;
- procedure dummy/callback invocation;
- callback forwarding to another routine;
- intrinsic references;
- runtime/I/O dependencies;
- common-block state dependencies;
- block-data initialization relationships;
- error-handler and machine-provider hooks.

A name appearing in an expression is classified using scope information. Ambiguous array/function uses remain unresolved candidates. Prologue `ROUTINES CALLED` and Netlib dependency products are separate evidence kinds reconciled later.

## Statement functions

A statement function is identified only in the declaration region and when its left-hand name/arguments satisfy statement-function grammar. It becomes a local callable node, not an external dependency. Ambiguity with array assignment creates a review item.

## Callbacks

A dummy argument is a callback candidate when it is declared `EXTERNAL`, invoked as a function, used as a `CALL` target, or forwarded into another procedure position. Record observed invocation forms, argument counts, alternate returns, and return-value contexts. Callback ABI remains unresolved until compiler/profile evidence is available.

## State and initialization

Record:

- named and blank `COMMON` membership and order;
- `SAVE` state;
- initialized data from `DATA` and declaration initializers;
- `EQUIVALENCE` groups;
- `BLOCK DATA` providers;
- entry points sharing local/static state.

Storage layout claims require target/compiler evidence before ABI generation when character, logical, complex, equivalence, or common padding is involved.

## Character and complex ABI facts

Source extraction records language-level facts only. Hidden character lengths, placement of length arguments, complex return conventions, logical representation, symbol mangling, and alternate-return ABI are build-profile claims established by probes, not by the source parser.

## Error behavior

Recognize status arguments, calls to XERROR/XERMSG families, STOP paths, and documented error sections as separate facts. Do not infer Rust error semantics automatically.

## Coverage

Per source file and program unit report:

- bytes and logical statements covered;
- declarations fully/partially parsed;
- executable statements fully/partially parsed;
- prologue capture and interpretation coverage;
- unresolved names;
- unsupported syntax spans;
- facts blocked from raw FFI or safe API generation.

## Deterministic stable IDs

IDs are derived from selected source-set ID, member path, program-unit kind/name, and source start offset. Entry IDs include parent unit and entry name. IDs do not depend on extraction order.
