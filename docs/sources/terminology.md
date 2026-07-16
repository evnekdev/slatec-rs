# Source and documentation terminology

This glossary standardizes terms used while documenting and later wrapping SLATEC. Definitions describe repository usage; historical terms are tied to the authoritative source register.

## Library and distribution terms

**SLATEC**  
The Sandia, Los Alamos, Air Force Weapons Laboratory Technical Exchange Committee. The 1993 guide explains that the organization was formed in 1974 and that its Common Mathematical Library subcommittee began constructing a portable mathematical Fortran library in 1977 ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

**SLATEC Common Mathematical Library (CML)**  
The general-purpose mathematical and statistical Fortran 77 library distributed by Netlib. The canonical index identifies its published distribution as Version 4.1, July 1993 ([`netlib-slatec-index`](https://www.netlib.org/slatec/)). Use **SLATEC CML** when the distinction from the parent committee matters.

**Version 4.1**  
The release identity displayed by the canonical Netlib index, readme, and table of contents. Do not assume every file currently reachable beneath Netlib is byte-for-byte part of one frozen 4.1 archive without comparing inventories and checksums.

**Main source**  
The source collection under Netlib’s `slatec/src` area. The index notes that some subsets, including BLAS/LINPACK/EISPACK/SLAP and FISHPACK/FFTPACK, were moved to separate sublibraries to make the main source more accessible ([`netlib-slatec-index`](https://www.netlib.org/slatec/)). Therefore “main source” is not always synonymous with “every routine historically associated with SLATEC.”

**Imported package**  
A pre-existing or separately maintained numerical package represented within the SLATEC distribution, such as BLAS, LINPACK, EISPACK, FFTPACK, or QUADPACK. Package membership and revision lineage must be established from acknowledgements, source comparison, and original package records; matching routine names alone are insufficient.

**User-callable routine**  
A routine intended to be called by library users. Section I of the SLATEC table of contents groups these routines by GAMS category ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

**Subsidiary routine**  
An internal/support routine not normally referenced directly by users. Section II of the table of contents lists subsidiary routines alphabetically, and Section III marks them with an asterisk ([`slatec-toc`](https://www.netlib.org/slatec/toc)). “Subsidiary” does not mean unimportant or safe to omit from linkage.

**Routine family**  
A project term for equivalent or closely related routines differentiated by precision or data type. The table of contents uses suffix markers such as S, D, C, I, H, L, and A to describe routine data types ([`slatec-toc`](https://www.netlib.org/slatec/toc)). Family membership should be recorded from explicit catalogue/prologue evidence rather than naming heuristics alone.

**Quick check**  
A historical SLATEC validation driver or associated test. The guide documents the quick-check philosophy, while the Netlib index warns that failures may arise from the test, machine, OS, compiler, or supplied software as well as the source ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`slatec-quick-checks`](https://www.netlib.org/slatec/)). A quick check is not equivalent to a modern exhaustive unit test.

## Source-format terms

**Prologue**  
The structured comment header attached to a SLATEC subprogram. The guide defines its format and explains that it supports automatic documentation processing ([`slatec-guide`](https://www.netlib.org/slatec/guide)). For routine facts, the prologue is authoritative documentation but must still be checked against declarations and executable code.

**Purpose field**  
The concise statement of a routine’s intended function in a prologue or package index. Repository documentation should paraphrase it and cite the exact source.

**Revision history**  
Dated change notes in an individual routine’s prologue. These are routine-level evidence; they do not automatically constitute a complete library-wide changelog.

**Machine constants**  
Values supplied historically through routines such as `D1MACH`, `R1MACH`, and `I1MACH`. The main Netlib index describes them as machine-specific configuration points ([`netlib-slatec-index`](https://www.netlib.org/slatec/)). Modern ports must distinguish historical behavior from project proposals for replacement.

**Error handler**  
The SLATEC error-reporting and termination mechanism, including routines such as `XERMSG`, `XERHLT`, and `FDUMP`. The guide and source index describe machine/site-dependent aspects. Documentation must not assume that historical process termination semantics are suitable for a safe Rust API.

## Classification terms

**GAMS**  
The Guide to Available Mathematical Software classification system, used to classify mathematical software by problem type. SLATEC’s table of contents is arranged by GAMS category, and the guide includes the classification scheme ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`gams-classification`](https://gams.nist.gov/)).

**GAMS class / code**  
A hierarchical alphanumeric category such as `D2b1b` or `H2a1a1`. Preserve the source spelling, normalize case only in a separate field, and retain mappings from every source because historical and current catalogues may differ.

**Taxonomy mapping**  
A project record linking a routine to one or more classification codes. A mapping can be source-stated, mechanically derived, or proposed; its provenance and status must be stored.

## Dependency terms

**Direct dependency**  
A routine explicitly referenced by another routine according to static source analysis or an authoritative direct-reference list. Netlib labels `tree1` as the list of routines directly referenced ([`slatec-dependency-direct`](https://www.netlib.org/slatec/tree1)). Intrinsics, external libraries, generated calls, and platform services should be modelled separately.

**Transitive dependency**  
A routine reachable through one or more direct dependency edges. Netlib labels `tree` as the list of routines directly or indirectly referenced ([`slatec-dependency-transitive`](https://www.netlib.org/slatec/tree)).

**Dependency closure**  
The root routine plus all required transitive dependencies under a stated model. A closure must identify the source snapshot and rules used; there is no timeless closure independent of version, compiler, and external-library assumptions.

**“Plus dependencies”**  
Netlib’s generated retrieval option shown beside many routine files. It returns a root source file together with dependencies known to Netlib’s index machinery (for examples, see [`netlib-linpack`](https://www.netlib.org/linpack/) or [`netlib-fftpack`](https://www.netlib.org/fftpack/)). Treat it as useful upstream evidence, not as the sole proof of a complete modern build closure.

**External dependency**  
A requirement not supplied by the selected SLATEC source set, such as a compiler intrinsic, runtime service, separately linked package, or user-supplied callback. External dependencies must not be silently folded into routine-to-routine edges.

## Provenance and evidence terms

**Canonical source**  
The most authoritative maintained or preserved source for a claim, normally the exact Netlib source/distribution artifact or original project publication. Canonical does not imply current, bug-free, or permissively licensed.

**Primary source**  
Material created by the responsible project, organization, or authors: source code, manuals, release records, standards, and original papers.

**Secondary source**  
A later explanation, catalogue, history, or implementation description. It may provide context but cannot silently override primary evidence.

**Source register**  
The human-readable inventory in `docs/sources/source-register.md`, keyed to machine-readable records in `metadata/sources.toml`.

**Stable source ID**  
A repository-controlled identifier such as `slatec-guide`. IDs remain stable even if URLs move; URL history belongs in metadata.

**Retrieved date**  
The date on which a source was accessed or verified. It is not the source’s publication or revision date.

**Checksum**  
A cryptographic digest of retrieved bytes, preferably SHA-256. Checksums are required for reproducible ingestion of downloaded archives and individual source files.

**Distillation**  
The process of converting source material into concise, structured, cited facts without copying substantial expressive text.

**Source fact**  
A statement directly supported by a cited source.

**Interpretation**  
A reasoned conclusion drawn from sources but not stated directly by them. It must be labelled.

**Project proposal**  
A new design choice for `slatec-rs`, such as crate boundaries or safe-wrapper behavior. A proposal must not be presented as a SLATEC requirement.

**Verified / Derived / Inferred / Unverified / Conflicting / Unknown**  
Evidence-status labels defined in `docs/sources/citation-policy.md`. Use them consistently in prose and metadata.

## Licensing terms

**Publicly accessible**  
Available without authentication at the time of retrieval. This does not by itself authorize copying, modification, or redistribution.

**Redistributable**  
Permitted to be redistributed under an explicit licence or authoritative rights statement applicable to the specific artifact. Record the evidence and conditions.

**Licence unknown**  
No applicable explicit licence was verified. This is not the same as proprietary, public domain, or forbidden; it signals that redistribution needs review.

**Attribution request**  
A request to credit authors or projects. It may accompany a licence or historical distribution statement and should be preserved exactly in rights metadata.
