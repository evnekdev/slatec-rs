# Provenance recording policy

## Scope

This policy applies to preserved archives, extracted files, selected canonical sources, normalized parser views, patches, generated metadata, tests, compiled objects, and release artifacts.

## Artifact record

Each immutable artifact record must contain:

- stable artifact ID;
- title and artifact type;
- requested URL and final URL;
- retrieval timestamp and method;
- byte length and SHA-256;
- content type and redirect chain when retrieved automatically;
- archive member manifest hash;
- source/host organization;
- rights status and rights-record IDs;
- exact notice paths and locators;
- preservation location or reason bytes are not stored;
- reviewer and review date.

## File record

Each source file must contain or reference:

- parent artifact ID and archive-relative path;
- raw and normalized hashes;
- program-unit identities;
- prologue authorship, library, references, and revision markers;
- historical package/family candidates with confidence;
- rights records and notice locators;
- selected/excluded status and reason;
- duplicate-provider resolution ID;
- patch chain and resulting hash.

## Rights evidence model

Rights fields must remain separate:

```text
access_status
copyright_notice
public_domain_statement
government_work_statement
licence_identifier
licence_text_locator
redistribution_permission
modification_permission
commercial_use_permission
attribution_requirement
notice_preservation_requirement
warranty_disclaimer
rights_status
review_notes
```

Unknown fields remain `unknown`; they are not converted to false, permissive, or public domain.

## Layered lineage

The lineage graph must preserve these distinct nodes:

1. downloaded artifact;
2. extracted raw member;
3. selected canonical provider;
4. normalized parser view;
5. project-patched source;
6. generated metadata or binding;
7. compiled object/library;
8. release package.

Every edge records the tool, version, command/configuration hash, timestamp, inputs, outputs, and reviewer status.

## Notice extraction

Automated notice scanning is a discovery aid only. It must:

- search case-insensitively for licence, copyright, public-domain, permission, redistribution, commercial-use, warranty, government-work, and attribution phrases;
- retain exact matched paths and line spans;
- store false-positive/review disposition;
- never assign an SPDX identifier solely from keywords;
- queue files with authored third-party provenance but no notice for manual review.

## Corrections and conflicts

Multiple notices or conclusions are preserved as separate evidence entries. A later reviewer may resolve a conflict through an explicit decision record, but original evidence is never deleted or overwritten.

## Release provenance

Every release containing upstream-derived material must embed or accompany:

- canonical corpus policy version;
- baseline archive SHA-256;
- selected provider manifest;
- patch manifest;
- generated-output manifest;
- rights-register version;
- unresolved/exception list;
- build environment and compiler identity;
- release artifact checksums.
