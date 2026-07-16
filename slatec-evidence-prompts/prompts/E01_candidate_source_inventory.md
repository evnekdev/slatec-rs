# E01 — Inventory All Candidate SLATEC Source Artifacts

Follow `slatec-evidence-prompts/SHARED_CONTEXT.md`.

## Objective

Create a complete inventory of candidate artifacts that could contribute to the canonical `slatec-rs` source corpus. This task is discovery and registration only. Do not select the canonical baseline yet.

## Required investigation

Locate and document all relevant official artifacts, including where available:

- the main SLATEC source archive;
- the live SLATEC source directory;
- full routine lists;
- `tree` and `tree1`;
- plus-dependency retrieval mechanisms or bundles;
- the SLATEC guide and table of contents;
- quick-check or installation archives;
- change logs and correction notices;
- machine-specific files;
- reorganized or relocated Netlib subsets, including linear algebra, FFTPACK/FISHPACK, FNLIB or special-function subsets, PCHIP, and other identifiable families;
- standalone packages likely incorporated into SLATEC, including BLAS, LINPACK, EISPACK, FFTPACK, FISHPACK, QUADPACK, MINPACK, SLAP, PCHIP, DASSL or related ODE/DAE packages, and relevant special-function packages.

For every artifact, record:

- stable project source ID;
- title and URL;
- host and artifact type;
- archive or directory name;
- apparent version and stated date;
- retrieval and inspection status;
- expected contents;
- known relationship to SLATEC;
- checksum status;
- rights notes;
- ambiguities.

Do not claim that directory co-location proves package membership.

## Required outputs

```text
docs/source-corpus/
    README.md
    candidate-artifacts.md
    artifact-relationships.md
    retrieval-backlog.md
metadata/
    candidate-artifacts.toml
```

`candidate-artifacts.toml` must be valid TOML and use stable IDs.

Relationship types must include, where applicable:

- duplicate;
- possible duplicate;
- relocated copy;
- independent upstream package;
- documentation-only;
- generated dependency product;
- machine-specific alternative;
- unknown.

## Completion criteria

- Every known candidate artifact is registered.
- Inspected and uninspected artifacts are clearly distinguished.
- No canonical source has been selected.
- Unresolved retrieval work is explicit.
