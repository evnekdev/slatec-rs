# Redistribution risk assessment

## Current conclusion

The project has enough evidence to parse and study the checksum-pinned corpus internally, but not enough evidence to publish a combined source tarball, vendored source tree, generated patched source distribution, or universally redistributable native binary with a single licence declaration.

## Risk levels

| Level | Meaning | Current examples |
|---|---|---|
| Low | Explicit permission matches the exact selected material and planned use. | Reference BLAS only after exact file/revision verification and compliance with credit/modification requests. |
| Medium | Access and provenance are clear, but licence scope or exact provider matching needs confirmation. | Project metadata; some generated factual inventories; source families with a separately verified permissive statement but unresolved matching. |
| High | No explicit licence found, heterogeneous third-party material, conflicting claims, or restrictive terms. | Selected SLATEC archive, quick checks, Linux overlay, browser archive, unresolved imported-package families. |

## Packaging matrix

| Proposed action | Default decision | Reason |
|---|---|---|
| Commit raw archives to normal Git | blocked | Rights unresolved; Git history creates durable redistribution. |
| Commit extracted upstream source | blocked | Same issue, plus file-level heterogeneity. |
| Publish archive hashes and manifests | permitted as project policy | Factual metadata and reproducibility information, with source links and no substantial copied expression. |
| Publish generated routine names, signatures, dependency facts, and hashes | conditionally permitted | Keep facts concise, cite sources, and avoid reproducing prologues or manuals wholesale. |
| Publish project-authored parser and metadata schema | permitted after project licence review | New project material, provided it does not embed substantial upstream source. |
| Publish patches without upstream source | requires review | A patch can reproduce expressive portions and enable reconstruction; assess size and content. |
| Publish native binaries linked from selected source | blocked pending review | Binary distribution is still distribution of compiled copyrighted material. |
| Use source privately for research and interoperability work | allowed as project operational policy, not a legal conclusion | Preserve provenance and access controls; seek institutional advice before external distribution. |
| Include `slatecm` browser code | blocked | Explicit noncommercial/commercial-derivative restrictions and notice obligations. |
| Label combined corpus GPL based on Linux readme | prohibited | The overlay author cannot establish GPL coverage for all upstream material; no GPL version/text was included. |

## Required release gate

Before a public source or binary release that contains upstream SLATEC material:

1. obtain legal or institutional review;
2. resolve exact providers and package revisions;
3. inspect every included file for notices;
4. map every file to a rights status and evidence locator;
5. identify third-party manuals, tests, data, and generated code separately;
6. preserve notices and attribution;
7. document the licence for project-authored code and patches;
8. generate a release-specific software bill of materials and rights report;
9. fail the build if any shipped file has `requires-review`, `conflicting`, or `no-explicit-license-found` without an approved exception.

## Clarification about public availability

Public downloadability from Netlib is evidence of access, not by itself evidence of permission for modification, relicensing, commercial use, or redistribution. Historical phrases such as “freely distributable” must be tied to an authoritative speaker, exact material, and intended legal scope before they control packaging.
