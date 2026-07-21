# QNC79

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Integrate a function using a 7-point adaptive Newton-Cotes quadrature rule.

## Description

QNC79 is a general purpose program for evaluation of one dimensional integrals of user defined functions. QNC79 will pick its own points for evaluation of the integrand and these will vary from problem to problem. Thus, QNC79 is not designed to integrate over data sets. Moderately smooth integrands will be integrated efficiently and reliably. For problems with strong singularities, oscillations etc., the user may wish to use more sophis- ticated routines such as those in QUADPACK. One measure of the reliability of QNC79 is the output parameter K, giving the number of integrand evaluations that were needed.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_nc79_f32`

## Providers

- Canonical provider: `main-src/src/qnc79.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qnc79.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qnc79.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qnc79.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [QNC79](https://www.netlib.org/slatec/src/qnc79.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FUN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | name of external function to be integrated.  This name must be in an EXTERNAL statement in your calling program.  You must write a Fortran function to evaluate FUN.  This should be of the form REAL FUNCTION FUN (X) C C     X can vary from A to B C     FUN(X) should be finite for all X on interval. C ... RETURN END |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | lower limit of integration |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | upper limit of integration (may be less than A) |
| 4 | `ERR` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `ANS` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | computed value of the integral.  Hopefully, ANS is accurate to within ERR * integral of ABS(FUN(X)). |
| 6 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a status code - Normal codes |
| 7 | `K` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

0 .LT. ERR .LT. 1.0E-3. -1  A equals B, or A and B are too nearly equal to allow normal integration.  ANS is set to zero. - Abnormal code K    - the number of function evaluations actually used to do the integration.  A value of K .GT. 1000 indicates a difficult problem; other programs may be more efficient. QNC79 will gracefully give up if K exceeds 2000.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::qnc79`. Native symbol: `qnc79_`. Declaration feature: `quadrature-nonadaptive`. Provider feature: `quadrature-nonadaptive`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::qnc79`
- Public declaration feature: `quadrature-nonadaptive`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_nc79_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
