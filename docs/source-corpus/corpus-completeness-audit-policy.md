# Full SLATEC corpus completeness audit policy

The checksum-pinned `slatec_src.tgz` selected by canonical-corpus policy v1 is
the reproducible **`main-src` subset**. Its 735 selected `src/*.f` files and
their generated manifests remain immutable evidence for that subset. Neither
this audit nor a live-directory retrieval changes its snapshot ID, content
hashes, or provider-selection authority.

The audit has four distinct scopes:

| Scope | Purpose | Provider-selection state |
|---|---|---|
| `main-src` | Pinned, reproducible source subset | selected only inside policy v1 |
| `relocated-slatec-subsets` | Live `src`, `lin`, `fishfft`, `fnlib`, and `pchip` completeness and provenance evidence | never selected automatically |
| `complete-selected-slatec-collection` | Explicit provider-resolution profile built from `main-src`, approved relocated providers, and reviewed supplemental FNLIB source | selected by policy v1 |
| `independent-upstream-comparison-packages` | Standalone package comparisons | never selected automatically |

The audit retrieves the official `list` and Version 4.1 `toc` catalogue
artifacts as checksum-recorded local evidence. It inventories only
SLATEC-hosted files for the proposed full collection. Standalone BLAS,
LINPACK, EISPACK, FFTPACK, FISHPACK, FN, PCHIP, and SLAP packages remain
independent comparison artifacts; matching names do not authorize a
substitution.

Provider relationships are decided per normalized declared program-unit name,
not by filename or retrieval order. The audit can describe a location as a
byte-identical relocation, normalized-identical relocation, modified
relocation, historical variant, alternate implementation, duplicate provider,
or unresolved. The derived [complete selected-corpus policy](complete-selected-corpus-policy.md)
resolves only the evidence-backed hosted-provider cases; it does not grant
authority to standalone upstream packages.

Downloaded catalogues and source bytes remain in ignored `evidence/full-corpus/`.
Committed output contains only hashes, paths, program-unit identities, counts,
relationship classifications, diagnostics, and concise catalogue comparisons.
