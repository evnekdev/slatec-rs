---
id: i1mach
name: I1MACH
source_file: slatec/src/i1mach.f
source_url: https://www.netlib.org/slatec/src/i1mach.f
source_access: verified
kind: function
precision: integer
gams_codes: ["R1"]
user_callable: true
origin_package: slatec-machine-constants
candidate_domain: runtime-support
---

# `I1MACH`

Return integer machine-dependent constants describing I/O units and numeric representation.

Official source: [slatec/src/i1mach.f](https://www.netlib.org/slatec/src/i1mach.f).

## Call sequence

```fortran
I = I1MACH(J)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `J` | `integer` | `in` | `verified` |

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

- Invalid selector behavior is source-defined and requires target verification.

## Direct dependencies

- XERMSG

## Related routines

- R1MACH
- D1MACH

## Unresolved ABI questions

- Fortran default INTEGER width and active machine table must match the compiled library.

## Authorship and revisions

Authorship was not extracted in this pilot pass.

Historical machine-table routine; exact revision history retained in source.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
