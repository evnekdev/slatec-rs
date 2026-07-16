# Prologue parser specification

## Principle

Prologues are documentary evidence, not executable truth. The parser preserves their exact source spans, recognizes known fields, records dialect and confidence, and retains every unrecognized line.

## Dialects

At minimum support:

- final SLATEC `C***` prologues with `DECK`, `PURPOSE`, `LIBRARY`, `CATEGORY`, `TYPE`, `KEYWORDS`, `AUTHOR`, `DESCRIPTION`, `REFERENCES`, `ROUTINES CALLED`, and `REVISION HISTORY` sections;
- older SLATEC/XERROR-era forms;
- QUADPACK headers and revision comments;
- LINPACK and BLAS compact headers;
- FFTPACK and FISHPACK comments;
- PCHIP prologues;
- DASSL and ODE-family headers;
- AMOS/special-function headers;
- SLAP package-specific comments;
- machine-constant and error-system files with nonstandard documentation.

Dialect recognition is evidence with its own confidence. Unknown dialects remain parseable as raw comment blocks.

## Block detection

A candidate prologue is a contiguous or dialect-linked comment region associated with a program unit. Detection must not consume unrelated copyright notices, archive-level banners, disabled source, or comments belonging to a previous unit without recording the ambiguity.

## Field representation

Each recognized field records:

- canonical field key;
- original heading text;
- raw content lines;
- minimally joined display text;
- exact physical and raw-byte spans;
- dialect parser and rule ID;
- extraction fact status;
- parse diagnostics;
- zero or more normalized candidates.

The raw value is always authoritative evidence. Normalized values are derived claims.

## Field-specific rules

- `DECK` and declared program-unit identity are compared but never silently unified.
- `TYPE` is retained verbatim; historical precision labels do not imply Rust genericity.
- `ROUTINES CALLED` produces documentation dependency claims, not selected source-call edges.
- `AUTHOR` preserves names and institutions without attempting automatic legal ownership conclusions.
- `REVISION HISTORY` preserves entries and dates as text; dates are normalized only when unambiguous.
- `CATEGORY`/GAMS codes preserve original spelling and order, with separately normalized identifiers.
- `DESCRIPTION` and `PURPOSE` are not copied into generated API prose without attribution and coverage controls.
- `SEE ALSO` and references form documentary relationships, not link requirements.

## Unrecognized content

All unrecognized headings and lines are stored in ordered raw fields. Coverage distinguishes complete block capture from complete semantic recognition. A parser may report `complete` capture and `partial` field interpretation.

## Conflicts

Differences among prologue, declaration, executable source, indexes, and external manuals create separate claims. Examples:

- prologue routine name differs from declaration;
- documented argument list differs from source;
- `ROUTINES CALLED` omits an executable call;
- revision history claims a correction not reflected in the selected provider.

These conflicts are never resolved by source precedence alone unless policy explicitly defines the scoped fact.

## Tests

Fixture tests assert exact block boundaries, recognized headings, raw preservation, normalized candidates, locators, dialect classification, and expected conflicts against syntax-derived facts.
