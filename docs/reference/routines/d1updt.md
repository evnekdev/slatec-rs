# D1UPDT

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DNSQ and DNSQE

## Description

Given an M by N lower trapezoidal matrix S, an M-vector U, and an N-vector V, the problem is to determine an orthogonal matrix Q such that t (S + U*V )*Q is again lower trapezoidal. This subroutine determines Q as the product of 2*(N - 1) transformations GV(N-1)*...*GV(1)*GW(1)*...*GW(N-1) where GV(I), GW(I) are Givens rotations in the (I,N) plane which eliminate elements in the I-th and N-th planes, respectively. Q itself is not accumulated, rather the information to recover the GV, GW rotations is returned. The SUBROUTINE statement is SUBROUTINE D1UPDT(M,N,S,LS,U,V,W,SING) where M is a positive integer input variable set to the number of rows of S. N is a positive integer input variable set to the number of columns of S. N must not exceed M. S is an array of length LS. On input S must contain the lower trapezoidal matrix S stored by columns. On output S contains the lower trapezoidal matrix produced as described above. LS is a positive integer input variable not less than (N*(2*M-N+1))/2. U is an input array of length M which must contain the vector U. V is an array of length N. On input V must contain the vector V. On output V(I) contains the information necessary to recover the Givens rotation GV(I) described above. W is an output array of length M. W(I) contains information necessary to recover the Givens rotation GW(I) described above. SING is a LOGICAL output variable. SING is set TRUE if any of the diagonal elements of the output S are zero. Otherwise SING is set FALSE.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DNSQ, DNSQE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/d1updt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/d1updt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/d1updt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/d1updt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Nonlinear equations](../families/nonlinear-equations.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `M` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Given an M by N lower trapezoidal matrix S, an M-vector U, and an N-vector V, the problem is to determine an orthogonal matrix Q such that t (S + U*V )*Q is again lower trapezoidal. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Given an M by N lower trapezoidal matrix S, an M-vector U, and an N-vector V, the problem is to determine an orthogonal matrix Q such that t (S + U*V )*Q is again lower trapezoidal. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `S` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Given an M by N lower trapezoidal matrix S, an M-vector U, and an N-vector V, the problem is to determine an orthogonal matrix Q such that t (S + U*V )*Q is again lower trapezoidal. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LS` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The SUBROUTINE statement is SUBROUTINE D1UPDT(M,N,S,LS,U,V,W,SING) where M is a positive integer input variable set to the number of rows of S. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `U` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Given an M by N lower trapezoidal matrix S, an M-vector U, and an N-vector V, the problem is to determine an orthogonal matrix Q such that t (S + U*V )*Q is again lower trapezoidal. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Given an M by N lower trapezoidal matrix S, an M-vector U, and an N-vector V, the problem is to determine an orthogonal matrix Q such that t (S + U*V )*Q is again lower trapezoidal. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The SUBROUTINE statement is SUBROUTINE D1UPDT(M,N,S,LS,U,V,W,SING) where M is a positive integer input variable set to the number of rows of S. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SING` | input | `LOGICAL` (`explicit`) | `*mut crate::FortranLogical` | scalar | The SUBROUTINE statement is SUBROUTINE D1UPDT(M,N,S,LS,U,V,W,SING) where M is a positive integer input variable set to the number of rows of S. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-logical`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
