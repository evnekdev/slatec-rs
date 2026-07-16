# Relocated SLATEC subsets

## Distribution-level finding

Netlib explicitly identifies four subdirectories as subsets removed from `slatec/src` to make the main source more accessible. This verifies the **relocation relationship**, but not byte identity, archive inclusion, package membership of every file, or revision equality with standalone upstream packages.

| Relocated directory | Netlib description | Relationship to live `src/` | Byte status | Important qualification |
|---|---|---|---|---|
| `slatec/lin/` | BLAS, LINPACK, EISPACK, SLAP subset | relocated copy | unresolved | Co-location combines several historical packages and does not prove exact upstream versions. |
| `slatec/fishfft/` | FISHPACK, FFTPACK subset | relocated copy | unresolved | At least two upstream families share the directory; shared support may complicate ownership. |
| `slatec/fnlib/` | FNLIB subset | relocated copy | unresolved | Netlib explicitly records uncertainty versus `/fn`. |
| `slatec/pchip/` | piecewise cubic Hermite approximation | relocated copy | unresolved | Standalone `/pchip` must be compared separately. |

## Linear algebra subset

Every file and program unit in `slatec/lin/` must be classified independently as BLAS, LINPACK, EISPACK, SLAP, SLATEC driver/support, or unresolved. The parent label is orientation, not ownership evidence.

Required duplicate checks include:

- same-named BLAS kernels versus `/blas`;
- LINPACK factorization/solve routines versus `/linpack`;
- EISPACK eigensystem routines versus `/eispack`;
- single- and double-precision SLAP 2.0 archives versus SLAP-labelled routines;
- duplicate machine/error support pulled by package-specific dependencies;
- same symbol supplied by more than one upstream family.

## FISHPACK/FFTPACK subset

The combined directory must be split by program-unit evidence, not merely by file naming. Compare:

- transform initializers, forward/backward transforms and internal radix kernels against `/fftpack`;
- elliptic/PDE solvers and their support against `/fishpack`;
- shared transform routines used by FISHPACK;
- revision markers and upstream `changes` records;
- precision variants, especially any later double-precision FFTPACK clone, which is an independent alternative until proved otherwise.

## FNLIB subset

The relationship among `slatec/fnlib/`, `/fn`, and `slatec/spfun` remains unresolved. Netlib itself states that it did not know whether `/fn` or `slatec/fnlib` was newer or more bug-free.

Inspection of `slatec/fnlib/besi0.f` confirms a SLATEC 4.x-style prologue, `C***LIBRARY SLATEC (FNLIB)`, revision history, machine-constant dependencies, and `XERMSG` integration. This does not establish whether the numerical coefficient data and executable statements match `/fn/besi0.f`.

The local comparison must classify each matching unit as:

- exact copy;
- prologue-only SLATEC adaptation;
- error/machine-support adaptation;
- corrected coefficient or executable change;
- renamed precision variant;
- present in only one collection.

## PCHIP subset

Inspection of `slatec/pchip/pchim.f` confirms SLATEC integration details: `*DECK`, `C***LIBRARY SLATEC (PCHIP)`, a detailed revision history, `PCHST` dependency, and `XERMSG` error calls. Comparison with standalone `/pchip` must determine whether differences are only distribution/prologue integration or include executable corrections.

## Required local output

For each relocated directory, generate:

1. complete file manifest;
2. program-unit manifest;
3. archive-member match, if present;
4. upstream file/program-unit match;
5. raw, newline-normalized, fixed-form-normalized and executable-token hashes;
6. difference classification;
7. duplicate-symbol and missing/additional-unit list;
8. revision-marker comparison;
9. unresolved manual-review queue.
