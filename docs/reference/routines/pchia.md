# PCHIA

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the definite integral of a piecewise cubic Hermite function over an arbitrary interval.

## Description

PCHIA: Piecewise Cubic Hermite Integrator, Arbitrary limits Evaluates the definite integral of the cubic Hermite function defined by N, X, F, D over the interval [A, B]. To provide compatibility with PCHIM and PCHIC, includes an

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::pchip::PiecewiseCubicHermite::integrate`

## Providers

- Canonical provider: `pchip/pchia.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchia.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/pchia.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [PCHIA](https://www.netlib.org/slatec/pchip/pchia.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) number of data points.  (Error return if N.LT.2 .) However, the resulting integral value will be highly suspect, if not. |
| 2 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (input) real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. However, the resulting integral value However, the resulting integral value will be highly suspect, if not. will be highly suspect, if not. |
| 3 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | and D-arrays. (input) real array of function values.  F(1+(I-1)*INCFD) is the value corresponding to X(I). |
| 4 | `D` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (INCFD, *) | (input) real array of derivative values.  D(1+(I-1)*INCFD) is the value corresponding to X(I). |
| 5 | `INCFD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ...) INTEGER  N, IERR REAL  X(N), F(INCFD,N), D(INCFD,N), A, B REAL  VALUE, PCHIA LOGICAL  SKIP VALUE = PCHIA (N, X, F, D, INCFD, SKIP, A, B, IERR) Parameters: VALUE -- (output) value of the requested integral. (input) increment between successive values in F and D. |
| 6 | `SKIP` | `input-output` | `scalar` | `LOGICAL` | `*mut crate::FortranLogical` | scalar | (input/output) logical variable which should be set to .TRUE. if the user wishes to skip checks for validity of preceding parameters, or to .FALSE. otherwise. This will save time in case these checks have already been performed (say, in PCHIM or PCHIC). will be set to .TRUE. on return with IERR.GE.0 . |
| 7 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) the limits of integration. NOTE:  There is no requirement that [A,B] be contained in contains data interval or the intervals do not intersect at all.) "Recoverable" errors: |
| 8 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) the limits of integration. NOTE:  There is no requirement that [A,B] be contained in contains data interval or the intervals do not intersect at all.) "Recoverable" errors: |
| 9 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (output) error flag. Normal return: 0  (no errors). Warning errors: 1  if  A  is outside the interval [X(1),X(N)]. 2  if  B  is outside the interval [X(1),X(N)]. 3  if both of the above are true.  (Note that this 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. (VALUE will be zero in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. 4  in case of an error return from PCHID (which should never occur). |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f32(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_f32,mut_f32,mut_i32)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::pchia`. Native symbol: `pchia_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `function:f32(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_f32,mut_f32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::pchia`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::pchip::PiecewiseCubicHermite::integrate`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
