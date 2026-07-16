---
id: ddassl
name: DDASSL
source_file: slatec/src/ddassl.f
source_url: https://www.netlib.org/slatec/src/ddassl.f
source_access: verified
kind: subroutine
precision: double
gams_codes: ["I1A2"]
user_callable: true
origin_package: dassl
candidate_domain: ode-dae
---

# `DDASSL`

Solve implicit differential-algebraic systems G(t,y,yprime)=0 with variable-step BDF methods.

Official source: [slatec/src/ddassl.f](https://www.netlib.org/slatec/src/ddassl.f).

## Call sequence

```fortran
CALL DDASSL(RES,NEQ,T,Y,YPRIME,TOUT,INFO,RTOL,ATOL,IDID,RWORK,LRW,IWORK,LIW,RPAR,IPAR,JAC)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `RES` | `external subroutine` | `ext` | `verified` |
| `NEQ` | `integer` | `in` | `verified` |
| `T` | `double` | `inout` | `verified` |
| `Y` | `double[]` | `inout` | `verified` |
| `YPRIME` | `double[]` | `inout` | `verified` |
| `TOUT` | `double` | `in` | `verified` |
| `INFO` | `integer[]` | `inout` | `verified` |
| `RTOL` | `double[]` | `inout` | `verified` |
| `ATOL` | `double[]` | `inout` | `verified` |
| `IDID` | `integer` | `out` | `verified` |
| `RWORK` | `double[]` | `inout` | `verified` |
| `LRW` | `integer` | `in` | `verified` |
| `IWORK` | `integer[]` | `inout` | `verified` |
| `LIW` | `integer` | `in` | `verified` |
| `RPAR` | `double[]` | `inout` | `verified` |
| `IPAR` | `integer[]` | `inout` | `verified` |
| `JAC` | `external subroutine` | `ext` | `verified` |

## Classification and provenance

- GAMS: `I1A2`
- User-callable: `true`
- Origin package/family: `dassl`
- Candidate project domain: `ode-dae`
- Source access: `verified`

## Callbacks

- RES computes residual DELTA for G(T,Y,YPRIME).
- JAC supplies dense or banded iteration matrix information when selected.

## Work arrays and persistent state

- RWORK and IWORK preserve solver state between continuation calls; minimum lengths depend on dense/banded options and NEQ.

## Documented errors and status

- IDID reports successful steps/interpolation and many negative failure/interrupt conditions.

## Direct dependencies

- DDAINI
- DDAJAC
- DDASTP
- DDATRP
- DDAWTS
- XERMSG

## Related routines

- SDASSL

## Unresolved ABI questions

- Long-lived mutable work arrays are solver state.
- Callbacks carry user parameter arrays but still require panic-safe trampolines.

## Authorship and revisions

Authors named by the source/package: Linda R. Petzold.

DASSL lineage and SLATEC revisions documented in source.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
