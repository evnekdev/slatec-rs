# Attribution policy

## Purpose

Attribution records provenance; it does not substitute for permission. Every distributed or generated artifact must retain the attribution required by its own evidence record.

## Required attribution data

For every imported source file, generated routine record, patch, or bundled binary, retain where available:

- original routine and file name;
- original authors and contributing institutions;
- historical package or family;
- exact source artifact ID, path, URL, retrieval date, and SHA-256;
- source prologue `AUTHOR`, `LIBRARY`, `REFERENCES`, and revision-history fields;
- copyright and licence text with exact locator;
- modifications, modifier, date, and patch ID;
- required citations or acknowledgement requests;
- unresolved or conflicting provenance.

## Distribution acknowledgements

A source or binary distribution built from the selected corpus should include:

1. a statement that it derives from the SLATEC Common Mathematical Library;
2. the exact baseline archive checksum and retrieval date;
3. the institutions listed in the SLATEC disclaimer/version routine, without implying that they endorse `slatec-rs`;
4. package-specific author credit where a provider has been verified;
5. a machine-readable inventory mapping every shipped file to its rights record;
6. project patch authors and patch identifiers;
7. a clear separation between original source, normalization, and project modifications.

## BLAS-specific requirement

For a file verified to fall under the official reference BLAS statement:

- give proper credit to the authors;
- if modified, rename the routine and document the changes as requested by the official BLAS page;
- do not apply the statement merely because a same-named routine is found under `slatec/lin`.

## Documentation and publications

Manuals, books, and papers are cited bibliographically. Do not copy substantial text, tables, or descriptions into project documentation. Distil technical facts and cite the exact source consulted.

## Project patches

Every patch must identify:

- copyright holder or author of the patch;
- contributor identity and date;
- project licence applicable to the patch;
- preimage and postimage hashes;
- whether the change incorporates third-party material;
- whether upstream attribution requirements remain satisfied.

## Prohibited attribution practices

- claiming that Netlib, SLATEC institutions, authors, or government agencies endorse the project;
- replacing original authorship with only the `slatec-rs` project name;
- removing historical notices during normalization;
- treating a citation request as proof of an open-source licence;
- attaching one SPDX identifier to heterogeneous upstream files without direct evidence.
