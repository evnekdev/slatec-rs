---
id: dsmv
name: DSMV
source_file: slatec/lin/dsmv.f
source_url: https://www.netlib.org/slatec/lin/dsmv.f
source_access: linked-unrendered
kind: subroutine
precision: double
gams_codes: ["D1B8"]
user_callable: true
origin_package: slap
candidate_domain: sparse-linear-algebra
---

# `DSMV`

Multiply a sparse matrix in SLAP Column format by a vector.

Official source: [slatec/lin/dsmv.f](https://www.netlib.org/slatec/lin/dsmv.f).

## Call sequence

```fortran
CALL DSMV(N, X, Y, NELT, IA, JA, A, ISYM)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `inferred` |
| `X` | `double[]` | `in` | `inferred` |
| `Y` | `double[]` | `out` | `inferred` |
| `NELT` | `integer` | `in` | `inferred` |
| `IA` | `integer[]` | `in` | `inferred` |
| `JA` | `integer[]` | `in` | `inferred` |
| `A` | `double[]` | `in` | `inferred` |
| `ISYM` | `integer` | `in` | `inferred` |

## Classification and provenance

- GAMS: `D1B8`
- User-callable: `true`
- Origin package/family: `slap`
- Candidate project domain: `sparse-linear-algebra`
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

- DSMTV
- DS2Y

## Unresolved ABI questions

- SLAP Column indexing, diagonal placement, and symmetry conventions require exact validation.

## Authorship and revisions

Authorship was not extracted in this pilot pass.

SLAP sparse kernel; exact history in source.

## Evidence note

The official Netlib path was located, but its body was not rendered during this pass. Interface details are provisional and must be checked before binding generation.
