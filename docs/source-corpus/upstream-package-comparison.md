# Standalone upstream package comparison

## Scope and status

This page records every upstream overlap identified by E01. Direct family-wide byte comparison was not possible because archive downloads were unavailable and several individual endpoints could not be rendered. Statuses therefore remain conservative.

| SLATEC source set | Upstream candidate | Status | Evidence obtained | Next comparison |
|---|---|---|---|---|
| `slatec/lin/` | reference BLAS | `possible-duplicate` | Netlib names BLAS in the relocated subset and points to `/blas`. | Pair all BLAS program units; detect SLATEC prologue/error adaptations and version drift. |
| `slatec/lin/` | LINPACK | `possible-duplicate` | Netlib names LINPACK and points to `/linpack`; upstream `dgefa.f` is an older compact LINPACK source form. | Compare each same-named unit with relocated SLATEC source, including executable-token hashes. |
| `slatec/lin/` | EISPACK | `possible-duplicate` | Netlib names EISPACK and points to `/eispack`. | Compare program-unit inventories, precision variants and tests. |
| `slatec/lin/` | SLAP 2.0 single | `possible-duplicate` | Netlib names SLAP; standalone archive advertises Version 2.0. | Extract `slap.tgz`, map units and revision markers. |
| `slatec/lin/` | SLAP 2.0 double | `possible-duplicate` | Same family evidence. | Extract `dlap.tgz`; detect precision conversion and local changes. |
| `slatec/fishfft/` | FISHPACK | `possible-duplicate` | Netlib names FISHPACK and points to `/fishpack`. | Compare files, support routines, `changes`, and embedded FFT dependencies. |
| `slatec/fishfft/` | FFTPACK | `possible-duplicate` | Netlib names FFTPACK and points to `/fftpack`. | Compare original Fortran units and revision markers; exclude later translations/clones unless separately classified. |
| `slatec/fishfft/` | FFTPACK double clone | `alternate-implementation` | E01 identified a separately advertised later double-precision clone. | Keep separate unless program-unit lineage and equivalence are proved. |
| `slatec/fnlib/` | `/fn` | `unresolved` | Netlib explicitly states that relative recency/correctness is unknown. | Compare every overlapping unit, coefficient table, revision record and executable token stream. |
| `slatec/fnlib/` | `slatec/spfun` | `unresolved` | Both concern elementary/special functions; exact contents were not inspected together. | Parse all bundled program units and compare names and bodies. |
| `slatec/pchip/` | `/pchip` | `possible-duplicate` | Netlib says “see also /pchip”; selected SLATEC source shows SLATEC-specific integration. | Compare all matching S/D variants and revision histories. |
| main/relocated SLATEC units | QUADPACK | `possible-duplicate` | Standalone package is a known relevant family. | Match by program-unit name, then classify SLATEC error/machine adaptations and source revisions. |
| main SLATEC units | MINPACK | `possible-duplicate` | Same-named nonlinear routines and family evidence exist, but derivation may be adapted. | Compare complete program-unit sets, callbacks and error handling. |
| main SLATEC units | DASSL | `possible-duplicate` | Standalone DASSL is a candidate for DAE routine lineage. | Compare DASSL and supporting routines by source and revision markers. |
| main SLATEC units | ODEPACK/related ODE families | `unresolved` | Functional and historical relationship is plausible, but direct incorporation is not established. | Build name/signature/source similarity map; do not infer ownership from domain. |
| main SLATEC special functions | AMOS | `possible-duplicate` | Current `src/` index attributes many complex Bessel/Airy units to D. E. Amos. | Compare exact AMOS package files, names, coefficient data, prologues and dependencies. |

## Representative source observations

### LINPACK form

The inspected upstream `linpack/dgefa.f` is a compact historical LINPACK source identifying an 08/14/78 version and BLAS dependencies `DAXPY`, `DSCAL`, and `IDAMAX`. The corresponding SLATEC relocated file could not be rendered in this pass, so no direct identity or difference claim is made.

### PCHIP integration

`slatec/pchip/pchim.f` contains a SLATEC prologue, a revision history through 1992, and a recorded 1990 conversion from `XERROR` to `XERMSG`. This establishes a concrete SLATEC adaptation history. It does not prove whether the executable interpolation formulas differ from standalone PCHIP.

### FNLIB integration

`slatec/fnlib/besi0.f` contains SLATEC/FNLIB identification, machine-constant calls, `XERMSG`, and revision-history updates. Comparison with `/fn` remains unresolved because the upstream file could not be fetched in this pass.

## Comparison rules

- Same routine name is only a candidate match.
- Precision-converted versions must be separate records linked by a conversion relationship.
- SLATEC prologue conversion and error-system integration must be recorded even if executable numerical code is identical.
- Coefficient-table changes count as executable/numerical differences.
- A newer date does not automatically make a source canonical.
- Package-index co-location does not determine ownership.
