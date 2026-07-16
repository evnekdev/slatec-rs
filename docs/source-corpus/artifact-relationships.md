# Candidate artifact relationships

No relationship below selects a canonical source. Edges describe discovery evidence or a comparison hypothesis.

| From | Relationship | To | Evidence / qualification |
|---|---|---|---|
| `slatec-source-archive` | possible duplicate | `slatec-source-directory` | Both are advertised as SLATEC source, but the live directory excludes relocated subsets and archive contents were not compared. |
| `slatec-source-archive` | possible duplicate | `slatec-lin-directory` | The archive is advertised as complete; Netlib says `lin/` was removed from `src/`. Byte identity and archive inclusion remain untested. |
| `slatec-source-archive` | possible duplicate | `slatec-fishfft-directory` | Same qualification. |
| `slatec-source-archive` | possible duplicate | `slatec-fnlib-directory` | Same qualification. |
| `slatec-source-archive` | possible duplicate | `slatec-pchip-directory` | Same qualification. |
| `slatec-lin-directory` | relocated copy | `slatec-source-directory` | Netlib explicitly describes the BLAS/LINPACK/EISPACK/SLAP subset as removed from `slatec/src`; exact historical paths and bytes are unverified. |
| `slatec-fishfft-directory` | relocated copy | `slatec-source-directory` | Netlib explicitly describes the FISHPACK/FFTPACK subset as removed. |
| `slatec-fnlib-directory` | relocated copy | `slatec-source-directory` | Netlib explicitly describes the FNLIB subset as removed. |
| `slatec-pchip-directory` | relocated copy | `slatec-source-directory` | Netlib explicitly describes the PCHIP subset as removed. |
| `slatec-lin-directory` | possible duplicate | `upstream-blas` | Same-named/reference BLAS routines are present, but exact revisions and local edits are not established. |
| `slatec-lin-directory` | possible duplicate | `upstream-linpack` | Exact source comparison required. |
| `slatec-lin-directory` | possible duplicate | `upstream-eispack` | Exact source comparison required. |
| `slatec-lin-directory` | possible duplicate | `upstream-slap-single` | SLAP-labelled membership is indicated by the parent index; Version 2.0 identity is not established. |
| `slatec-lin-directory` | possible duplicate | `upstream-slap-double` | Same qualification. |
| `slatec-fishfft-directory` | possible duplicate | `upstream-fishpack` | Exact source and `changes` comparison required. |
| `slatec-fishfft-directory` | possible duplicate | `upstream-fftpack` | Exact source comparison required; standalone directory also contains later clones/translations. |
| `slatec-fnlib-directory` | unknown | `upstream-fn` | Netlib explicitly says it does not know which is newer or more bug-free. |
| `slatec-pchip-directory` | possible duplicate | `upstream-pchip` | Netlib says “see also /pchip”; exact identity unverified. |
| `slatec-spfun` | unknown | `slatec-fnlib-directory` | Both concern elementary/special functions; ownership and overlap uninspected. |
| `slatec-spfun` | unknown | `upstream-fn` | Exact overlap uninspected. |
| `slatec-tree1` | generated dependency product | `slatec-source-archive` | Likely generated from a SLATEC snapshot, but generator and input snapshot are unknown. |
| `slatec-tree` | generated dependency product | `slatec-tree1` | Appears to be transitive closure or related product; generation rules are unknown. |
| `netlib-plus-dependencies` | generated dependency product | package source directories | Dynamic closure service observed on package indexes; output may reflect current Netlib metadata. |
| `slatec-linux-archive` | machine-specific alternative | `slatec-source-archive` | Linux-oriented variant; modifications and provenance must be inspected. |
| `slatec-machine-constants` | machine-specific alternative | `slatec-source-directory` | Machine alternatives are embedded/commented within main sources. |
| `slatec-machine-error-hooks` | machine-specific alternative | `slatec-source-directory` | Site-replaceable FDUMP/XERHLT behavior is described by the index. |
| `slatec-quick-check-directory` | possible duplicate | `slatec-quick-check-archive` | Both are advertised as quick checks; file inventories were not compared. |
| `slatec-spfunchk` | documentation-only | `slatec-spfun` | Test/validation companion, not numerical implementation. |
| `slatec-guide` | documentation-only | SLATEC distribution artifacts | Governs historical policy and format, not executable membership. |
| `slatec-toc` | documentation-only | SLATEC source artifacts | Catalogue evidence; source declarations remain closer to executable facts. |
| `slatec-routine-list` | documentation-only | SLATEC source artifacts | Completeness cross-check only until inspected. |
| `slatec-slprep` | documentation-only | SLATEC prologues | Tool may reveal parsing conventions but is not numerical source. |
| `upstream-quadpack` | independent upstream package | SLATEC routines | Known relevant package family; exact incorporated revision/files require comparison. |
| `upstream-minpack` | independent upstream package | SLATEC routines | Relevant nonlinear family; exact derivation and adaptations require comparison. |
| `upstream-dassl` | independent upstream package | SLATEC routines | Relevant DAE family; endpoint and exact relationship uninspected. |
| `upstream-odepack` | independent upstream package | SLATEC routines | Relevant ODE family; may be related rather than directly incorporated. |
| `upstream-amos` | independent upstream package | SLATEC special functions | Candidate upstream provenance for Amos-associated routines; exact membership unverified. |
