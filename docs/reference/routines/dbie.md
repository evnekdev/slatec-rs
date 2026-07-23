# DBIE

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the Bairy function for a negative argument and an exponentially scaled Bairy function for a non-negative argument.

## Description

DBIE(X) calculates the double precision Airy function of the second kind or the double precision exponentially scaled Airy function of the second kind, depending on the value of the double precision argument X. Evaluate BI(X) for X .LE. 0.0 and BI(X)*EXP(-ZETA) where ZETA = 2/3 * X**(3/2) for X .GE. 0.0 Series for BIF on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.45E-32 log weighted error 31.84 significant figures required 30.85 decimal places required 32.40 Series for BIG on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.29E-33 log weighted error 32.89 significant figures required 31.48 decimal places required 33.45 Series for BIF2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 6.08E-32 log weighted error 31.22 approx significant figures required 30.8 decimal places required 31.80 Series for BIG2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 4.91E-33 log weighted error 32.31 approx significant figures required 31.6 decimal places required 32.90 Series for BIP1 on the interval 1.25000E-01 to 3.53553E-01 with weighted error 1.06E-32 log weighted error 31.98 significant figures required 30.61 decimal places required 32.81 Series for BIP2 on the interval 0. to 1.25000E-01 with weighted error 4.04E-33 log weighted error 32.39 significant figures required 31.15 decimal places required 33.37

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
- GAMS classifications: `C10D`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::airy::airy_bi_scaled`

## Providers

- Canonical provider: `fnlib/dbie.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dbie.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dbie.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DBIE](https://www.netlib.org/slatec/fnlib/dbie.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input value at which the source-defined function is evaluated: Calculate the Bairy function for a negative argument and an exponentially scaled Bairy function for a non-negative argument |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `unavailable`.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::airy::dbie`. Native symbol: `dbie_`. Declaration feature: `special-airy`. Provider feature: `special-airy`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::special::airy::dbie`
- Public declaration feature: `special-airy`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::special::airy::airy_bi_scaled`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
