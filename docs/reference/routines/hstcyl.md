# HSTCYL

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates.

## Description

HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates

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

- Canonical provider: `fishfft/hstcyl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hstcyl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hstcyl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [HSTCYL](https://www.netlib.org/slatec/fishfft/hstcyl.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | dimensional Poisson equation. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *    Parameter Description     * * * * * * * * * * must be less than B and must be less than B and negative. dimensional array of length N that specifies the boundary values (if any) of the solution at R = A.  When MBDCND = 1 or 2, dimensional array of length N that specifies the boundary values of the solution at R = B.  When MBDCND = 1,4, or 5, specifies the boundary values of the solution at Z = C.   When NBDCND = 1 or 2, dimensional array of length M that specifies the boundary values of the solution at Z = D.  when NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the modified Helmholtz equation.  For I=1,2,...,M and J=1,2,...,N dimensional array that must be provided by the user for work space.  W may require up to 13M + 4N + M*INT(log2(N)) locations.  The actual number of locations used is computed by HSTCYL and is returned in the location W(1). stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCYL then computes this that a solution exists.  HSTCYL then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also solution; hence, the solution is not unique.  The value of meaningful solution has been obtained. |
| 2 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | must be less than B and A)/M.  M must be greater than 2. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of grid points in the interval (A,B).  The grid points in the R-direction are given by R(I) = A + (I-0.5)DR for A)/M.  M must be greater than 2. specifies the boundary values of the solution at Z = C.   When NBDCND = 1 or 2, J=1,2,...,N. are listed in the table below. The solution process employed results in a loss of no more than four significant digits for N and M as large as 64.  More detailed information about accuracy can be found in the documentation for subroutine POISTG which is the routine that actually solves the finite difference equations. N)    MBDCND    NBDCND    T(MSECS) |
| 4 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at R = A and R = B. = 1  If the solution is specified at R = A (see note below) and R = B. = 2  If the solution is specified at R = A (see note below) and the derivative of the solution with respect to R is specified at R = B. = 3  If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4  If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5  If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6  If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE:  If A = 0, do not use MBDCND = 1,2,3, or 4, but instead 5 or 6.  The resulting approximation gives the only meaningful boundary condition, i.e. dU/dR = 0. (see D. Greenspan, 'Introductory Numerical Analysis Of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) 3 or 4, 5 or 6, BDA is a dummy variable. 2,3, or 6, 5 or 6. |
| 5 | `BDA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(A,Z(J)) ,          J=1,2,...,N. (d/dR)U(A,Z(J)) ,    J=1,2,...,N. |
| 6 | `BDB` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(B,Z(J)) ,          J=1,2,...,N. (d/dR)U(B,Z(J)) ,    J=1,2,...,N. |
| 7 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | must be less must be less than D. than D. |
| 8 | `D` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results must be less than D. C)/N.  N must be greater than 2. |
| 9 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of unknowns in the interval (C,D).  The unknowns in the Z-direction are given by Z(J) = C + (J-0.5)DZ, C)/N.  N must be greater than 2. are listed in are listed in the table below. the table below. The solution process employed results in a loss The solution process employed results in a loss of no more than four significant digits for N and M of no more than four significant digits for N and M as large as 64.  More detailed information about as large as 64.  More detailed information about accuracy can be found in the documentation for accuracy can be found in the documentation for subroutine POISTG which is the routine that subroutine POISTG which is the routine that actually solves the finite difference equations. actually solves the finite difference equations. |
| 10 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary conditions at Z = C and Z = D. = 0  If the solution is periodic in Z, i.e. U(I,J) = U(I,N+J). = 1  If the solution is specified at Z = C and Z = D. = 2  If the solution is specified at Z = C and the derivative of the solution with respect to Z is specified at Z = D. = 3  If the derivative of the solution with respect to Z is specified at Z = C and Z = D. = 4  If the derivative of the solution with respect to Z is specified at Z = C and the solution is specified at Z = D. 3 or 4, 0, BDC is a dummy variable. 2 or 3, 0, BDD is a dummy variable. |
| 11 | `BDC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(R(I),C) ,              I=1,2,...,M. (d/dZ)U(R(I),C),         I=1,2,...,M. |
| 12 | `BDD` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(R(I),D) ,              I=1,2,...,M. (d/dZ)U(R(I),D) ,        I=1,2,...,M. |
| 13 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the modified Helmholtz equation.  If LAMBDA is greater than 0, a solution may not exist.  However, HSTCYL will attempt to find a solution.  LAMBDA must be zero |
| 14 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | F(R(I),Z(J)) . must be dimensioned at least M X N. must be at least M. Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),Z(J)) for |
| 15 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the array F as it appears in the program calling HSTCYL.  This parameter is used to specify the must be at least M. |
| 16 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCYL then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that |
| 17 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. =  0  No error =  1  A .LT. 0 =  2  A .GE. B =  3  MBDCND .LT. 1 or MBDCND .GT. 6 =  4  C .GE. D =  5  N .LE. 2 =  6  NBDCND .LT. 0 or NBDCND .GT. 4 =  7  A = 0 and MBDCND = 1,2,3, or 4 =  8  A .GT. 0 and MBDCND .GE. 5 =  9  M .LE. 2 = 10  IDIMF .LT. M = 11  LAMBDA .GT. 0 = 12  A=0, MBDCND .GE. 5, ELMBDA .NE. 0 Since this is the only means of indicating a possibly incorrect call to HSTCYL, the user should test IERROR after the call. |
| 18 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension OF   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    HSTCYL,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension OF   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    HSTCYL,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required       COSGEN,MERGE,TRIX,TRI3,PIMACH Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hstcyl`. Native symbol: `hstcyl_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hstcyl`
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
