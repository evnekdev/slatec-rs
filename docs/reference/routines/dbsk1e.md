# DBSK1E

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order one.

## Description

DBSK1E(S) computes the double precision exponentially scaled modified (hyperbolic) Bessel function of the third kind of order one for positive double precision argument X. Series for BK1 on the interval 0. to 4.00000E+00 with weighted error 9.16E-32 log weighted error 31.04 significant figures required 30.61 decimal places required 31.64 Series for AK1 on the interval 1.25000E-01 to 5.00000E-01 with weighted error 3.07E-32 log weighted error 31.51 significant figures required 30.71 decimal places required 32.30 Series for AK12 on the interval 0. to 1.25000E-01 with weighted error 2.41E-32 log weighted error 31.62 significant figures required 30.25 decimal places required 32.38

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::bessel::bessel_k1_scaled`

## Providers

- Canonical provider: `fnlib/dbsk1e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dbsk1e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dbsk1e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [DBSK1E](https://www.netlib.org/slatec/fnlib/dbsk1e.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input positive double-precision argument. The function returns the exponentially scaled order-one modified Bessel K value at `X`; native code does not modify the scalar. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_f64)`.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dbsk1e`. Native symbol: `dbsk1e_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f64(mut_f64)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dbsk1e`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::bessel::bessel_k1_scaled`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
