# SSICS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Incompl. Cholesky Decomposition Preconditioner SLAP Set Up. Routine to generate the Incomplete Cholesky decomposition, L*D*L-trans, of a symmetric positive definite matrix, A, which is stored in SLAP Column format. The unit lower triangular matrix L is stored by rows, and the inverse of the diagonal matrix D is stored.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM INTEGER NEL, IEL(NEL), JEL(NEL), IWARN REAL A(NELT), EL(NEL), D(N), R(N) CALL SSICS( N, NELT, IA, JA, A, ISYM, NEL, IEL, JEL, EL, D, R, $ IWARN ) =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix

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

- Canonical provider: `lin/ssics.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssics.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssics.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SSICS](https://www.netlib.org/slatec/lin/ssics.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Order of the Matrix. NELT+1, where  N  is the number of columns in  the NELT+1, where  N  is  the number of rows  in |
| 2 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of elements in arrays IA, JA, and A. zeros in the matrix. Here is an example of the  SLAP Column  storage format for a zeros in  the matrix. Here is an example of the SLAP Row storage format for a  5x5 |
| 3 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | INOUT    Integer IA(NELT). zero. The JA array holds the offsets into the IA, A arrays for the beginning of   each    column.    That  is,    IA(JA(ICOL)), 1),  A(JA(ICOL+1)-1) points to the 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 A(IA(IROW)) points  to  the beginning  of the IROW-th row in JA and A.   JA(IA(IROW+1)-1), A(IA(IROW+1)-1) points to the  end of the  IROW-th row.  Note that we always NELT+1, where  N  is  the number of rows  in 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| With the SLAP  format some  of  the   "inner  loops" of this routine should vectorize  on  machines with hardware support for vector   gather/scatter  operations.  Your compiler  may require a compiler directive to  convince it that  there are no  implicit  vector  dependencies.  Compiler directives for the Alliant    FX/Fortran and CRI   CFT/CFT77 compilers  are supplied with the standard SLAP distribution. The IC factorization does not always exist for SPD matrices. In the event that a zero pivot is found it is set  to be 1.0 and the factorization proceeds.   The integer variable IWARN is set to the last row where the Diagonal was fudged.  This eventuality hardly ever occurs in practice. |
| 4 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | INOUT    Integer JA(NELT). th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| ==================== S L A P Row format ==================== This routine requires  that the matrix A  be  stored  in the SLAP  Row format.   In this format  the non-zeros are stored counting across  rows (except for the diagonal  entry, which must appear first in each "row") and  are stored in the real array A.  In other words, for each row in the matrix put the A(IA(IROW)) points  to  the beginning  of the IROW-th row in JA and A.   JA(IA(IROW+1)-1), A(IA(IROW+1)-1) points to the  end of the  IROW-th row.  Note that we always 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 |
| 5 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | INOUT    Real A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. tion goes well.  It is set to the row index corresponding to the last zero pivot found.  See "Description", below. zero elements going down   the  column (except  the diagonal)  in th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 zero elements   going  across the  row (except   the diagonal) in order.   The  JA array  holds   the column   index for  each non-zero.   The IA  array holds the  offsets into  the JA, A arrays  for   the   beginning  of   each  row.   That    is, 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 5x5 Matrix         SLAP Row format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  3    4  5    6  7    8    9 10 11 11 12 15 \| 22 21 \| 33 35 \| 44 \| 55 51 53 |
| 6 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the lower triangle of the matrix is stored. |
| 7 | `NEL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Number of non-zeros in the lower triangle of A.   Also corresponds to the length of the IEL, JEL, EL arrays. |
| 8 | `IEL` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | OUT      Integer IEL(NEL). contain the unit lower triangular factor  of the incomplete decomposition   of the A  matrix  stored  in  SLAP Row format.   The Diagonal of   ones   *IS*   stored.     See "Description", below for more details about the SLAP Row fmt. |
| 9 | `JEL` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | OUT      Integer JEL(NEL). contain the unit lower triangular factor  of the incomplete decomposition   of the A  matrix  stored  in  SLAP Row format.   The Diagonal of   ones   *IS*   stored.     See "Description", below for more details about the SLAP Row fmt. |
| 10 | `EL` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NEL) | OUT      Real     EL(NEL). contain the unit lower triangular factor  of the incomplete decomposition   of the A  matrix  stored  in  SLAP Row format.   The Diagonal of   ones   *IS*   stored.     See "Description", below for more details about the SLAP Row fmt. |
| 11 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | trans, of a symmetric positive definite matrix, A, which is stored in SLAP Column format.  The unit lower triangular matrix L is stored by rows, and the inverse of the diagonal matrix D is stored. OUT      Real D(N) 1./DIAG(A). |
| 12 | `R` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | WORK     Real R(N). Temporary real workspace needed for the factorization. |
| 13 | `IWARN` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. |

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

Canonical Rust path: `slatec_sys::linear_algebra::dense::ssics`. Native symbol: `ssics_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssics`
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
