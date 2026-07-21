# FZERO

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Search for a zero of a function F(X) in a given interval (B,C). It is designed primarily for problems where F(B) and F(C) have opposite signs.

## Description

FZERO searches for a zero of a REAL function F(X) between the given REAL values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B-C) .LE. 2.*(RW*ABS(B)+AE). The method used is an efficient combination of bisection and the secant rule and is due to T. J. Dekker.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F1B`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::roots::find_root_f32`

## Providers

- Canonical provider: `main-src/src/fzero.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/fzero.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/fzero.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/fzero.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [FZERO](https://www.netlib.org/slatec/src/fzero.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `F` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Name of the REAL external function.  This name must be in an EXTERNAL statement in the calling program. F must be a function of one REAL argument. |
| 2 | `B` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | One end of the REAL interval (B,C).  The value returned for B usually is the better approximation to a zero of F. |
| 3 | `C` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | The other end of the REAL interval (B,C) |
| 4 | `R` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | A (better) REAL guess of a zero of F which could help in speeding up convergence.  If F(B) and F(R) have opposite signs, a root will be found in the interval (B,R); if not, but F(R) and F(C) have opposite signs, a root will be found in the interval (R,C); otherwise, the interval (B,C) will be searched for a possible root.  When no better guess is known, it is recommended that r be set to B or C, since if R is not interior to the interval (B,C), it will be ignored. |
| 5 | `RE` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `AE` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `IFLAG` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

If the requested RE is less than machine precision, then RW is set to approximately machine precision. the given interval (B,C) contains the origin, then a nonzero value should be chosen for AE. IFLAG :OUT   - A status code.  User must check IFLAG after each call.  Control returns to the user from FZERO in all cases. 1  B is within the requested tolerance of a zero. The interval (B,C) collapsed to the requested tolerance, the function changes sign in (B,C), and F(X) decreased in magnitude as (B,C) collapsed. 2  F(B) = 0.  However, the interval (B,C) may not have collapsed to the requested tolerance. 3  B may be near a singular point of F(X). The interval (B,C) collapsed to the requested tol- erance and the function changes sign in (B,C), but F(X) increased in magnitude as (B,C) collapsed, i.e. ABS(F(B out)) .GT. MAX(ABS(F(B in)),ABS(F(C in))) 4  No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether B is near a local minimum of F(X), or B is near a zero of even multiplicity, or neither of these. 5  Too many (.GT. 500) function evaluations used.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::roots::scalar::fzero`. Native symbol: `fzero_`. Declaration feature: `raw-family-roots-scalar`. Provider feature: `roots-scalar`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::roots::scalar::fzero`
- Public declaration feature: `raw-family-roots-scalar`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::roots::find_root_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
