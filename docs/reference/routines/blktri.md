# BLKTRI

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a block tridiagonal system of linear equations (usually resulting from the discretization of separable two-dimensional elliptic equations).

## Description

Subroutine BLKTRI Solves a System of Linear Equations of the Form AN(J)*X(I,J-1) + AM(I)*X(I-1,J) + (BN(J)+BM(I))*X(I,J) + CN(J)*X(I,J+1) + CM(I)*X(I+1,J) = Y(I,J) for I = 1,2,...,M and J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N), X(I,N+1) = X(I,1), X(0,J) = X(M,J), X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations. Boundary conditions may be Dirichlet, Neumann, or Periodic.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `FISHPACK elliptic PDE solvers`
- Mathematical domain: `pde-integral-equations`
- Package provenance: `fishpack`
- GAMS classifications: `I2B4B`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fishfft/blktri.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/blktri.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/blktri.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [BLKTRI](https://www.netlib.org/slatec/fishfft/blktri.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IFLG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = 0 Initialization only. Certain quantities that depend on NP,. |
| 2 | `NP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = 0 If AN(1) and CN(N) are not zero, which corresponds to periodic boundary conditions. = 1 If AN(1) and CN(N) are zero. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. The number of unknowns in the J-direction. N must be greater than 4. |
| 4 | `AN` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. One-dimensional arrays of length N that specify the coefficients in the linear equations given above. |
| 5 | `BN` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. One-dimensional arrays of length N that specify the coefficients in the linear equations given above. |
| 6 | `CN` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. One-dimensional arrays of length N that specify the coefficients in the linear equations given above. |
| 7 | `MP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input M-direction coupling selector. `MP=0` requests the periodic coefficient case and requires the endpoint off-diagonal coefficients to be nonzero; `MP=1` selects the noncyclic endpoint case where those endpoint coefficients are zero. |
| 8 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the I-direction. M must be greater than 4. |
| 9 | `AM` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length M that specify the coefficients in the linear equations given above. |
| 10 | `BM` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length M that specify the coefficients in the linear equations given above. |
| 11 | `CM` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length M that specify the coefficients in the linear equations given above. |
| 12 | `IDIMY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the two-dimensional array Y as it appears in the program calling BLKTRI. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M. |
| 13 | `Y` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMY, *) | A two-dimensional array that specifies the values of the right side of the linear system of equations given above. Y must be dimensioned at least M*N. Contains the solution X. |
| 14 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable status output. `0` means success; `1` means `M < 5`; `2` means `N < 5`; `3` means `IDIMY < M`; `4` reports a coefficient-array failure; and `5` reports an invalid negative product in the tridiagonal coefficient condition. Except for `0`, no solution is produced. |
| 15 | `W` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for must have dimension (K-2)*L+K+5+MAX(2N,6M) If NP=0 define K=INT(log2(N-1))+1 and set L=2**(K+1) then must have dimension (K-2)*L+K+5+2N+MAX(2N,6M) IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if BLKTRI will be called again with IFLG=1. W(1) contains the number of locations required by W in floating point format. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error. |
| `IERROR` | `1` | 1 M is less than 5. |
| `IERROR` | `2` | 2 N is less than 5. |
| `IERROR` | `3` | 3 IDIMY is less than M. |
| `IERROR` | `4` | 4 BLKTRI failed while computing results that depend on the coefficient arrays AN, BN, CN. Check these arrays. |
| `IERROR` | `5` | 5 AN(J)*CN(J-1) is less than 0 for some J. Possible reasons for this condition are 1. The arrays AN and CN are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::blktri`. Native symbol: `blktri_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::blktri`
- Public declaration feature: `fishpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
