# CLNREL

[Family: Elementary and transcendental functions](../families/elementary-and-transcendental-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate ln(1+X) accurate in the sense of relative error.

## Description

CLNREL(Z) = LOG(1+Z) with relative error accuracy near Z = 0. Let RHO = ABS(Z) and R**2 = ABS(1+Z)**2 = (1+X)**2 + Y**2 = 1 + 2*X + RHO**2 . Now if RHO is small we may evaluate CLNREL(Z) accurately by LOG(1+Z) = CMPLX (LOG(R), CARG(1+Z)) = CMPLX (0.5*LOG(R**2), CARG(1+Z)) = CMPLX (0.5*ALNREL(2*X+RHO**2), CARG(1+Z))

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Elementary and transcendental functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C4B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fnlib/clnrel.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/clnrel.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/clnrel.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [CLNREL](https://www.netlib.org/slatec/fnlib/clnrel.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `Z` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

Let   RHO = ABS(Z)  and R**2 = ABS(1+Z)**2 = (1+X)**2 + Y**2 = 1 + 2*X + RHO**2 . Now if RHO is small we may evaluate CLNREL(Z) accurately by LOG(1+Z) = CMPLX  (LOG(R), CARG(1+Z)) = CMPLX  (0.5*LOG(R**2), CARG(1+Z)) = CMPLX  (0.5*ALNREL(2*X+RHO**2), CARG(1+Z))

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::complex::clnrel`. Native symbol: `clnrel_`. Declaration feature: `special-complex`. Provider feature: `special-complex`. ABI fingerprint: `function:complex32(mut_complex32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::complex::clnrel`
- Public declaration feature: `special-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
