# DSILUS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Incomplete LU Decomposition Preconditioner SLAP Set Up. Routine to generate the incomplete LDU decomposition of a matrix. The unit lower triangular factor L is stored by rows and the unit upper triangular factor U is stored by columns. The inverse of the diagonal matrix D is stored. No fill in is allowed.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM INTEGER NL, IL(NL), JL(NL), NU, IU(NU), JU(NU) INTEGER NROW(N), NCOL(N) DOUBLE PRECISION A(NELT), L(NL), DINV(N), U(NU) CALL DSILUS( N, NELT, IA, JA, A, ISYM, NL, IL, JL, L, $ DINV, NU, IU, JU, U, NROW, NCOL )

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

- Canonical provider: `lin/dsilus.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsilus.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DSILUS](https://www.netlib.org/slatec/lin/dsilus.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer Order of the Matrix. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a NELT+1, where N is the number of rows |
| 2 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of elements in arrays IA, JA, and A. zeros  in the matrix. Here is an example of the SLAP Row storage format for a  5x5 |
| 3 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer IA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we NELT+1, where N is the number of rows 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| |
| 4 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer JA(NELT). A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), 1) points to  the  end of the   ICOL-th column. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| ==================== S L A P Row format ==================== This routine requires  that the matrix A  be  stored  in the SLAP  Row format.   In this format  the non-zeros are stored counting across  rows (except for the diagonal  entry, which must  appear first  in each  "row")  and  are stored  in the double precision  array A.  In other words, for each row  in the matrix  put the diagonal  entry in A.   Then put in  the other  non-zero elements  going across  the row  (except the diagonal) in order.  The JA array holds the column index for each non-zero.  The IA array holds the offsets  into the JA, 1), A(IA(IROW+1)-1) 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 |
| 5 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | IN       Double Precision A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. arrays  for  the  beginning  of each   column.   That  is, 1) points to  the  end of the   ICOL-th column. denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 arrays  for  the   beginning  of  each  row.    That  is, 1), A(IA(IROW+1)-1) are  the last elements  of the  IROW-th row.   Note  that we 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  3    4  5    6  7    8    9 10 11 11 12 15 \| 22 21 \| 33 35 \| 44 \| 55 51 53 |
| 6 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the lower triangle of the matrix is stored. |
| 7 | `NL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Number of non-zeros in the L array. |
| 8 | `IL` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NL) | OUT      Integer IL(NL). contain the unit lower triangular factor of  the incomplete decomposition  of some  matrix stored  in   SLAP Row format.     The   Diagonal  of ones  *IS*  stored.  See "DESCRIPTION", below for more details about the SLAP format. contain the unit  lower triangular factor of the incomplete decomposition of the A matrix  stored in SLAP |
| 9 | `JL` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NL) | OUT      Integer JL(NL). contain the unit lower triangular factor of  the incomplete decomposition  of some  matrix stored  in   SLAP Row format.     The   Diagonal  of ones  *IS*  stored.  See "DESCRIPTION", below for more details about the SLAP format. contain the unit  lower triangular factor of the incomplete decomposition of the A matrix  stored in SLAP |
| 10 | `L` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NL) | 60 Livermore, CA 94550 (510) 423-3141 seager@llnl.gov OUT      Double Precision L(NL). contain the unit lower triangular factor of  the incomplete decomposition  of some  matrix stored  in   SLAP Row format.     The   Diagonal  of ones  *IS*  stored.  See "DESCRIPTION", below for more details about the SLAP format. contain the unit  lower triangular factor of the incomplete decomposition of the A matrix  stored in SLAP |
| 11 | `DINV` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Writable array of at least `N` entries. On return it stores the inverse diagonal factor `D^-1` from the no-fill incomplete LDU preconditioner; it is consumed by the related SLAP solve routines. |
| 12 | `NU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Number of non-zeros in the U array. |
| 13 | `IU` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NU) | OUT      Integer IU(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, |
| 14 | `JU` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NU) | OUT      Integer JU(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, |
| 15 | `U` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NU) | OUT      Double Precision     U(NU). contain   the unit upper triangular factor of the incomplete  decomposition    of some matrix  stored in SLAP Column  format.   The Diagonal of ones   *IS*  stored.  See "Description", below  for  more  details  about  the   SLAP format. contain  the unit upper factor of the  incomplete decomposition of  the A matrix  stored in SLAP Column format This ILU factorization can be computed by the DSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, |
| 16 | `NROW` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (N) | WORK     Integer NROW(N). zero elements in the I-th row of L. |
| 17 | `NCOL` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (N) | WORK     Integer NCOL(N). zero elements in the I-th column of U. |

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

Canonical Rust path: `slatec_sys::linear_algebra::dense::dsilus`. Native symbol: `dsilus_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsilus`
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
