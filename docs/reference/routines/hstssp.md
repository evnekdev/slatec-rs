# HSTSSP

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in spherical coordinates and on the surface of the unit sphere (radius of 1).

## Description

HSTSSP solves the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in spherical coordinates and on the surface of the unit sphere (radius of 1) (1/SIN(THETA))(d/dTHETA)(SIN(THETA)(dU/dTHETA)) + (1/SIN(THETA)**2)(d/dPHI)(dU/dPHI) + LAMBDA*U = F(THETA,PHI) where THETA is colatitude and PHI is longitude. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * *

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
- GAMS classifications: `I2B1A1A`
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

- Canonical provider: `fishfft/hstssp.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hstssp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hstssp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [HSTSSP](https://www.netlib.org/slatec/fishfft/hstssp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i. e. A. LE. THETA. B. |
| 2 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i. e. A. LE. THETA. B. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of grid points in the interval (A,B). The grid points in the THETA-direction are given by THETA(I) = A + (I-0. 5)DTHETA for I=1,2,. ,M where DTHETA =(B-A)/M. M must be greater than 2. |
| 4 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at THETA = A and THETA = B. = 1 If the solution is specified at THETA = A and THETA = B. (see note 3 below) = 2 If the solution is specified at THETA = A and the derivative of the solution with respect to THETA is specified at THETA = B (see notes 2 and 3 below). = 3 If the derivative of the solution with respect to THETA is specified at THETA = A (see notes 1, 2 below) and THETA = B. = 4 If the derivative of the solution with respect to THETA is specified at THETA = A (see notes 1 and 2 below) and the solution is specified at THETA = B. = 5 If the solution is unspecified at THETA = A = 0 and the solution is specified at THETA = B. |
| 5 | `BDA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length N that specifies the boundary values (if any) of the solution at THETA = A. When U(A,PHI(J)) , J=1,2,. ,N. When MBDCND = 3, 4, or 8, (d/dTHETA)U(A,PHI(J)) , J=1,2,. When MBDCND has any other value, BDA is a dummy variable. |
| 6 | `BDB` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length N that specifies the boundary values of the solution at THETA = B. When MBDCND = 1,4, or 5, U(B,PHI(J)) , J=1,2,. ,N. When MBDCND = 2,3, or 6, (d/dTHETA)U(B,PHI(J)) , J=1,2,. When MBDCND has any other value, BDB is a dummy variable. |
| 7 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of PHI (longitude), i. e. C. LE. PHI. D. |
| 8 | `D` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of PHI (longitude), i. e. C. LE. PHI. D. |
| 9 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the interval (C,D). The unknowns in the PHI-direction are given by PHI(J) = C + (J-0. 5)DPHI, J=1,2,. ,N, where DPHI = (D-C)/N. N must be greater than 2. |
| 10 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at PHI = C and PHI = D. = 0 If the solution is periodic in PHI, i. e. U(I,J) = U(I,N+J). = 1 If the solution is specified at PHI = C and PHI = D (see note below). = 2 If the solution is specified at PHI = C and the derivative of the solution with respect to PHI is specified at = 3 If the derivative of the solution with respect to PHI is specified at PHI = C and PHI = D. |
| 11 | `BDC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one dimensional array of length M that specifies the boundary values of the solution at PHI = C. When NBDCND = 1 or 2, U(THETA(I),C) , I=1,2,. ,M. When NBDCND = 3 or 4, (d/dPHI)U(THETA(I),C), I=1,2,. When NBDCND = 0, BDC is a dummy variable. |
| 12 | `BDD` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length M that specifies the boundary values of the solution at PHI = D. When NBDCND = 1 or 4, U(THETA(I),D) , I=1,2,. ,M. When NBDCND = 2 or 3, (d/dPHI)U(THETA(I),D) , I=1,2,. When NBDCND = 0, BDD is a dummy variable. |
| 13 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the Helmholtz equation. If LAMBDA is greater than 0, a solution may not exist. However, HSTSSP will attempt to find a solution. |
| 14 | `F` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | A two-dimensional array that specifies the values of the right side of the Helmholtz equation. For I=1,2,. ,M and J=1,2,. ,N F(THETA(I),PHI(J)). must be dimensioned at least M X N. Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),PHI(J)) for I=1,2,. |
| 15 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the array F as it appears in the program calling HSTSSP. This parameter is used to specify the variable dimension of F. IDIMF must be at least M. |
| 16 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a con- stant, calculated and subtracted from F, which ensures that a solution exists. HSTSSP then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution; hence, the solution is not unique. The value of should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem. |
| 17 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for numbers 0 and 14, a solution is not attempted. = 0 No error = 1 A. LT. 0 or B. GT. |
| 18 | `W` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space. W may require up to 13M + 4N + M*INT(log2(N)) locations. The actual number of locations used is computed by HSTSSP and is returned in the location W(1). contains the required length of W. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 A .LT. 0 or B .GT. PI |
| `IERROR` | `2` | 2 A .GE. B |
| `IERROR` | `3` | 3 MBDCND .LT. 1 or MBDCND .GT. 9 |
| `IERROR` | `4` | 4 C .GE. D |
| `IERROR` | `5` | 5 N .LE. 2 |
| `IERROR` | `6` | 6 NBDCND .LT. 0 or NBDCND .GT. 4 |
| `IERROR` | `7` | 7 A .GT. 0 and MBDCND = 5, 6, or 9 |
| `IERROR` | `8` | 8 A = 0 and MBDCND = 3, 4, or 8 |
| `IERROR` | `9` | 9 B .LT. PI and MBDCND .GE. 7 |
| `IERROR` | `10` | 10 B = PI and MBDCND = 2,3, or 6 |
| `IERROR` | `11` | 11 MBDCND .GE. 5 and NDBCND = 1, 2, or 4 |
| `IERROR` | `12` | 12 IDIMF .LT. M |
| `IERROR` | `13` | 13 M .LE. 2 |
| `IERROR` | `14` | 14 LAMBDA .GT. 0 Since this is the only means of indicating a possibly incorrect call to HSTSSP, the user should test IERROR after the call. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hstssp`. Native symbol: `hstssp_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hstssp`
- Public declaration feature: `fishpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::differential_equations::pde::StaggeredSphereSurfaceHelmholtz2d::solve`
- Safe-facade link test: `focused_native_linked`
- Safe-facade runtime test: `native_manufactured_solution`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
