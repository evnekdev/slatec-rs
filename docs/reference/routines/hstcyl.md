# HSTCYL

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates.

## Description

HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of R, i.e. A .LE. R .LE. B. A must be less than B and A must be non-negative. M The number of grid points in the interval (A,B). The grid points in the R-direction are given by R(I) = A + (I-0.5)DR for I=1,2,...,M where DR =(B-A)/M. M must be greater than 2. MBDCND Indicates the type of boundary conditions at R = A and R = B. = 1 If the solution is specified at R = A (see note below) and R = B. = 2 If the solution is specified at R = A (see note below) and the derivative of the solution with respect to R is specified at R = B. = 3 If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4 If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5 If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6 If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE: If A = 0, do not use MBDCND = 1,2,3, or 4, but instead use MBDCND = 5 or 6. The resulting approximation gives the only meaningful boundary condition, i.e. dU/dR = 0. (see D. Greenspan, 'Introductory Numerical Analysis Of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) BDA A one-dimensional array of length N that specifies the boundary values (if any) of the solution at R = A. When MBDCND = 1 or 2, BDA(J) = U(A,Z(J)) , J=1,2,...,N. When MBDCND = 3 or 4, BDA(J) = (d/dR)U(A,Z(J)) , J=1,2,...,N. When MBDCND = 5 or 6, BDA is a dummy variable. BDB A one-dimensional array of length N that specifies the boundary values of the solution at R = B. When MBDCND = 1,4, or 5, BDB(J) = U(B,Z(J)) , J=1,2,...,N. When MBDCND = 2,3, or 6, BDB(J) = (d/dR)U(B,Z(J)) , J=1,2,...,N. C,D The range of Z, i.e. C .LE. Z .LE. D. C must be less than D. N The number of unknowns in the interval (C,D). The unknowns in the Z-direction are given by Z(J) = C + (J-0.5)DZ, J=1,2,...,N, where DZ = (D-C)/N. N must be greater than 2. NBDCND Indicates the type of boundary conditions at Z = C and Z = D. = 0 If the solution is periodic in Z, i.e. U(I,J) = U(I,N+J). = 1 If the solution is specified at Z = C and Z = D. = 2 If the solution is specified at Z = C and the derivative of the solution with respect to Z is specified at Z = D. = 3 If the derivative of the solution with respect to Z is specified at Z = C and Z = D. = 4 If the derivative of the solution with respect to Z is specified at Z = C and the solution is specified at Z = D. BDC A one dimensional array of length M that specifies the boundary values of the solution at Z = C. When NBDCND = 1 or 2, BDC(I) = U(R(I),C) , I=1,2,...,M. When NBDCND = 3 or 4, BDC(I) = (d/dZ)U(R(I),C), I=1,2,...,M. When NBDCND = 0, BDC is a dummy variable. BDD A one-dimensional array of length M that specifies the boundary values of the solution at Z = D. when NBDCND = 1 or 4, BDD(I) = U(R(I),D) , I=1,2,...,M. When NBDCND = 2 or 3, BDD(I) = (d/dZ)U(R(I),D) , I=1,2,...,M. When NBDCND = 0, BDD is a dummy variable. ELMBDA The constant LAMBDA in the modified Helmholtz equation. If LAMBDA is greater than 0, a solution may not exist. However, HSTCYL will attempt to find a solution. LAMBDA must be zero when MBDCND = 5 or 6. F A two-dimensional array that specifies the values of the right side of the modified Helmholtz equation. For I=1,2,...,M and J=1,2,...,N F(I,J) = F(R(I),Z(J)) . F must be dimensioned at least M X N. IDIMF The row (or first) dimension of the array F as it appears in the program calling HSTCYL. This parameter is used to specify the variable dimension of F. IDIMF must be at least M. W A one-dimensional array that must be provided by the user for work space. W may require up to 13M + 4N + M*INT(log2(N)) locations. The actual number of locations used is computed by HSTCYL and is returned in the location W(1). * * * * * * On Output * * * * * * F Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),Z(J)) for I=1,2,...,M, J=1,2,...,N. PERTRB If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HSTCYL then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution; hence, the solution is not unique. The value of PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. IERROR An error flag that indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. = 0 No error = 1 A .LT. 0 = 2 A .GE. B = 3 MBDCND .LT. 1 or MBDCND .GT. 6 = 4 C .GE. D = 5 N .LE. 2 = 6 NBDCND .LT. 0 or NBDCND .GT. 4 = 7 A = 0 and MBDCND = 1,2,3, or 4 = 8 A .GT. 0 and MBDCND .GE. 5 = 9 M .LE. 2 = 10 IDIMF .LT. M = 11 LAMBDA .GT. 0 = 12 A=0, MBDCND .GE. 5, ELMBDA .NE. 0 Since this is the only means of indicating a possibly incorrect call to HSTCYL, the user should test IERROR after the call. W W(1) contains the required length of W. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension OF BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments W(see argument list) Latest June 1, 1977 Revision Subprograms HSTCYL,POISTG,POSTG2,GENBUN,POISD2,POISN2,POISP2, Required COSGEN,MERGE,TRIX,TRI3,PIMACH Special NONE Conditions Common NONE Blocks I/O NONE Precision Single Specialist Roland Sweet Language FORTRAN History Written by Roland Sweet at NCAR in March, 1977 Algorithm This subroutine defines the finite-difference equations, incorporates boundary data, adjusts the right side when the system is singular and calls either POISTG or GENBUN which solves the linear system of equations. Space 8228(decimal) = 20044(octal) locations on the Required NCAR Control Data 7600 Timing and The execution time T on the NCAR Control Data Accuracy 7600 for subroutine HSTCYL is roughly proportional to M*N*log2(N). Some typical values are listed in the table below. The solution process employed results in a loss of no more than four significant digits for N and M as large as 64. More detailed information about accuracy can be found in the documentation for subroutine POISTG which is the routine that actually solves the finite difference equations. M(=N) MBDCND NBDCND T(MSECS) ----- ------ ------ -------- 32 1-6 1-4 56 64 1-6 1-4 230 Portability American National Standards Institute Fortran. The machine dependent constant PI is defined in function PIMACH. Required COS Resident Routines Reference Schumann, U. and R. Sweet,'A Direct Method For The Solution of Poisson's Equation With Neumann Boundary Conditions On A Staggered Grid Of Arbitrary Size,' J. Comp. Phys. 20(1976), pp. 171-182. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

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

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of R, i.e. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | M The number of grid points in the interval (A,B). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MBDCND` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | MBDCND Indicates the type of boundary conditions at R = A and R = B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Greenspan, 'Introductory Numerical Analysis Of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) BDA A one-dimensional array of length N that specifies the boundary values (if any) of the solution at R = A. | Greenspan, 'Introductory Numerical Analysis Of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) BDA A one-dimensional array of length N that specifies the boundary values (if any) of the solution at R = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDB A one-dimensional array of length N that specifies the boundary values of the solution at R = B. | BDB A one-dimensional array of length N that specifies the boundary values of the solution at R = B. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Greenspan, 'Introductory Numerical Analysis Of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) BDA A one-dimensional array of length N that specifies the boundary values (if any) of the solution at R = A. | Greenspan, 'Introductory Numerical Analysis Of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) BDA A one-dimensional array of length N that specifies the boundary values (if any) of the solution at R = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBDCND` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | NBDCND Indicates the type of boundary conditions at Z = C and Z = D. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDC` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDC A one dimensional array of length M that specifies the boundary values of the solution at Z = C. | BDC A one dimensional array of length M that specifies the boundary values of the solution at Z = C. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDD` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDD A one-dimensional array of length M that specifies the boundary values of the solution at Z = D. | BDD A one-dimensional array of length M that specifies the boundary values of the solution at Z = D. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELMBDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | ELMBDA The constant LAMBDA in the modified Helmholtz equation. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (IDIMF, *) | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. | HSTCYL solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in cylindrical coordinates (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ)C + LAMBDA*(1/R**2)*U = F(R,Z) This two-dimensional modified Helmholtz equation results from the Fourier transform of a three-dimensional Poisson equation. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDIMF` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IDIMF The row (or first) dimension of the array F as it appears in the program calling HSTCYL. | IDIMF The row (or first) dimension of the array F as it appears in the program calling HSTCYL. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PERTRB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | PERTRB If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IERROR An error flag that indicates invalid input parameters. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | W A one-dimensional array that must be provided by the user for work space. | W A one-dimensional array that must be provided by the user for work space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::pde::fishpack::hstcyl`. Native symbol: `hstcyl_`. Feature: `fishpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hstcyl`
- Compatibility aliases: `slatec_sys::pde::fishpack::numerical::hstcyl`
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
