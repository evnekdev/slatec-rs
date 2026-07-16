# SLATEC chronology

## Scope

This chronology records dates only when supported by an opened authoritative source. “Approximate” is used when the source gives only a month or year. Counts are transcribed from the July 1993 SLATEC Guide and retain its term **records**, whose precise physical-file meaning requires further archival study.

## Chronology

| Date | Event | Evidence and status |
|---|---|---|
| 1974 | The computer centers of Sandia National Laboratories Albuquerque, Los Alamos National Laboratory, and Air Force Weapons Laboratory formed the Sandia, Los Alamos, Air Force Weapons Laboratory Technical Exchange Committee to foster exchange of technical information. | **Verified.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| 1977 | The Common Mathematical Library Subcommittee decided to construct a mathematical Fortran subprogram library usable across the three founding sites. Portable, non-proprietary software for their supercomputers was the stated primary impetus. | **Verified.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| 1980 | The computer centers of Sandia National Laboratories Livermore and Lawrence Livermore National Laboratory joined the parent committee and subcommittees. | **Verified.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| 1981 | The National Bureau of Standards and Oak Ridge National Laboratory were invited to participate in the mathematical-library subcommittee. | **Verified.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| April 1982 | Version 1.0 released: 114,328 records and 491 user-callable routines. | **Verified; month precision.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| September 1982 | Walter H. Vandevender and Karen H. Haskell published “The SLATEC Mathematical Subroutine Library” in *ACM SIGNUM Newsletter*, documenting the early library. | **Bibliographically identified; full text not opened in this pass.** The title is referenced by later catalogues; use only after primary-paper retrieval. |
| May 1984 | Version 2.0 released: 151,864 records and 646 user-callable routines. | **Verified; month precision.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| April 1986 | Version 3.0 released: 196,013 records and 704 user-callable routines. | **Verified; month precision.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| August 1987 | Version 3.1 released: 197,931 records and 707 user-callable routines. | **Verified; month precision.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| August 1989 | Version 3.2 released: 203,587 records and 709 user-callable routines. | **Verified; month precision.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| December 1992 | Version 4.0 released: 298,954 records and 901 user-callable routines. | **Verified; month precision.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| 1 July 1993 | Version 4.1 released: 290,907 records and 902 user-callable routines. | **Verified; exact date.** [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. |
| July 1993 | The preserved Guide was issued by Kirby W. Fong, Thomas H. Jefferson, Tokihiko Suyehiro, and Lee Walton. It documents governance, submission, source format, quick checks, classification, machine constants, error handling, and distribution structure. | **Verified; month precision.** [`slatec-guide`](https://www.netlib.org/slatec/guide). |
| After July 1993, date not established | The library and associated materials became and remain preserved through Netlib, which currently labels the distribution Version 4.1, July 1993. Some subsets are exposed in separate Netlib subdirectories. | **Verified present preservation; migration date unknown.** [`netlib-slatec-index`](https://www.netlib.org/slatec/). |
| Present retrieval, 16 July 2026 | Netlib serves the main index, source archive link, individual source tree, Guide, table of contents, quick-check archive link, and dependency resources. | **Verified access status, not evidence of active committee maintenance.** [`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-source-tree`](https://www.netlib.org/slatec/src/). |

## Release growth summary

| Version | Release date | Records | User-callable routines | Change from previous user-callable count |
|---:|---|---:|---:|---:|
| 1.0 | April 1982 | 114,328 | 491 | — |
| 2.0 | May 1984 | 151,864 | 646 | +155 |
| 3.0 | April 1986 | 196,013 | 704 | +58 |
| 3.1 | August 1987 | 197,931 | 707 | +3 |
| 3.2 | August 1989 | 203,587 | 709 | +2 |
| 4.0 | December 1992 | 298,954 | 901 | +192 |
| 4.1 | 1 July 1993 | 290,907 | 902 | +1 |

All values in this table are from [`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2. The table does not imply that routine-count growth measures the importance of a release; changes may also include replacements, removals, corrections, documentation, subsidiary routines, and reorganization.

## Interpretation

The chronology shows long periods of incremental growth punctuated by a large increase between 3.2 and 4.0. The source opened for this pass does not enumerate the exact packages or routines responsible, so attributing that increase to specific additions would be speculation. Likewise, the decrease in record count from 4.0 to 4.1 despite one additional user-callable routine indicates that record count is not a simple routine count and may reflect source or distribution restructuring.

## Implications for `slatec-rs`

**Project proposal:** Store release claims at two levels: a library-release table containing only authoritative aggregate facts, and routine-level revision histories extracted from exact source prologues. Do not reconstruct per-release routine membership until historical snapshots or authoritative change lists are obtained.

**Project proposal:** Record date precision (`day`, `month`, `year`, `approximate`, or `unknown`) rather than silently converting month-only dates to arbitrary days.

## Unresolved questions

- What does “record” denote in each release inventory: physical source lines, card-image records, or another distribution unit?
- Are complete source snapshots for Versions 1.0 through 4.0 preserved in an accessible institutional archive?
- What exact changes produced the large 3.2-to-4.0 increase and the 4.0-to-4.1 record-count decrease?
- Can the 1982 SIGNUM article be obtained from an authoritative open repository?
- Are post-4.1 corrections recorded in an official Netlib artifact, and can that artifact be retrieved reproducibly?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide), especially Sections 2–4.
- [`netlib-slatec-index`](https://www.netlib.org/slatec/).
- [`netlib-slatec-readme`](https://www.netlib.org/slatec/readme).
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/).
