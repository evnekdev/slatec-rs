# DCHEX

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Update the Cholesky factorization A=TRANS(R)*R of a positive definite matrix A of order P under diagonal permutations of the form TRANS(E)*A*E, where E is a permutation matrix.

## Description

DCHEX updates the Cholesky factorization A = TRANS(R)*R of a positive definite matrix A of order P under diagonal permutations of the form TRANS(E)*A*E where E is a permutation matrix. Specifically, given an upper triangular matrix R and a permutation matrix E (which is specified by K, L, and JOB), DCHEX determines an orthogonal matrix U such that U*R*E = RR, where RR is upper triangular. At the users option, the transformation U will be multiplied into the array Z. If A = TRANS(X)*X, so that R is the triangular part of the QR factorization of X, then RR is the triangular part of the QR factorization of X*E, i.e. X with its columns permuted. For a less terse description of what DCHEX does and how it may be applied, see the LINPACK guide. The matrix Q is determined as the product U(L-K)*...*U(1) of plane rotations of the form ( C(I) S(I) ) ( ) , ( -S(I) C(I) ) where C(I) is double precision. The rows these rotations operate on are described below. There are two types of permutations, which are determined by the value of JOB. 1. Right circular shift (JOB = 1). The columns are rearranged in the following order. 1,...,K-1,L,K,K+1,...,L-1,L+1,...,P. U is the product of L-K rotations U(I), where U(I) acts in the (L-I,L-I+1)-plane. 2. Left circular shift (JOB = 2). The columns are rearranged in the following order 1,...,K-1,K+1,K+2,...,L,K,L+1,...,P. U is the product of L-K rotations U(I), where U(I) acts in the (K+I-1,K+I)-plane. On Entry R DOUBLE PRECISION(LDR,P), where LDR .GE. P. R contains the upper triangular factor that is to be updated. Elements of R below the diagonal are not referenced. LDR INTEGER. LDR is the leading dimension of the array R. P INTEGER. P is the order of the matrix R. K INTEGER. K is the first column to be permuted. L INTEGER. L is the last column to be permuted. L must be strictly greater than K. Z DOUBLE PRECISION(LDZ,N)Z), where LDZ .GE. P. Z is an array of NZ P-vectors into which the transformation U is multiplied. Z is not referenced if NZ = 0. LDZ INTEGER. LDZ is the leading dimension of the array Z. NZ INTEGER. NZ is the number of columns of the matrix Z. JOB INTEGER. JOB determines the type of permutation. JOB = 1 right circular shift. JOB = 2 left circular shift. On Return R contains the updated factor. Z contains the updated matrix Z. C DOUBLE PRECISION(P). C contains the cosines of the transforming rotations. S DOUBLE PRECISION(P). S contains the sines of the transforming rotations.

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

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DCHEX](https://www.netlib.org/slatec/lin/dchex.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `R` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDR, *) | Array argument classified by fixed-form executable read/write analysis. |
| 2 | `LDR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `P` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `Z` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDZ, *) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `LDZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 9 | `C` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `S` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 11 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

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
