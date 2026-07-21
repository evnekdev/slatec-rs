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

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [HW3CRT](https://www.netlib.org/slatec/fishfft/hw3crt.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `XS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of X, i.e. XS .LE. X .LE. XF . must be less than XF. 1,2,...,M+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) |
| 2 | `XF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of X, i.e. XS .LE. X .LE. XF . XS)/L is the panel width.  L must be at least 5 . 1,2,...,M+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) |
| 3 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (XS,XF) is subdivided.  Hence, there will be L+1 grid points in the X-direction given by X(I) = XS+(I-1)DX for I=1,2,...,L+1, U(I,J,K). = 1  If the solution is specified at X = XS and X = XF. = 2  If the solution is specified at X = XS and the derivative of the solution with respect to X is specified at X = XF. = 3  If the derivative of the solution with respect to X is specified at X = XS and X = XF. = 4  If the derivative of the solution with respect to X is specified at X = XS and the solution is specified at X=XF. J=1,2,...,M+1, and K=1,2,...,N+1. M=N)     LBDCND(=MBDCND=NBDCND)      T(MSECS) |
| 4 | `LBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at X = XS and X = XF. = 0  If the solution is periodic in X, i.e. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(1,J,K)         F(L+1,J,K) |
| 5 | `BDXS` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XS. (d/dX)U(XS,Y(J),Z(K)), J=1,2,...,M+1, is a dummy variable. must be dimensioned at least (M+1)*(N+1). |
| 6 | `BDXF` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to X at X = XF. (d/dX)U(XF,Y(J),Z(K)), J=1,2,...,M+1, is a dummy variable. must be dimensioned at least (M+1)*(N+1). |
| 7 | `YS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Y, i.e. YS .LE. Y .LE. YF. must be less than YF. 1,2,...,L+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) |
| 8 | `YF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Y, i.e. YS .LE. Y .LE. YF. YS)/M is the panel width.  M must be at least 5 . 1,2,...,L+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) |
| 9 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (YS,YF) is subdivided.  Hence, there will be M+1 grid points in the Y-direction given by Y(J) = YS+(J-1)DY for J=1,2,...,M+1, U(I,J,K). = 1  If the solution is specified at Y = YS and Y = YF. = 2  If the solution is specified at Y = YS and the derivative of the solution with respect to Y is specified at Y = YF. = 3  If the derivative of the solution with respect to Y is specified at Y = YS and Y = YF. = 4  If the derivative of the solution with respect to Y is specified at Y = YS and the solution is specified at Y=YF. |
| 10 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at Y = YS and Y = YF. = 0  If the solution is periodic in Y, i.e. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(I,1,K)         F(I,M+1,K) |
| 11 | `BDYS` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YS. (d/dY)U(X(I),YS,Z(K)), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(N+1). |
| 12 | `BDYF` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Y at Y = YF. (d/dY)U(X(I),YF,Z(K)), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(N+1). |
| 13 | `ZS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Z, i.e. ZS .LE. Z .LE. ZF. must be less than ZF. 1,2,...,L+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) |
| 14 | `ZF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of Z, i.e. ZS .LE. Z .LE. ZF. ZS)/N is the panel width.  N must be at least 5. 1,2,...,L+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) |
| 15 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (ZS,ZF) is subdivided.  Hence, there will be N+1 grid points in the Z-direction given by Z(K) = ZS+(K-1)DZ for K=1,2,...,N+1, U(I,J,K). = 1  If the solution is specified at Z = ZS and Z = ZF. = 2  If the solution is specified at Z = ZS and the derivative of the solution with respect to Z is specified at Z = ZF. = 3  If the derivative of the solution with respect to Z is specified at Z = ZS and Z = ZF. = 4  If the derivative of the solution with respect to Z is specified at Z = ZS and the solution is specified at Z=ZF. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |
| 16 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at Z = ZS and Z = ZF. = 0  If the solution is periodic in Z, i.e. 3 or 4, is a dummy variable. 2 or 3, is a dummy variable. F(I,J,1)         F(I,J,N+1) |
| 17 | `BDZS` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZS. (d/dZ)U(X(I),Y(J),ZS), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(M+1). |
| 18 | `BDZF` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDIMF, *) | A two-dimensional array that specifies the values of the derivative of the solution with respect to Z at Z = ZF. (d/dZ)U(X(I),Y(J),ZF), I=1,2,...,L+1, is a dummy variable. must be dimensioned at least (L+1)*(M+1). |
| 19 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the Helmholtz equation. If LAMBDA .GT. 0, a solution may not exist.  However, HW3CRT will attempt to find a solution. |
| 20 | `LDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the arrays F,BDYS,BDYF,BDZS, and BDZF as it appears in the program calling HW3CRT. this parameter is used to specify the variable dimension of these arrays.  LDIMF must be at least L+1. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |
| 21 | `MDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The column (or second) dimension of the array F and the row (or first) dimension of the arrays BDXS and BDXF as it appears in the program calling HW3CRT.  This parameter is used to specify the variable dimension of these arrays. must be at least M+1. argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |
| 22 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 3; dimensions (LDIMF, MDIMF, *) | A three-dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any).  For I=2,3,...,L, J=2,3,...,M, and K=2,3,...,N F(X(I),Y(J),Z(K)). On the boundaries F is defined by 1,2,...,M+1 1,2,...,N+1 1,2,...,N+1 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) 4      F(XS,Y(J),Z(K))   U(XF,Y(J),Z(K)) 1,2,...,L+1 1,2,...,N+1 1,2,...,N+1 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) 4      F(X(I),YS,Z(K))   U(X(I),YF,Z(K)) 1,2,...,L+1 1,2,...,M+1 1,2,...,M+1 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) 4      F(X(I),Y(J),ZS)   U(X(I),Y(J),ZF) must be dimensioned at least (L+1)*(M+1)*(N+1). NOTE: If the table calls for both the solution U and the right side F on a boundary, then the solution must be specified. Contains the solution U(I,J,K) of the finite difference approximation for the grid point (X(I),Y(J),Z(K)) for argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |
| 23 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist.  PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists.  PWSCRT then computes this solution, which is a least squares solution to the original approximation.  This solution is not unique and is unnormalized.  The value of PERTRB should be small compared to the right side F.  Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. |
| 24 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters.  Except for numbers 0 and 12, a solution is not attempted. =  0  No error =  1  XS .GE. XF =  2  L .LT. 5 =  3  LBDCND .LT. 0 .OR. LBDCND .GT. 4 =  4  YS .GE. YF =  5  M .LT. 5 =  6  MBDCND .LT. 0 .OR. MBDCND .GT. 4 =  7  ZS .GE. ZF =  8  N .LT. 5 =  9  NBDCND .LT. 0 .OR. NBDCND .GT. 4 = 10  LDIMF .LT. L+1 = 11  MDIMF .LT. M+1 = 12  LAMBDA .GT. 0 Since this is the only means of indicating a possibly incorrect call to HW3CRT, the user should test IERROR after the call. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDXS(MDIMF,N+1),BDXF(MDIMF,N+1),BDYS(LDIMF,N+1), Arguments      BDYF(LDIMF,N+1),BDZS(LDIMF,M+1),BDZF(LDIMF,M+1), |
| 25 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 5*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)) argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: A one-dimensional array that must be provided by the user for work space.  The length of W must be at least 30 + L + M + 5*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)) argument list) Latest         December 1, 1978 Revision Subprograms    HW3CRT,POIS3D,POS3D1,TRIDQ,RFFTI,RFFTF,RFFTF1, Required       RFFTB,RFFTB1,COSTI,COST,SINTI,SINT,COSQI,COSQF, COSQF1,COSQB,COSQB1,SINQI,SINQF,SINQB,CFFTI, CFFTI1,CFFTB,CFFTB1,PASSB2,PASSB3,PASSB4,PASSB, CFFTF,CFFTF1,PASSF1,PASSF2,PASSF3,PASSF4,PASSF, PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
