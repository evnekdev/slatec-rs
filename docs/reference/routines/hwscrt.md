# HWSCRT

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solves the standard five-point finite difference approximation to the Helmholtz equation in Cartesian coordinates.

## Description

Subroutine HWSCRT solves the standard five-point finite difference approximation to the Helmholtz equation in Cartesian coordinates: (d/dX)(dU/dX) + (d/dY)(dU/dY) + LAMBDA*U = F(X,Y). * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of X, i.e., A .LE. X .LE. B. A must be less than B. M The number of panels into which the interval (A,B) is subdivided. Hence, there will be M+1 grid points in the X-direction given by X(I) = A+(I-1)DX for I = 1,2,...,M+1, where DX = (B-A)/M is the panel width. M must be greater than 3. MBDCND Indicates the type of boundary conditions at X = A and X = B. = 0 If the solution is periodic in X, i.e., U(I,J) = U(M+I,J). = 1 If the solution is specified at X = A and X = B. = 2 If the solution is specified at X = A and the derivative of the solution with respect to X is specified at X = B. = 3 If the derivative of the solution with respect to X is specified at X = A and X = B. = 4 If the derivative of the solution with respect to X is specified at X = A and the solution is specified at X = B. BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. When MBDCND = 3 or 4, BDA(J) = (d/dX)U(A,Y(J)), J = 1,2,...,N+1 . When MBDCND has any other value, BDA is a dummy variable. BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = B. When MBDCND = 2 or 3, BDB(J) = (d/dX)U(B,Y(J)), J = 1,2,...,N+1 . When MBDCND has any other value BDB is a dummy variable. C,D The range of Y, i.e., C .LE. Y .LE. D. C must be less than D. N The number of panels into which the interval (C,D) is subdivided. Hence, there will be N+1 grid points in the Y-direction given by Y(J) = C+(J-1)DY for J = 1,2,...,N+1, where DY = (D-C)/N is the panel width. N must be greater than 3. NBDCND Indicates the type of boundary conditions at Y = C and Y = D. = 0 If the solution is periodic in Y, i.e., U(I,J) = U(I,N+J). = 1 If the solution is specified at Y = C and Y = D. = 2 If the solution is specified at Y = C and the derivative of the solution with respect to Y is specified at Y = D. = 3 If the derivative of the solution with respect to Y is specified at Y = C and Y = D. = 4 If the derivative of the solution with respect to Y is specified at Y = C and the solution is specified at Y = D. BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = C. When NBDCND = 3 or 4, BDC(I) = (d/dY)U(X(I),C), I = 1,2,...,M+1 . When NBDCND has any other value, BDC is a dummy variable. BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = D. When NBDCND = 2 or 3, BDD(I) = (d/dY)U(X(I),D), I = 1,2,...,M+1 . When NBDCND has any other value, BDD is a dummy variable. ELMBDA The constant LAMBDA in the Helmholtz equation. If LAMBDA .GT. 0, a solution may not exist. However, HWSCRT will attempt to find a solution. F A two-dimensional array which specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M and J = 2,3,...,N F(I,J) = F(X(I),Y(J)). On the boundaries F is defined by MBDCND F(1,J) F(M+1,J) ------ --------- -------- 0 F(A,Y(J)) F(A,Y(J)) 1 U(A,Y(J)) U(B,Y(J)) 2 U(A,Y(J)) F(B,Y(J)) J = 1,2,...,N+1 3 F(A,Y(J)) F(B,Y(J)) 4 F(A,Y(J)) U(B,Y(J)) NBDCND F(I,1) F(I,N+1) ------ --------- -------- 0 F(X(I),C) F(X(I),C) 1 U(X(I),C) U(X(I),D) 2 U(X(I),C) F(X(I),D) I = 1,2,...,M+1 3 F(X(I),C) F(X(I),D) 4 F(X(I),C) U(X(I),D) F must be dimensioned at least (M+1)*(N+1). NOTE: If the table calls for both the solution U and the right side F at a corner then the solution must be specified. IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSCRT. This parameter is used to specify the variable dimension of F. IDIMF must be at least M+1 . W A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations. The actual number of locations used is computed by HWSCRT and is returned in location W(1). * * * * * * On Output * * * * * * F Contains the solution U(I,J) of the finite difference approximation for the grid point (X(I),Y(J)), I = 1,2,...,M+1, J = 1,2,...,N+1 . PERTRB If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HWSCRT then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution. Hence, the solution is not unique. The value of PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. IERROR An error flag that indicates invalid input parameters. Except for numbers 0 and 6, a solution is not attempted. = 0 No error. = 1 A .GE. B. = 2 MBDCND .LT. 0 or MBDCND .GT. 4 . = 3 C .GE. D. = 4 N .LE. 3 = 5 NBDCND .LT. 0 or NBDCND .GT. 4 . = 6 LAMBDA .GT. 0 . = 7 IDIMF .LT. M+1 . = 8 M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSCRT, the user should test IERROR after the call. W W(1) contains the required length of W. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension of BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments W(see argument list) Latest June 1, 1976 Revision Subprograms HWSCRT,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required TRIX,TRI3,PIMACH Special NONE Conditions Common NONE Blocks I/O NONE Precision Single Specialist Roland Sweet Language FORTRAN History Standardized September 1, 1973 Revised April 1, 1976 Algorithm The routine defines the finite difference equations, incorporates boundary data, and adjusts the right side of singular systems and then calls GENBUN to solve the system. Space 13110(octal) = 5704(decimal) locations on the NCAR Required Control Data 7600 Timing and The execution time T on the NCAR Control Data Accuracy 7600 for subroutine HWSCRT is roughly proportional to M*N*log2(N), but also depends on the input parameters NBDCND and MBDCND. Some typical values are listed in the table below. The solution process employed results in a loss of no more than three significant digits for N and M as large as 64. More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. M(=N) MBDCND NBDCND T(MSECS) ----- ------ ------ -------- 32 0 0 31 32 1 1 23 32 3 3 36 64 0 0 128 64 1 1 96 64 3 3 142 Portability American National Standards Institute FORTRAN. The machine dependent constant PI is defined in function PIMACH. Reference Swarztrauber, P. and R. Sweet, 'Efficient FORTRAN Subprograms for The Solution Of Elliptic Equations' NCAR TN/IA-109, July, 1975, 138 pp. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::differential_equations::pde::CartesianHelmholtz2d::solve`

## Providers

- Canonical provider: `fishfft/hwscrt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hwscrt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hwscrt.f) — `verified_cached`
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
| `A` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of X, i.e., A .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of X, i.e., A .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | M The number of panels into which the interval (A,B) is subdivided. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MBDCND` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | MBDCND Indicates the type of boundary conditions at X = A and X = B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = B. | BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = B. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | C,D The range of Y, i.e., C .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Subroutine HWSCRT solves the standard five-point finite difference approximation to the Helmholtz equation in Cartesian coordinates: (d/dX)(dU/dX) + (d/dY)(dU/dY) + LAMBDA*U = F(X,Y). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBDCND` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | NBDCND Indicates the type of boundary conditions at Y = C and Y = D. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDC` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = C. | BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = C. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDD` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = D. | BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = D. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELMBDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | ELMBDA The constant LAMBDA in the Helmholtz equation. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (IDIMF, *) | Subroutine HWSCRT solves the standard five-point finite difference approximation to the Helmholtz equation in Cartesian coordinates: (d/dX)(dU/dX) + (d/dY)(dU/dY) + LAMBDA*U = F(X,Y). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDIMF` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSCRT. | IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSCRT. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PERTRB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | PERTRB If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IERROR An error flag that indicates invalid input parameters. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | W A one-dimensional array that must be provided by the user for work space. | W A one-dimensional array that must be provided by the user for work space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::pde::fishpack::hwscrt`. Native symbol: `hwscrt_`. Feature: `raw-family-fishpack-cartesian-2d`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::pde::fishpack::hwscrt`
- Compatibility aliases: `slatec_sys::fishpack_cartesian_2d::hwscrt`
- Public declaration feature: `raw-family-fishpack-cartesian-2d`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::differential_equations::pde::CartesianHelmholtz2d::solve`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
