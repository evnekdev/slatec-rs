# DSVDC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform the singular value decomposition of a rectangular matrix.

## Description

DSVDC is a subroutine to reduce a double precision NxP matrix X by orthogonal transformations U and V to diagonal form. The diagonal elements S(I) are the singular values of X. The columns of U are the corresponding left singular vectors, and the columns of V the right singular vectors. On Entry

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
- GAMS classifications: `D6`
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

- Canonical provider: `lin/dsvdc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsvdc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dsvdc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DSVDC](https://www.netlib.org/slatec/lin/dsvdc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDX, *) | DOUBLE PRECISION(LDX,P), where LDX. GE. N. contains the matrix whose singular value decomposition is to be computed. X is destroyed by DSVDC. |
| 2 | `LDX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array X. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the number of rows of the matrix X. |
| 4 | `P` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the number of columns of the matrix X. |
| 5 | `S` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(MM), where MM=MIN(N+1,P). The first MIN(N,P) entries of S contain the singular values of X arranged in descending order of magnitude. |
| 6 | `E` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(P). E ordinarily contains zeros. However see the discussion of INFO for exceptions. |
| 7 | `U` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDU, *) | DOUBLE PRECISION(LDU,K), where LDU. GE. N. If JOBA. EQ. 1, then K. |
| 8 | `LDU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array U. (See below). |
| 9 | `V` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDV, *) | DOUBLE PRECISION(LDV,P), where LDV. GE. P. contains the matrix of right singular vectors. is not referenced if JOB. EQ. |
| 10 | `LDV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array V. (See below). |
| 11 | `WORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(N). is a scratch array. |
| 12 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. JOB controls the computation of the singular vectors. It has the decimal expansion AB with the following meaning A. EQ. 0 do not compute the left singular A. 1 return the N left singular vectors in U. |
| 13 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. The singular values (and their corresponding singular vectors) S(INFO+1),S(INFO+2),. ,S(M) are correct (here M=MIN(N,P)). Thus if. EQ. 0, all the singular values and their vectors are correct. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`WORK`: DOUBLE PRECISION(N). is a scratch array.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::dsvdc`. Native symbol: `dsvdc_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsvdc`
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
