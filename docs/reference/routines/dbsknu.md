# DBSKNU

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBESK

## Description

Abstract **** A DOUBLE PRECISION routine **** DBSKNU computes N member sequences of K Bessel functions K/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. Equations of the references are implemented on small orders DNU for K/SUB(DNU)/(X) and K/SUB(DNU+1)/(X). Forward recursion with the three term recursion relation generates higher orders FNU+I-1, I=1,...,N. The parameter KODE permits K/SUB(FNU+I-1)/(X) values or scaled values EXP(X)*K/SUB(FNU+I-1)/(X), I=1,N to be returned. To start the recursion FNU is normalized to the interval -0.5.LE.DNU.LT.0.5. A special form of the power series is implemented on 0.LT.X.LE.X1 while the Miller algorithm for the K Bessel function in terms of the confluent hypergeometric function U(FNU+0.5,2*FNU+1,X) is implemented on X1.LT.X.LE.X2. For X.GT.X2, the asymptotic expansion for large X is used. When FNU is a half odd integer, a special formula for DNU=-0.5 and DNU+1.0=0.5 is used to start the recursion. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in DOUBLE PRECISION arithmetic. DBSKNU assumes that a significant digit SINH function is available. Description of Arguments INPUT X,FNU are DOUBLE PRECISION X - X.GT.0.0D0 FNU - Order of initial K function, FNU.GE.0.0D0 N - Number of members of the sequence, N.GE.1 KODE - A parameter to indicate the scaling option KODE= 1 returns Y(I)= K/SUB(FNU+I-1)/(X) I=1,...,N = 2 returns Y(I)=EXP(X)*K/SUB(FNU+I-1)/(X) I=1,...,N OUTPUT Y is DOUBLE PRECISION Y - A vector whose first N components contain values for the sequence Y(I)= K/SUB(FNU+I-1)/(X), I=1,...,N or Y(I)=EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N depending on KODE NZ - Number of components set to zero due to underflow, NZ= 0 , normal return NZ.NE.0 , first NZ components of Y set to zero due to underflow, Y(I)=0.0D0,I=1,...,NZ Error Conditions Improper input arguments - a fatal error Overflow - a fatal error Underflow with KODE=1 - a non-fatal error (NZ.NE.0)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBESK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbsknu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbsknu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbsknu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbsknu.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSKNU computes N member sequences of K Bessel functions K/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNU` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSKNU computes N member sequences of K Bessel functions K/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The parameter KODE permits K/SUB(FNU+I-1)/(X) values or scaled values EXP(X)*K/SUB(FNU+I-1)/(X), I=1,N to be returned. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSKNU computes N member sequences of K Bessel functions K/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Description of Arguments INPUT X,FNU are DOUBLE PRECISION X - X.GT.0.0D0 FNU - Order of initial K function, FNU.GE.0.0D0 N - Number of members of the sequence, N.GE.1 KODE - A parameter to indicate the scaling option KODE= 1 returns Y(I)= K/SUB(FNU+I-1)/(X) I=1,...,N = 2 returns Y(I)=EXP(X)*K/SUB(FNU+I-1)/(X) I=1,...,N OUTPUT Y is DOUBLE PRECISION Y - A vector whose first N components contain values for the sequence Y(I)= K/SUB(FNU+I-1)/(X), I=1,...,N or Y(I)=EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N depending on KODE NZ - Number of components set to zero due to underflow, NZ= 0 , normal return NZ.NE.0 , first NZ components of Y set to zero due to underflow, Y(I)=0.0D0,I=1,...,NZ Error Conditions Improper input arguments - a fatal error Overflow - a fatal error Underflow with KODE=1 - a non-fatal error (NZ.NE.0) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NZ` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Description of Arguments INPUT X,FNU are DOUBLE PRECISION X - X.GT.0.0D0 FNU - Order of initial K function, FNU.GE.0.0D0 N - Number of members of the sequence, N.GE.1 KODE - A parameter to indicate the scaling option KODE= 1 returns Y(I)= K/SUB(FNU+I-1)/(X) I=1,...,N = 2 returns Y(I)=EXP(X)*K/SUB(FNU+I-1)/(X) I=1,...,N OUTPUT Y is DOUBLE PRECISION Y - A vector whose first N components contain values for the sequence Y(I)= K/SUB(FNU+I-1)/(X), I=1,...,N or Y(I)=EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N depending on KODE NZ - Number of components set to zero due to underflow, NZ= 0 , normal return NZ.NE.0 , first NZ components of Y set to zero due to underflow, Y(I)=0.0D0,I=1,...,NZ Error Conditions Improper input arguments - a fatal error Overflow - a fatal error Underflow with KODE=1 - a non-fatal error (NZ.NE.0) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
