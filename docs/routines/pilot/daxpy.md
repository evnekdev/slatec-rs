---
id: daxpy
name: DAXPY
source_file: slatec/lin/daxpy.f
source_url: https://www.netlib.org/slatec/lin/daxpy.f
source_access: linked-unrendered
kind: subroutine
precision: double
gams_codes: ["D1A7"]
user_callable: true
origin_package: blas
candidate_domain: linear-algebra-kernels
---

# `DAXPY`

Compute DY := DA*DX + DY with arbitrary vector increments.

Official source: [slatec/lin/daxpy.f](https://www.netlib.org/slatec/lin/daxpy.f).

## Call sequence

```fortran
CALL DAXPY(N, DA, DX, INCX, DY, INCY)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `inferred` |
| `DA` | `double` | `in` | `inferred` |
| `DX` | `double[]` | `in` | `inferred` |
| `INCX` | `integer` | `in` | `inferred` |
| `DY` | `double[]` | `inout` | `inferred` |
| `INCY` | `integer` | `in` | `inferred` |

## Classification and provenance

- GAMS: `D1A7`
- User-callable: `true`
- Origin package/family: `blas`
- Candidate project domain: `linear-algebra-kernels`
- Source access: `linked-unrendered`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

No item was verified in this pilot pass.

## Direct dependencies

No item was verified in this pilot pass.

## Related routines

- SAXPY
- CAXPY

## Unresolved ABI questions

- Negative and non-unit increments require checked starting-offset calculations.

## Authorship and revisions

Authors named by the source/package: J. J. Dongarra.

BLAS routine; exact SLATEC-copy revision requires source comparison.

## Evidence note

The official Netlib path was located, but its body was not rendered during this pass. Interface details are provisional and must be checked before binding generation.
