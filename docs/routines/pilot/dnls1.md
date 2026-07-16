---
id: dnls1
name: DNLS1
source_file: slatec/src/dnls1.f
source_url: https://www.netlib.org/slatec/src/dnls1.f
source_access: verified
kind: subroutine
precision: double
gams_codes: ["K1B1A"]
user_callable: true
origin_package: minpack-derived
candidate_domain: optimization
---

# `DNLS1`

Minimize a sum of squares by a modified Levenberg-Marquardt algorithm.

Official source: [slatec/src/dnls1.f](https://www.netlib.org/slatec/src/dnls1.f).

## Call sequence

```fortran
CALL DNLS1(FCN,IOPT,M,N,X,FVEC,FJAC,LDFJAC,FTOL,XTOL,GTOL,MAXFEV,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `FCN` | `external subroutine` | `ext` | `verified` |
| `IOPT` | `integer` | `in` | `verified` |
| `M` | `integer` | `in` | `verified` |
| `N` | `integer` | `in` | `verified` |
| `X` | `double[]` | `inout` | `verified` |
| `FVEC` | `double[]` | `out` | `verified` |
| `FJAC` | `double[LDFJAC,*]` | `out` | `verified` |
| `LDFJAC` | `integer` | `in` | `verified` |
| `FTOL` | `double` | `in` | `verified` |
| `XTOL` | `double` | `in` | `verified` |
| `GTOL` | `double` | `in` | `verified` |
| `MAXFEV` | `integer` | `in` | `verified` |
| `EPSFCN` | `double` | `in` | `verified` |
| `DIAG` | `double[]` | `inout` | `verified` |
| `MODE` | `integer` | `in` | `verified` |
| `FACTOR` | `double` | `in` | `verified` |
| `NPRINT` | `integer` | `in` | `verified` |
| `INFO` | `integer` | `out` | `verified` |
| `NFEV` | `integer` | `out` | `verified` |
| `NJEV` | `integer` | `out` | `verified` |
| `IPVT` | `integer[]` | `out` | `verified` |
| `QTF` | `double[]` | `out` | `verified` |
| `WA1` | `double[]` | `work` | `verified` |
| `WA2` | `double[]` | `work` | `verified` |
| `WA3` | `double[]` | `work` | `verified` |
| `WA4` | `double[]` | `work` | `verified` |

## Classification and provenance

- GAMS: `K1B1A`
- User-callable: `true`
- Origin package/family: `minpack-derived`
- Candidate project domain: `optimization`
- Source access: `verified`

## Callbacks

- FCN evaluates residuals and, depending on IOPT/IFLAG, may also supply Jacobian columns or matrix entries.

## Work arrays and persistent state

- WA1..WA3 length N; WA4 length M.

## Documented errors and status

- INFO reports several convergence tests, evaluation limit, tolerance limits, poor progress, or improper input.

## Direct dependencies

- DCKDER
- DENORM
- DFDJC3
- DLMPAR
- DQRFAC
- DQRSOLV
- XERMSG

## Related routines

- DNLS1E
- DNSQ
- DCOV

## Unresolved ABI questions

- Callback behavior changes with IOPT and IFLAG.
- FJAC and work arrays have mode-dependent output validity.

## Authorship and revisions

Authors named by the source/package: Jorge J. More, Burton S. Garbow, Kenneth E. Hillstrom.

Combines MINPACK LMDER and LMDIF; SLATEC adaptation history in source.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
