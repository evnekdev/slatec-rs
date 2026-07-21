# POIS3D

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a three-dimensional block tridiagonal linear system which arises from a finite difference approximation to a three-dimensional Poisson equation using the Fourier transform package FFTPAK written by Paul Swarztrauber.

## Description

Subroutine POIS3D solves the linear system of equations

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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::differential_equations::pde::Pois3dProblem::solve`

## Providers

- Canonical provider: `fishfft/pois3d.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/pois3d.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/pois3d.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [POIS3D](https://www.netlib.org/slatec/fishfft/pois3d.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `LPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the values that X(0,J,K) and X(L+1,J,K) are assumed to have. = 0 If X(0,J,K) = X(L,J,K) and X(L+1,J,K) = X(1,J,K). = 1 If X(0,J,K) = X(L+1,J,K) = 0. = 2 If X(0,J,K) = 0 and X(L+1,J,K) = X(L-1,J,K). = 3 If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = X(L-1,J,K). = 4 If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0. |
| 2 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the I-direction. L must be at least 3. M=N) LPEROD MPEROD T(MSECS) E 16 0 0 272 1. E-13 15 1 1 287 4. E-13 17 3 3 338 2. E-13 32 0 0 1755 2. |
| 3 | `C1` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | + C2*(X(I,J-1,K)-2. *X(I,J,K)+X(I,J+1,K)) + A(K)*X(I,J,K-1)+B(K)*X(I,J,K)+C(K)*X(I,J,K+1) = F(I,J,K) for I=1,2,. ,L , J=1,2,. ,M , and K=1,2,. ,N. The indices K-1 and K+1 are evaluated modulo N, i. |
| 4 | `MPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the values that X(I,0,K) and X(I,M+1,K) are assumed to have. = 0 If X(I,0,K) = X(I,M,K) and X(I,M+1,K) = X(I,1,K). = 1 If X(I,0,K) = X(I,M+1,K) = 0. = 2 If X(I,0,K) = 0 and X(I,M+1,K) = X(I,M-1,K). = 3 If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = X(I,M-1,K). = 4 If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0. |
| 5 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the J-direction. M must be at least 3. |
| 6 | `C2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The real constant which appears in the above equation. |
| 7 | `NPEROD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0 If A(1) and C(N) are not zero. = 1 If A(1) = C(N) = 0. |
| 8 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the K-direction. N must be at least 3. |
| 9 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length N that specify the coefficients in the linear equations given above. If NPEROD = 0 the array elements must not depend upon the index K, but must be constant. Specifically, the subroutine checks the following condition C(1) C(K) = -0. 5*B(K) = 1, K=1,2,. ,N and, when NPEROD = 1 C(N) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed. |
| 10 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length N that specify the coefficients in the linear equations given above. If NPEROD = 0 the array elements must not depend upon the index K, but must be constant. Specifically, the subroutine checks the following condition B(1) for K=1,2,. ,N. |
| 11 | `C` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | One-dimensional arrays of length N that specify the coefficients in the linear equations given above. If NPEROD = 0 the array elements must not depend upon the index K, but must be constant. Specifically, the subroutine checks the following condition C(1). |
| 12 | `LDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the three-dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension of F. LDIMF must be at least L. |
| 13 | `MDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The column (or second) dimension of the three-dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension of F. MDIMF must be at least M. |
| 14 | `F` | `input-output` | `array` | `REAL` | `*mut f32` | rank 3; dimensions (LDIMF, MDIMF, *) | A three-dimensional array that specifies the values of the right side of the linear system of equations given above. F must be dimensioned at least L x M x N. Contains the solution X. |
| 15 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error = 1 If LPEROD. LT. 0 or. GT. |
| 16 | `W` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space. The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 If LPEROD .LT. 0 or .GT. 4 |
| `IERROR` | `2` | 2 If L .LT. 3 |
| `IERROR` | `3` | 3 If MPEROD .LT. 0 or .GT. 4 |
| `IERROR` | `4` | 4 If M .LT. 3 |
| `IERROR` | `5` | 5 If NPEROD .LT. 0 or .GT. 1 |
| `IERROR` | `6` | 6 If N .LT. 3 |
| `IERROR` | `7` | 7 If LDIMF .LT. L |
| `IERROR` | `8` | 8 If MDIMF .LT. M |
| `IERROR` | `9` | 9 If A(K) .NE. C(1) or C(K) .NE. C(1) or B(I) .NE.B(1) |
| `IERROR` | `1` | ,2,...,N. |
| `IERROR` | `10` | 10 If NPEROD = 1 and A(1) .NE. 0 or C(N) .NE. 0 Since this is the only means of indicating a possibly incorrect call to POIS3D, the user should test IERROR after the call. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::pois3d`. Native symbol: `pois3d_`. Declaration feature: `raw-family-fishpack-pois3d`. Provider feature: `fishpack-pois3d`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::pde::fishpack::pois3d`
- Public declaration feature: `raw-family-fishpack-pois3d`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::differential_equations::pde::Pois3dProblem::solve`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
