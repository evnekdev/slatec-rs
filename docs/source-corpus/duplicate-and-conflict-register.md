# Duplicate and conflict register

No provider is selected by this register.

| ID | Program unit or set | Providers/candidates | Status | Measured or required evidence |
|---|---|---|---|---|
| `DUP-001` | pinned `main-src` subset versus live SLATEC-hosted trees | `4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc` archive; live `src/` plus relocated subsets | `unresolved` | The 735-unit archive subset is inventoried; live and relocated bytes remain required for a complete-library audit. |
| `DUP-002` | source archive internal declarations | 735 `.f` files | `verified-identical` as uniqueness result, not provider identity | Deterministic scanner found 735 unique names and zero duplicate declared names within this archive. |
| `DUP-003` | linear algebra providers | archive; `slatec/lin`; BLAS; LINPACK; EISPACK; SLAP | `possible-duplicate` | External provider bytes not compared. |
| `DUP-004` | FISHPACK/FFTPACK providers | archive; relocated subset; standalone packages | `possible-duplicate` | External provider bytes not compared. |
| `DUP-005` | FNLIB/FN/special-function providers | archive; `slatec/fnlib`; `/fn`; `spfun` | `unresolved` | Netlib records relative recency/correctness uncertainty. |
| `DUP-006` | PCHIP providers | archive; relocated subset; standalone PCHIP | `possible-duplicate` | External provider bytes not compared. |
| `DUP-007` | quick checks | `a095f74665e165fa1a4bd3f9ab6a4573135e21b1d002c05607eb9394e1c0f2ca` archive; live `chk/` | `possible-duplicate` | Archive has 406 units and exactly 54 numbered drivers; live bytes required. |
| `DUP-008` | Linux machine constants | source `D1MACH`, `I1MACH`, `R1MACH`; Linux overlay copies | `verified-different` | Raw and executable-normalized hashes differ for all three. |
| `DUP-009` | `D1MACH` provider | source archive; Linux `DLAMCH`-based overlay | `alternate-implementation` | Linux version adds LAPACK dependency and cached runtime inquiry. |
| `DUP-010` | `R1MACH` provider | source archive; Linux `SLAMCH`-based overlay | `alternate-implementation` | Linux version adds LAPACK dependency and cached runtime inquiry. |
| `DUP-011` | `I1MACH` provider | source archive selectable blocks; Linux IBM-PC selection | `alternate-implementation` | Linux overlay activates one constant block. |
| `DUP-012` | `SGEIR` history | `sgeir.f`; retained `sgeir.f.0` | `verified-different` | Archive `changes` records an executable conditional correction in 1999. Parse the `.0` provider before any historical build. |
| `DUP-013` | quadrature constants | corrected `qk15w.f`, `dqk15w.f`; earlier copies elsewhere | `possible-duplicate` | Archive `changes` records coefficient corrections dated 2023. Locate prior providers for exact pairwise diff. |
| `DUP-014` | `RD` documentation history | current `rd.f`; earlier copy | `possible-duplicate` | Archive `changes` records a 1994 comment correction. |
| `DUP-015` | old error package | `slatec/err`; main error subsystem | `possible-duplicate` | Live `err/` bytes and state/symbol comparison required. |
| `DUP-016` | site hooks | `FDUMP`, `XERHLT` defaults and replacements | `alternate-implementation` | Provider and runtime policy required. |
| `DUP-017` | dependency closures | `tree1`; `tree`; plus-dependencies; parsed calls; object symbols | `verified-different` | Distinct evidence models; preserve edge provenance. |
| `DUP-018` | browser/tool archive | `slatecm.tgz`; numerical source | `verified-different` | Archive contains C/browser assets, not a numerical provider; its copyright notice is separately restrictive and must not be generalized to SLATEC source. |

## Rights-related conflict

`slatecm.tgz` contains an explicit University of Michigan notice governing the browser software. That notice is artifact-specific. It is evidence against applying one blanket rights conclusion across all Netlib-hosted SLATEC materials.

## Provider policy pending E03

- retain source-set-qualified identities;
- keep archive checksum and retrieval date in every derived record;
- reject duplicate program-unit providers unless an explicit resolution exists;
- preserve corrected and historical variants separately;
- never let directory order or extraction order choose a provider;
- retain unresolved relationships for E03 rather than editing uncertainty away.
