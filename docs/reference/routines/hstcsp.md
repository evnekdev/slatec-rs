# HSTCSP

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation in spherical coordinates assuming axisymmetry (no dependence on longitude).

## Description

HSTCSP solves the standard five-point finite difference approximation on a staggered grid to the modified Helmholtz equation spherical coordinates assuming axisymmetry (no dependence on longitude). (1/R**2)(d/dR)(R**2(dU/dR)) + 1/(R**2*SIN(THETA))(d/dTHETA)(SIN(THETA)(dU/dTHETA)) + (LAMBDA/(R*SIN(THETA))**2)U = F(THETA,R) where THETA is colatitude and R is the radial coordinate. This two-dimensional modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * * * * * * * On Input * * * * * * INTL = 0 On initial entry to HSTCSP or if any of the arguments C, D, N, or NBDCND are changed from a previous call. = 1 If C, D, N, and NBDCND are all unchanged from previous call to HSTCSP. NOTE: A call with INTL = 0 takes approximately 1.5 times as much time as a call with INTL = 1. Once a call with INTL = 0 has been made then subsequent solutions corresponding to different F, BDA, BDB, BDC, and BDD can be obtained faster with INTL = 1 since initialization is not repeated. A,B The range of THETA (colatitude), i.e. A .LE. THETA .LE. B. A must be less than B and A must be non-negative. A and B are in radians. A = 0 corresponds to the north pole and B = PI corresponds to the south pole. * * IMPORTANT * * * If B is equal to PI, then B must be computed using the statement B = PIMACH(DUM) This insures that B in the user's program is equal to PI in this

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

