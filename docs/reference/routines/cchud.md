# CCHUD

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Update an augmented Cholesky decomposition of the triangular part of an augmented QR decomposition.

## Description

CCHUD updates an augmented Cholesky decomposition of the triangular part of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector X, a column vector Z, and a scalar Y, CCHUD determines a unitary matrix U and a scalar ZETA such that (R Z) (RR ZZ ) U * ( ) = ( ) , (X Y) ( 0 ZETA) where RR is upper triangular. If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) appended. In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the updated problem is SQRT(RHO**2 + ZETA**2). CCHUD will simultaneously update several triplets (Z,Y,RHO). For a less terse description of what CCHUD does and how it may be applied see the LINPACK Guide. The matrix U is determined as the product U(P)*...*U(1), where U(I) is a rotation in the (I,P+1) plane of the form ( (CI) S(I) ) ( ) . ( -CONJG(S(I)) (CI) ) The rotations are chosen so that C(I) is real. On Entry

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
- GAMS classifications: `D7B`
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

- Canonical provider: `lin/cchud.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cchud.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cchud.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CCHUD](https://www.netlib.org/slatec/lin/cchud.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `R` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDR, *) | COMPLEX(LDR,P), where LDR. GE. P. contains the upper triangular matrix that is to be updated. The part of R below the diagonal is not referenced. |
| 2 | `LDR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array R. |
| 3 | `P` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the order of the matrix R. |
| 4 | `X` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(P). contains the row to be added to R. X is not altered by CCHUD. |
| 5 | `Z` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDZ, *) | COMPLEX(LDZ,NZ), where LDZ. GE. P. is an array containing NZ P-vectors to be updated with R. not altered by CCHUD. |
| 6 | `LDZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array Z. |
| 7 | `NZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the number of vectors to be updated NZ may be zero, in which case Z, Y, and RHO are not referenced. |
| 8 | `Y` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(NZ). contains the scalars for updating the vectors not altered by CCHUD. |
| 9 | `RHO` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(NZ). contains the norms of the residual vectors that are to be updated. If RHO(J) is negative, it is left unaltered. contain the updated quantities. |
| 10 | `C` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(P). contains the cosines of the transforming rotations. |
| 11 | `S` | `output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(P). contains the sines of the transforming rotations. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchud`. Native symbol: `cchud_`. Declaration feature: `linear-algebra-complex`. Provider feature: `linear-algebra-complex`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_complex32_array_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchud`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
