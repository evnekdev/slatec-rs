---
id: zbesj
name: ZBESJ
source_file: slatec/src/zbesj.f
source_url: https://www.netlib.org/slatec/src/zbesj.f
source_access: verified
kind: subroutine
precision: complex-double-pair
gams_codes: ["C10A3"]
user_callable: true
origin_package: amos-special-functions
candidate_domain: special-functions
---

# `ZBESJ`

Compute a sequence of complex Bessel J values for a complex argument and nonnegative order.

Official source: [slatec/src/zbesj.f](https://www.netlib.org/slatec/src/zbesj.f).

## Call sequence

```fortran
CALL ZBESJ(ZR,ZI,FNU,KODE,N,CYR,CYI,NZ,IERR)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `ZR` | `double` | `in` | `verified` |
| `ZI` | `double` | `in` | `verified` |
| `FNU` | `double` | `in` | `verified` |
| `KODE` | `integer` | `in` | `verified` |
| `N` | `integer` | `in` | `verified` |
| `CYR` | `double[]` | `out` | `verified` |
| `CYI` | `double[]` | `out` | `verified` |
| `NZ` | `integer` | `out` | `verified` |
| `IERR` | `integer` | `out` | `verified` |

## Classification and provenance

- GAMS: `C10A3`
- User-callable: `true`
- Origin package/family: `amos-special-functions`
- Candidate project domain: `special-functions`
- Source access: `verified`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- IERR reports input, overflow/range, significance, or algorithm failures; NZ counts underflowed components.

## Direct dependencies

- ZBINU
- ZBIRY
- ZMLRI
- ZRATI
- ZS1S2
- ZUCHK

## Related routines

- ZBESY
- ZBESI
- ZBESK
- CBESJ

## Unresolved ABI questions

- Complex values are represented as separate real/imaginary arrays, avoiding native complex ABI but requiring paired-length validation.

## Authorship and revisions

Authors named by the source/package: Donald E. Amos.

Amos sequence routine; source prologue contains algorithm revisions.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