- Canonical provider: `fishfft/hstcsp.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hstcsp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hstcsp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [HSTCSP](https://www.netlib.org/slatec/fishfft/hstcsp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `INTL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `A` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | 0, do not use MBDCND = 1,2,3,4,7 or 8, 0  and/or B = PI the only meaningful boundary condition is dU/dTHETA = 0.  (See D. Greenspan, 'Numerical Analysis of Elliptic Boundary Value Problems,' Harper and Row, 1965, Chapter 5.) BDA dimensional array of length N that specifies the boundary values (if any) of the solution at THETA = A.  When dimensional array of length N that specifies the boundary values of the solution at THETA = B.  When MBDCND = 1, 4, or 5, dimensional array of length M that specifies the boundary values of the solution at R = D.  When NBDCND = 1 or 4, dimensional array that specifies the values of the right side of the modified Helmholtz equation.  For I=1,2,...,M and J=1,2,...,N dimensional array that must be provided by the user for work space.  With K = INT(log2(N))+1 and L = 2**(K+1), W may require up to (K-2)*L+K+MAX(2N,6M)+4(N+M)+5 locations.  The actual number of locations used is computed by HSTCSP and is returned in the location W(1). * * * * *   On Output   * * * * * * F Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),R(J)) for I=1,2,...,M, J=1,2,...,N. PERTRB If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation stant, calculated and subtracted from F, which ensures stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCSP then computes this that a solution exists.  HSTCSP then computes this solution, which is a least squares solution to the solution, which is a least squares solution to the original approximation.  This solution plus any constant is also original approximation.  This solution plus any constant is also a solution; hence, the solution is not unique.  The value of a solution; hence, the solution is not unique.  The value of PERTRB should be small compared to the right side F. PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that problem.  This comparison should always be made to insure that a meaningful solution has been obtained. a meaningful solution has been obtained. |
| 3 | `B` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | A)/M.  M must be greater than 4. MBDCND Indicates the type of boundary conditions at THETA = A and THETA = B. = 1  If the solution is specified at THETA = A and THETA = B. (See notes 1, 2 below) = 2  If the solution is specified at THETA = A and the derivative of the solution with respect to THETA is specified at THETA = B (See notes 1, 2 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = A (See notes 1, 2 below) and THETA = B. = 4  If the derivative of the solution with respect to THETA is specified at THETA = A (See notes 1, 2 below) and the solution is specified at THETA = B. = 5  If the solution is unspecified at THETA = A = 0 and the solution is specified at THETA = B. (See note 2 below) = 6  If the solution is unspecified at THETA = A = 0 and the derivative of the solution with respect to THETA is specified at THETA = B (See note 2 below). = 7  If the solution is specified at THETA = A and the solution is unspecified at THETA = B = PI. = 8  If the derivative of the solution with respect to THETA is specified at THETA = A (See note 1 below) and the solution is unspecified at THETA = B = PI. = 9  If the solution is unspecified at THETA = A = 0 and THETA = B = PI. PI, do not use MBDCND = 1,2,3,4,5 or 6, |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | A)/M.  M must be greater than 4. MBDCND Indicates the type of boundary conditions at THETA = A and THETA = B. = 1  If the solution is specified at THETA = A and THETA = B. (See notes 1, 2 below) = 2  If the solution is specified at THETA = A and the derivative of the solution with respect to THETA is specified at THETA = B (See notes 1, 2 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = A (See notes 1, 2 below) and THETA = B. = 4  If the derivative of the solution with respect to THETA is specified at THETA = A (See notes 1, 2 below) and the solution is specified at THETA = B. = 5  If the solution is unspecified at THETA = A = 0 and the solution is specified at THETA = B. (See note 2 below) = 6  If the solution is unspecified at THETA = A = 0 and the derivative of the solution with respect to THETA is specified at THETA = B (See note 2 below). = 7  If the solution is specified at THETA = A and the solution is unspecified at THETA = B = PI. = 8  If the derivative of the solution with respect to THETA is specified at THETA = A (See note 1 below) and the solution is unspecified at THETA = B = PI. = 9  If the solution is unspecified at THETA = A = 0 and THETA = B = PI. |
| 5 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 5, 6, or 9. 7, 8, or 9. 1, 2, or 7, 3, 4, or 8, 2,3, or 6, |
| 6 | `BDA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(A,R(J)) ,              J=1,2,...,N. (d/dTHETA)U(A,R(J)) ,    J=1,2,...,N. When MBDCND has any other value, BDA is a dummy variable. BDB |
| 7 | `BDB` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(B,R(J)) ,              J=1,2,...,N. (d/dTHETA)U(B,R(J)) ,    J=1,2,...,N. When MBDCND has any other value, BDB is a dummy variable. C,D The range of R , i.e. C .LE. R .LE. D. |
| 8 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | negative. negative. N N The number of unknowns in the interval (C,D).  The unknowns in The number of unknowns in the interval (C,D).  The unknowns in the R-direction are given by R(J) = C + (J-0.5)DR, the R-direction are given by R(J) = C + (J-0.5)DR, 0 and MBDCND = 3,6,8 or 9, the system of equations to be solved is singular.  The unique solution is determined by extrapolation to the specification of U(THETA(1),C).  But in these cases the right side of the system will be perturbed by the constant PERTRB. |
| 9 | `D` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | negative. N The number of unknowns in the interval (C,D).  The unknowns in the R-direction are given by R(J) = C + (J-0.5)DR, C)/N.  N must be greater than 4. NBDCND Indicates the type of boundary conditions at R = C and R = D. = 1  If the solution is specified at R = C and R = D. = 2  If the solution is specified at R = C and the derivative of the solution with respect to R is specified at R = D. (See note 1 below) = 3  If the derivative of the solution with respect to R is specified at R = C and R = D. = 4  If the derivative of the solution with respect to R is specified at R = C and the solution is specified at R = D. = 5  If the solution is unspecified at R = C = 0 (See note 2 below) and the solution is specified at R = D. = 6  If the solution is unspecified at R = C = 0 (See note 2 below) and the derivative of the solution with respect to R is specified at R = D. |
| 10 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | C)/N.  N must be greater than 4. NBDCND Indicates the type of boundary conditions at R = C and R = D. = 1  If the solution is specified at R = C and R = D. = 2  If the solution is specified at R = C and the derivative of the solution with respect to R is specified at R = D. (See note 1 below) = 3  If the derivative of the solution with respect to R is specified at R = C and R = D. = 4  If the derivative of the solution with respect to R is specified at R = C and the solution is specified at R = D. = 5  If the solution is unspecified at R = C = 0 (See note 2 below) and the solution is specified at R = D. = 6  If the solution is unspecified at R = C = 0 (See note 2 below) and the derivative of the solution with respect to R is specified at R = D. |
| 11 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 5 or 6 cannot be used with MBDCND = 1, 2, 4, 5, or 7 (the former indicates that the solution is unspecified at R = 0; the latter indicates that the 1 or 2. BDC A one dimensional array of length M that specifies the boundary values of the solution at R = C.   When NBDCND = 1 or 2, 3 or 4, 2 or 3, |
| 12 | `BDC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(THETA(I),C) ,              I=1,2,...,M. (d/dR)U(THETA(I),C),         I=1,2,...,M. When NBDCND has any other value, BDC is a dummy variable. BDD |
| 13 | `BDD` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | U(THETA(I),D) ,              I=1,2,...,M. (d/dR)U(THETA(I),D) ,        I=1,2,...,M. When NBDCND has any other value, BDD is a dummy variable. ELMBDA The constant LAMBDA in the modified Helmholtz equation.  If LAMBDA is greater than 0, a solution may not exist.  However, HSTCSP will attempt to find a solution. F |
| 14 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 15 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | F(THETA(I),R(J)) . F must be dimensioned at least M X N. IDIMF The row (or first) dimension of the array F as it appears in the program calling HSTCSP.  This parameter is used to specify the variable dimension of F.  IDIMF must be at least M. W |
| 16 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 17 | `PERTRB` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | stant, calculated and subtracted from F, which ensures that a solution exists.  HSTCSP then computes this solution, which is a least squares solution to the original approximation.  This solution plus any constant is also a solution; hence, the solution is not unique.  The value of PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.  This comparison should always be made to insure that a meaningful solution has been obtained. |
| 18 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 19 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

=  1  A .LT. 0 or B .GT. PI =  2  A .GE. B =  3  MBDCND .LT. 1 or MBDCND .GT. 9 =  4  C .LT. 0 =  5  C .GE. D =  6  NBDCND .LT. 1 or NBDCND .GT. 6 =  7  N .LT. 5 =  8  NBDCND = 5 or 6 and MBDCND = 1, 2, 4, 5, or 7 =  9  C .GT. 0 and NBDCND .GE. 5 = 10  ELMBDA .GT. 0 = 11  IDIMF .LT. M = 12  M .LT. 5 = 13  A = 0 and MBDCND =1,2,3,4,7 or 8 = 14  B = PI and MBDCND .LE. 6 = 15  A .GT. 0 and MBDCND = 5, 6, or 9 = 16  B .LT. PI and MBDCND .GE. 7 = 17  LAMBDA .NE. 0 and NBDCND .GE. 5 Since this is the only means of indicating a possibly the call. W W(1) contains the required length of W.  Also  W contains intermediate values that must not be destroyed if HSTCSP will be called again with INTL = 1. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDA(N),BDB(N),BDC(M),BDD(M),F(IDIMF,N), Arguments      W(See argument list) Latest         June 1979 Revision Subprograms    HSTCSP,HSTCS1,BLKTRI,BLKTR1,INDXA,INDXB,INDXC, Required       PROD,PRODP,CPROD,CPRODP,PPADD,PSGF,BSRH,PPSGF, PPSPF,COMPB,TEVLS,R1MACH Special        NONE Conditions Common         CBLKT Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hstcsp`. Native symbol: `hstcsp_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hstcsp`
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
