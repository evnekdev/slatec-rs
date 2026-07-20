# DPOCH1

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate a generalization of Pochhammer's symbol starting from first order.

## Description

Evaluate a double precision generalization of Pochhammer's symbol for double precision A and X for special situations that require especially accurate values when X is small in POCH1(A,X) = (POCH(A,X)-1)/X = (GAMMA(A+X)/GAMMA(A) - 1.0)/X . This specification is particularly suited for stably computing expressions such as (GAMMA(A+X)/GAMMA(A) - GAMMA(B+X)/GAMMA(B))/X = POCH1(A,X) - POCH1(B,X) Note that POCH1(A,0.0) = PSI(A) When ABS(X) is so small that substantial cancellation will occur if the straightforward formula is used, we use an expansion due to Fields and discussed by Y. L. Luke, The Special Functions and Their Approximations, Vol. 1, Academic Press, 1969, page 34. The ratio POCH(A,X) = GAMMA(A+X)/GAMMA(A) is written by Luke as (A+(X-1)/2)**X * polynomial in (A+(X-1)/2)**(-2) . In order to maintain significance in POCH1, we write for positive a (A+(X-1)/2)**X = EXP(X*LOG(A+(X-1)/2)) = EXP(Q) = 1.0 + Q*EXPREL(Q) . Likewise the polynomial is written POLY = 1.0 + X*POLY1(A,X) . Thus, POCH1(A,X) = (POCH(A,X) - 1) / X = EXPREL(Q)*(Q/X + Q*POLY1(A,X)) + POLY1(A,X)

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
- GAMS classifications: `C1`
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

- Canonical provider: `fnlib/dpoch1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dpoch1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dpoch1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Evaluate a double precision generalization of Pochhammer's symbol for double precision A and X for special situations that require especially accurate values when X is small in POCH1(A,X) = (POCH(A,X)-1)/X = (GAMMA(A+X)/GAMMA(A) - 1.0)/X . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Evaluate a double precision generalization of Pochhammer's symbol for double precision A and X for special situations that require especially accurate values when X is small in POCH1(A,X) = (POCH(A,X)-1)/X = (GAMMA(A+X)/GAMMA(A) - 1.0)/X . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f64` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:f64(mut_f64,mut_f64)`.

### ABI and safety

Canonical path: `slatec_sys::special::dpoch1`. Native symbol: `dpoch1_`. Feature: `special`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:f64(mut_f64,mut_f64)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dpoch1`
- Compatibility aliases: `slatec_sys::special::numerical::dpoch1`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
