# HWSPLR

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a finite difference approximation to the Helmholtz equation in polar coordinates.

## Description

Subroutine HWSPLR solves a finite difference approximation to the Helmholtz equation in polar coordinates: (1/R)(d/dR)(R(dU/dR)) + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA). * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of R, i.e., A .LE. R .LE. B. A must be less than B and A must be non-negative. M The number of panels into which the interval (A,B) is subdivided. Hence, there will be M+1 grid points in the R-direction given by R(I) = A+(I-1)DR, for I = 1,2,...,M+1, where DR = (B-A)/M is the panel width. M must be greater than 3. MBDCND Indicates the type of boundary condition at R = A and R = B. = 1 If the solution is specified at R = A and R = B. = 2 If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. = 3 If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4 If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is specified at R = B. = 5 If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6 If the solution is unspecified at R = A = 0 and the derivative of the solution with respect to R is specified at R = B. NOTE: If A = 0, do not use MBDCND = 3 or 4, but instead use MBDCND = 1,2,5, or 6 . BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. When MBDCND = 3 or 4, BDA(J) = (d/dR)U(A,THETA(J)), J = 1,2,...,N+1 . When MBDCND has any other value, BDA is a dummy variable. BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. When MBDCND = 2,3, or 6, BDB(J) = (d/dR)U(B,THETA(J)), J = 1,2,...,N+1 . When MBDCND has any other value, BDB is a dummy variable. C,D The range of THETA, i.e., C .LE. THETA .LE. D. C must be less than D. N The number of panels into which the interval (C,D) is subdivided. Hence, there will be N+1 grid points in the THETA-direction given by THETA(J) = C+(J-1)DTHETA for J = 1,2,...,N+1, where DTHETA = (D-C)/N is the panel width. N must be greater than 3. NBDCND Indicates the type of boundary conditions at THETA = C and at THETA = D. = 0 If the solution is periodic in THETA, i.e., U(I,J) = U(I,N+J). = 1 If the solution is specified at THETA = C and THETA = D (see note below). = 2 If the solution is specified at THETA = C and the derivative of the solution with respect to THETA is specified at THETA = D (see note below). = 4 If the derivative of the solution with respect to THETA is specified at THETA = C and the solution is specified at THETA = D (see note below). NOTE: When NBDCND = 1,2, or 4, do not use MBDCND = 5 or 6 (the former indicates that the solution is specified at R = 0, the latter indicates the solution is unspecified at R = 0). Use instead MBDCND = 1 or 2 . BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = C. When NBDCND = 3 or 4, BDC(I) = (d/dTHETA)U(R(I),C), I = 1,2,...,M+1 . When NBDCND has any other value, BDC is a dummy variable. BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = D. When NBDCND = 2 or 3, BDD(I) = (d/dTHETA)U(R(I),D), I = 1,2,...,M+1 . When NBDCND has any other value, BDD is a dummy variable. ELMBDA The constant LAMBDA in the Helmholtz equation. If LAMBDA .LT. 0, a solution may not exist. However, HWSPLR will attempt to find a solution. F A two-dimensional array that specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,...,M and J = 2,3,...,N F(I,J) = F(R(I),THETA(J)). On the boundaries F is defined by MBDCND F(1,J) F(M+1,J) ------ ------------- ------------- 1 U(A,THETA(J)) U(B,THETA(J)) 2 U(A,THETA(J)) F(B,THETA(J)) 3 F(A,THETA(J)) F(B,THETA(J)) 4 F(A,THETA(J)) U(B,THETA(J)) J = 1,2,...,N+1 5 F(0,0) U(B,THETA(J)) 6 F(0,0) F(B,THETA(J)) NBDCND F(I,1) F(I,N+1) ------ --------- --------- 0 F(R(I),C) F(R(I),C) 1 U(R(I),C) U(R(I),D) 2 U(R(I),C) F(R(I),D) I = 1,2,...,M+1 3 F(R(I),C) F(R(I),D) 4 F(R(I),C) U(R(I),D) F must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSPLR. This parameter is used to specify the variable dimension of F. IDIMF must be at least M+1 . W A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations. The actual number of locations used is computed by HWSPLR and is returned in location W(1). * * * * * * On Output * * * * * * F Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)), I = 1,2,...,M+1, J = 1,2,...,N+1 . PERTRB If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HWSPLR then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution. Hence, the solution is not unique. PERTRB should be small compared to the right side. Otherwise, a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. IERROR An error flag that indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. = 0 No error. = 1 A .LT. 0 . = 2 A .GE. B. = 3 MBDCND .LT. 1 or MBDCND .GT. 6 . = 4 C .GE. D. = 5 N .LE. 3 = 6 NBDCND .LT. 0 or .GT. 4 . = 7 A = 0, MBDCND = 3 or 4 . = 8 A .GT. 0, MBDCND .GE. 5 . = 9 MBDCND .GE. 5, NBDCND .NE. 0 and NBDCND .NE. 3 . = 10 IDIMF .LT. M+1 . = 11 LAMBDA .GT. 0 . = 12 M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSPLR, the user should test IERROR after the call. W W(1) contains the required length of W. *Long Description: * * * * * * * Program Specifications * * * * * * * * * * * * Dimension of BDA(N+1),BDB(N+1),BDC(M+1),BDD(M+1),F(IDIMF,N+1), Arguments W(see argument list) Latest June 1, 1976 Revision Subprograms HWSPLR,GENBUN,POISD2,POISN2,POISP2,COSGEN,MERGE, Required TRIX,TRI3,PIMACH Special None Conditions Common NONE Blocks I/O Precision Single Specialist Roland Sweet Language FORTRAN History Standardized April 1, 1973 Revised January 1, 1976 Algorithm The routine defines the finite difference equations, incorporates boundary data, and adjusts the right side of singular systems and then calls GENBUN to solve the system. Space 13430(octal) = 5912(decimal) locations on the NCAR Required Control Data 7600 Timing and The execution time T on the NCAR Control Data Accuracy 7600 for subroutine HWSPLR is roughly proportional to M*N*log2(N), but also depends on the input parameters NBDCND and MBDCND. Some typical values are listed in the table below. The solution process employed results in a loss of no more than three significant digits for N and M as large as 64. More detailed information about accuracy can be found in the documentation for subroutine GENBUN which is the routine that solves the finite difference equations. M(=N) MBDCND NBDCND T(MSECS) ----- ------ ------ -------- 32 1 0 31 32 1 1 23 32 3 3 36 64 1 0 128 64 1 1 96 64 3 3 142 Portability American National Standards Institute FORTRAN. The machine dependent constant PI is defined in function PIMACH. Required COS Resident Routines Reference Swarztrauber, P. and R. Sweet, 'Efficient FORTRAN Subprograms For The Solution Of Elliptic Equations' NCAR TN/IA-109, July, 1975, 138 pp. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

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

