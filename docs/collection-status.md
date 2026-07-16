# Documentation collection status

## Assessment

**Status: collection stage complete with conditions.**

The required subject areas are represented and the project can advance to bulk extraction. The collection is not yet ready to support production FFI generation or claims of complete routine/package coverage.

## Completion criteria

| Criterion | Status | Evidence and qualification |
|---|---|---|
| Primary sources are registered | met | source register and `metadata/sources.toml` include the Netlib index, guide, table of contents, source tree/archive, dependency resources, quick checks and major package indexes |
| Architecture and conventions are documented | met | library structure, source organization, prologues, naming, precision, machine constants, errors and dependency model are covered |
| Mathematical domains and imported packages are mapped | met, preliminary | GAMS, domain and package records exist; routine-level boundaries remain to be verified |
| Routine metadata pilot is coherent | met for pilot | twenty records and pages exercise the schema; several located-only sources remain marked unverified |
| Dependency extraction is specified | met | typed graph, evidence reconciliation, SCC, ownership and linker validation outputs are defined |
| Rust mapping is explicitly proposed | met | Rust documents consistently label architecture and safety decisions as proposals |
| Unresolved issues are recorded | met | Rust open questions, this audit and the collection research register retain unresolved claims |

## Inventory

The current pre-audit collection contains approximately:

- **63 Markdown files** under `docs/` and `metadata/`;
- **6 TOML metadata files**;
- **69 documentation/metadata artifacts** in total;
- **20 pilot routine pages and records**.

The exact count should be regenerated from a local checkout in continuous integration because code-search results are not a substitute for a repository tree manifest.

## Coverage by area

| Area | Coverage | Confidence |
|---|---|---|
| Source policy and authority | strong | high |
| Historical overview | broad | medium–high |
| Release chronology | partial | medium |
| Fortran architecture and conventions | strong | high |
| GAMS taxonomy | broad | high for broad classes; incomplete at routine leaves |
| Package provenance | broad candidates | medium |
| Dense/sparse linear algebra | representative | medium–high |
| Nonlinear equations and optimization | representative | medium–high |
| Quadrature | representative | high for QUADPACK families |
| ODE/DAE/BVP/PDE | broad but family boundaries incomplete | medium |
| Interpolation and PCHIP | representative | medium–high |
| Transforms | representative | medium |
| Special functions | broad but provenance fragmented | medium |
| Statistics and utilities | preliminary | low–medium |
| Routine catalogue | 20-routine pilot only | low coverage, high schema value |
| Dependency graph | specification and worked example only | low extracted coverage |
| Rust architecture | detailed proposal | high as design rationale, unvalidated operationally |

## Ready to advance

The following work may begin:

1. pinning and checksumming the full source corpus;
2. deterministic source/prologue parsing;
3. complete routine inventory generation;
4. dependency and symbol extraction;
5. package-version comparison;
6. monolithic native-build prototype;
7. low-risk safe pilot design after ABI validation.

## Not yet ready

The collection does not justify:

- generating bindings for every routine automatically;
- declaring every argument intent verified;
- publishing domain-specific native crates;
- claiming thread safety;
- claiming a blanket public-domain or uniform licence;
- substituting system BLAS/LAPACK without provider validation;
- claiming exact package revisions;
- treating Netlib `tree`, `tree1` or plus-dependencies as a complete linker model;
- promising cross-compiler ABI compatibility.

## Quality gates before bulk routine extraction

- obtain a complete local repository/source checkout;
- generate a canonical file manifest;
- run a real Markdown link checker;
- parse all TOML and YAML front matter;
- validate every source ID reference;
- pin the source archive checksum;
- identify prologue dialects and parser fallbacks;
- define manual-review queues for unknown argument intent and provenance;
- preserve raw source locators and parser versions;
- make generated outputs reproducible.

## Recommended next milestone

**Milestone: complete source and metadata baseline.**

Deliverables:

- canonical source snapshot manifest;
- full routine/source/program-unit inventory;
- first-pass prologue extraction;
- direct call graph and unresolved references;
- package comparison report;
- compiler/object symbol inventory;
- regenerated coverage report.

Codex High is the appropriate execution environment because this milestone requires repository-wide parsing, deterministic file generation, compilation and validation.
