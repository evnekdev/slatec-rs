---
id: dnsq
name: DNSQ
source_file: slatec/src/dnsq.f
source_url: https://www.netlib.org/slatec/src/dnsq.f
source_access: verified
kind: subroutine
precision: double
gams_codes: ["F2A"]
user_callable: true
origin_package: minpack-derived
candidate_domain: nonlinear-equations
---

# `DNSQ`

Solve a system of nonlinear equations by a Powell hybrid method.

Official source: [slatec/src/dnsq.f](https://www.netlib.org/slatec/src/dnsq.f).

## Call sequence

```fortran
CALL DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV,ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV,NJEV,R,LR,QTF,WA1,WA2,WA3,WA4)
```

## Arguments

| Name | Fortran form | Intent | Evidence |
|---|---|---|---|
| `FCN` | `external subroutine` | `ext` | `verified` |
| `JAC` | `external subroutine` | `ext` | `verified` |
| `IOPT` | `integer` | `in` | `verified` |
| `N` | `integer` | `in` | `verified` |
| `X` | `double[]` | `inout` | `verified` |
| `FVEC` | `double[]` | `out` | `verified` |
| `FJAC` | `double[LDFJAC,*]` | `out` | `verified` |
| `LDFJAC` | `integer` | `in` | `verified` |
| `XTOL` | `double` | `in` | `verified` |
| `MAXFEV` | `integer` | `in` | `verified` |
| `ML` | `integer` | `in` | `verified` |
| `MU` | `integer` | `in` | `verified` |
| `EPSFCN` | `double` | `in` | `verified` |
| `DIAG` | `double[]` | `inout` | `verified` |
| `MODE` | `integer` | `in` | `verified` |
| `FACTOR` | `double` | `in` | `verified` |
| `NPRINT` | `integer` | `in` | `verified` |
| `INFO` | `integer` | `out` | `verified` |
| `NFEV` | `integer` | `out` | `verified` |
| `NJEV` | `integer` | `out` | `verified` |
| `R` | `double[]` | `out` | `verified` |
| `LR` | `integer` | `in` | `verified` |
| `QTF` | `double[]` | `out` | `verified` |
| `WA1` | `double[]` | `work` | `verified` |
| `WA2` | `double[]` | `work` | `verified` |
| `WA3` | `double[]` | `work` | `verified` |
| `WA4` | `double[]` | `work` | `verified` |

## Classification and provenance

- GAMS: `F2A`
- User-callable: `true`
- Origin package/family: `minpack-derived`
- Candidate project domain: `nonlinear-equations`
- Source access: `verified`

## Callbacks

- FCN evaluates F(X) and uses a mutable IFLAG control/status argument.
- JAC evaluates a dense Jacobian and uses leading dimension LDFJAC.

## Work arrays and persistent state

- R stores a packed upper-triangular factor; LR must satisfy N*(N+1)/2.
- WA1..WA4 each require length N.

## Documented errors and status

- INFO reports convergence, limits, poor progress, or improper input; negative callback flags request termination.

## Direct dependencies

- DDOGLG
- DENORM
- DFDJC1
- DQFORM
- DQRFAC
- DR1MPYQ
- DR1UPDT
- XERMSG

## Related routines

- DNSQE
- DNLS1
- DCKDER

## Unresolved ABI questions

- Two callback signatures and mutable control flags require dedicated trampolines.
- Packed R length can overflow integer arithmetic.

## Authorship and revisions

Authors named by the source/package: Jorge J. More, Burton S. Garbow, Kenneth E. Hillstrom.

Combines MINPACK HYBRD and HYBRDJ; SLATEC adaptation history in source.

## Evidence note

Core identity and interface fields were checked against the opened official Netlib source. Descriptions are paraphrases; the source remains authoritative.
