# SNRM2

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the Euclidean length (L2 norm) of a vector.

## Description

B L A S Subprogram Description of Parameters

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1A3B`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level1::snrm2, slatec::blas::level1::snrm2_strided`

## Providers

- Canonical provider: `lin/snrm2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/snrm2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SNRM2](https://www.netlib.org/slatec/lin/snrm2.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of elements in input vector(s) vector stored in SX with storage increment INCX . 0. must be .GE. 1 Four Phase Method using two built-in constants that are hopefully applicable to all machines. CUTLO = maximum of  SQRT(U/EPS)  over all known machines. CUTHI = minimum of  SQRT(V)      over all known machines. where EPS = smallest no. such that EPS + 1. .GT. 1. U   = smallest positive no.   (underflow limit) V   = largest  no.            (overflow  limit) Brief Outline of Algorithm. Phase 1 scans zero components. Move to phase 2 when a component is nonzero and .LE. CUTLO Move to phase 3 when a component is .GT. CUTLO Move to phase 4 when a component is .GE. CUTHI/M where M = N for X() real and M = 2*N for complex. Values for CUTLO and CUTHI. From the environmental parameters listed in the IMSL converter document the limiting values are as follows: CUTLO, S.P.   U/EPS = 2**(-102) for  Honeywell.  Close seconds are Univac and DEC at 2**(-103) Thus CUTLO = 2**(-51) = 4.44089E-16 CUTHI, S.P.   V = 2**127 for Univac, Honeywell, and DEC. Thus CUTHI = 2**(63.5) = 1.30438E19 CUTLO, D.P.   U/EPS = 2**(-67) for Honeywell and DEC. Thus CUTLO = 2**(-33.5) = 8.23181D-11 CUTHI, D.P.   same as S.P.  CUTHI = 1.30438D19 DATA CUTLO, CUTHI /8.232D-11,  1.304D19/ DATA CUTLO, CUTHI /4.441E-16,  1.304E19/ |
| 2 | `SX` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | single precision vector with N elements |
| 3 | `INCX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | storage spacing between elements of SX must be .GE. 1 Four Phase Method using two built-in constants that are hopefully applicable to all machines. CUTLO = maximum of  SQRT(U/EPS)  over all known machines. CUTHI = minimum of  SQRT(V)      over all known machines. where EPS = smallest no. such that EPS + 1. .GT. 1. U   = smallest positive no.   (underflow limit) V   = largest  no.            (overflow  limit) Brief Outline of Algorithm. Phase 1 scans zero components. Move to phase 2 when a component is nonzero and .LE. CUTLO Move to phase 3 when a component is .GT. CUTLO Move to phase 4 when a component is .GE. CUTHI/M where M = N for X() real and M = 2*N for complex. Values for CUTLO and CUTHI. From the environmental parameters listed in the IMSL converter document the limiting values are as follows: CUTLO, S.P.   U/EPS = 2**(-102) for  Honeywell.  Close seconds are Univac and DEC at 2**(-103) Thus CUTLO = 2**(-51) = 4.44089E-16 CUTHI, S.P.   V = 2**127 for Univac, Honeywell, and DEC. Thus CUTHI = 2**(63.5) = 1.30438E19 CUTLO, D.P.   U/EPS = 2**(-67) for Honeywell and DEC. Thus CUTLO = 2**(-33.5) = 8.23181D-11 CUTHI, D.P.   same as S.P.  CUTHI = 1.30438D19 DATA CUTLO, CUTHI /8.232D-11,  1.304D19/ DATA CUTLO, CUTHI /4.441E-16,  1.304E19/ |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `unavailable`.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::snrm2`. Native symbol: `snrm2_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::snrm2`
- Public declaration feature: `blas-level1`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::blas::level1::snrm2`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
