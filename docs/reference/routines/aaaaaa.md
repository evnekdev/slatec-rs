# AAAAAA

[Family: Shared numerical utilities](../families/shared-numerical-utilities.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLATEC Common Mathematical Library disclaimer and version.

## Description

The SLATEC Common Mathematical Library is issued by the following Air Force Weapons Laboratory, Albuquerque Lawrence Livermore National Laboratory, Livermore Los Alamos National Laboratory, Los Alamos National Institute of Standards and Technology, Washington National Energy Research Supercomputer Center, Livermore Oak Ridge National Laboratory, Oak Ridge Sandia National Laboratories, Albuquerque Sandia National Laboratories, Livermore All questions concerning the distribution of the library should be directed to the NATIONAL ENERGY SOFTWARE CENTER, 9700 Cass Ave., Argonne, Illinois 60439, and not to the authors of the subprograms. * * * * * Notice * * * * * This material was prepared as an account of work sponsored by the United States Government. Neither the United States, nor the Department of Energy, nor the Department of Defense, nor any of their employees, nor any of their contractors, subcontractors, or their employees, makes any warranty, expressed or implied, or assumes any legal liability or responsibility for the accuracy, completeness, or usefulness of any information, apparatus, product, or process disclosed, or represents that its use would not infringe upon privately owned rights. *Usage: CHARACTER * 16 VER CALL AAAAAA (VER) *Arguments: VER:OUT will contain the version number of the SLATEC CML. *Description: This routine contains the SLATEC Common Mathematical Library disclaimer and can be used to return the library version number.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `uncategorized`
- Package provenance: `unknown`
- GAMS classifications: `Z`
- Family evidence: `netlib_gams` (`low`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/aaaaaa.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/aaaaaa.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/aaaaaa.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Shared numerical utilities](../families/shared-numerical-utilities.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `VER` | unavailable | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | *Usage: CHARACTER * 16 VER CALL AAAAAA (VER) *Arguments: VER:OUT will contain the version number of the SLATEC CML. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-character`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
