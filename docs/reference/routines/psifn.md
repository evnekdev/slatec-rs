# PSIFN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute derivatives of the Psi function.

## Description

The following definitions are used in PSIFN: Definition 1 PSI(X) = d/dx (ln(GAMMA(X)), the first derivative of the LOG GAMMA function. Definition 2 K K PSI(K,X) = d /dx (PSI(X)), the K-th derivative of PSI(X). ___________________________________________________________________ PSIFN computes a sequence of SCALED derivatives of the PSI function; i.e. for fixed X and M it computes the M-member sequence ((-1)**(K+1)/GAMMA(K+1))*PSI(K,X) for K = N,...,N+M-1 where PSI(K,X) is as defined above. For KODE=1, PSIFN returns the scaled derivatives as described. KODE=2 is operative only when K=0 and in that case PSIFN returns -PSI(X) + LN(X). That is, the logarithmic behavior for large X is removed when KODE=1 and K=0. When sums or differences of PSI functions are computed the logarithmic terms can be combined analytically and computed separately to help retain significant digits. Note that CALL PSIFN(X,0,1,1,ANS) results in ANS = -PSI(X)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C7C`
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

- Canonical provider: `main-src/src/psifn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/psifn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/psifn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/psifn.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
