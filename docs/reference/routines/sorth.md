# SORTH

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Internal routine for SGMRES.

## Description

This routine orthogonalizes the vector VNEW against the previous KMP vectors in the V array. It uses a modified Gram-Schmidt orthogonalization procedure with conditional reorthogonalization. *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). On output, the new vector orthogonal to V(*,i0) to V(*,LL), where i0 = max(1, LL-KMP+1). V :IN Real V(N,LL) The N x LL array containing the previous LL orthogonal vectors V(*,1) to V(*,LL). HES :INOUT Real HES(LDHES,LL) On input, an LL x LL upper Hessenberg matrix containing, in HES(I,K), K.lt.LL, the scaled inner products of A*V(*,K) and V(*,i). On return, column LL of HES is filled in with the scaled inner products of A*V(*,LL) and V(*,i). N :IN Integer The order of the matrix A, and the length of VNEW. LL :IN Integer The current order of the matrix HES. LDHES :IN Integer The leading dimension of the HES array. KMP :IN Integer The number of previous vectors the new vector VNEW must be made orthogonal to (KMP .le. MAXL). SNORMW :OUT REAL Scalar containing the l-2 norm of VNEW.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
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

- Canonical provider: `lin/sorth.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sorth.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sorth.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `VNEW` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | This routine orthogonalizes the vector VNEW against the previous KMP vectors in the V array. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (N, *) | This routine orthogonalizes the vector VNEW against the previous KMP vectors in the V array. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `HES` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDHES, *) | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LL` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDHES` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KMP` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This routine orthogonalizes the vector VNEW against the previous KMP vectors in the V array. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SNORMW` | output | `REAL` (`explicit`) | `*mut f32` | scalar | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). | *Usage: INTEGER N, LL, LDHES, KMP REAL VNEW(N), V(N,LL), HES(LDHES,LL), SNORMW CALL SORTH(VNEW, V, HES, N, LL, LDHES, KMP, SNORMW) *Arguments: VNEW :INOUT Real VNEW(N) On input, the vector of length N containing a scaled product of the Jacobian and the vector V(*,LL). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
