# DNSQ

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Find a zero of a system of a N nonlinear functions in N variables by a modification of the Powell hybrid method.

## Description

1. Purpose. The purpose of DNSQ is to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method. The user must provide a subroutine which calculates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD and HYBRDJ. 2. Subroutine and Type Statements. SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. Parameters. Parameters designated as input parameters must be specified on entry to DNSQ and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNSQ. FCN is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) CALCULATE THE FUNCTIONS AT X AND RETURN THIS VECTOR IN FVEC. RETURN

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::nonlinear::solve_system_expert, slatec::nonlinear::solve_system_with_jacobian`

## Providers

- Canonical provider: `main-src/src/dnsq.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnsq.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnsq.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnsq.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Nonlinear equations](../families/nonlinear-equations.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `FCN` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JAC` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IOPT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The purpose of DNSQ is to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FVEC` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FJAC` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDFJAC, *) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDFJAC` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XTOL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MAXFEV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ML` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MU` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSFCN` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DIAG` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FACTOR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NPRINT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NFEV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NJEV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `QTF` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA1` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA2` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA3` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA4` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, * ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, * NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::nonlinear::dnsq`. Native symbol: `dnsq_`. Feature: `nonlinear-expert`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::nonlinear::dnsq`
- Compatibility aliases: `none`
- Public declaration feature: `nonlinear-expert`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::nonlinear::solve_system_expert`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
