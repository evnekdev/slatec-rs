# C9LN2R

[Family: Elementary and transcendental functions](../families/elementary-and-transcendental-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate LOG(1+Z) from second order relative accuracy so that LOG(1+Z) = Z - Z**2/2 + Z**3*C9LN2R(Z).

## Description

Evaluate LOG(1+Z) from 2-nd order with relative error accuracy so that LOG(1+Z) = Z - Z**2/2 + Z**3*C9LN2R(Z). Now LOG(1+Z) = 0.5*LOG(1+2*X+ABS(Z)**2) + I*CARG(1+Z), where X = REAL(Z) and Y = AIMAG(Z). We find Z**3 * C9LN2R(Z) = -X*ABS(Z)**2 - 0.25*ABS(Z)**4 + (2*X+ABS(Z)**2)**3 * R9LN2R(2*X+ABS(Z)**2) + I * (CARG(1+Z) + (X-1)*Y) The imaginary part must be evaluated carefully as (ATAN(Y/(1+X)) - Y/(1+X)) + Y/(1+X) - (1-X)*Y = (Y/(1+X))**3 * R9ATN1(Y/(1+X)) + X**2*Y/(1+X) Now we divide through by Z**3 carefully. Write 1/Z**3 = (X-I*Y)/ABS(Z)**3 * (1/ABS(Z)**3) then C9LN2R(Z) = ((X-I*Y)/ABS(Z))**3 * (-X/ABS(Z) - ABS(Z)/4 + 0.5*((2*X+ABS(Z)**2)/ABS(Z))**3 * R9LN2R(2*X+ABS(Z)**2) + I*Y/(ABS(Z)*(1+X)) * ((X/ABS(Z))**2 + + (Y/(ABS(Z)*(1+X)))**2 * R9ATN1(Y/(1+X)) ) ) If we let XZ = X/ABS(Z) and YZ = Y/ABS(Z) we may write C9LN2R(Z) = (XZ-I*YZ)**3 * (-XZ - ABS(Z)/4 + 0.5*(2*XZ+ABS(Z))**3 * R9LN2R(2*X+ABS(Z)**2) + I*YZ/(1+X) * (XZ**2 + (YZ/(1+X))**2*R9ATN1(Y/(1+X)) ))

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Elementary and transcendental functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C4B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fnlib/c9ln2r.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/c9ln2r.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/c9ln2r.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Elementary and transcendental functions](../families/elementary-and-transcendental-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `Z` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | Evaluate LOG(1+Z) from 2-nd order with relative error accuracy so that LOG(1+Z) = Z - Z**2/2 + Z**3*C9LN2R(Z). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut crate::Complex32` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `not_publicly_owned`.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
