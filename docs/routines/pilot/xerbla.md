---
id: xerbla
name: XERBLA
source_file: slatec/lin/xerbla.f
source_url: https://www.netlib.org/slatec/lin/xerbla.f
source_access: linked-unrendered
kind: subroutine
precision: all
gams_codes: ["R3C"]
user_callable: false
origin_package: blas
candidate_domain: runtime-support
---

# `XERBLA`

Report an illegal argument detected by a BLAS-style routine.

Official source: [slatec/lin/xerbla.f](https://www.netlib.org/slatec/lin/xerbla.f).

## Call sequence

```fortran
CALL XERBLA(SRNAME, INFO)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `SRNAME` | `character` | `in` | `inferred` |
| `INFO` | `integer` | `in` | `inferred` |

## Classification and provenance

- GAMS: `R3C`
- User-callable: `false`
- Origin package/family: `blas`
- Candidate project domain: `runtime-support`
- Source access: `linked-unrendered`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- This routine is itself the BLAS illegal-argument reporting hook.

## Direct dependencies

No item was verified in this pilot pass.

## Related routines

- XERMSG

## Unresolved ABI questions

- Character hidden-length ABI and termination/printing behavior must be tested.

## Authorship and revisions

Authorship was not extracted in this pilot pass.

Reference BLAS support routine; exact SLATEC adaptation requires source inspection.

## Evidence note

The official Netlib path was located, but its body was not rendered during this pass. Interface details are provisional and must be checked before binding generation.
