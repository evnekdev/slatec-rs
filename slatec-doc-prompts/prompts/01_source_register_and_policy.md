# Prompt 01 — Authoritative source register and documentation policy

You are working on a new repository called `slatec-rs`. This conversation is standalone, so use all context below without assuming access to earlier chats.

## Project context

`slatec-rs` will eventually provide Rust FFI and safe wrappers for the SLATEC Common Mathematical Library. At this stage, we are building a rigorous, cited knowledge base. Do not implement Rust code.

## Task

Research and create the foundational source register and documentation policy for the project.

Browse the web and prioritize official Netlib and original primary sources. Identify all authoritative open-access materials needed to understand SLATEC, including:

- main Netlib SLATEC index;
- guide/manual;
- table of contents or routine index;
- source tree;
- dependency-tree resources and “plus dependencies” mechanism;
- release/change records;
- GAMS taxonomy documentation;
- original documentation for imported packages such as BLAS, LINPACK, EISPACK, FFTPACK and QUADPACK;
- any official machine-readable indexes or metadata.

For each source, record:

- title;
- organization or author;
- stable URL;
- source type;
- authority tier;
- coverage;
- expected use in this repository;
- known limitations;
- licensing or redistribution considerations where verifiable;
- retrieval date.

Also create a citation and distillation policy that explains:

- source precedence;
- how to handle conflicts between source files, guides and secondary sources;
- how to distinguish quotation, paraphrase, interpretation and project proposal;
- limits on copying source text;
- citation format for Markdown files;
- how to mark uncertainty;
- how to record archived or unstable links;
- how future generated documents should preserve provenance.

## Required files

Create exactly these repository-relative files:

```text
docs/sources/source-register.md
docs/sources/citation-policy.md
docs/sources/terminology.md
metadata/sources.toml
```

`metadata/sources.toml` should assign stable IDs such as `netlib-slatec-index`, `slatec-guide`, and `quadpack-manual`.

## Quality requirements

- Cite all factual claims.
- Prefer primary sources over summaries.
- Do not claim a source exists unless you opened or verified it.
- Note ambiguous versioning or missing publication dates.
- Do not copy long passages.
- Make the files useful to future automated tooling.

## Deliverable

Return a downloadable ZIP containing only the four requested files, preserving repository-relative paths. Do not edit GitHub through a connector. In the final chat response, provide a brief summary and the ZIP download link.
