# Shared project context for every standalone conversation

The project is `slatec-rs`, a future Rust ecosystem around the public-domain/openly distributed SLATEC Common Mathematical Library. The immediate task is **documentation and knowledge collection only**. Do not implement Rust bindings or native build code.

Long-term architecture under consideration:

- raw unsafe Rust FFI crates, eventually divided by mathematical domain;
- safe Rust wrapper crates;
- a facade crate;
- machine-readable routine metadata and dependency graphs.

The knowledge base must support:

1. human understanding of SLATEC;
2. exact routine and package provenance;
3. future routine-catalogue generation;
4. dependency-graph construction;
5. evidence-based domain boundaries for Rust crates;
6. safe-API and FFI planning.

Primary sources should include, where available:

- official Netlib SLATEC index and source tree;
- the SLATEC Guide;
- SLATEC table of contents;
- source-file routine prologues;
- dependency trees and “plus dependencies” pages;
- SLATEC change records;
- GAMS classification material;
- original manuals or papers for imported packages such as BLAS, LINPACK, EISPACK, FFTPACK and QUADPACK.

Source precedence:

1. official source code and routine prologues;
2. official manuals, guides, tables and change records;
3. original algorithm/package publications;
4. reputable secondary historical or modern explanatory sources.

Every document must clearly distinguish:

- **Historical or technical facts from sources**;
- **Modern interpretation**;
- **Proposals for `slatec-rs`**, which are not facts about SLATEC.

Use repository-relative paths. Return a downloadable ZIP containing the files, not merely pasted Markdown.
