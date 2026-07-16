# Fixed-form Fortran parser specification

## Input and source positions

The parser consumes a line-ending-normalized view plus a mapping to raw byte offsets. Every token and node records parser-view byte range, physical line/column range, and resolvable raw-member locator.

Default fixed-form columns:

- column 1: comment indicator or ordinary text;
- columns 1–5: statement label;
- column 6: continuation indicator;
- columns 7–72: statement field;
- columns 73 onward: sequence/trailing field retained as raw text.

Column policy is configurable only through a versioned parser profile. The selected profile is stored in each extraction run.

## Physical-line classification

Recognize:

- blank lines;
- comment lines beginning with `C`, `c`, `*`, or `!` in column 1;
- compiler directives as preserved unknown/directive lines;
- initial statement lines;
- continuation lines with nonblank, nonzero column 6;
- malformed lines with diagnostics.

Inline `!` comments may be recognized only outside literals and Hollerith payloads. They remain dialect-specific and are not assumed for strict FORTRAN 77 input.

## Lexical requirements

The lexer must preserve original spelling while producing case-insensitive keywords and identifiers. It must support:

- integer, real, double precision, complex, double complex, logical, and character literals;
- Hollerith constants such as `5HHELLO`, with exact length accounting;
- quoted strings with doubled quote escaping;
- dot operators and logical constants;
- statement labels and alternate-return labels;
- identifiers containing letters, digits, `$`, or `_` only under explicit dialect profiles;
- token adjacency arising from ignored spaces in fixed-form source.

## Logical statement construction

Continuation lines are joined only for parsing; all physical spans remain attached. Comments between continuation lines are retained but do not break the logical statement. The tool must detect unterminated literals, malformed continuation, and truncation at the configured statement-field boundary.

## Program units

Parse and record:

- `PROGRAM`;
- `SUBROUTINE`;
- typed and untyped `FUNCTION`, including `COMPLEX` and `DOUBLE COMPLEX` functions;
- `BLOCK DATA`, named or unnamed;
- implicit main programs where no opening statement exists;
- `ENTRY` points as callable identities sharing one implementation owner;
- `END`, typed end forms where present, and unterminated units.

Multiple program units in one file are mandatory. File name and primary unit name are independent facts.

## Declarations and state

Support at minimum:

- `IMPLICIT` and `IMPLICIT NONE` when encountered;
- scalar/array type declarations and dimensions;
- `DIMENSION`;
- `PARAMETER`;
- `EXTERNAL`, `INTRINSIC`;
- `COMMON`, including blank common and multiple blocks;
- `SAVE`, named and blanket;
- `DATA` including implied-do syntax where parseable;
- `EQUIVALENCE`;
- `CHARACTER*n`, `CHARACTER*(*)`, and per-entity lengths;
- `COMPLEX`, `DOUBLE COMPLEX`, and `LOGICAL` declarations;
- `ENTRY` argument lists and result behavior;
- `FORMAT` statements and labels.

Unsupported declaration extensions are emitted as lossless unknown statements with diagnostics.

## Executable constructs needed for extraction

The syntax tree must represent sufficiently to extract names and control-sensitive roles:

- `CALL`, including alternate returns;
- assignment and statement functions;
- function references in expressions;
- arithmetic, logical, relational, and concatenation expressions;
- `IF`, arithmetic IF, logical IF, block IF when present;
- `DO`, labeled DO, and implied-do contexts;
- assigned and computed `GOTO`;
- `ASSIGN`;
- `RETURN`, including alternate-return expressions;
- `STOP`, `PAUSE`;
- `READ`, `WRITE`, `PRINT`, `OPEN`, `CLOSE`, `INQUIRE`, `BACKSPACE`, `REWIND`, `ENDFILE`;
- procedure argument forwarding and calls through procedure dummy arguments.

The parser need not perform full control-flow analysis in E06 implementation scope, but syntax loss must not prevent later analysis.

## Ambiguity handling

FORTRAN fixed form permits ambiguities such as assignment versus statement function and array reference versus function reference. The parser records syntactic candidates and lets semantic extraction resolve using declarations and scope. When still ambiguous, it creates competing claims and a review item.

## Error recovery

Recovery synchronizes at logical statement boundaries and program-unit boundaries. No malformed region is discarded. Every unparsed span is attached to the containing source file and contributes to coverage status.

## Conformance outputs

For each fixture, tests assert:

- logical statements and physical spans;
- program-unit boundaries;
- exact declared names and argument order;
- raw round-trip reconstruction of tokenized regions;
- expected diagnostics;
- deterministic syntax-tree serialization.
