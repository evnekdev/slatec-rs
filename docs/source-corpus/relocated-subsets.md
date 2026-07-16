# Relocated SLATEC subsets

## Distribution relationship

Netlib's root index explicitly describes `lin/`, `fishfft/`, `fnlib/`, and `pchip/` as source groups removed from the live `slatec/src/` directory. This verifies relocation as a distribution-layout fact. It does not prove that current live subset files are byte-identical to the corresponding members of the supplied source archive.

## Evidence available from the source archive

The archive contains one flat `src/` directory with 735 Fortran source files. The unit inventory includes names associated with all four relocated families, so the current archive is broader than the current live main `src/` surface described by Netlib. Exact family membership remains evidence-driven and must not be inferred solely from names.

| Relocated set | Archive-side evidence | Relationship status | Required completion work |
|---|---|---|---|
| `slatec/lin` | BLAS-, LINPACK-, EISPACK-, and SLAP-associated names occur in the 735-unit archive inventory. | `possible-duplicate` at file/unit level; relocation verified at layout level | Snapshot every live file; pair by path and unit; classify upstream and SLATEC adaptations. |
| `slatec/fishfft` | FISHPACK and FFTPACK routine families occur in the archive. | `possible-duplicate` | Snapshot directory; separate shared support from family ownership; compare with standalone packages. |
| `slatec/fnlib` | Elementary/special-function families occur in the archive; archive `changes` proves later corrections can be applied in place. | `unresolved` relative to `/fn`; `possible-duplicate` relative to archive | Download both collections and `spfun`; compare coefficients, revision blocks, error calls, and executable tokens. |
| `slatec/pchip` | Single- and double-precision PCHIP units occur in the archive. | `possible-duplicate` | Download relocated and standalone PCHIP copies and compare every precision pair. |

## Important correction to the earlier E02 state

The source archive is now known to contain post-1993 corrections. Consequently, “archive copy” must not be equated with “original Version 4.1 copy.” Every relocated/live comparison must cite the exact archive SHA-256 and live retrieval date.

## No provider selection

No relocated subset replaces the archive copy in this task. E03 must define a policy for retaining original, corrected, relocated, and upstream evidence without silently collapsing them.
