# FDJAC3

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNLS1 and SNLS1E

## Description

This subroutine computes a forward-difference approximation to the M by N Jacobian matrix associated with a specified problem of M functions in N variables. The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an external statement in the user calling program, and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER LDFJAC,M,N,IFLAG REAL X(N),FVEC(M),FJAC(LDFJAC,N) When IFLAG.EQ.1 calculate the functions at X and return this vector in FVEC. RETURN

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SNLS1, SNLS1E`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/fdjac3.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/fdjac3.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/fdjac3.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/fdjac3.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Approximation](../families/approximation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `FCN` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This subroutine computes a forward-difference approximation to the M by N Jacobian matrix associated with a specified problem of M functions in N variables. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This subroutine computes a forward-difference approximation to the M by N Jacobian matrix associated with a specified problem of M functions in N variables. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FVEC` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FJAC` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDFJAC, *) | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDFJAC` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IFLAG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSFCN` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE FDJAC3(FCN,M,N,X,FVEC,FJAC,LDFJAC,IFLAG,EPSFCN,WA) where FCN is the name of the user-supplied subroutine which calculates the functions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
