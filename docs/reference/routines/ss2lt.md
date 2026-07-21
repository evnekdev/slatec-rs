# SS2LT

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Lower Triangle Preconditioner SLAP Set Up. Routine to store the lower triangle of a matrix stored in the SLAP Column format.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM INTEGER NEL, IEL(NEL), JEL(NEL) REAL A(NELT), EL(NEL) CALL SS2LT( N, NELT, IA, JA, A, ISYM, NEL, IEL, JEL, EL ) =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have

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

- Canonical provider: `lin/ss2lt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ss2lt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SS2LT](https://www.netlib.org/slatec/lin/ss2lt.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Order of the Matrix. |
| 2 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Number of non-zeros stored in A. |
| 3 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IA(NELT). |
| 4 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | JA(NELT). NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '\|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 \|11 12 0 0 15\| A: 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 \|21 22 0 0 0\| IA: 1 2 5 \| 2 1 \| 3 5 \| 4 \| 5 1 3 \| 0 0 33 0 35\| JA: 1 4 6 8 9 12 \| 0 0 0 44 0\| \|51 0 53 0 55\|. |
| 5 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | A(NELT). These arrays should hold the matrix A in the SLAP Column format. See "Description", below. |
| 6 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the lower triangle of the matrix is stored. |
| 7 | `NEL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Number of non-zeros in the lower triangle of A. Also corresponds to the length of the IEL, JEL, EL arrays. |
| 8 | `IEL` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | IEL(NEL). the lower triangle of the A matrix stored in SLAP Column format. See "Description", below, for more details bout the SLAP Column format. |
| 9 | `JEL` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | JEL(NEL). the lower triangle of the A matrix stored in SLAP Column format. See "Description", below, for more details bout the SLAP Column format. |
| 10 | `EL` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | EL(NEL). the lower triangle of the A matrix stored in SLAP Column format. See "Description", below, for more details bout the SLAP Column format. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::ss2lt`. Native symbol: `ss2lt_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ss2lt`
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
