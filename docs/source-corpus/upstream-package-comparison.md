# Upstream package comparison

## Scope and current evidence

The supplied archives allow a complete internal inventory of the current Netlib SLATEC source download, but no standalone upstream package bytes were supplied. Therefore package relationships remain candidate comparisons rather than verified identity claims.

The source archive contains 735 unique declared Fortran program-unit names and no duplicate declarations within the archive. This removes one internal duplicate-definition concern for this particular checksum, but it does not resolve collisions introduced by combining it with upstream or relocated source sets.

| Upstream candidate | SLATEC comparison surface | Current status | Evidence still required |
|---|---|---|---|
| BLAS | archive and `slatec/lin` | `possible-duplicate` | exact routine/version map; source and executable-token hashes; error-handler changes |
| LINPACK | archive and `slatec/lin` | `possible-duplicate` | compare each same-named unit and retained revision text |
| EISPACK | archive and `slatec/lin` | `possible-duplicate` | unit and driver/test comparison |
| SLAP 2.0 | archive and `slatec/lin` | `possible-duplicate` | compare single/double archives, storage helpers, and callbacks |
| FISHPACK | archive and `slatec/fishfft` | `possible-duplicate` | compare source plus upstream `changes` evidence |
| FFTPACK | archive and `slatec/fishfft` | `possible-duplicate` | distinguish original precision families from later clones |
| FN | archive, `slatec/fnlib`, and `spfun` | `unresolved` | exhaustive coefficient/revision comparison; Netlib itself reports uncertain relative correctness |
| PCHIP | archive, relocated subset, standalone package | `possible-duplicate` | all single/double unit pairs and chronology |
| QUADPACK | archive routines and standalone QUADPACK | `possible-duplicate` | compare legacy prologues, error adaptation, and body hashes |
| MINPACK | archive routines/derivatives and standalone MINPACK | `possible-duplicate` | lineage, rename, callback, and body comparison |
| DASSL | archive DAE family and standalone DASSL | `possible-duplicate` | exact unit/revision map |
| ODEPACK | archive ODE families and ODEPACK | `unresolved` | distinguish ancestry from functional similarity |
| AMOS | archive complex special functions and AMOS | `possible-duplicate` | complete unit pairing and revision comparison |

## Post-release correction consequence

The source archive's `changes` file records modifications in 1994, 1999, and 2023. Upstream comparison must therefore support three-way chronology where possible:

1. original package revision;
2. historical SLATEC incorporation/adaptation;
3. later Netlib correction applied to the downloadable SLATEC archive.

A same-named file must remain a separate provider until all three dimensions are resolved.
