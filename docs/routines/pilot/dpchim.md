---
id: dpchim
name: DPCHIM
source_file: slatec/pchip/dpchim.f
source_url: https://www.netlib.org/slatec/pchip/dpchim.f
source_access: verified
kind: subroutine
precision: double
gams_codes: ["E1A"]
user_callable: true
origin_package: pchip
candidate_domain: interpolation
---

# `DPCHIM`

Set derivatives for a monotone piecewise cubic Hermite interpolant.

Official source: [slatec/pchip/dpchim.f](https://www.netlib.org/slatec/pchip/dpchim.f).

## Call sequence

```fortran
CALL DPCHIM(N, X, F, D, INCFD, IERR)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `verified` |
| `X` | `double[]` | `in` | `verified` |
| `F` | `double[INCFD,*]` | `in` | `verified` |
| `D` | `double[INCFD,*]` | `out` | `verified` |
| `INCFD` | `integer` | `in` | `verified` |
| `IERR` | `integer` | `out` | `verified` |

## Classification and provenance

- GAMS: `E1A`
- User-callable: `true`
- Origin package/family: `pchip`
- Candidate project domain: `interpolation`
- Source access: `verified`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- IERR reports invalid dimensions or non-increasing X; positive values may count monotonicity switches.

## Direct dependencies

- DPCHST
- XERMSG

## Related routines

- DPCHIC
- DPCHFE
- DPCHFD

## Unresolved ABI questions

- Strided F/D arrays use INCFD and are not ordinary contiguous Rust slices.

## Authorship and revisions

Authors named by the source/package: Fred N. Fritsch.

PCHIP routine with Version 4 prologue and later maintenance revisions.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
