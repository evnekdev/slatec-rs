# DEXINT

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute an M member sequence of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0.

## Description

DEXINT computes M member sequences of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0. The exponential integral is defined by E(N,X)=integral on (1,infinity) of EXP(-XT)/T**N where X=0.0 and N=1 cannot occur simultaneously. Formulas and notation are found in the NBS Handbook of Mathematical Functions (ref. 1). The power series is implemented for X .LE. XCUT and the confluent hypergeometric representation E(A,X) = EXP(-X)*(X**(A-1))*U(A,A,X) is computed for X .GT. XCUT. Since sequences are computed in a stable fashion by recurring away from X, A is selected as the integer closest to X within the constraint N .LE. A .LE. N+M-1. For the U computation, A is further modified to be the nearest even integer. Indices are carried forward or backward by the two term recursion relation K*E(K+1,X) + X*E(K,X) = EXP(-X) once E(A,X) is computed. The U function is computed by means of the backward recursive Miller algorithm applied to the three term contiguous relation for U(A+K,A,X), K=0,1,... This produces accurate ratios and determines U(A+K,A,X), and hence E(A,X), to within a multiplicative constant C. Another contiguous relation applied to C*U(A,A,X) and C*U(A+1,A,X) gets C*U(A+1,A+1,X), a quantity proportional to E(A+1,X). The normalizing constant C is obtained from the two term recursion relation above with K=A. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C5`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dexint.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dexint.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dexint.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dexint.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DEXINT](https://www.netlib.org/slatec/src/dexint.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | 1 and  X .GE. 0.0 for N .GE. 2 1 and  X .GE. 0.0 for N .GE. 2 N       order of the first member of the sequence, N .GE. 1 N       order of the first member of the sequence, N .GE. 1 |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 and  X .GE. 0.0 for N .GE. 2 N       order of the first member of the sequence, N .GE. 1 |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `TOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `EN` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 8 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

KODE    a selection parameter for scaled values KODE=1   returns        E(N+K,X), K=0,1,...,M-1. =2   returns EXP(X)*E(N+K,X), K=0,1,...,M-1. M       number of exponential integrals in the sequence, M .GE. 1 TOL     relative accuracy wanted, ETOL .LE. TOL .LE. 0.1 ETOL is the larger of double precision unit roundoff = D1MACH(4) and 1.0D-18 Output    * EN is a double precision vector * EN      a vector of dimension at least M containing values EN(K) = E(N+K-1,X) or EXP(X)*E(N+K-1,X), K=1,M depending on KODE NZ      underflow indicator NZ=0   a normal return NZ=M   X exceeds XLIM and an underflow occurs. EN(K)=0.0D0 , K=1,M returned on KODE=1 IERR=0, normal return, computation completed algorithm termination condition not met

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dexint`. Native symbol: `dexint_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dexint`
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
