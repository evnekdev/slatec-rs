# GAMLN

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the logarithm of the Gamma function

## Description

GAMLN COMPUTES THE NATURAL LOG OF THE GAMMA FUNCTION FOR Z.GT.0. THE ASYMPTOTIC EXPANSION IS USED TO GENERATE VALUES GREATER THAN ZMIN WHICH ARE ADJUSTED BY THE RECURSION G(Z+1)=Z*G(Z) FOR Z.LE.ZMIN. THE FUNCTION WAS MADE AS PORTABLE AS POSSIBLE BY COMPUTING ZMIN FROM THE NUMBER OF BASE 10 DIGITS IN A WORD, RLN=MAX(-ALOG10(R1MACH(4)),0.5E-18) LIMITED TO 18 DIGITS OF (RELATIVE) ACCURACY. SINCE INTEGER ARGUMENTS ARE COMMON, A TABLE LOOK UP ON 100 VALUES IS USED FOR SPEED OF EXECUTION. DESCRIPTION OF ARGUMENTS

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C7A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/gamln.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/gamln.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/gamln.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/gamln.f) — `verified_cached`
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
| `Z` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | GAMLN COMPUTES THE NATURAL LOG OF THE GAMMA FUNCTION FOR Z.GT.0. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f32` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `not_publicly_owned`.
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
