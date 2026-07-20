# QELG

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine determines the limit of a given sequence of approximations, by means of the Epsilon algorithm of P. Wynn. An estimate of the absolute error is also given. The condensed Epsilon table is computed. Only those elements needed for the computation of the next diagonal are preserved.

## Description

Epsilon algorithm Standard fortran subroutine Real version PARAMETERS N - Integer EPSTAB(N) contains the new element in the first column of the epsilon table. EPSTAB - Real Vector of dimension 52 containing the elements of the two lower diagonals of the triangular epsilon table. The elements are numbered starting at the right-hand corner of the triangle. RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `quadpack`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qelg.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qelg.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qelg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qelg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Epsilon algorithm Standard fortran subroutine Real version PARAMETERS N - Integer EPSTAB(N) contains the new element in the first column of the epsilon table. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSTAB` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (52) | Epsilon algorithm Standard fortran subroutine Real version PARAMETERS N - Integer EPSTAB(N) contains the new element in the first column of the epsilon table. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RES3LA` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (3) | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NRES` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) | RESULT - Real Resulting approximation to the integral ABSERR - Real Estimate of the absolute error computed from RESULT and the 3 previous results RES3LA - Real Vector of dimension 3 containing the last 3 results NRES - Integer Number of calls to the routine (should be zero at first call) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
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
