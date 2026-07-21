# CCHDD

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Downdate an augmented Cholesky decomposition or the triangular factor of an augmented QR decomposition.

## Description

CCHDD downdates an augmented Cholesky decomposition or the triangular factor of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector X, a column vector Z, and a scalar Y, CCHDD determines a unitary matrix U and a scalar ZETA such that (R Z ) (RR ZZ) U * ( ) = ( ) , (0 ZETA) ( X Y) where RR is upper triangular. If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) removed. In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the downdated problem is SQRT(RHO**2 - ZETA**2). CCHDD will simultaneously downdate several triplets (Z,Y,RHO) along with R. For a less terse description of what CCHDD does and how it may be applied, see the LINPACK Guide. The matrix U is determined as the product U(1)*...*U(P) where U(I) is a rotation in the (P+1,I)-plane of the form ( C(I) -CONJG(S(I)) ) ( ) . ( S(I) C(I) ) the rotations are chosen so that C(I) is real. The user is warned that a given downdating problem may be impossible to accomplish or may produce inaccurate results. For example, this can happen if X is near a vector whose removal will reduce the rank of R. Beware. On Entry

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

- Canonical provider: `lin/cchdd.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cchdd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cchdd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CCHDD](https://www.netlib.org/slatec/lin/cchdd.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `R` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDR, *) | COMPLEX(LDR,P), where LDR. GE. P. contains the upper triangular matrix that is to be downdated. The part of R below the diagonal is not referenced. |
| 2 | `LDR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array R. |
| 3 | `P` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the order of the matrix R. |
| 4 | `X` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(P). contains the row vector that is to be removed from R. X is not altered by CCHDD. |
| 5 | `Z` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDZ, *) | COMPLEX(LDZ,NZ), where LDZ. GE. P. is an array of NZ P-vectors which are to be downdated along with R. contain the downdated quantities. |
| 6 | `LDZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array Z. |
| 7 | `NZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the number of vectors to be downdated NZ may be zero, in which case Z, Y, and RHO are not referenced. |
| 8 | `Y` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(NZ). contains the scalars for the downdating of the vectors Z. Y is not altered by CCHDD. |
| 9 | `RHO` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(NZ). contains the norms of the residual vectors that are to be downdated. |
| 10 | `C` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(P). contains the cosines of the transforming rotations. |
| 11 | `S` | `output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(P). contains the sines of the transforming rotations. |
| 12 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is set as follows. INFO = 0 if the entire downdating was successful. INFO =-1 if R could not be downdated. in this case, all quantities are left unaltered. INFO = 1 if some RHO could not be downdated. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 if the entire downdating was successful. |
| `INFO` | `-1` | if R could not be downdated. in this case, all quantities are left unaltered. |
| `INFO` | `1` | 1 if some RHO could not be downdated. The offending RHO's are set to -1. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchdd`. Native symbol: `cchdd_`. Declaration feature: `linear-algebra-complex`. Provider feature: `linear-algebra-complex`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_complex32_array_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchdd`
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
