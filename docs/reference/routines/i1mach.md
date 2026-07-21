# I1MACH

[Family: Runtime and machine support](../families/runtime-and-machine-support.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Return integer machine dependent constants.

## Description

I1MACH can be used to obtain machine-dependent parameters for the local machine environment. It is a function subprogram with one (input) argument and can be referenced as follows: K = I1MACH(I) where I=1,...,16. The (output) value of K above is determined by the (input) value of I. The results for various values of I are discussed below. I/O unit numbers: I1MACH( 1) = the standard input unit. I1MACH( 2) = the standard output unit. I1MACH( 3) = the standard punch unit. I1MACH( 4) = the standard error message unit. Words: I1MACH( 5) = the number of bits per integer storage unit. I1MACH( 6) = the number of characters per integer storage unit. Integers: assume integers are represented in the S-digit, base-A form sign ( X(S-1)*A**(S-1) + ... + X(1)*A + X(0) ) where 0 .LE. X(I) .LT. A for I=0,...,S-1. I1MACH( 7) = A, the base. I1MACH( 8) = S, the number of base-A digits. I1MACH( 9) = A**S - 1, the largest magnitude. Floating-Point Numbers: Assume floating-point numbers are represented in the T-digit, base-B form sign (B**E)*( (X(1)/B) + ... + (X(T)/B**T) ) where 0 .LE. X(I) .LT. B for I=1,...,T, 0 .LT. X(1), and EMIN .LE. E .LE. EMAX. I1MACH(10) = B, the base. Single-Precision: I1MACH(11) = T, the number of base-B digits. I1MACH(12) = EMIN, the smallest exponent E. I1MACH(13) = EMAX, the largest exponent E. Double-Precision: I1MACH(14) = T, the number of base-B digits. I1MACH(15) = EMIN, the smallest exponent E. I1MACH(16) = EMAX, the largest exponent E. To alter this function for a particular environment, the desired set of DATA statements should be activated by removing the C from column 1. Also, the values of I1MACH(1) - I1MACH(4) should be checked for consistency with the local operating system.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `integer_or_index`
- Scalar kind: `integer`
- Primary family: `Runtime and machine support`
- Mathematical domain: `runtime-support`
- Package provenance: `slatec-machine-constants`
- GAMS classifications: `R1`
- Family evidence: `reviewed_override` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/i1mach.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/i1mach.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/i1mach.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `support_unit_minimal`
- Description provenance: `source_prologue`
- Assessment: the support identity records its role, side-effect boundary, and non-public disposition
- Dedicated family page: [Runtime and machine support](../families/runtime-and-machine-support.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `I` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | It is a function subprogram with one (input) argument and can be referenced as follows: K = I1MACH(I) where I=1,...,16. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut crate::FortranInteger` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `not_publicly_owned`.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `support-routine`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
