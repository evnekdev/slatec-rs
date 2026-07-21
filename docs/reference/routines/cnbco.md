# CNBCO

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Factor a band matrix using Gaussian elimination and estimate the condition number.

## Description

CNBCO factors a complex band matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, CNBFA is slightly faster. To solve A*X = B , follow CNBCO by CNBSL. To compute INVERSE(A)*C , follow CNBCO by CNBSL. To compute DETERMINANT(A) , follow CNBCO by CNBDI. On Entry

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2C2`
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

- Canonical provider: `main-src/src/cnbco.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cnbco.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cnbco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cnbco.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [CNBCO](https://www.netlib.org/slatec/src/cnbco.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `ABE` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | COMPLEX(LDA, NC) contains the matrix in band storage.  The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE. NC must be .GE. 2*ML+MU+1 . See the comments below for details. an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written  A = L*U  where L is a product of permutation and unit lower triangular matrices and  U  is upper triangular. A(I,J) 10    CONTINUE 20 CONTINUE This uses columns  1  through  ML+MU+1  of ABE . Furthermore,  ML  additional columns are needed in starting with column  ML+MU+2  for elements generated during the triangularization.  The total number of columns needed in  ABE  is  2*ML+MU+1 . Example:  If the original matrix is 11 12 13  0  0  0 21 22 23 24  0  0 0 32 33 34 35  0 0  0 43 44 45 46 0  0  0 54 55 56 0  0  0  0 65 66 |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the leading dimension of the array ABE. must be .GE. N . |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the order of the original matrix. 6, ML = 1, MU = 2, LDA .GE. 5  and ABE should contain 11 12 13  +     , * = not used 21 22 23 24  +     , + = used for pivoting 32 33 34 35  + 43 44 45 46  + 54 55 56  *  + 65 66  *  *  + |
| 4 | `ML` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER number of diagonals below the main diagonal. 0 .LE. ML .LT. N . (band width below the diagonal) |
| 5 | `MU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER number of diagonals above the main diagonal. 0 .LE. MU .LT. N . More efficient if ML .LE. MU . On Return (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 |
| 6 | `IPVT` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | INTEGER(N) an integer vector of pivot indices. |
| 7 | `RCOND` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | REAL an estimate of the reciprocal condition of  A . For the system  A*X = B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate underflows. |
| 8 | `Z` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(N) a work vector whose contents are usually unimportant. If  A  is close to a singular matrix, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . Band Storage If  A  is a band matrix, the following program segment will set up the input. |

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

Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cnbco`. Native symbol: `cnbco_`. Declaration feature: `linear-algebra-complex`. Provider feature: `linear-algebra-complex`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_array_rank1,mut_f32,mut_complex32_array_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cnbco`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
