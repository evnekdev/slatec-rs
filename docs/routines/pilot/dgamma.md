---
id: dgamma
name: DGAMMA
source_file: slatec/fnlib/dgamma.f
source_url: https://www.netlib.org/slatec/fnlib/dgamma.f
source_access: verified
kind: function
precision: double
gams_codes: ["C7A"]
user_callable: true
origin_package: fnlib
candidate_domain: special-functions
---

# `DGAMMA`

Compute the complete gamma function.

Official source: [slatec/fnlib/dgamma.f](https://www.netlib.org/slatec/fnlib/dgamma.f).

## Call sequence

```fortran
Y = DGAMMA(X)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `X` | `double` | `in` | `verified` |

## Classification and provenance

- GAMS: `C7A`
- User-callable: `true`
- Origin package/family: `fnlib`
- Candidate project domain: `special-functions`
- Source access: `verified`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- Domain and overflow conditions are reported through the SLATEC error subsystem.

## Direct dependencies

- D9LGMC
- D9LGIT
- D9LGIC
- DINT
- DLNGAM
- XERMSG

## Related routines

- GAMMA
- CGAMMA
- DGAMR
- DLNGAM

## Unresolved ABI questions

- Exceptional behavior may involve global error policy instead of a return status.

## Authorship and revisions

Authors named by the source/package: Wayne Fullerton.

FNLIB routine; source prologue records revisions.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
