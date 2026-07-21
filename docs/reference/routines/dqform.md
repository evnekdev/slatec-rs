# DQFORM

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DNSQ and DNSQE

## Description

This subroutine proceeds from the computed QR factorization of an M by N matrix A to accumulate the M by M orthogonal matrix Q from its factored form. The subroutine statement is SUBROUTINE DQFORM(M,N,Q,LDQ,WA) where M is a positive integer input variable set to the number of rows of A and the order of Q. N is a positive integer input variable set to the number of columns of A. Q is an M by M array. On input the full lower trapezoid in the first MIN(M,N) columns of Q contains the factored form. On output Q has been accumulated into a square matrix. LDQ is a positive integer input variable not less than M which specifies the leading dimension of the array Q. WA is a work array of length M.

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

- Canonical provider: `main-src/src/dqform.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqform.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqform.f) — `verified_cached`
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
| `M` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This subroutine proceeds from the computed QR factorization of an M by N matrix A to accumulate the M by M orthogonal matrix Q from its factored form. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This subroutine proceeds from the computed QR factorization of an M by N matrix A to accumulate the M by M orthogonal matrix Q from its factored form. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Q` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDQ, *) | This subroutine proceeds from the computed QR factorization of an M by N matrix A to accumulate the M by M orthogonal matrix Q from its factored form. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDQ` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The subroutine statement is SUBROUTINE DQFORM(M,N,Q,LDQ,WA) where M is a positive integer input variable set to the number of rows of A and the order of Q. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE DQFORM(M,N,Q,LDQ,WA) where M is a positive integer input variable set to the number of rows of A and the order of Q. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
