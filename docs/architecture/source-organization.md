# Source organization and distribution

## Scope

This page documents how SLATEC source was organized for maintenance and distribution, how Netlib currently exposes it, and what that means for reproducible ingestion and later native linking.

## Historical distribution form

The 1993 guide describes distribution-tape source as ASCII text with exactly 80 characters per line. Subprograms in the combined source file were ordered alphabetically, using digits before letters. Each subprogram was preceded by a `*DECK name` line for source-maintenance tooling, and no other comments appeared between subprograms ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

Inside each source unit, fixed-form Fortran conventions applied. Statements began in column 7; column 6 held continuation markers; source and comments could not extend beyond column 72; and columns 73–80 were reserved for identification or sequence numbers. The guide required uppercase source outside comments and character constants and an `END` statement beginning in column 7 ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

These are source-management conventions in addition to Fortran 77 syntax. A modern compiler may accept looser formatting, but any parser intended to preserve historical structure must recognize the fixed columns and SLATEC sentinel comments.

## Current Netlib presentation

Netlib exposes:

- a complete source archive, `slatec_src.tgz`;
- individually retrievable routines in the main `src` subdirectory;
- a separate quick-check archive and individual checks;
- moved sublibraries for linear algebra and SLAP (`lin`), FISHPACK/FFTPACK (`fishfft`), FNLIB (`fnlib`) and PCHIP (`pchip`);
- documentation and installation tools;
- a table of contents, GAMS map, routine list, direct dependency list and direct-or-indirect dependency list ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

The index explains that several package subsets were removed from the main `slatec/src` view to make it more accessible. This is a presentation reorganization, not evidence that those packages ceased to be part of historical SLATEC. It also explicitly records uncertainty over which of two FNLIB copies is newer or less defective ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

## Unit of documentation and compilation

The guide's distribution model describes one alphabetized source stream, but Netlib commonly serves one routine per `.f` file. A `*DECK` marker identifies a logical source unit in the historical stream; a modern filesystem file is therefore not necessarily the original archival unit of compilation or maintenance ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`slatec-source-tree`](https://www.netlib.org/slatec/src/)).

**Inferred:** Automated tooling should distinguish:

- archival distribution file;
- `*DECK` logical unit;
- Fortran program unit (`SUBROUTINE`, `FUNCTION`, `BLOCK DATA`, and any `ENTRY` points);
- Netlib retrieval file;
- compiler object file produced by the chosen build.

Treating all five as identical would lose provenance and can produce incorrect symbol or dependency metadata.

## User-callable and subsidiary source

The prologue marker `C***SUBSIDIARY` identifies a routine intended mainly for calls from other library routines. User-callable routines omit this marker. The final table of contents independently classifies the two populations and marks subsidiary routines with an asterisk in its complete alphabetical list ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

Because the distinction is conventional rather than enforced by Fortran linkage, source ingestion should preserve it as metadata. A future wrapper generator should default to exposing only verified user-callable routines through safe APIs, while raw FFI coverage may include subsidiary symbols required for linking and testing.

## Incorporated package sources

The `LIBRARY` prologue field may list `SLATEC` alone or identify sublibraries/packages in parentheses. This field is evidence of declared package association, but exact provenance still requires comparing source text, revision histories and upstream package versions ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

Older package files may use earlier prologue dialects. For example, the Netlib QUADPACK `DQAG` source uses fields such as `DATE WRITTEN`, `REVISION DATE` and `CATEGORY NO.` and calls the older `XERROR` interface, rather than matching every field and spelling required by the final 1993 guide ([`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)). This is a concrete warning against assuming one grammar covers all SLATEC-related source.

## Implications for static-library construction

**Inferred:** Building one archive from every discovered `.f` file is not automatically correct. Risks include duplicate routine names across the main archive and moved package trees, multiple historical variants, machine-specific replacements and obsolete compatibility implementations.

**Project implications:**

1. Start from a checksum-pinned source snapshot and inventory every program unit.
2. Preserve original paths and declared package identities.
3. Detect duplicate linker symbols before compilation.
4. Record selected and excluded variants explicitly.
5. Compile fixed-form source with settings appropriate to its actual dialect rather than silently rewriting it.
6. Keep machine-specific and site-replaceable units in a separately auditable layer.
7. Validate the produced archive against direct and transitive dependency closures and representative quick checks.

## Open questions

- Does the complete source archive contain package copies that are absent from the current main source directory?
- Are any Netlib files multi-unit sources, or do all current retrieval files map one-to-one to `*DECK` units?
- Which files require nonstandard fixed-form line-length or argument-mismatch compiler options?
- Which package directories contain duplicate or divergent definitions of the same external symbol?
- Are `BLOCK DATA` units required to initialize any common blocks, and can static linkers discard them unless forced into the final link?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/)
- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)
