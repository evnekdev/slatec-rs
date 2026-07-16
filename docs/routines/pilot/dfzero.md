---
id: dfzero
name: DFZERO
source_file: slatec/src/dfzero.f
source_url: https://www.netlib.org/slatec/src/dfzero.f
source_access: verified
kind: subroutine
precision: double
gams_codes: ["F1A"]
user_callable: true
origin_package: individual
candidate_domain: nonlinear-equations
---

# `DFZERO`

Find a zero of a scalar function in an interval using bisection and secant interpolation.

Official source: [slatec/src/dfzero.f](https://www.netlib.org/slatec/src/dfzero.f).

## Call sequence

```fortran
CALL DFZERO(F, B, C, R, RE, AE, IFLAG)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `F` | `external function` | `ext` | `verified` |
| `B` | `double` | `inout` | `verified` |
| `C` | `double` | `inout` | `verified` |
| `R` | `double` | `in` | `verified` |
| `RE` | `double` | `in` | `verified` |
| `AE` | `double` | `in` | `verified` |
| `IFLAG` | `integer` | `out` | `verified` |

## Classification and provenance

- GAMS: `F1A`
- User-callable: `true`
- Origin package/family: `individual`
- Candidate project domain: `nonlinear-equations`
- Source access: `verified`

## Callbacks

- DOUBLE PRECISION FUNCTION F(X); DOUBLE PRECISION X

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- IFLAG distinguishes successful contraction, endpoint roots, possible singularity, no sign change, and excessive evaluations.

## Direct dependencies

- D1MACH

## Related routines

- FZERO

## Unresolved ABI questions

- Callback has no user-data argument.
- B and C are both inputs and outputs.

## Authorship and revisions

Authors named by the source/package: L. F. Shampine, H. A. Watts.

Sandia zero-finder; revision history retained in source.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
