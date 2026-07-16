# Canonical source-corpus policy

**Policy version:** 1  
**Programme task:** E03  
**Decision date:** 2026-07-16  
**Policy status:** conditional, operational  

## Decision

The primary `slatec-rs` source baseline is the exact maintained Netlib archive registered as `slatec-source-archive`:

- artifact: `slatec_src.tgz`;
- SHA-256: `4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc`;
- compressed size: 1,768,291 bytes;
- regular members: 741;
- Fortran members selected for numerical-source consideration: 735.

This is a checksum-pinned **maintained Netlib snapshot**, not an assertion that the bytes are the untouched July 1993 Version 4.1 release. Its `changes` member records post-4.1 corrections in 1994, 1999, and 2023. The checksum and retrieval record therefore define the baseline more precisely than the historical version label.

## Corpus layers

The project must keep five physically and logically separate layers.

1. **Preserved original artifact** — the unmodified `slatec_src.tgz` response bytes and acquisition metadata.
2. **Selected canonical source** — deterministic extraction of the selected members from that exact archive.
3. **Normalization-only view** — reversible or semantics-preserving parser input generated from canonical source.
4. **Project patches** — separately stored, reviewed transformations applied after canonical extraction and normalization.
5. **Generated outputs** — manifests, parsed metadata, dependency graphs, object inventories, bindings, documentation, and build products.

No layer may overwrite the preceding layer.

## Selected source set

The default canonical numerical source set is all regular archive members matching `src/*.f`, using archive member bytes, with the following rules:

- current `src/sgeir.f` is selected;
- retained historical `src/sgeir.f.0` is preserved as evidence but excluded from the default numerical source set;
- corrected current `src/qk15w.f` and `src/dqk15w.f` are selected because they are the providers present in the pinned archive;
- archive support records such as `changes`, `MD5`, `.depend`, `index`, and `index.html` are preserved as evidence but excluded from compilation;
- no file from a live Netlib directory, relocated subset, upstream package, Linux overlay, legacy error directory, quick-check archive, or browser archive enters the default numerical source set merely because its name matches.

The source archive contains no detected duplicate declared program-unit names among its 735 selected Fortran files. This permits a deterministic initial provider map within the baseline, but does not resolve duplicates against external source sets.

## Relocated subsets and live trees

The live `slatec/src`, `slatec/lin`, `slatec/fishfft`, `slatec/fnlib`, and `slatec/pchip` directories are comparison and provenance sources under policy version 1. They do **not** override or supplement the pinned archive automatically.

A live or relocated file may replace an archive provider only after all of the following are recorded:

1. exact retrieval URL and timestamp;
2. raw SHA-256;
3. program-unit identity;
4. classified source difference;
5. evidence that the replacement is an intentional correction or required provider;
6. rights review;
7. an explicit policy-version update or approved patch/provider decision.

If a live file is byte-identical to the baseline provider, it may be recorded as another location of the same content, but it still does not become a second canonical provider.

## Corrections and historical variants

Corrections already contained in the pinned baseline are part of policy version 1. Earlier variants are retained as historical evidence and must not replace corrected providers by filename order or extraction order.

Later upstream or Netlib corrections are not incorporated silently. They require either:

- a new pinned baseline and policy-version change; or
- a project patch with upstream provenance, preimage hash, postimage hash, rationale, review, and tests.

Historical reproduction builds may select an earlier provider only through an explicit named profile. Such a profile is not the default canonical corpus.

## Duplicate providers

Provider selection is by explicit source ID and program-unit resolution record, never by search path, archive order, directory order, case folding, or last-write-wins behavior.

The default rules are:

- the pinned archive provider wins for each unique program unit present in the selected baseline;
- a same-named external provider is excluded and recorded as unresolved until compared;
- a build fails if two selected files declare the same program unit and no approved resolution exists;
- renamed, split, combined, `ENTRY`, and `BLOCK DATA` relationships require program-unit-level resolution, not filename matching;
- dependency products such as `tree`, `tree1`, and plus-dependencies never select providers.

## Machine-specific files

The baseline archive versions of `D1MACH`, `I1MACH`, and `R1MACH` are the canonical source records under policy version 1. This does not mean that their embedded historical machine selections are suitable for every build.

The Linux-overlay versions are excluded from the default corpus because they are verified executable alternatives and introduce LAPACK dependencies through `SLAMCH` and `DLAMCH`. They may be enabled only by an explicit machine-provider profile after compilation and numerical validation.

`FDUMP` and `XERHLT` remain explicit site-hook providers. Any replacement must be a named project patch or provider profile. A build must not choose a site hook implicitly.

## Tests and tooling

`slatec_chk.tgz` is the canonical candidate historical test corpus, pinned separately by SHA-256 `a095f74665e165fa1a4bd3f9ab6a4573135e21b1d002c05607eb9394e1c0f2ca`. It is not part of the numerical source provider set.

`slatecm.tgz` is excluded from numerical source and build inputs. It may be preserved as documentation-tooling evidence subject to its separate rights notice.

`slatec4linux.tgz` is an optional overlay candidate, not part of the default canonical source.

## Conditional blockers

This policy is operational for deterministic parsing of the pinned archive. It remains conditional for claims of globally authoritative or historically original SLATEC content because these comparisons are unresolved:

- archive versus current live `src` bytes;
- archive versus relocated subset bytes;
- archive versus standalone upstream package revisions;
- FNLIB versus `/fn` relative correctness;
- current live quick checks versus the pinned test archive;
- exact rights conclusions for numerical source and test artifacts.

These blockers do not prevent reproducible work against the pinned baseline. They prevent claims that the baseline is the only, earliest, latest, or legally redistributable corpus.
