---
id: xermsg
name: XERMSG
source_file: slatec/src/xermsg.f
source_url: https://www.netlib.org/slatec/src/xermsg.f
source_access: verified
kind: subroutine
precision: all
gams_codes: ["R3C"]
user_callable: true
origin_package: slatec-error
candidate_domain: runtime-support
---

# `XERMSG`

Process error messages for SLATEC and other libraries.

Official source: [slatec/src/xermsg.f](https://www.netlib.org/slatec/src/xermsg.f).

## Call sequence

```fortran
CALL XERMSG (LIBRAR, SUBROU, MESSG, NERR, LEVEL)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `LIBRAR` | `character` | `in` | `verified` |
| `SUBROU` | `character` | `in` | `verified` |
| `MESSG` | `character` | `in` | `verified` |
| `NERR` | `integer` | `in` | `verified` |
| `LEVEL` | `integer` | `in` | `verified` |

## Classification and provenance

- GAMS: `R3C`
- User-callable: `true`
- Origin package/family: `slatec-error`
- Candidate project domain: `runtime-support`
- Source access: `verified`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

No item was verified in this pilot pass.

## Documented errors and status

- Invalid NERR or LEVEL is treated as a fatal error inside XERMSG.

## Direct dependencies

- FDUMP
- J4SAVE
- XERCNT
- XERHLT
- XERPRN
- XERSVE

## Related routines

- XSETF
- NUMXER
- XERCLR

## Unresolved ABI questions

- Fortran CHARACTER arguments may carry compiler-specific hidden lengths.
- Uses shared error-package state through J4SAVE/XERSVE; thread safety is unverified.

## Authorship and revisions

Authors named by the source/package: Kirby Fong.

Written 1988; Version 4.0 prologue conversion 1989; revisions through 1992.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
