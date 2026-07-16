---
id: d1mach
name: D1MACH
source_file: slatec/src/d1mach.f
source_url: https://www.netlib.org/slatec/src/d1mach.f
source_access: verified
kind: function
precision: double
gams_codes: ["R1"]
user_callable: true
origin_package: slatec-machine-constants
candidate_domain: runtime-support
---

# `D1MACH`

Return double-precision floating-point machine-dependent constants.

Official source: [slatec/src/d1mach.f](https://www.netlib.org/slatec/src/d1mach.f).

## Call sequence

```fortran
D = D1MACH(I)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `I` | `integer` | `in` | `verified` |

## Classification and provenance

- GAMS: `R1`
- User-callable: `true`
- Origin package/family: `slatec-machine-constants`
- Candidate project domain: `runtime-support`
- Source access: `verified`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- Calls XERMSG for an invalid selector.

## Direct dependencies

- XERMSG

## Related routines

- R1MACH
- I1MACH

## Unresolved ABI questions

- Historical source selects machine tables through DATA statements; active target configuration must be verified.

## Authorship and revisions

Authors named by the source/package: P. A. Fox, A. D. Hall, N. L. Schryer.

Written 1975; platform tables updated through 1993.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
