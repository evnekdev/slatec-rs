# HW3CRT

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the standard seven-point finite difference approximation to the Helmholtz equation in Cartesian coordinates.

## Description

Subroutine HW3CRT solves the standard seven-point finite difference approximation to the Helmholtz equation in Cartesian coordinates: (d/dX)(dU/dX) + (d/dY)(dU/dY) + (d/dZ)(dU/dZ) + LAMBDA*U = F(X,Y,Z) . * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * *

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

- Canonical provider: `fishfft/hw3crt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hw3crt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hw3crt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [HW3CRT](https://www.netlib.org/slatec/fishfft/hw3crt.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `XS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of X, i. e. XS. LE. X. XF. |
| 2 | `XF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of X, i. e. XS. LE. X. XF. |
| 3 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (XS,XF) is subdivided. Hence, there will be L+1 grid points in the X-direction given by X(I) = XS+(I-1)DX for I=1,2,. ,L+1, where DX = (XF-XS)/L is the panel width. L must be at least 5. M=N) LBDCND(=MBDCND=NBDCND) T(MSECS) 16 0 300 16 1 302 16 3 348 32 0 1925 32 1 1929 32 3 2109 Portability American National Standards Institute FORTRAN. The machine dependent constant PI is defined in function PIMACH. |
| 4 | `LBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at X = XS and X = XF. = 0 If the solution is periodic in X, i. e. U(L+I,J,K) = U(I,J,K). = 1 If the solution is specified at X = XS and X = XF. = 2 If the solution is specified at X = XS and the derivative of the solution with respect to X is specified at X = XF. |
| 5 | `BDXS` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XS. when LBDCND = 3 or 4, (d/dX)U(XS,Y(J),Z(K)), J=1,2,. ,M+1, K=1,2,. ,N+1. When LBDCND has any other value, BDXS is a dummy variable. must be dimensioned at least (M+1)*(N+1). |
| 6 | `BDXF` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XF. When LBDCND = 2 or 3, (d/dX)U(XF,Y(J),Z(K)), J=1,2,. ,M+1, K=1,2,. ,N+1. When LBDCND has any other value, BDXF is a dummy variable. must be dimensioned at least (M+1)*(N+1). |
| 7 | `YS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Y, i. e. YS. LE. Y. YF. |
| 8 | `YF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Y, i. e. YS. LE. Y. YF. |
| 9 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (YS,YF) is subdivided. Hence, there will be M+1 grid points in the Y-direction given by Y(J) = YS+(J-1)DY for J=1,2,. ,M+1, where DY = (YF-YS)/M is the panel width. M must be at least 5. |
| 10 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at Y = YS and Y = YF. = 0 If the solution is periodic in Y, i. e. U(I,M+J,K) = U(I,J,K). = 1 If the solution is specified at Y = YS and Y = YF. = 2 If the solution is specified at Y = YS and the derivative of the solution with respect to Y is specified at Y = YF. |
| 11 | `BDYS` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YS. When MBDCND = 3 or 4, (d/dY)U(X(I),YS,Z(K)), I=1,2,. ,L+1, K=1,2,. ,N+1. When MBDCND has any other value, BDYS is a dummy variable. must be dimensioned at least (L+1)*(N+1). |
| 12 | `BDYF` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YF. When MBDCND = 2 or 3, (d/dY)U(X(I),YF,Z(K)), I=1,2,. ,L+1, K=1,2,. ,N+1. When MBDCND has any other value, BDYF is a dummy variable. must be dimensioned at least (L+1)*(N+1). |
| 13 | `ZS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Z, i. e. ZS. LE. Z. ZF. |
| 14 | `ZF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Z, i. e. ZS. LE. Z. ZF. |
| 15 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (ZS,ZF) is subdivided. Hence, there will be N+1 grid points in the Z-direction given by Z(K) = ZS+(K-1)DZ for K=1,2,. ,N+1, where DZ = (ZF-ZS)/N is the panel width. N must be at least 5. |
| 16 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at Z = ZS and Z = ZF. = 0 If the solution is periodic in Z, i. e. U(I,J,N+K) = U(I,J,K). = 1 If the solution is specified at Z = ZS and Z = ZF. = 2 If the solution is specified at Z = ZS and the derivative of the solution with respect to Z is specified at Z = ZF. |
| 17 | `BDZS` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZS. When NBDCND = 3 or 4, (d/dZ)U(X(I),Y(J),ZS), I=1,2,. ,L+1, J=1,2,. ,M+1. When NBDCND has any other value, BDZS is a dummy variable. must be dimensioned at least (L+1)*(M+1). |
| 18 | `BDZF` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZF. When NBDCND = 2 or 3, (d/dZ)U(X(I),Y(J),ZF), I=1,2,. ,L+1, J=1,2,. ,M+1. When NBDCND has any other value, BDZF is a dummy variable. must be dimensioned at least (L+1)*(M+1). |
| 19 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the Helmholtz equation. If LAMBDA. GT. 0, a solution may not exist. However, HW3CRT will attempt to find a solution. |
| 20 | `LDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the arrays F,BDYS,BDYF,BDZS, and BDZF as it appears in the program calling HW3CRT. this parameter is used to specify the variable dimension of these arrays. LDIMF must be at least L+1. |
| 21 | `MDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The column (or second) dimension of the array F and the row (or first) dimension of the arrays BDXS and BDXF as it appears in the program calling HW3CRT. This parameter is used to specify the variable dimension of these arrays. must be at least M+1. |
| 22 | `F` | `input-output` | `array` | `REAL` | `*mut f32` | rank 3; dimensions (LDIMF, MDIMF, *) | A three-dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I=2,3,. ,L, J=2,3,. ,M, and K=2,3,. ,N F(X(I),Y(J),Z(K)). On the boundaries F is defined by must be dimensioned at least (L+1)*(M+1)*(N+1). |
| 23 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. PWSCRT then computes this solution, which is a least squares solution to the original approximation. This solution is not unique and is unnormalized. The value of PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem. |
| 24 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for numbers 0 and 12, a solution is not attempted. = 0 No error = 1 XS. GE. XF = 2 L. LT. |
| 25 | `W` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space. The length of W must be at least 30 + L + M + 5*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 XS .GE. XF |
| `IERROR` | `2` | 2 L .LT. 5 |
| `IERROR` | `3` | 3 LBDCND .LT. 0 .OR. LBDCND .GT. 4 |
| `IERROR` | `4` | 4 YS .GE. YF |
| `IERROR` | `5` | 5 M .LT. 5 |
| `IERROR` | `6` | 6 MBDCND .LT. 0 .OR. MBDCND .GT. 4 |
| `IERROR` | `7` | 7 ZS .GE. ZF |
| `IERROR` | `8` | 8 N .LT. 5 |
| `IERROR` | `9` | 9 NBDCND .LT. 0 .OR. NBDCND .GT. 4 |
| `IERROR` | `10` | 10 LDIMF .LT. L+1 |
| `IERROR` | `11` | 11 MDIMF .LT. M+1 |
| `IERROR` | `12` | 12 LAMBDA .GT. 0 Since this is the only means of indicating a possibly incorrect call to HW3CRT, the user should test IERROR after the call. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hw3crt`. Native symbol: `hw3crt_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank3,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hw3crt`
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
