# Source reconciliation method

**Task:** E02 — archive, live-tree and relocated-subset reconciliation  
**Investigation date:** 2026-07-16  
**Canonical corpus selected:** no

## Evidence boundary

This task compares all overlapping source sets registered by E01. It does not select a duplicate provider. Netlib's live indexes and selected individual source files were inspectable, but binary archive retrieval was unavailable in the execution container because `www.netlib.org` could not be resolved there. Consequently, no archive checksum, byte-identity claim, archive-member inventory, or archive-to-live diff is reported.

Relationship statuses have the following meaning:

- `verified-identical`: raw bytes were compared and matched. No relationship reached this status in this pass.
- `verified-different`: inspected primary evidence proves that two artifacts differ in relevant content or role.
- `likely-identical`: strong direct evidence suggests identity, but raw bytes were not compared.
- `possible-duplicate`: names or distribution descriptions overlap, but identity is not established.
- `alternate-implementation`: artifacts intentionally provide site-, platform-, precision-, or package-specific alternatives.
- `unresolved`: evidence is insufficient for a narrower classification.

## Required reproducible local procedure

### 1. Acquisition

For each immutable download, save:

- requested and final URL;
- UTC retrieval timestamp;
- HTTP status, redirects, content type and byte length;
- unmodified response bytes;
- SHA-256 checksum.

Primary downloads are `slatec_src.tgz`, `slatec_chk.tgz`, `slatec4linux.tgz`, `slatecm.tgz`, SLAP archives, and any upstream package archives used for comparison.

For live Netlib directories, preserve the index response and retrieve every advertised file. Record per-file SHA-256 and the index retrieval timestamp.

### 2. Safe extraction and path normalization

Reject archive members with absolute paths, `..` traversal, device files or links escaping the extraction root. Preserve original member names. Create a comparison key by replacing backslashes with `/`, removing a leading `./`, and applying ASCII lowercase only in a separate `casefold_path` field.

### 3. File-level hashes

For every regular file calculate:

1. raw SHA-256;
2. text-normalized SHA-256 after CRLF/CR to LF only;
3. fixed-form Fortran normalized SHA-256 after line-ending normalization and removal of trailing horizontal whitespace;
4. executable-token SHA-256 after parsing Fortran and excluding comments and prologue text.

Do not use normalization hashes as substitutes for preserving raw bytes.

### 4. Program-unit extraction

Parse fixed-form Fortran into program units and record:

- kind: `PROGRAM`, `SUBROUTINE`, `FUNCTION`, `BLOCK DATA`, or alternate `ENTRY`;
- declared name and casefolded name;
- file, starting and ending line;
- argument list;
- all additional program units in the same file;
- revision-history markers and `*DECK`/prologue identity;
- `COMMON`, `SAVE`, `DATA`, `EXTERNAL`, and `ENTRY` declarations.

A file-name match is not sufficient evidence of program-unit identity.

### 5. Difference classification

Classify each non-identical pair in this order:

- line-ending or trailing-whitespace only;
- comments only;
- prologue/documentation only;
- declaration-only;
- machine-constant selection;
- error-system adaptation;
- precision conversion;
- executable-code change;
- renamed or split/combined program units;
- unrelated same-name file.

Store a unified diff and a machine-readable summary. Never collapse conflicting providers by last-write-wins.

### 6. Pairing strategy

Pair in descending confidence:

1. identical relative path;
2. case-insensitive path match;
3. same primary program-unit name;
4. matching `*DECK` name;
5. same routine name advertised by Netlib indexes;
6. manually reviewed suspected rename.

### 7. Minimum validation commands

A local implementation should produce deterministic output equivalent to:

```text
sha256sum <download>
tar -tzf <archive>
tar -xzf <archive> --directory <safe-root>
```

The comparison tool must record its version and configuration. A Fortran-aware parser is required before making comment-only or executable-code-only claims.

## Evidence obtained in this pass

The Netlib root index directly states that `lin`, `fishfft`, `fnlib`, and `pchip` were removed from `slatec/src`; this verifies relocation as a distribution-organization fact, not byte identity. It also states that `/fn` is another version and that Netlib did not know which copy was newer or more correct.

The root index identifies five machine-specific program units in the source distribution: `D1MACH`, `I1MACH`, `R1MACH`, `FDUMP`, and `XERHLT`. The first three require site-specific constant selection; the latter two are site-replaceable error/traceback hooks. These are alternate implementations or configurations by design.

Selected source inspection confirmed SLATEC-specific prologue and error-system integration in `slatec/pchip/pchim.f` and `slatec/fnlib/besi0.f`, including `*DECK`, `C***LIBRARY SLATEC`, revision histories, and calls to `XERMSG`. This is evidence that comparisons with upstream copies must distinguish prologue/error adaptation from numerical executable changes.

## Limitations

No archive bytes were obtained, so all archive relationships remain `unresolved` or `possible-duplicate`. No raw, normalized, or executable-token checksum is reported. Selected source observations are examples and do not establish family-wide identity or difference.
