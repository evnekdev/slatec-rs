# Fixture corpus

## Purpose

The fixture corpus is a tracked, minimal, rights-reviewed test corpus. Fixtures should normally be short project-authored reproductions of syntax patterns. Verbatim upstream excerpts may be included only when E04 permits it and provenance is explicit.

## Fixture classes

| Class | Required cases |
|---|---|
| Physical fixed form | comments, labels, continuation, columns 73+, tabs under explicit profile, malformed continuation |
| Literals | quote escaping, Hollerith payloads containing commas/parentheses, complex constants, logical literals |
| Program units | subroutine, typed function, program, named/unnamed block data, implicit main, multiple units per file |
| Arguments | arrays, adjustable dimensions, assumed character length, logical, complex, procedure arguments, alternate returns |
| Declarations | implicit typing, external/intrinsic, common, save, data, equivalence, parameter, dimension |
| Calls | direct CALL, function expression, nested reference, intrinsic, callback invocation, callback forwarding |
| Ambiguity | array versus function, assignment versus statement function, external versus intrinsic, entry-specific argument lists |
| Control syntax | arithmetic IF, assigned/computed GOTO, labeled DO, alternate RETURN |
| Prologues | modern SLATEC, legacy SLATEC, QUADPACK, BLAS, LINPACK, FFTPACK, PCHIP, DASSL, AMOS, SLAP |
| Providers | machine constants, XERMSG/XERROR hooks, block data initialization, duplicate provider candidates |
| ABI risks | character args, character functions if encountered, complex functions, logical args, common/equivalence |
| Failure | truncated file, invalid label, unterminated literal, unsupported extension, conflicting header/declaration |

## Named integration fixtures

Create at least these scenario fixtures:

- `modern_slatec_subroutine`;
- `legacy_xerror_function`;
- `quadpack_callback_driver`;
- `blas_compact_header`;
- `linpack_statement_function_or_array`;
- `fftpack_multiple_helpers`;
- `pchip_revision_and_xermsg`;
- `dassl_external_callbacks`;
- `amos_complex_function`;
- `slap_common_and_workspace`;
- `machine_constants_alternatives`;
- `block_data_common_initializer`;
- `multiple_program_units_one_file`;
- `character_hidden_length_risk`;
- `alternate_returns`;
- `unparsed_span_preservation`.

## Fixture structure

Each fixture directory contains:

```text
source.f
expected.syntax.json
expected.semantic.json
expected.prologue.json
expected.diagnostics.json
fixture.toml
```

`fixture.toml` records authorship, rights status, dialect profile, expected parser/schema versions, and whether the source is synthetic, transformed, or verbatim.

## Golden-file rules

Golden files use canonical JSON. Reviewers approve intentional updates. A bulk rewrite must include a semantic diff report and parser-version change. Timestamps and platform paths are forbidden in golden output.

## Corpus-derived regression tests

The full canonical corpus may be used locally as an integration input without committing extracted source. Tests compare manifests, counts, diagnostics classes, and semantic hashes. They must not require redistribution of the archive.

## Fuzz and property testing

Generate fixed-form whitespace, continuation, case, label, and comment variants around accepted fixtures. Required properties include no panic, bounded resource use, deterministic output, locator resolution, and preservation of unparsed bytes.
