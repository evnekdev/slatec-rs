# Duplicate and conflict register

No provider is selected by this register. Each row is a duplicate/conflict risk requiring explicit resolution or preservation.

| ID | Program unit or set | Providers/candidates | Status | Conflict risk | Required evidence |
|---|---|---|---|---|---|
| `DUP-001` | complete SLATEC source | `slatec_src.tgz`; live `src/` plus relocated subsets | `unresolved` | Missing or modified files if one distribution form is used alone. | Archive/live manifests and hashes. |
| `DUP-002` | linear algebra units | archive; `slatec/lin`; BLAS; LINPACK; EISPACK; SLAP | `possible-duplicate` | Duplicate symbols and mixed revisions. | Program-unit map and executable diffs. |
| `DUP-003` | FISHPACK/FFTPACK units | archive; `slatec/fishfft`; `/fishpack`; `/fftpack` | `possible-duplicate` | Shared transform support and ambiguous owner. | Unit-level dependency and source comparison. |
| `DUP-004` | FNLIB special functions | archive; `slatec/fnlib`; `/fn`; `spfun` | `unresolved` | Netlib records unknown relative correctness; coefficient or fix divergence possible. | Exhaustive per-unit comparison. |
| `DUP-005` | PCHIP | archive; `slatec/pchip`; `/pchip` | `possible-duplicate` | SLATEC error/prologue adaptation and possible corrections. | S/D routine diffs and revision chronology. |
| `DUP-006` | quick checks | `slatec_chk.tgz`; live `chk/` | `possible-duplicate` | Archive/live test drift or missing support files. | Member and file hashes; driver mapping. |
| `DUP-007` | Linux-adapted distribution | `slatec4linux.tgz`; complete archive | `alternate-implementation` | Portability edits may silently change numerical or error behavior. | Classified full archive diff. |
| `DUP-008` | machine constants | `D1MACH`; `I1MACH`; `R1MACH` selectable target blocks or replacements | `alternate-implementation` | Wrong constants or multiple active definitions. | Parse active/commented variants and test target values. |
| `DUP-009` | fatal/error hooks | default and site replacements for `FDUMP`, `XERHLT` | `alternate-implementation` | Different termination/traceback behavior; duplicate symbols. | Provider policy and object-symbol check. |
| `DUP-010` | old error package | `slatec/err`; main SLATEC error subsystem | `possible-duplicate` | `XERSAV`/compatibility symbols may conflict or implement different semantics. | Retrieve `err/`, compare names, COMMON state and call graph. |
| `DUP-011` | QUADPACK routines | SLATEC copies; standalone QUADPACK | `possible-duplicate` | Revision and XERROR/XERMSG adaptation differences. | File and program-unit comparison. |
| `DUP-012` | MINPACK routines | SLATEC copies/derivatives; standalone MINPACK | `possible-duplicate` | Renamed/adapted routines and callback differences. | Signature, body and dependency comparison. |
| `DUP-013` | DASSL/DAE routines | SLATEC DAE units; standalone DASSL | `possible-duplicate` | Different revisions or support routines. | Exact lineage and unit map. |
| `DUP-014` | ODE families | SLATEC DEPAC/DRIVE/related units; ODEPACK | `unresolved` | Functional similarity may be mistaken for direct duplication. | Historical and source-level lineage evidence. |
| `DUP-015` | Amos-associated special functions | SLATEC `src/`; standalone AMOS | `possible-duplicate` | Local prologue/error changes or algorithm revisions. | Complete AMOS-to-SLATEC map. |
| `DUP-016` | documentation tooling support | `subsid`; numerical subsidiaries | `unresolved` | Same symbol names could collide if tooling is linked with numerical corpus. | Parse all program units in tooling sources. |
| `DUP-017` | dependency closures | `tree1`; `tree`; plus-dependencies; parsed calls; object symbols | `verified-different` | These are different evidence products and must not overwrite one another. | Preserve provenance and reconcile edge-by-edge. |
| `DUP-018` | FFTPACK precision variants | SLATEC/FFTPACK originals; later double clone | `alternate-implementation` | Generic API may imply unsupported semantic equivalence. | Precision-family and numerical normalization audit. |

## Verified conflict principles

1. The live distribution layout differs structurally from a single `src/` directory because four subsets are separately exposed.
2. `/fn` and `slatec/fnlib` have unresolved relative recency/correctness according to Netlib itself.
3. Machine constants and fatal/traceback hooks are intentionally site-dependent.
4. Generated dependency products represent different relationship scopes and cannot be treated as interchangeable source truth.

## Provider policy pending E03

Until a canonical corpus policy is approved:

- retain every provider record;
- do not copy same-named files over one another;
- assign unique source-set-qualified identities;
- fail generation on duplicate program-unit definitions unless an explicit resolution record exists;
- preserve alternate implementations even when one is not selected for a build;
- carry unresolved conflicts into metadata and generated reports.
