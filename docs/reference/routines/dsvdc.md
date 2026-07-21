# DSVDC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform the singular value decomposition of a rectangular matrix.

## Description

DSVDC is a subroutine to reduce a double precision NxP matrix X by orthogonal transformations U and V to diagonal form. The diagonal elements S(I) are the singular values of X. The columns of U are the corresponding left singular vectors, and the columns of V the right singular vectors. On Entry X DOUBLE PRECISION(LDX,P), where LDX .GE. N. X contains the matrix whose singular value decomposition is to be computed. X is destroyed by DSVDC. LDX INTEGER. LDX is the leading dimension of the array X. N INTEGER. N is the number of rows of the matrix X. P INTEGER. P is the number of columns of the matrix X. LDU INTEGER. LDU is the leading dimension of the array U. (See below). LDV INTEGER. LDV is the leading dimension of the array V. (See below). WORK DOUBLE PRECISION(N). WORK is a scratch array. JOB INTEGER. JOB controls the computation of the singular vectors. It has the decimal expansion AB with the following meaning A .EQ. 0 do not compute the left singular vectors. A .EQ. 1 return the N left singular vectors in U. A .GE. 2 return the first MIN(N,P) singular vectors in U. B .EQ. 0 do not compute the right singular vectors. B .EQ. 1 return the right singular vectors in V. On Return S DOUBLE PRECISION(MM), where MM=MIN(N+1,P). The first MIN(N,P) entries of S contain the singular values of X arranged in descending order of magnitude. E DOUBLE PRECISION(P). E ordinarily contains zeros. However see the discussion of INFO for exceptions. U DOUBLE PRECISION(LDU,K), where LDU .GE. N. If JOBA .EQ. 1, then K .EQ. N. If JOBA .GE. 2, then K .EQ. MIN(N,P). U contains the matrix of right singular vectors. U is not referenced if JOBA .EQ. 0. If N .LE. P or if JOBA .EQ. 2, then U may be identified with X in the subroutine call. V DOUBLE PRECISION(LDV,P), where LDV .GE. P. V contains the matrix of right singular vectors. V is not referenced if JOB .EQ. 0. If P .LE. N, then V may be identified with X in the subroutine call. INFO INTEGER. The singular values (and their corresponding singular vectors) S(INFO+1),S(INFO+2),...,S(M) are correct (here M=MIN(N,P)). Thus if INFO .EQ. 0, all the singular values and their vectors are correct. In any event, the matrix B = TRANS(U)*X*V is the bidiagonal matrix with the elements of S on its diagonal and the elements of E on its super-diagonal (TRANS(U) is the transpose of U). Thus the singular values of X and B are the same.

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

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DSVDC](https://www.netlib.org/slatec/lin/dsvdc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDX, *) | Array argument classified by fixed-form executable read/write analysis. |
| 2 | `LDX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `P` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `S` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `E` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `U` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDU, *) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `LDU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `V` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDV, *) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `LDV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 12 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: Workspace argument classified by fixed-form executable read/write analysis.

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
