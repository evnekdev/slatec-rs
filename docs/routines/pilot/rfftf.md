---
id: rfftf
name: RFFTF
source_file: slatec/fishfft/rfftf.f
source_url: https://www.netlib.org/slatec/fishfft/rfftf.f
source_access: verified
kind: subroutine
precision: single
gams_codes: ["J1A1"]
user_callable: true
origin_package: fftpack
candidate_domain: transforms
---

# `RFFTF`

Compute a forward transform of a real periodic sequence in FFTPACK packed form.

Official source: [slatec/fishfft/rfftf.f](https://www.netlib.org/slatec/fishfft/rfftf.f).

## Call sequence

```fortran
CALL RFFTF(N, R, WSAVE)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `verified` |
| `R` | `real[]` | `inout` | `verified` |
| `WSAVE` | `real[]` | `inout` | `verified` |

## Classification and provenance

- GAMS: `J1A1`
- User-callable: `true`
- Origin package/family: `fftpack`
- Candidate project domain: `transforms`
- Source access: `verified`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

- WSAVE must be initialized by RFFTI for the same N.

## Documented errors and status

No item was verified in this pilot pass.

## Direct dependencies

- RFFTF1

## Related routines

- RFFTI
- RFFTB
- EZFFTF

## Unresolved ABI questions

- Output uses a package-specific packed real-frequency layout.

## Authorship and revisions

Authors named by the source/package: Paul N. Swarztrauber.

FFTPACK routine; exact source history is in the package.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
