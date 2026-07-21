# XERCNT

[Family: Error handling](../families/error-handling.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Allow user control over handling of errors.

## Description

Abstract Allows user control over handling of individual errors. Just after each message is recorded, but before it is processed any further (i.e., before it is printed or a decision to abort is made), a call is made to XERCNT. If the user has provided his own version of XERCNT, he can then override the value of KONTROL used in processing this message by redefining its value. KONTRL may be set to any value from -2 to 2. The meanings for KONTRL are the same as in XSETF, except that the value of KONTRL changes only for this message. If KONTRL is set to a value outside the range from -2 to 2, it will be moved back into that range.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Error handling`
- Mathematical domain: `runtime-support`
- Package provenance: `slatec-error`
- GAMS classifications: `R3C`
- Family evidence: `package_provenance` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/xercnt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xercnt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xercnt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Error handling](../families/error-handling.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `LIBRAR` | unavailable | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SUBROU` | unavailable | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MESSG` | unavailable | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NERR` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LEVEL` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KONTRL` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | KONTRL may be set to any value from -2 to 2. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `support-routine`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
