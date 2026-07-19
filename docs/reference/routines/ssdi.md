# SSDI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Diagonal Matrix Vector Multiply. Routine to calculate the product X = DIAG*B, where DIAG is a diagonal matrix.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. B :IN Real B(N). Vector to multiply the diagonal by. X :OUT Real X(N). Result of DIAG*B. NELT :DUMMY Integer. IA :DUMMY Integer IA(NELT). JA :DUMMY Integer JA(NELT). A :DUMMY Real A(NELT). ISYM :DUMMY Integer. These are for compatibility with SLAP MSOLVE calling sequence. RWORK :IN Real RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine SSDS or SSD2S. The length of RWORK must be >= IWORK(4)+N. IWORK :IN Integer IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP preconditioner setup routines SSDS or SSD2S. *Description: This routine is supplied with the SLAP package to perform the MSOLVE operation for iterative drivers that require diagonal Scaling (e.g., SSDCG, SSDBCG). It conforms to the SLAP MSOLVE CALLING CONVENTION and hence does not require an interface routine as do some of the other preconditioners supplied with SLAP.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1B4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/ssdi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssdi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::blas::level1::ssdi`
- Current legacy Rust paths: `slatec_sys::families::blas_level1::ssdi`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
