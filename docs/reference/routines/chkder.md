# CHKDER

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Check the gradients of M nonlinear functions in N variables, evaluated at a point X, for consistency with the functions themselves.

## Description

This subroutine is a companion routine to SNLS1,SNLS1E,SNSQ,and SNSQE which may be used to check the calculation of the Jacobian. SUBROUTINE CHKDER This subroutine checks the gradients of M nonlinear functions in N variables, evaluated at a point X, for consistency with the functions themselves. The user must call CKDER twice, first with MODE = 1 and then with MODE = 2.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::nonlinear::check_jacobian_f32`

## Providers

- Canonical provider: `main-src/src/chkder.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/chkder.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/chkder.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/chkder.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CHKDER](https://www.netlib.org/slatec/src/chkder.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of functions. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of variables. |
| 3 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an input array of length N. |
| 4 | `FVEC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an array of length M. On input when MODE = 2, must contain the functions evaluated at X. |
| 5 | `FJAC` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDFJAC, *) | is an M by N array. On input when MODE = 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. |
| 6 | `LDFJAC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input parameter not less than M which specifies the leading dimension of the array FJAC. |
| 7 | `XP` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an array of length N. On output when MODE = 1, is set to a neighboring point of X. |
| 8 | `FVECP` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an array of length M. On input when MODE = 2, must contain the functions evaluated at XP. |
| 9 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1. On input, X must contain the point of evaluation. On output, XP is set to a neighboring point. 2. On input, FVEC must contain the functions and the rows of FJAC must contain the gradients of the respective functions each evaluated at X, and FVECP must contain the functions evaluated at XP. On output, ERR contains measures of correctness of the respective gradients. |
| 10 | `ERR` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an array of length M. On output when MODE = 2, contains measures of correctness of the respective gradients. If there is no severe loss of significance, then if ERR(I) is 1. 0 the I-th gradient is correct, while if ERR(I) is 0. 0 the I-th gradient is incorrect. For values of ERR between 0. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::nonlinear::jacobian_check::chkder`. Native symbol: `chkder_`. Declaration feature: `nonlinear`. Provider feature: `nonlinear-jacobian`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::nonlinear::jacobian_check::chkder`
- Public declaration feature: `nonlinear`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::nonlinear::check_jacobian_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
