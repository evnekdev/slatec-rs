# D1MPYQ

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DNSQ and DNSQE

## Description

Given an M by N matrix A, this subroutine computes A*Q where Q is the product of 2*(N - 1) transformations GV(N-1)*...*GV(1)*GW(1)*...*GW(N-1) and GV(I), GW(I) are Givens rotations in the (I,N) plane which eliminate elements in the I-th and N-th planes, respectively. Q itself is not given, rather the information to recover the GV, GW rotations is supplied. The SUBROUTINE statement is SUBROUTINE D1MPYQ(M,N,A,LDA,V,W) where M is a positive integer input variable set to the number of rows of A. N IS a positive integer input variable set to the number of columns of A. A is an M by N array. On input A must contain the matrix to be postmultiplied by the orthogonal matrix Q described above. On output A*Q has replaced A. LDA is a positive integer input variable not less than M which specifies the leading dimension of the array A. V is an input array of length N. V(I) must contain the information necessary to recover the Givens rotation GV(I) described above. W is an input array of length N. W(I) must contain the information necessary to recover the Givens rotation GW(I) described above.

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

- Canonical provider: `main-src/src/d1mpyq.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/d1mpyq.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/d1mpyq.f) — `verified_cached`
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
| `M` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Given an M by N matrix A, this subroutine computes A*Q where Q is the product of 2*(N - 1) transformations GV(N-1)*...*GV(1)*GW(1)*...*GW(N-1) and GV(I), GW(I) are Givens rotations in the (I,N) plane which eliminate elements in the I-th and N-th planes, respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Given an M by N matrix A, this subroutine computes A*Q where Q is the product of 2*(N - 1) transformations GV(N-1)*...*GV(1)*GW(1)*...*GW(N-1) and GV(I), GW(I) are Givens rotations in the (I,N) plane which eliminate elements in the I-th and N-th planes, respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDA, *) | Given an M by N matrix A, this subroutine computes A*Q where Q is the product of 2*(N - 1) transformations GV(N-1)*...*GV(1)*GW(1)*...*GW(N-1) and GV(I), GW(I) are Givens rotations in the (I,N) plane which eliminate elements in the I-th and N-th planes, respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The SUBROUTINE statement is SUBROUTINE D1MPYQ(M,N,A,LDA,V,W) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The SUBROUTINE statement is SUBROUTINE D1MPYQ(M,N,A,LDA,V,W) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The SUBROUTINE statement is SUBROUTINE D1MPYQ(M,N,A,LDA,V,W) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
