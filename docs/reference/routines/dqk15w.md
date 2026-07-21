# DQK15W

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F*W over (A,B), with error estimate J = Integral of ABS(F*W) over (A,B)

## Description

Integration rules Standard fortran subroutine Double precision version

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

- Canonical provider: `main-src/src/dqk15w.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqk15w.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqk15w.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqk15w.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DQK15W](https://www.netlib.org/slatec/src/dqk15w.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. |
| 2 | `W` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Function subprogram defining the integrand WEIGHT function W(X). The actual name for W needs to be declared E X T E R N A L in the calling program. |
| 3 | `P1` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameters in the WEIGHT function. |
| 4 | `P2` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameters in the WEIGHT function. |
| 5 | `P3` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameters in the WEIGHT function. |
| 6 | `P4` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Parameters in the WEIGHT function. |
| 7 | `KP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Key for indicating the type of WEIGHT function. |
| 8 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Lower limit of integration. |
| 9 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Upper limit of integration. |
| 10 | `RESULT` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Approximation to the integral I is computed by applying the 15-point Kronrod rule (RESK) obtained by optimal addition of abscissae to the 7-point Gauss rule (RESG). |
| 11 | `ABSERR` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT). |
| 12 | `RESABS` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Approximation to the integral of ABS(F). |
| 13 | `RESASC` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Approximation to the integral of ABS(F-I/(B-A)). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::callbacks::dqk15w`. Native symbol: `dqk15w_`. Declaration feature: `quadrature-callbacks`. Provider feature: `quadrature`. ABI fingerprint: `subroutine:void(fn:f64(ref_f64),fn:f64(ref_f64,ref_f64,ref_f64,ref_f64,ref_f64,ref_i32),mut_f64,mut_f64,mut_f64,mut_f64,mut_i32,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::dqk15w`
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
