# GENBUN

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve by a cyclic reduction algorithm the linear system of equations that results from a finite difference approximation to certain 2-d elliptic PDE's on a centered grid .

## Description

Subroutine GENBUN solves the linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I = 1,2,...,M and J = 1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e., X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to 0, X(I,2), or X(I,N) and X(I,N+1) may be equal to 0, X(I,N-1), or X(I,1) depending on an input parameter. * * * * * * * Parameter Description * * * * * * * * * *

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

- Canonical provider: `fishfft/genbun.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/genbun.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/genbun.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [GENBUN](https://www.netlib.org/slatec/fishfft/genbun.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the values that X(I,0) and X(I,N+1) are assumed to have. = 0 If X(I,0) = X(I,N) and X(I,N+1) = X(I,1). = 1 If X(I,0) = X(I,N+1) = 0. = 2 If X(I,0) = 0 and X(I,N+1) = X(I,N-1). = 3 If X(I,0) = X(I,2) and X(I,N+1) = X(I,N-1). = 4 If X(I,0) = X(I,2) and X(I,N+1) = 0. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the J-direction. N must be greater than 2. |
| 3 | `MPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = 0 if A(1) and C(M) are not zero. = 1 if A(1) = C(M) = 0. |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the I-direction. M must be greater than 2. N) MPEROD NPEROD T(MSECS) E 31 0 0 36 6. E-14 31 1 1 21 4. E-13 31 1 3 41 3. E-13 32 0 0 29 9. |
| 5 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition C(1) C(I) = -0. 5*B(I) = 1, I=1,2,. ,M and, when MPEROD = 1 C(M) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed. |
| 6 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition B(1) for I=1,2,. ,M. |
| 7 | `C` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition C(1). |
| 8 | `IDIMY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the two-dimensional array Y as it appears in the program calling GENBUN. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M. |
| 9 | `Y` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMY, *) | A two-dimensional array that specifies the values of the right side of the linear system of equations given above. Y must be dimensioned at least M*N. Contains the solution X. |
| 10 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error. = 1 M. LE. 2 = 2 N. |
| 11 | `W` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space. W may require up to 4*N + (10 + INT(log2(N)))*M locations. The actual number of locations used is computed by GENBUN and is returned in location W(1). contains the required length of W. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error. |
| `IERROR` | `1` | 1 M .LE. 2 |
| `IERROR` | `2` | 2 N .LE. 2 |
| `IERROR` | `3` | 3 IDIMY .LT. M |
| `IERROR` | `4` | 4 NPEROD .LT. 0 or NPEROD .GT. 4 |
| `IERROR` | `5` | 5 MPEROD .LT. 0 or MPEROD .GT. 1 |
| `IERROR` | `6` | 6 A(I) .NE. C(1) or C(I) .NE. C(1) or B(I) .NE. B(1) for |
| `IERROR` | `1` | ,2,...,M. |
| `IERROR` | `7` | 7 A(1) .NE. 0 or C(M) .NE. 0 and MPEROD = 1 |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::genbun`. Native symbol: `genbun_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::genbun`
- Public declaration feature: `fishpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
