# Citation and distillation policy

## Purpose

Documentation in `slatec-rs` must remain traceable to authoritative evidence while avoiding the accidental presentation of interpretation or proposed Rust design as historical fact. This policy applies to hand-written pages, generated metadata, routine catalogues, architecture notes, and future API documentation.

## Source precedence

Use evidence in this order:

1. **The exact routine source being described**, especially its prologue, argument declarations, executable code, revision history, and called-routine list.
2. **Canonical SLATEC distribution records**, including the Netlib SLATEC index, Version 4.1 table of contents, guide, source archive, source directories, and dependency trees.
3. **Original imported-package sources and manuals**, such as Netlib BLAS, LINPACK, EISPACK, FFTPACK, and QUADPACK materials.
4. **Original papers, reports, standards, and institutional records** issued by authors or responsible organizations.
5. **Reputable secondary sources**, used for orientation or historical synthesis only.

A lower-precedence source may add context but must not silently override a higher-precedence source.

## Handling conflicts

When sources disagree:

1. Record the disagreement explicitly.
2. Identify the exact artifacts and, where possible, their dates, versions, paths, and checksums.
3. Prefer the evidence closest to the claim. For example, executable declarations govern the actual source signature, while the guide governs documented project policy.
4. Do not “correct” historical source silently. Preserve the historical statement and add a project note.
5. If a SLATEC copy differs from an upstream package copy, describe them as distinct revisions until a documented lineage is established.
6. Mark unresolved conflicts with the uncertainty notation below and create a follow-up verification item.

## Statement classes

Every nontrivial statement should be recognizable as one of these classes:

- **Quotation:** exact words from a source. Use quotation marks or a Markdown block quote, keep excerpts short, and cite immediately.
- **Paraphrase:** a faithful restatement of a source. Cite the source in the same sentence or paragraph.
- **Interpretation:** a reasoned conclusion drawn from one or more sources. Introduce it with wording such as “This suggests,” “We interpret,” or “For this project, this is treated as,” and cite the evidence.
- **Project proposal:** a new `slatec-rs` decision or recommendation. Label it `Project proposal`, `Project policy`, or `Planned`, and do not cite historical sources as though they mandated the decision.

Generated pages should preserve this distinction in structured fields where practical, for example `statement_kind = "source_fact" | "interpretation" | "proposal"`.

## Copying limits

- Prefer concise paraphrase over reproduction.
- Copy only what is necessary to identify an interface, preserve a short definition, or discuss a specific discrepancy.
- Do not reproduce manuals, catalogue descriptions, or source prologues wholesale in documentation.
- For Fortran source, retain only minimal excerpts needed for technical explanation unless redistribution rights have been verified.
- Routine names, argument names, classifications, dates, and other facts may be recorded, but expressive descriptions should be distilled and attributed.
- A publicly accessible URL is not evidence of an open-source licence.

## Markdown citation format

Use stable source IDs from `metadata/sources.toml` in prose and provide a direct link on first use.

Preferred first citation:

```markdown
The Netlib index identifies the distribution as SLATEC CML Version 4.1, July 1993 ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).
```

Later citations in the same page may use:

```markdown
... as described by [`slatec-guide`](../sources/source-register.md#core-slatec-sources).
```

For routine-specific facts, link directly to the exact source file and include the stable source ID when available:

```markdown
`DCHDC` computes a Cholesky decomposition with optional pivoting ([`netlib-linpack`](https://www.netlib.org/linpack/dchdc.f)).
```

For bibliographic sources, include author, title, year, and DOI or institutional URL in a References section. Citations must point to the evidence actually consulted, not merely to a related home page.

## Granularity and placement

- Cite every factual paragraph unless all its claims are supported by the same immediately preceding citation and the relationship is unambiguous.
- Put citations next to the supported claim, not only in a page-wide bibliography.
- Tables must include a source column or an unambiguous source note.
- Generated routine records should carry field-level provenance when values can come from different inputs.

## Uncertainty notation

Use one of these labels:

- **Verified:** directly confirmed in an opened primary source.
- **Derived:** mechanically extracted or computed from verified sources; record the method and inputs.
- **Inferred:** supported by evidence but not stated directly.
- **Unverified:** plausible or referenced but not successfully inspected.
- **Conflicting:** authoritative sources disagree.
- **Unknown:** evidence has not been found.

Example:

```markdown
**Unverified:** The `tree` endpoint is linked by the Netlib SLATEC index as the transitive dependency list, but its contents were not successfully retrieved during the 2026-07-16 source-register pass.
```

Never replace uncertainty with a guess.

## Archived, generated, and unstable links

For every source record:

- retain the canonical URL;
- record retrieval date;
- record HTTP status and content type during automated ingestion;
- calculate a cryptographic checksum for downloaded immutable inputs;
- record redirect targets;
- distinguish a stable identifier such as a DOI from a mutable download URL;
- store an archival URL only when legally and institutionally appropriate;
- label CGI-generated “plus dependencies” URLs as generated/unstable and retain the requested root routine;
- do not claim an archived copy is authoritative merely because it is preserved.

When a link fails, keep the record, mark its access status, and cite the authoritative page that advertised it.

## Provenance for generated documents

Every generated artifact should be reproducible from declared inputs. At minimum, generated front matter or adjacent metadata should record:

```yaml
generated: true
generator: "tool-name and version"
generated_at: "YYYY-MM-DDThh:mm:ssZ"
source_ids:
  - netlib-slatec-index
  - slatec-source-tree
source_artifacts:
  - url: "https://www.netlib.org/slatec/src/example.f"
    sha256: "..."
transform: "brief description or script path"
review_status: "unreviewed | reviewed | verified"
```

For field-level metadata, prefer records such as:

```toml
purpose.value = "..."
purpose.statement_kind = "paraphrase"
purpose.source_id = "slatec-source-tree"
purpose.source_path = "src/example.f"
purpose.source_locator = "prologue: PURPOSE"
```

Generated documents must not erase prior provenance when data are merged. Conflicts should produce multiple candidate values or a conflict record, not silent last-write-wins behavior.

## Licensing claims

Only make a positive licensing statement when an explicit notice, licence file, or authoritative rights statement has been opened and recorded. Otherwise use wording such as:

> No explicit licence was verified for this artifact; redistribution requires review.

Keep separate fields for:

- access status;
- copyright statement;
- licence identifier;
- attribution request;
- modification condition;
- redistribution conclusion;
- evidence URL and locator.

Do not apply modern SPDX identifiers by analogy.

## Review checklist

Before merging documentation:

- factual claims have adjacent citations;
- source IDs exist in `metadata/sources.toml`;
- quotations are short and exact;
- interpretations and proposals are labelled;
- version-specific claims identify the version;
- failed retrievals are marked unverified;
- imported-package lineage is not assumed from routine names alone;
- licensing language is evidence-based;
- generated files retain source IDs, retrieval dates, and checksums where available.
