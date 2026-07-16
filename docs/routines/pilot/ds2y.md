---
id: ds2y
name: DS2Y
source_file: slatec/lin/ds2y.f
source_url: https://www.netlib.org/slatec/lin/ds2y.f
source_access: linked-unrendered
kind: subroutine
precision: double
gams_codes: ["D1B9"]
user_callable: true
origin_package: slap
candidate_domain: sparse-linear-algebra
---

# `DS2Y`

Convert sparse matrix data from SLAP Triad format to SLAP Column format.

Official source: [slatec/lin/ds2y.f](https://www.netlib.org/slatec/lin/ds2y.f).

## Call sequence

```fortran
CALL DS2Y(N, NELT, IA, JA, A, ISYM)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `inferred` |
| `NELT` | `integer` | `inout` | `inferred` |
| `IA` | `integer[]` | `inout` | `inferred` |
| `JA` | `integer[]` | `inout` | `inferred` |
| `A` | `double[]` | `inout` | `inferred` |
| `ISYM` | `integer` | `in` | `inferred` |

## Classification and provenance

- GAMS: `D1B9`
- User-callable: `true`
- Origin package/family: `slap`
- Candidate project domain: `sparse-linear-algebra`
- Source access: `linked-unrendered`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- Invalid sparse structure may be reported through the SLATEC error subsystem.

## Direct dependencies

- QS2I1
- XERMSG

## Related routines

- DSMV
- DSMTV

## Unresolved ABI questions

- Conversion reorders arrays in place and uses one-based indices.
- Duplicate-entry semantics require source verification.

## Authorship and revisions

Authorship was not extracted in this pilot pass.

SLAP storage conversion routine; exact history in source.

## Evidence note

The official Netlib path was located, but its body was not rendered during this pass. Interface details are provisional and must be checked before binding generation.
