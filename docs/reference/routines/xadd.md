# XADD

[Family: Arithmetic and extended-range arithmetic](../families/arithmetic-and-extended-range-arithmetic.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

To provide single-precision floating-point arithmetic with an extended exponent range.

## Description

REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). (Z,IZ) IS ADJUSTED BEFORE RETURNING. THE INPUT OPERANDS NEED NOT BE IN ADJUSTED FORM, BUT THEIR PRINCIPAL PARTS MUST SATISFY RADIX**(-2L).LE.ABS(X).LE.RADIX**(2L), RADIX**(-2L).LE.ABS(Y).LE.RADIX**(2L).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Arithmetic and extended-range arithmetic`
- Mathematical domain: `numeric-support`
- Package provenance: `unknown`
- GAMS classifications: `A3D`
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

- Canonical provider: `main-src/src/xadd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xadd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xadd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xadd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Arithmetic and extended-range arithmetic](../families/arithmetic-and-extended-range-arithmetic.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | input | `REAL` (`explicit`) | `*mut f32` | scalar | REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IX` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | input | `REAL` (`explicit`) | `*mut f32` | scalar | REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IY` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Z` | input | `REAL` (`explicit`) | `*mut f32` | scalar | REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IZ` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-scalar-subroutines`
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
