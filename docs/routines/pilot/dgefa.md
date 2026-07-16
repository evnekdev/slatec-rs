---
id: dgefa
name: DGEFA
source_file: slatec/lin/dgefa.f
source_url: https://www.netlib.org/slatec/lin/dgefa.f
source_access: linked-unrendered
kind: subroutine
precision: double
gams_codes: ["D2A1"]
user_callable: true
origin_package: linpack
candidate_domain: dense-linear-algebra
---

# `DGEFA`

Factor a general double-precision matrix by Gaussian elimination with partial pivoting.

Official source: [slatec/lin/dgefa.f](https://www.netlib.org/slatec/lin/dgefa.f).

## Call sequence

```fortran
CALL DGEFA(A, LDA, N, IPVT, INFO)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `A` | `double[LDA,*]` | `inout` | `inferred` |
| `LDA` | `integer` | `in` | `inferred` |
| `N` | `integer` | `in` | `inferred` |
| `IPVT` | `integer[]` | `out` | `inferred` |
| `INFO` | `integer` | `out` | `inferred` |

## Classification and provenance

- GAMS: `D2A1`
- User-callable: `true`
- Origin package/family: `linpack`
- Candidate project domain: `dense-linear-algebra`
- Source access: `linked-unrendered`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- INFO identifies a zero pivot; factorization may still be usable for some operations.

## Direct dependencies

- DAXPY
- DSCAL
- IDAMAX

## Related routines

- DGESL
- DGECO
- DGEDI

## Unresolved ABI questions

- Column-major leading dimension and one-based pivot indices must be preserved.
- A is overwritten by LU factors.

## Authorship and revisions

Authors named by the source/package: Cleve Moler.

LINPACK routine; exact incorporated revision requires source diff.

## Evidence note

The official Netlib path was located, but its body was not rendered during this pass. Interface details are provisional and must be checked before binding generation.
