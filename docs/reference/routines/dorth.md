# DORTH

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Internal routine for DGMRES.

## Description

This routine orthogonalizes the vector VNEW against the previous KMP vectors in the V array. It uses a modified Gram-Schmidt orthogonalization procedure with conditional reorthogonalization. *Usage: INTEGER N, LL, LDHES, KMP DOUBLE PRECISION VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL DORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Double Precision VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). On output, the new vector orthogonal to V(*,i0) to V(*,LL), where i0 = max(1, LL-KMP+1). V :IN Double Precision V(N,LL) The N x LL array containing the previous LL orthogonal vectors V(*,1) to V(*,LL). HES :INOUT Double Precision HES(LDHES,LL) On input, an LL x LL upper Hessenberg matrix containing, in HES(I,K), K.lt.LL, the scaled inner products of A*V(*,K) and V(*,i). On return, column LL of HES is filled in with the scaled inner products of A*V(*,LL) and V(*,i). N :IN Integer The order of the matrix A, and the length of VNEW. LL :IN Integer The current order of the matrix HES. LDHES :IN Integer The leading dimension of the HES array. KMP :IN Integer The number of previous vectors the new vector VNEW must be made orthogonal to (KMP .le. MAXL). SNORMW :OUT DOUBLE PRECISION Scalar containing the l-2 norm of VNEW.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2A4`
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

- Canonical provider: `lin/dorth.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dorth.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dorth.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
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
