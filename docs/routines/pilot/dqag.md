---
id: dqag
name: DQAG
source_file: quadpack/dqag.f
source_url: https://www.netlib.org/quadpack/dqag.f
source_access: verified
kind: subroutine
precision: double
gams_codes: ["H2A1A1"]
user_callable: true
origin_package: quadpack
candidate_domain: quadrature
---

# `DQAG`

Integrate a user function over a finite interval using globally adaptive Gauss-Kronrod quadrature.

Official source: [quadpack/dqag.f](https://www.netlib.org/quadpack/dqag.f).

## Call sequence

```fortran
CALL DQAG(F,A,B,EPSABS,EPSREL,KEY,RESULT,ABSERR,NEVAL,IER,LIMIT,LENW,LAST,IWORK,WORK)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `F` | `external function` | `ext` | `verified` |
| `A` | `double` | `in` | `verified` |
| `B` | `double` | `in` | `verified` |
| `EPSABS` | `double` | `in` | `verified` |
| `EPSREL` | `double` | `in` | `verified` |
| `KEY` | `integer` | `in` | `verified` |
| `RESULT` | `double` | `out` | `verified` |
| `ABSERR` | `double` | `out` | `verified` |
| `NEVAL` | `integer` | `out` | `verified` |
| `IER` | `integer` | `out` | `verified` |
| `LIMIT` | `integer` | `in` | `verified` |
| `LENW` | `integer` | `in` | `verified` |
| `LAST` | `integer` | `out` | `verified` |
| `IWORK` | `integer[]` | `work` | `verified` |
| `WORK` | `double[]` | `work` | `verified` |

## Classification and provenance

- GAMS: `H2A1A1`
- User-callable: `true`
- Origin package/family: `quadpack`
- Candidate project domain: `quadrature`
- Source access: `verified`

## Callbacks

- DOUBLE PRECISION FUNCTION F(X); DOUBLE PRECISION X

## Work arrays and persistent state

- IWORK length at least LIMIT.
- WORK length at least 4*LIMIT; partitions hold interval endpoints, results, and errors.

## Documented errors and status

- IER reports invalid input, subdivision exhaustion, roundoff, bad integrand behavior, or nonconvergence.

## Direct dependencies

- DQAGE
- XERROR

## Related routines

- DQAGS
- DQAGP
- DQAGI

## Unresolved ABI questions

- Callback has no explicit user-data parameter or portable error channel.
- Workspace arithmetic must be checked before allocation.

## Authorship and revisions

Authors named by the source/package: R. Piessens, E. de Doncker.

QUADPACK legacy prologue; exact SLATEC revision requires archive comparison.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
