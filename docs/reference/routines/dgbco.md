# DGBCO

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Factor a band matrix by Gaussian elimination and estimate the condition number of the matrix.

## Description

DGBCO factors a double precision band matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, DGBFA is slightly faster. To solve A*X = B , follow DGBCO by DGBSL. To compute INVERSE(A)*C , follow DGBCO by DGBSL. To compute DETERMINANT(A) , follow DGBCO by DGBDI. On Entry

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
- GAMS classifications: `D2A2`
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

- Canonical provider: `lin/dgbco.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dgbco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dgbco.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DGBCO](https://www.netlib.org/slatec/lin/dgbco.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `ABD` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDA, *) | DOUBLE PRECISION(LDA, N) contains the matrix in band storage. The columns of the matrix are stored in the columns of ABD and the diagonals of the matrix are stored in rows ML+1 through 2*ML+MU+1 of ABD. See the comments below for details. an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written A = L*U where L is a product of permutation and unit lower triangular matrices and U is upper triangular. A(I,J) 10 CONTINUE 20 CONTINUE This uses rows ML+1 through 2*ML+MU+1 of ABD. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the leading dimension of the array ABD. must be. GE. 2*ML + MU + 1. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the order of the original matrix. |
| 4 | `ML` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER number of diagonals below the main diagonal. 0. LE. ML. LT. N. |
| 5 | `MU` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER number of diagonals above the main diagonal. 0. LE. MU. LT. N. |
| 6 | `IPVT` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | INTEGER(N) an integer vector of pivot indices. |
| 7 | `RCOND` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | DOUBLE PRECISION an estimate of the reciprocal condition of A. For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND. If RCOND is so small that the logical expression 1. 0 + RCOND. EQ. 1. |
| 8 | `Z` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z). Band Storage If A is a band matrix, the following program segment will set up the input. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::banded::dgbco`. Native symbol: `dgbco_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::dgbco`
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
