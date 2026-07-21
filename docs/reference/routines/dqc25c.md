# DQC25C

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F*W over (A,B) with error estimate, where W(X) = 1/(X-C)

## Description

Integration rules for the computation of CAUCHY PRINCIPAL VALUE integrals Standard fortran subroutine Double precision version

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dqc25c.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqc25c.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqc25c.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqc25c.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DQC25C](https://www.netlib.org/slatec/src/dqc25c.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Left end point of the integration interval. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Right end point of the integration interval, B. GT. A. |
| 4 | `C` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameter in the WEIGHT function. |
| 5 | `RESULT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Approximation to the integral is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. In the other case the 15-point Kronrod rule obtained by optimal addition of abscissae to the 7-point Gauss rule, is applied. |
| 6 | `ABSERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT). |
| 7 | `KRUL` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used. |
| 8 | `NEVAL` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of integrand evaluations. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::dqc25c`. Native symbol: `dqc25c_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f64(ref_f64),mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqc25c`
- Public declaration feature: `quadrature-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
