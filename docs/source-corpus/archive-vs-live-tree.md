# Main archive versus live Netlib trees

## Result

The main archive-to-live comparison remains **unresolved** because `slatec_src.tgz` could not be downloaded in the execution container. The Netlib index advertises the archive as the complete SLATEC source and separately says that individual routines can be retrieved from `src/`. The same index states that four subsets were removed from `src/` to `lin/`, `fishfft/`, `fnlib/`, and `pchip/`.

This proves that the present live `src/` directory is not, by itself, the full browsable distribution surface. It does not prove whether the archive contains byte-identical copies of every relocated file.

## Recorded relationships

| Left artifact | Right artifact | Status | Evidence | Required local comparison |
|---|---|---|---|---|
| `slatec-source-archive` | `slatec-source-directory` | `unresolved` | Both are advertised as source distribution forms; live `src/` excludes relocated groups. | Archive member inventory against all live `src/` files by raw and normalized hashes. |
| `slatec-source-archive` | `slatec-lin-directory` | `possible-duplicate` | `lin/` is explicitly a subset removed from `src/`; archive is advertised as complete. | Match archive paths/program units to every `lin/` file. |
| `slatec-source-archive` | `slatec-fishfft-directory` | `possible-duplicate` | Same distribution statement. | Match all FISHPACK/FFTPACK files and support units. |
| `slatec-source-archive` | `slatec-fnlib-directory` | `possible-duplicate` | Same distribution statement. | Match all FNLIB files; include `spfun` and `/fn` comparisons separately. |
| `slatec-source-archive` | `slatec-pchip-directory` | `possible-duplicate` | Same distribution statement. | Match all PCHIP files and program units. |
| `slatec-quick-check-archive` | `slatec-quick-check-directory` | `possible-duplicate` | Archive and directory are both advertised as quick checks; directory is described as 54 drivers. | Compare archive members with `test01.f` through `test54.f` and support files. |
| `slatec-linux-archive` | `slatec-source-archive` | `alternate-implementation` | Root index separately advertises “SLATEC for linux”. Contents were not obtained. | Full archive diff classified as build, machine constants, error hooks, source patches, tests, or docs. |
| `slatec-browser-archive` | numerical source artifacts | `verified-different` | Root index describes it as an experimental browser for libraries using SLATEC prologues. | No numerical-provider comparison; inspect only for parser/tooling evidence. |

## Live-tree observations

The current `src/` index exposes individual fixed-form Fortran files and Netlib plus-dependencies links. It contains source families outside the four relocated groups, including Amos-associated special functions, interpolation, differential-equation, optimization, and other routines. Presence in this directory records distribution placement only and must not be converted into package ownership.

The root index identifies these machine-specific units within the source distribution:

| Program unit | Role | Reconciliation status |
|---|---|---|
| `D1MACH` | double-precision machine constants | `alternate-implementation` |
| `I1MACH` | integer machine constants | `alternate-implementation` |
| `R1MACH` | single-precision machine constants | `alternate-implementation` |
| `FDUMP` | site traceback hook | `alternate-implementation` |
| `XERHLT` | fatal-error termination hook | `alternate-implementation` |

These must be represented as explicit provider/configuration choices. A build must not silently select a commented machine block or replace a site hook.

## Unresolved archive questions

1. Does `slatec_src.tgz` include all four relocated subsets?
2. Are archive copies byte-identical to current live copies?
3. Are files absent from both current `src/` and relocated indexes present in the archive?
4. Does the archive preserve original directory structure or flatten files?
5. Are multiple program units packed into any archive members that do not correspond one-to-one with live files?
6. Does the Linux archive contain later fixes or only portability edits?
7. Does `slatec_chk.tgz` contain exactly the 54 advertised drivers plus support files?
