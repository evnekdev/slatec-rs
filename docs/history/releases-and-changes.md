# SLATEC releases and change evidence

## Scope

This page separates verified library releases from routine-level revision histories, later repository reorganization, and unverified post-release correction claims. It is deliberately conservative: a version number or change is recorded as fact only when supported by an opened authoritative artifact.

## Verified numbered releases

| Version | Release date | Records | User-callable routines | Primary evidence |
|---:|---|---:|---:|---|
| 1.0 | April 1982 | 114,328 | 491 | [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2 |
| 2.0 | May 1984 | 151,864 | 646 | [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2 |
| 3.0 | April 1986 | 196,013 | 704 | [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2 |
| 3.1 | August 1987 | 197,931 | 707 | [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2 |
| 3.2 | August 1989 | 203,587 | 709 | [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2 |
| 4.0 | December 1992 | 298,954 | 901 | [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2 |
| 4.1 | 1 July 1993 | 290,907 | 902 | [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2 |

The Netlib index independently identifies the preserved distribution as Version 4.1, July 1993 and describes it as containing more than 1,400 total routines. This is compatible with the Guide's 902 **user-callable** routines because the library also contains subsidiary routines ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

## What the aggregate record does not establish

The Guide provides release dates and counts but not a release-by-release inventory of additions, removals, replacements, or corrected files. It therefore supports statements about aggregate growth, but not claims such as “Version 4.0 added package X” unless a separate authoritative source is found.

The fall from 298,954 records in Version 4.0 to 290,907 in Version 4.1, alongside an increase from 901 to 902 user-callable routines, demonstrates that the Guide's record count is not interchangeable with routine count. The reason for that decrease is **unknown** from the opened sources.

## Routine-level change evidence

SLATEC's required prologue format included a chronological `REVISION HISTORY` section with a date written and short descriptions of later revisions. For any individual routine, this prologue is the preferred starting point for change history, subject to confirmation against the executable source and exact file revision ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 8).

Routine-level histories do not automatically define library releases. A date in a prologue may predate adoption into SLATEC, describe an upstream package revision, or reflect a correction after a numbered release. Future metadata should therefore keep separate fields for:

- original date written;
- source-prologue revision entries;
- upstream package release or revision;
- SLATEC numbered release membership;
- Netlib retrieval artifact and checksum;
- project-local corrections or portability patches.

## Distribution reorganization is not a new SLATEC release

The current Netlib index exposes a main `src` collection and separate SLATEC subsets for BLAS/LINPACK/EISPACK/SLAP, FISHPACK/FFTPACK, FNLIB, and PCHIP. Netlib explicitly says these subsets were removed from `slatec/src` to make the main source more accessible. No opened source assigns this reorganization a new SLATEC version number ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

Consequently, a present-day directory listing must not be mistaken for the original physical layout of the July 1993 release. The complete archive, current subdirectories, table of contents, and source prologues need to be reconciled before a canonical inventory is generated.

## Post-4.1 status

**Verified:** The official Netlib landing page still labels the distribution Version 4.1, July 1993 ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

**Unknown:** No later numbered release or later official Guide was verified in this research pass.

**Unverified:** Secondary references report isolated corrections after 4.1 and refer to a change file. The attempted authoritative change endpoint was not successfully rendered during this pass, so dates, affected routines, and wording from that file are not reproduced here. This must be resolved by downloading and checksumming the relevant Netlib artifact or archive member.

**Unknown:** The existence of current files on Netlib does not establish whether an active SLATEC committee presently reviews or maintains them.

## Proposed reconstruction method

**Project proposal:** Build a reproducible release/change ledger in the following order:

1. Acquire and checksum the complete Version 4.1 source and quick-check archives.
2. Inventory all archive members and compare them with the current Netlib subdirectories.
3. Parse each routine's prologue, especially `LIBRARY`, `AUTHOR`, `REFERENCES`, `ROUTINES CALLED`, and `REVISION HISTORY`.
4. Preserve every raw revision-history entry with its source path and checksum before normalization.
5. Locate historical snapshots for Versions 1.0–4.0 through OSTI, laboratory archives, software centers, or institutional reports.
6. Diff snapshots at the routine and text level, distinguishing additions, deletions, renames, package moves, documentation changes, and executable-code changes.
7. Treat post-4.1 corrections as a separate “unversioned upstream corrections” series unless an authoritative source assigns a version.
8. Publish machine-readable provenance and conflict records rather than silently selecting one history.

## Unresolved questions

- Where are the authoritative release notes or change lists for Versions 1.0–4.1?
- Is a complete official `changes` artifact present in the source archive, and does it contain post-1993 corrections?
- Are older source snapshots held by OSTI's historical software-distribution services or participating laboratories?
- Did Netlib modify any source files while reorganizing subsets?
- Are source-prologue revision entries complete and consistently maintained across imported packages?
- Which corrections are code changes versus documentation, portability, or test changes?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide), Sections 2 and 8.
- [`netlib-slatec-index`](https://www.netlib.org/slatec/).
- [`netlib-slatec-readme`](https://www.netlib.org/slatec/readme).
- [`slatec-source-archive`](https://www.netlib.org/slatec/slatec_src.tgz).
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/).
- [`slatec-toc`](https://www.netlib.org/slatec/toc).
