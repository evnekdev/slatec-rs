# DBSI1E

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one.

## Description

DBSI1E(X) calculates the double precision exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one for double precision argument X. The result is I1(X) multiplied by EXP(-ABS(X)). Series for BI1 on the interval 0. to 9.00000E+00 with weighted error 1.44E-32 log weighted error 31.84 significant figures required 31.45 decimal places required 32.46 Series for AI1 on the interval 1.25000E-01 to 3.33333E-01 with weighted error 2.81E-32 log weighted error 31.55 significant figures required 29.93 decimal places required 32.38 Series for AI12 on the interval 0. to 1.25000E-01 with weighted error 1.83E-32 log weighted error 31.74 significant figures required 29.97 decimal places required 32.66

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
- Safe Rust paths: `slatec::special::bessel::bessel_i1_scaled`

## Providers

- Canonical provider: `fnlib/dbsi1e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dbsi1e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dbsi1e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DBSI1E](https://www.netlib.org/slatec/fnlib/dbsi1e.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | calculates the double precision exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one for double precision argument X.  The result is I1(X) multiplied by EXP(-ABS(X)). Series for BI1        on the interval  0.          to  9.00000E+00 with weighted error   1.44E-32 log weighted error  31.84 significant figures required  31.45 decimal places required  32.46 Series for AI1        on the interval  1.25000E-01 to  3.33333E-01 with weighted error   2.81E-32 log weighted error  31.55 significant figures required  29.93 decimal places required  32.38 Series for AI12       on the interval  0.          to  1.25000E-01 with weighted error   1.83E-32 log weighted error  31.74 significant figures required  29.97 decimal places required  32.66 |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_f64)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dbsi1e`. Native symbol: `dbsi1e_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f64(mut_f64)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dbsi1e`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::bessel::bessel_i1_scaled`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
