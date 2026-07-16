---
id: dpchfe
name: DPCHFE
source_file: slatec/pchip/dpchfe.f
source_url: https://www.netlib.org/slatec/pchip/dpchfe.f
source_access: linked-unrendered
kind: subroutine
precision: double
gams_codes: ["E3"]
user_callable: true
origin_package: pchip
candidate_domain: interpolation
---

# `DPCHFE`

Evaluate a piecewise cubic Hermite function at an array of points.

Official source: [slatec/pchip/dpchfe.f](https://www.netlib.org/slatec/pchip/dpchfe.f).

## Call sequence

```fortran
CALL DPCHFE(N,X,F,D,INCFD,SKIP,NE,XE,FE,IERR)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `inferred` |
| `X` | `double[]` | `in` | `inferred` |
| `F` | `double[INCFD,*]` | `in` | `inferred` |
| `D` | `double[INCFD,*]` | `in` | `inferred` |
| `INCFD` | `integer` | `in` | `inferred` |
| `SKIP` | `logical` | `inout` | `inferred` |
| `NE` | `integer` | `in` | `inferred` |
| `XE` | `double[]` | `in` | `inferred` |
| `FE` | `double[]` | `out` | `inferred` |
| `IERR` | `integer` | `out` | `inferred` |

## Classification and provenance

- GAMS: `E3`
- User-callable: `true`
- Origin package/family: `pchip`
- Candidate project domain: `interpolation`
- Source access: `linked-unrendered`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- IERR reports invalid data or extrapolation counts according to the source prologue.

## Direct dependencies

- DCHFEV
- XERMSG

## Related routines

- DPCHIM
- DPCHFD

## Unresolved ABI questions

- Fortran LOGICAL representation is compiler-specific.
- SKIP is mutable protocol state.

## Authorship and revisions

Authors named by the source/package: Fred N. Fritsch.

PCHIP evaluation routine; exact history is in source.

## Evidence note

The official Netlib path was located, but its body was not rendered during this pass. Interface details are provisional and must be checked before binding generation.
