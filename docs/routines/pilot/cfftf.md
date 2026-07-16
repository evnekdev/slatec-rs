---
id: cfftf
name: CFFTF
source_file: slatec/fishfft/cfftf.f
source_url: https://www.netlib.org/slatec/fishfft/cfftf.f
source_access: linked-unrendered
kind: subroutine
precision: complex-single-storage
gams_codes: ["J1A2"]
user_callable: true
origin_package: fftpack
candidate_domain: transforms
---

# `CFFTF`

Compute a forward complex discrete Fourier transform using an initialized FFTPACK work array.

Official source: [slatec/fishfft/cfftf.f](https://www.netlib.org/slatec/fishfft/cfftf.f).

## Call sequence

```fortran
CALL CFFTF(N, C, WSAVE)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `N` | `integer` | `in` | `inferred` |
| `C` | `real[]` | `inout` | `inferred` |
| `WSAVE` | `real[]` | `inout` | `inferred` |

## Classification and provenance

- GAMS: `J1A2`
- User-callable: `true`
- Origin package/family: `fftpack`
- Candidate project domain: `transforms`
- Source access: `linked-unrendered`

## Callbacks

No item was verified in this pilot pass.

## Work arrays and persistent state

- WSAVE must previously be initialized by CFFTI for the same N.

## Documented errors and status

No item was verified in this pilot pass.

## Direct dependencies

- CFFTF1

## Related routines

- CFFTI
- CFFTB

## Unresolved ABI questions

- Complex data are stored in a real array using FFTPACK conventions.
- Whether WSAVE is mutated during execution requires verification.

## Authorship and revisions

Authors named by the source/package: Paul N. Swarztrauber.

FFTPACK routine; exact SLATEC-copy revision requires source comparison.

## Evidence note

The official Netlib path was located, but its body was not rendered during this pass. Interface details are provisional and must be checked before binding generation.
