# FFTDOC

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Documentation for FFTPACK, a collection of Fast Fourier Transform routines.

## Description

* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Version 3 June 1979 A Package of Fortran Subprograms for The Fast Fourier Transform of Periodic and Other Symmetric Sequences By Paul N Swarztrauber National Center For Atmospheric Research, Boulder, Colorado 80307 which is sponsored by the National Science Foundation * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * This package consists of programs which perform Fast Fourier Transforms for both complex and real periodic sequences and certain other symmetric sequences that are listed below. 1. RFFTI Initialize RFFTF and RFFTB 2. RFFTF Forward transform of a real periodic sequence 3. RFFTB Backward transform of a real coefficient array 4. EZFFTI Initialize EZFFTF and EZFFTB 5. EZFFTF A simplified real periodic forward transform 6. EZFFTB A simplified real periodic backward transform 7. SINTI Initialize SINT 8. SINT Sine transform of a real odd sequence 9. COSTI Initialize COST 10. COST Cosine transform of a real even sequence 11. SINQI Initialize SINQF and SINQB 12. SINQF Forward sine transform with odd wave numbers 13. SINQB Unnormalized inverse of SINQF 14. COSQI Initialize COSQF and COSQB 15. COSQF Forward cosine transform with odd wave numbers 16. COSQB Unnormalized inverse of COSQF 17. CFFTI Initialize CFFTF and CFFTB 18. CFFTF Forward transform of a complex periodic sequence 19. CFFTB Unnormalized inverse of CFFTF

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `FFTPACK transforms`
- Mathematical domain: `transforms`
- Package provenance: `unknown`
- GAMS classifications: `J1`
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

- Canonical provider: `main-src/src/fftdoc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/fftdoc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/fftdoc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [FFTPACK transforms](../families/fftpack-transforms.md)

No independently callable argument list is present in the selected interface inventory for this identity.
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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
