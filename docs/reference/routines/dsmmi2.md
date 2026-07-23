# DSMMI2

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLAP Backsolve for LDU Factorization of Normal Equations. To solve a system of the form (L*D*U)*(L*D*U)' X = B, where L is a unit lower triangular matrix, D is a diagonal matrix, and U is a unit upper triangular matrix and ' denotes transpose.

## Description

Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) DOUBLE PRECISION B(N), X(N), L(NL), DINV(N), U(NU) CALL DSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) This routine is supplied with the SLAP package as a routine to perform the MSOLVE operation in the SBCGN iteration routine for the driver DSLUCN. It must be called via the SLAP MSOLVE calling sequence convention interface routine DSMMTI. THIS ROUTINE ITSELF DOES NOT CONFORM TO THE **** SLAP MSOLVE CALLING CONVENTION **** IL, JL, L should contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. IU, JU, U should contain the unit upper factor SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double precision array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| ==================== S L A P Row format ==================== SLAP Row format. In this format the non-zeros are stored counting across rows (except for the diagonal entry, which must appear first in each "row") and are stored in the double precision array A. In other words, for each row in other non-zero elements going across the row (except the diagonal) in order. The JA array holds the column index for each non-zero. The IA array holds the offsets into the JA, A arrays for the beginning of each row. That is, JA(IA(IROW)),A(IA(IROW)) are the first elements of the IROW- th row in JA and A, and JA(IA(IROW+1)-1), A(IA(IROW+1)-1) are the last elements of the IROW-th row. Note that we always have IA(N+1) = NELT+1, where N is the number of rows Here is an example of the SLAP Row storage format for a 5x5 Matrix (in the A and JA arrays '|' denotes the end of a row): 5x5 Matrix SLAP Row format for 5x5 matrix on left. |11 12 0 0 15| A: 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 |21 22 0 0 0| JA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| IA: 1 4 6 8 9 12 With the SLAP format the "inner loops" of this routine should vectorize on machines with hardware support for vector gather/scatter operations. Your compiler may require a compiler directive to convince it that there are no implicit vector dependencies. Compiler directives for the Alliant FX/Fortran and CRI CFT/CFT77 compilers are supplied with the standard SLAP distribution. SEE ALSO DSILUS

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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

- Canonical provider: `lin/dsmmi2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsmmi2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DSMMI2](https://www.netlib.org/slatec/lin/dsmmi2.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Order of the Matrix. |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision B(N). Right hand side. |
| 3 | `X` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision X(N). Solution of (L*D*U)(L*D*U)trans x = b. |
| 4 | `IL` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IL(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the DSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array. |
| 5 | `JL` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | JL(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the DSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array. |
| 6 | `L` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Double Precision L(NL). the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the DSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array. |
| 7 | `DINV` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision DINV(N). Inverse of the diagonal matrix D. |
| 8 | `IU` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IU(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the DSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array. |
| 9 | `JU` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | JU(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the DSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array. |
| 10 | `U` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Double Precision U(NU). the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the DSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::dsmmi2`. Native symbol: `dsmmi2_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsmmi2`
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
