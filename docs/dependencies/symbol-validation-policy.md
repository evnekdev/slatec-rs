# Symbol validation policy

## Inventory

Compile each source file independently where the language and package permit, then collect object symbols before archive creation. Record:

- compiler, version, target, flags, dialect, integer/real defaults;
- source and object SHA-256;
- defined global, undefined, weak, local, and common symbols;
- symbol size/type when available;
- original tool output checksum;
- normalization rules and tool version.

## Symbol normalization

Keep raw symbols authoritative. A normalized candidate may remove documented compiler-specific case and underscore decoration only under a named ABI profile. Hidden character-length helpers, complex-return helpers, module/runtime prefixes, and platform decoration must not be guessed away.

## Duplicate symbols

A duplicate group contains every defining source, program unit, object, archive, raw spelling, normalized spelling, semantic relationship, selected provider, and policy. Duplicate selected definitions are fatal unless an explicit provider resolution exists.

Classifications are `byte_identical_source`, `executable_equivalent`, `corrected_variant`, `machine_variant`, `shim`, `unrelated_collision`, or `unknown`.

## COMMON symbols

Record each declaration's member sequence, declared types, dimensions, character lengths, equivalence effects, source span, and compiler-observed symbol. A successful link does not prove compatible COMMON layout. Cross-unit layout agreement and target ABI validation are separate gates.

## BLOCK DATA

Map initialized common blocks to block-data units and object/archive ownership. Validate whether ordinary archive extraction retains the initializer. When not guaranteed, require a documented force-link or component packaging rule and a runtime initialization test.

## Runtime and host leakage

Classify compiler/runtime symbols separately. Link validation must use controlled search paths and report symbols resolved from system libraries or unrelated host installations.
