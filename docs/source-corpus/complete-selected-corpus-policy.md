# Complete SLATEC-hosted selected-corpus policy

**Policy version:** 1
**Status:** operational for deterministic source-provider selection; conditional
for redistribution and historical-originality claims.

## Identity model

The checksum-pinned archive remains the immutable `main-src` snapshot. The
complete selected corpus is a separate deterministic snapshot derived from
that evidence, the checksum-recorded SLATEC-hosted directory indexes, source
hashes, the official `list` and Version 4.1 `toc`, and this policy. It never
rewrites the archive snapshot ID or its member hashes.

The selection profile is declared in
[`metadata/complete-corpus-selection.toml`](../../metadata/complete-corpus-selection.toml).
It selects one provider per normalized top-level program-unit identity:

1. use `main-src` when present;
2. otherwise use the exact SLATEC-hosted `lin`, `fishfft`, `fnlib`, or `pchip`
   provider;
3. select the explicitly reviewed `spfun` FNLIB records, whose source
   prologues identify FNLIB despite their absence from the current catalogues;
4. never substitute a standalone upstream package.

Machine constants, error services, and site hooks are selected infrastructure
units, not numerical public-API claims. The profile identifies them by
declared program-unit name rather than filename.

## Catalogue reconciliation

The Version 4.1 TOC fixed-column alphabetic index contains 902 current
user-callable identities and 539 subsidiary identities. The earlier 932
user-callable result came from two distinct errors:

- 26 real Version 4.1 user candidates are absent from the current `list` and
  every audited SLATEC-hosted provider; they are retained as
  `historical_or_obsolete`, not selected;
- four tokens (`A6B`, `C4B`, `INDICATES`, and `PRECEEDING`) are category or
  explanatory text that the former whitespace heuristic promoted to names.

Every reconciliation record has catalogue line locators and an explicit
disposition in `generated/selected-corpus/catalogue-classification.json`.
The 26 historical records are not filled from standalone BLAS or other
upstream packages.

## Supplemental hosted material

`spfun` contains 35 independently scanned FNLIB source program units with
FNLIB prologue markers. They are selected as `unexpected_numerical_source`
records and remain review-scoped because neither official catalogue enumerates
them. `subsid`, `slprep`, and `sladoc` are source-processing or documentation
tools and are excluded. The legacy `err` directory is byte-identical evidence
for already selected `main-src` error providers and is excluded as redundant.

The preserved `src/sgeir.f.0` differs from current `src/sgeir.f` in one
executable physical line. The current immutable `main-src` provider is
selected under the documented correction precedence; the old provider remains
an excluded historical variant with hashes and line-number-only difference
metadata.

## Output and limits

`slatec-corpus select-full-corpus --offline` writes compact factual metadata
under `generated/selected-corpus/`. Detailed catalogue and source evidence
stays ignored under `evidence/`. The tool rejects generated output above 4 MB
and fails an authoritative selection if an unresolved identity affects a
selected provider.

Successful selection proves reproducibility of this provider policy against
the cached evidence. It does not prove that the collection is historically
original, redistributable, ABI-safe, dependency-complete, or ready for native
compilation.
