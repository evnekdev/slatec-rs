# CINVIT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvectors of a complex upper Hessenberg associated with specified eigenvalues using inverse iteration.

## Description

This subroutine is a translation of the ALGOL procedure CXINVIT by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP. VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of A COMPLEX UPPER Hessenberg matrix corresponding to specified eigenvalues, using inverse iteration.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C2B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/cinvit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cinvit.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cinvit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [CINVIT](https://www.netlib.org/slatec/lin/cinvit.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (AR,AI).  N is an INTEGER variable.  N must be less than or equal to NM. if more than MM eigenvectors have been requested (the MM eigenvectors calculated to this point are in ZR and ZI), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the if both error situations occur. |
| 3 | `AR` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). are unaltered. |
| 4 | `AI` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). are unaltered. |
| 5 | `WR` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contain the real and imaginary parts, respectively, of the eigenvalues of the matrix.  The eigenvalues must be stored in a manner identical to that of subroutine  COMLR, which recognizes possible splitting of the matrix.  WR and may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. |
| 6 | `WI` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contain the real and imaginary parts, respectively, of the eigenvalues of the matrix.  The eigenvalues must be stored in a manner identical to that of subroutine  COMLR, which recognizes possible splitting of the matrix.  WR and dimensional REAL arrays, dimensioned WR(N) and are unaltered. |
| 7 | `SELECT` | `input` | `array` | `LOGICAL` | `*mut crate::FortranLogical` | rank 1; dimensions (N) | specifies the eigenvectors to be found.  The eigenvector corresponding to the J-th eigenvalue is is a is a one-dimensional LOGICAL array, dimensioned SELECT(N). one-dimensional LOGICAL array, dimensioned SELECT(N). are unaltered. |
| 8 | `MM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | should be set to an upper bound for the number of eigenvectors to be found.  MM is an INTEGER variable. |
| 9 | `M` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of eigenvectors actually found.  M is an INTEGER variable. |
| 10 | `ZR` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1.  Any vector which fails the acceptance test is set to zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and are set to zero vectors, |
| 11 | `ZI` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1.  Any vector which fails the acceptance test is set to zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and are set to zero vectors, |
| 12 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero       for normal return, |
| 13 | `RM1` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (N, *) | dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY |
| 14 | `RM2` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (N, *) | dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY |
| 15 | `RV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process. |
| 16 | `RV2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process. |

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

Canonical Rust path: `slatec_sys::linear_algebra::eigen::cinvit`. Native symbol: `cinvit_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cinvit`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
