# DPCHDF

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Computes divided differences for DPCHCE and DPCHSP

## Description

DPCHDF: DPCHIP Finite Difference Formula Uses a divided difference formulation to compute a K-point approximation to the derivative at X(K) based on the data in X and S. Called by DPCHCE and DPCHSP to compute 3- and 4-point boundary derivative approximations. On input: K is the order of the desired derivative approximation. K must be at least 3 (error return if not). X contains the K values of the independent variable. X need not be ordered, but the values **MUST** be distinct. (Not checked here.) S contains the associated slope values: S(I) = (F(I+1)-F(I))/(X(I+1)-X(I)), I=1(1)K-1. (Note that S need only be of length K-1.) On return: S will be destroyed. IERR will be set to -1 if K.LT.2 . DPCHDF will be set to the desired derivative approximation if IERR=0 or to zero if IERR=-1.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `pchip/dpchdf.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dpchdf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dpchdf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [PCHIP](../families/pchip.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `K` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DPCHDF: DPCHIP Finite Difference Formula Uses a divided difference formulation to compute a K-point approximation to the derivative at X(K) based on the data in X and S. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (K) | DPCHDF: DPCHIP Finite Difference Formula Uses a divided difference formulation to compute a K-point approximation to the derivative at X(K) based on the data in X and S. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `S` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (K) | DPCHDF: DPCHIP Finite Difference Formula Uses a divided difference formulation to compute a K-point approximation to the derivative at X(K) based on the data in X and S. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IERR will be set to -1 if K.LT.2 . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f64` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `not_publicly_owned`.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-scalar-functions`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