- Canonical provider: `fishfft/hwsplr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hwsplr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hwsplr.f) — `verified_cached`
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
| `A` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Subroutine HWSPLR solves a finite difference approximation to the Helmholtz equation in polar coordinates: (1/R)(d/dR)(R(dU/dR)) + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | * * * * * * * * Parameter Description * * * * * * * * * * * * * * * * On Input * * * * * * A,B The range of R, i.e., A .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | M The number of panels into which the interval (A,B) is subdivided. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MBDCND` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | MBDCND Indicates the type of boundary condition at R = A and R = B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. | BDB A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | C,D The range of THETA, i.e., C .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Subroutine HWSPLR solves a finite difference approximation to the Helmholtz equation in polar coordinates: (1/R)(d/dR)(R(dU/dR)) + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. | BDA A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBDCND` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | NBDCND Indicates the type of boundary conditions at THETA = C and at THETA = D. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDC` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = C. | BDC A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = C. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BDD` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = D. | BDD A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = D. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELMBDA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | ELMBDA The constant LAMBDA in the Helmholtz equation. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (IDIMF, *) | Subroutine HWSPLR solves a finite difference approximation to the Helmholtz equation in polar coordinates: (1/R)(d/dR)(R(dU/dR)) + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDIMF` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSPLR. | IDIMF The row (or first) dimension of the array F as it appears in the program calling HWSPLR. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PERTRB` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | PERTRB If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERROR` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IERROR An error flag that indicates invalid input parameters. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | W A one-dimensional array that must be provided by the user for work space. | W A one-dimensional array that must be provided by the user for work space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::pde::fishpack::hwsplr`. Native symbol: `hwsplr_`. Feature: `fishpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hwsplr`
- Compatibility aliases: `slatec_sys::pde::fishpack::numerical::hwsplr`
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
