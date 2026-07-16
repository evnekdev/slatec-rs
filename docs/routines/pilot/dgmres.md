---
id: dgmres
name: DGMRES
source_file: slatec/lin/dgmres.f
source_url: https://www.netlib.org/slatec/lin/dgmres.f
source_access: verified
kind: subroutine
precision: double
gams_codes: ["D2A4"]
user_callable: true
origin_package: slap
candidate_domain: sparse-linear-algebra
---

# `DGMRES`

Solve a sparse linear system using preconditioned restarted GMRES.

Official source: [slatec/lin/dgmres.f](https://www.netlib.org/slatec/lin/dgmres.f).

## Call sequence

```fortran
CALL DGMRES(N,B,X,NELT,IA,JA,A,ISYM,MATVEC,MSOLVE,ITOL,TOL,ITMAX,ITER,ERR,IERR,IUNIT,SB,SX,RGWK,LRGW,IWGK,LIW)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `verified` |
| `B` | `double[]` | `in` | `verified` |
| `X` | `double[]` | `inout` | `verified` |
| `NELT` | `integer` | `in` | `verified` |
| `IA` | `integer[]` | `inout` | `verified` |
| `JA` | `integer[]` | `inout` | `verified` |
| `A` | `double[]` | `inout` | `verified` |
| `ISYM` | `integer` | `in` | `verified` |
| `MATVEC` | `external subroutine` | `ext` | `verified` |
| `MSOLVE` | `external subroutine` | `ext` | `verified` |
| `ITOL` | `integer` | `in` | `verified` |
| `TOL` | `double` | `inout` | `verified` |
| `ITMAX` | `integer` | `in` | `verified` |
| `ITER` | `integer` | `out` | `verified` |
| `ERR` | `double` | `out` | `verified` |
| `IERR` | `integer` | `out` | `verified` |
| `IUNIT` | `integer` | `in` | `verified` |
| `SB` | `double[]` | `in` | `verified` |
| `SX` | `double[]` | `in` | `verified` |
| `RGWK` | `double[]` | `work` | `verified` |
| `LRGW` | `integer` | `in` | `verified` |
| `IWGK` | `integer[]` | `work` | `verified` |
| `LIW` | `integer` | `in` | `verified` |

## Classification and provenance

- GAMS: `D2A4`
- User-callable: `true`
- Origin package/family: `slap`
- Candidate project domain: `sparse-linear-algebra`
- Source access: `verified`

## Callbacks

- MATVEC computes A*x or a documented operator action.
- MSOLVE applies the preconditioner/approximate solve.

## Work arrays and persistent state

- RGWK and IWGK lengths depend on restart and problem options; source checks minimum sizes.

## Documented errors and status

- IERR reports convergence, iteration/workspace/input failures, and callback-related conditions.

## Direct dependencies

- D1MACH
- DCOPY
- DNRM2
- DPIGMR
- DSCAL
- XERMSG

## Related routines

- DSDGMR
- DSLUGM
- DBCG

## Unresolved ABI questions

- Sparse arrays may be reordered or converted in place.
- Two callbacks and Fortran logical-unit diagnostics complicate reentrancy.

## Authorship and revisions

Authors named by the source/package: Mark K. Seager.

SLAP GMRES driver; source prologue records package revisions.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
