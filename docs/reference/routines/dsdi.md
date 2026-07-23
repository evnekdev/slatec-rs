# DSDI

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Diagonal Matrix Vector Multiply. Routine to calculate the product X = DIAG*B, where DIAG is a diagonal matrix.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) DOUBLE PRECISION B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL DSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) This routine is supplied with the SLAP package to perform the MSOLVE operation for iterative drivers that require diagonal Scaling (e.g., DSDCG, DSDBCG). It conforms to the SLAP MSOLVE CALLING CONVENTION and hence does not require an interface routine as do some of the other pre- conditioners supplied with SLAP. SEE ALSO DSDS, DSD2S

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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

- Canonical provider: `lin/dsdi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsdi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DSDI](https://www.netlib.org/slatec/lin/dsdi.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Order of the Matrix. |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision B(N). Vector to multiply the diagonal by. |
| 3 | `X` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision X(N). Result of DIAG*B. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | DUMMY Integer. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | DUMMY Integer IA(NELT). |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | DUMMY Integer JA(NELT). |
| 7 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | DUMMY Double Precision A(NELT). |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | DUMMY Integer. These are for compatibility with SLAP MSOLVE calling sequence. |
| 9 | `RWORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Double Precision RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine DSDS or DSD2S. The length of RWORK must be >= IWORK(4)+N. |
| 10 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (10) | IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines DSDS or DSD2S. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`RWORK`: Double Precision RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine DSDS or DSD2S. The length of RWORK must be >= IWORK(4)+N.

`IWORK`: Integer IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP pre- conditioner setup routines DSDS or DSD2S.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::dsdi`. Native symbol: `dsdi_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::dsdi`
- Public declaration feature: `blas-level1`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
