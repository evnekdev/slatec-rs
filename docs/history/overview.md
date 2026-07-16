# SLATEC: historical and conceptual overview

## Scope

This page explains why the SLATEC Common Mathematical Library (CML) was created, how it was governed and assembled, how it was distributed, and what can responsibly be said about its present status. It distinguishes sourced history, modern interpretation, and implications for `slatec-rs`.

## Sourced history

### Origins and purpose

SLATEC was the acronym of the Sandia, Los Alamos, Air Force Weapons Laboratory Technical Exchange Committee. The committee was formed in 1974 by the computer centers of Sandia National Laboratories Albuquerque, Los Alamos National Laboratory, and the Air Force Weapons Laboratory to exchange technical information. In 1977 its Common Mathematical Library Subcommittee decided to construct a shared library of mathematical Fortran subprograms usable on the different computers at the three sites. The stated primary impetus was portable, non-proprietary mathematical software for the member sites' supercomputers ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2).

The problem was not merely the absence of numerical algorithms. The participating centers operated different machines and local software environments, so a useful common library had to control portability, machine constants, error behavior, documentation format, testing, naming, and installation practices. The 1993 Guide therefore treats coding conventions, embedded prologues, quick checks, machine-dependent routines, and error handling as library-wide infrastructure rather than incidental style rules ([`slatec-guide`](https://www.netlib.org/slatec/guide), Sections 6–14 and Appendices B–D).

### Expansion beyond the founding sites

The collaboration widened in stages. In 1980, the computer centers of Sandia National Laboratories Livermore and Lawrence Livermore National Laboratory joined the parent committee and subcommittees. In 1981, the National Bureau of Standards—now the National Institute of Standards and Technology—and Oak Ridge National Laboratory were invited to participate in the mathematical-library subcommittee because of their interest in the project ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2).

By July 1993 the Guide listed voting members from Air Force Phillips Laboratory at Kirtland, Lawrence Livermore National Laboratory's Livermore Computer Center and NERSC, Los Alamos National Laboratory, NIST, Oak Ridge National Laboratory, Sandia/California, and Sandia/New Mexico. The document names W. Robert Boland of LANL as chairman and records the individual representatives listed in [participating institutions](participating-institutions.md) ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 3).

### A curated common library, not one newly invented package

SLATEC was both an integration project and a development venue. Its source prologue format included a `LIBRARY SLATEC` field that could also name other libraries, collections, or packages from which a routine came or to which it belonged. The Guide required new routines to build on existing library routines unless there was a compelling reason not to; it specifically cited LINPACK and EISPACK as capabilities that should be reused rather than duplicated ([`slatec-guide`](https://www.netlib.org/slatec/guide), Sections 6 and 8).

The preserved Netlib distribution makes this composite nature visible. It identifies separate SLATEC subsets for BLAS/LINPACK/EISPACK/SLAP, FISHPACK/FFTPACK, FNLIB, and PCHIP, while the main source directory contains many other routines and package families. Netlib notes that some subsets were removed from the main `src` directory to make it more accessible, not that they ceased to belong to the historical SLATEC collection ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

At the same time, routine prologues name authors and institutional affiliations from a much wider numerical-software community than the voting committee alone. The current Netlib source index, for example, attributes routines to contributors at Sandia, LANL, NIST/NBS, Bell Laboratories, and other organizations. Exact provenance must therefore be established routine by routine from the source prologue and, where relevant, comparison with an upstream package ([`slatec-source-tree`](https://www.netlib.org/slatec/src/); [`citation-policy`](../sources/citation-policy.md)).

### Selection, standardization, and maintenance

The subcommittee met several times per year, rotating hosts among member sites. Proposed routines were introduced, discussed, circulated for machine-readable testing, revised, and eventually voted on. Acceptance criteria included usefulness, robustness, maintainability, compliance with SLATEC standards, quality of documentation, and free distributability. The Guide describes a committee member as the routine's sponsor or “champion” during this process ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 5).

The library's consistency was enforced through technical policy. Routines were expected to use ANSI Fortran 77 without machine-dependent extensions; obtain machine constants through `D1MACH`, `I1MACH`, and `R1MACH`; use the common error-handling system; avoid mutable saved or common-block work storage that obstructed multiprocessing; provide embedded documentation; and include quick-check programs for user-callable routines. Equivalent precision variants were expected to remain structurally similar ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 6).

The source code was also the principal routine manual. The Guide states that SLATEC supplied routine documentation in source comment lines and prescribed a rigid prologue format so documentation and metadata could be extracted automatically. Required fields included purpose, library/package identity, GAMS category, type and precision family, authorship, references, called routines, and revision history ([`slatec-guide`](https://www.netlib.org/slatec/guide), Sections 2 and 8).

### Releases and historical scale

The Guide records seven named releases: Version 1.0 in April 1982, 2.0 in May 1984, 3.0 in April 1986, 3.1 in August 1987, 3.2 in August 1989, 4.0 in December 1992, and 4.1 on 1 July 1993. Across these releases the number of user-callable routines grew from 491 to 902, while the distribution's record count rose from 114,328 in Version 1.0 to 298,954 in Version 4.0 and then fell to 290,907 in Version 4.1 ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 2; [chronology](chronology.md)).

Netlib identifies the preserved distribution as SLATEC CML Version 4.1, July 1993, containing more than 1,400 general-purpose mathematical and statistical routines written in Fortran 77. The difference between “user-callable” and total routines is important: the total includes subsidiary routines not intended as the public entry points ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

### Distribution model

In 1993 the Guide stated that the library was in the public domain and distributed by the Energy Science and Technology Software Center in Oak Ridge. It also described a structured distribution containing source, quick checks, documentation tools, and supporting files ([`slatec-guide`](https://www.netlib.org/slatec/guide), Sections 4 and Appendix D).

Today Netlib preserves a complete source archive, a quick-check archive, individual routine files, the Guide, the table of contents, GAMS mappings, direct and transitive dependency lists, and historical installation/documentation tools. Netlib also exposes generated “plus dependencies” retrieval for many routines. These preserved interfaces support both human browsing and future machine-assisted reconstruction of the library ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-source-tree`](https://www.netlib.org/slatec/src/)).

### Preservation and present maintenance status

**Verified:** Netlib currently serves the Version 4.1 distribution and associated documentation. The source index is browsable and individual files remain retrievable ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-source-tree`](https://www.netlib.org/slatec/src/)).

**Unknown:** No authoritative current statement was found that the historical SLATEC CML Subcommittee still meets, accepts submissions, or issues coordinated releases. The 1993 Guide describes an active committee and says the guide would be updated periodically, but no later official guide or numbered release was verified during this research pass ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`netlib-slatec-readme`](https://www.netlib.org/slatec/readme)).

**Unverified:** Secondary accounts mention a very small number of post-4.1 corrections without a new version number. Because the corresponding authoritative change artifact was not successfully opened in this pass, those corrections are not incorporated as verified release history here. See [releases and changes](releases-and-changes.md).

## Modern interpretation

SLATEC can be understood as an early cross-institutional software-governance project as much as a numerical library. Its committee process, portability rules, standardized metadata, reusable infrastructure, and tests addressed the problem of moving trusted scientific software among incompatible systems before modern package managers, continuous integration, or standardized binary interfaces existed. This interpretation follows from the policies in the Guide rather than from a claim that its authors used modern software-engineering terminology ([`slatec-guide`](https://www.netlib.org/slatec/guide), Sections 5–14).

Its composite structure also explains why “SLATEC” should not be treated as a single homogeneous algorithm family. Some routines came from established packages, some were adapted from other collections, and others were authored for or incorporated directly into the common library. The appropriate unit of provenance is therefore the exact routine and source revision, with package-level grouping used only when supported by the prologue and source comparison ([`slatec-guide`](https://www.netlib.org/slatec/guide), Section 8; [`slatec-source-tree`](https://www.netlib.org/slatec/src/)).

The preserved library remains influential primarily as source, algorithms, test material, and historical design evidence. Continued availability does not by itself prove active upstream maintenance, modern compiler portability, numerical suitability for every present-day use, or uniform licensing across every imported artifact. Those questions require separate verification.

## Implications for `slatec-rs`

**Project implication:** `slatec-rs` should model SLATEC as a versioned collection of routines with explicit package and institutional provenance, not as an undifferentiated monolith.

**Project implication:** Routine metadata should preserve the prologue's distinctions—user-callable versus subsidiary, precision family, GAMS category, authors, references, routines called, and revision history—while recording the exact source file and checksum used.

**Project implication:** Imported-package identity should not be inferred from names alone. A SLATEC copy and a current Netlib package copy must be treated as potentially distinct revisions until compared.

**Project implication:** Historical quick checks and dependency trees should be preserved as evidence and test inputs, but independently verified against parsed source and modern builds.

**Project implication:** The 1993 public-domain statement is strong evidence about the intended SLATEC library policy, but redistribution decisions for a modern combined Rust/native package should still retain file-level notices and investigate the provenance of imported components as required by the project citation policy.

## Unresolved questions

- Is there an authoritative archive of subcommittee minutes, ballots, or release notes before Version 4.1?
- Which exact routine additions, removals, and corrections distinguish each numbered release?
- Were any official corrections made after 1 July 1993 without changing the version number, and where is the canonical change record?
- When did active committee maintenance cease, if it ceased formally?
- What exact source snapshot underlies each present Netlib subdirectory after later reorganization?
- Which imported routines differ from their separately preserved BLAS, LINPACK, EISPACK, FFTPACK, FISHPACK, FNLIB, PCHIP, QUADPACK, or SLAP counterparts?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide), Fong, Jefferson, Suyehiro, and Walton, *Guide to the SLATEC Common Mathematical Library*, July 1993.
- [`netlib-slatec-index`](https://www.netlib.org/slatec/), canonical Netlib distribution index.
- [`netlib-slatec-readme`](https://www.netlib.org/slatec/readme), brief release identity record.
- [`slatec-toc`](https://www.netlib.org/slatec/toc), Version 4.1 table of contents.
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/), individual source files and routine metadata.
- [`citation-policy`](../sources/citation-policy.md), repository source-precedence and uncertainty policy.
