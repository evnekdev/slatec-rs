# DCHEX

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Update the Cholesky factorization A=TRANS(R)*R of a positive definite matrix A of order P under diagonal permutations of the form TRANS(E)*A*E, where E is a permutation matrix.

## Description

DCHEX updates the Cholesky factorization A = TRANS(R)*R of a positive definite matrix A of order P under diagonal permutations of the form TRANS(E)*A*E where E is a permutation matrix. Specifically, given an upper triangular matrix R and a permutation matrix E (which is specified by K, L, and JOB), DCHEX determines an orthogonal matrix U such that U*R*E = RR, where RR is upper triangular. At the users option, the transformation U will be multiplied into the array Z. If A = TRANS(X)*X, so that R is the triangular part of the QR factorization of X, then RR is the triangular part of the QR factorization of X*E, i.e. X with its columns permuted. For a less terse description of what DCHEX does and how it may be applied, see the LINPACK guide. The matrix Q is determined as the product U(L-K)*...*U(1) of plane rotations of the form ( C(I) S(I) ) ( ) , ( -S(I) C(I) ) where C(I) is double precision. The rows these rotations operate on are described below. There are two types of permutations, which are determined by the value of JOB. 1. Right circular shift (JOB = 1). The columns are rearranged in the following order. 1,...,K-1,L,K,K+1,...,L-1,L+1,...,P. U is the product of L-K rotations U(I), where U(I) acts in the (L-I,L-I+1)-plane. 2. Left circular shift (JOB = 2). 1,...,K-1,K+1,K+2,...,L,K,L+1,...,P. acts in the (K+I-1,K+I)-plane. On Entry

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

- Canonical provider: `lin/dchex.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dchex.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dchex.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DCHEX](https://www.netlib.org/slatec/lin/dchex.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `R` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDR, *) | DOUBLE PRECISION(LDR,P), where LDR. GE. P. contains the upper triangular factor that is to be updated. Elements of R below the diagonal are not referenced. contains the updated factor. |
| 2 | `LDR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array R. |
| 3 | `P` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the order of the matrix R. |
| 4 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the first column to be permuted. |
| 5 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the last column to be permuted. must be strictly greater than K. |
| 6 | `Z` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDZ, *) | DOUBLE PRECISION(LDZ,N)Z), where LDZ. GE. P. is an array of NZ P-vectors into which the transformation U is multiplied. Z is not referenced if NZ = 0. contains the updated matrix Z. |
| 7 | `LDZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array Z. |
| 8 | `NZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the number of columns of the matrix Z. |
| 9 | `C` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(P). contains the cosines of the transforming rotations. |
| 10 | `S` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(P). contains the sines of the transforming rotations. |
| 11 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. JOB determines the type of permutation. 1 right circular shift. 2 left circular shift. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::dchex`. Native symbol: `dchex_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::dchex`
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
