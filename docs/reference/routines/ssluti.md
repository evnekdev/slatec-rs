# SSLUTI

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLAP MTSOLV for LDU Factorization. This routine acts as an interface between the SLAP generic MTSOLV calling convention and the routine that actually -T computes (LDU) B = X.

## Description

It is assumed that RWORK and IWORK have initialized with the information required for SSLUI4:

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2E`
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

- Canonical provider: `lin/ssluti.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssluti.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssluti.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [SSLUTI](https://www.netlib.org/slatec/lin/ssluti.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input order of the sparse linear system and required vector length. |
| 2 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Readable right-hand-side vector of length `N` for the SLAP lower-triangular preconditioner solve. The computed vector is returned through `X`, not by retaining `B`. |
| 3 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Writable output vector of length `N`. The interface computes the lower-triangular preconditioner solve `L^-1 * B` into this storage. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input number of stored nonzero entries in the sparse arrays `IA`, `JA`, and `A`. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Readable integer sparse-index array with at least `NELT` entries. Together with `JA` it describes the SLAP column-format matrix or preconditioner data. |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Readable integer sparse-offset/index array with at least `NELT` entries as declared by this interface. Together with `IA` and `A` it identifies the SLAP column-format entries. |
| 7 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Readable sparse value array with at least `NELT` entries, stored in the SLAP column format described by `IA` and `JA`. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input sparse-storage symmetry flag. `0` means all nonzero entries are present; `1` means symmetric storage contains only the lower triangle. |
| 9 | `RWORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Readable real work/state array initialized for the underlying SLAP `*SLI2` routine. Its offsets are supplied through `IWORK`; native code uses it as preconditioner storage and does not retain it. |
| 10 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (10) | Starting location of IL in IWORK. Starting location of JL in IWORK. Starting location of IU in IWORK. Starting location of JU in IWORK. Starting location of L in RWORK. Starting location of DINV in RWORK. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`RWORK`: Readable real work/state array initialized for the underlying SLAP `*SLI2` routine. Its offsets are supplied through `IWORK`; native code uses it as preconditioner storage and does not retain it.

`IWORK`: Starting location of IL in IWORK. Starting location of JL in IWORK. Starting location of IU in IWORK. Starting location of JU in IWORK. Starting location of L in RWORK. Starting location of DINV in RWORK. Starting location of U in RWORK. See the DESCRIPTION of SSLUI4 for details.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::ssluti`. Native symbol: `ssluti_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssluti`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
