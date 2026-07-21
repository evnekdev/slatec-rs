# SLLTI2

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLAP Backsolve routine for LDL' Factorization. Routine to solve a system of the form L*D*L' X = B, where L is a unit lower triangular matrix and D is a diagonal matrix and ' means transpose.

## Description

Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL), DINV(N) CALL SLLTI2( N, B, X, NEL, IEL, JEL, EL, DINV ) This routine is supplied with the SLAP package as a routine to perform the MSOLVE operation in the SCG iteration routine for the driver routine SSICCG. It must be called via the SLAP MSOLVE calling sequence convention interface routine SSLLI. THIS ROUTINE ITSELF DOES NOT CONFORM TO THE **** SLAP MSOLVE CALLING CONVENTION **** IEL, JEL, EL should contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. This IC factorization can be computed by the SSICS routine. The diagonal (which is all one's) is stored. ==================== S L A P Row format ==================== This routine requires that the matrix A be stored in the SLAP Row format. In this format the non-zeros are stored counting across rows (except for the diagonal entry, which must appear first in each "row") and are stored in the real array A. In other words, for each row in the matrix put the diagonal entry in A. Then put in the other non-zero elements going across the row (except the diagonal) in order. The JA array holds the column index for each non-zero. The IA array holds the offsets into the JA, A arrays for the beginning of each row. That is, JA(IA(IROW)), A(IA(IROW)) points to the beginning of the IROW-th row in JA and A. JA(IA(IROW+1)-1), A(IA(IROW+1)-1) points to the end of the IROW-th row. Note that we always have IA(N+1) = NELT+1, where N is the number of rows in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Row storage format for a 5x5 Matrix (in the A and JA arrays '|' denotes the end of a row): 5x5 Matrix SLAP Row format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 |21 22 0 0 0| JA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| IA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| With the SLAP Row format the "inner loop" of this routine should vectorize on machines with hardware support for vector gather/scatter operations. Your compiler may require a compiler directive to convince it that there are no implicit vector dependencies. Compiler directives for the Alliant FX/Fortran and CRI CFT/CFT77 compilers are supplied with the standard SLAP distribution.

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

- Canonical provider: `lin/sllti2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sllti2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [SLLTI2](https://www.netlib.org/slatec/lin/sllti2.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | b. NEL    :IN       Integer. Number of non-zeros in the EL array. IEL    :IN       Integer IEL(NEL). JEL    :IN       Integer JEL(NEL). EL     :IN       Real     EL(NEL). IEL, JEL, EL contain the unit lower triangular factor   of the incomplete decomposition   of the A  matrix  stored in SLAP Row format.   The diagonal of ones *IS* stored.  This structure can be set  up  by  the SS2LT routine.  See  the "Description", below for more details about the  SLAP  Row format. DINV   :IN       Real DINV(N). Inverse of the diagonal matrix D. |
| 4 | `NEL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IEL` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `JEL` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `EL` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NEL) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `DINV` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::sllti2`. Native symbol: `sllti2_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sllti2`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
