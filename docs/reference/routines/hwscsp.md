# HWSCSP

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a finite difference approximation to the modified Helmholtz equation in spherical coordinates assuming axisymmetry (no dependence on longitude).

## Description

Subroutine HWSCSP solves a finite difference approximation to the modified Helmholtz equation in spherical coordinates assuming axisymmetry (no dependence on longitude) (1/R**2)(d/dR)((R**2)(d/dR)U) + (1/(R**2)SIN(THETA))(d/dTHETA)(SIN(THETA)(d/dTHETA)U) + (LAMBDA/(RSIN(THETA))**2)U = F(THETA,R). This two dimensional modified Helmholtz equation results from the Fourier transform of the three dimensional Poisson equation

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

- Canonical provider: `fishfft/hwscsp.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/hwscsp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/hwscsp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [HWSCSP](https://www.netlib.org/slatec/fishfft/hwscsp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `INTL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = 0  On initial entry to HWSCSP or if any of the arguments 0 takes approximately 1.5 times as 1.  Once a call with 0 has been made then subsequent solutions corresponding to different F, BDTS, BDTF, BDRS, BDRF can 1 since initialization is not repeated. 1.  W(1) contains the number of locations which W must have. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   BDTS(N+1),BDTF(N+1),BDRS(M+1),BDRF(M+1), Arguments      F(IDIMF,N+1),W(see argument list) Latest         June 1979 Revision Subprograms    HWSCSP,HWSCS1,BLKTRI,BLKTR1,PROD,PRODP,CPROD,CPRODP Required       ,COMBP,PPADD,PSGF,BSRH,PPSGF,PPSPF,TEVLS,INDXA, ,INDXB,INDXC,R1MACH Special Conditions Common         CBLKT Blocks I/O            NONE Precision      Single Specialist     Paul N Swarztrauber Language       FORTRAN |
| 2 | `TS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds zero corresponds to the north pole and a TF of PI corresponds to the south pole. to the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement |
| 3 | `TF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i.e., TS .LE. THETA .LE. TF. are in radians.  A TS of are in radians.  A TS of zero corresponds to the north pole and a TF of PI corresponds zero corresponds to the north pole and a TF of PI corresponds to the south pole. to the south pole. * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * * * * * * * * * * * * * * IMPORTANT * * * * * * * * * * * * * * If TF is equal to PI then it must be computed using the statement If TF is equal to PI then it must be computed using the statement PIMACH(DUM). This insures that TF in the users program is equal to PI in this program which permits several tests of the TS)/M is the panel width. PI, do not use MBDCND = 2,3, or 6, but 1,2,...,N+1 6      F(0,R(J))         F(TF,R(J)) 7      U(TS,R(J))        F(PI,R(J)) 8      F(TS,R(J))        F(PI,R(J)) 9      F(0,R(J))         F(PI,R(J)) |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (TS,TF) is subdivided.  Hence, there will be M+1 grid points in the THETA-direction given by THETA(K) = (I-1)DTHETA+TS for TS)/M is the panel width. |
| 5 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary condition at THETA = TS and THETA = TF. = 1  If the solution is specified at THETA = TS and THETA = TF. = 2  If the solution is specified at THETA = TS and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 3  If the derivative of the solution with respect to THETA is specified at THETA = TS and THETA = TF (see notes 1,2 below). = 4  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is specified at THETA = TF. = 5  If the solution is unspecified at THETA = TS = 0 and the solution is specified at THETA = TF. = 6  If the solution is unspecified at THETA = TS = 0 and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 7  If the solution is specified at THETA = TS and the solution is unspecified at THETA = TF = PI. = 8  If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is unspecified at THETA = TF = PI. = 9  If the solution is unspecified at THETA = TS = 0 and THETA = TF = PI. NOTES:  1.  If TS = 0, do not use MBDCND = 3,4, or 8, but 5,6, or 9  . 7,8, or 9  . is a dummy variable. is a dummy variable. 1,2,4,5, or 7 (the former indicates that the solution is unspecified at R = 0, the latter indicates that the solution is specified). Use instead 5,6,7,8, or 9, ELMBDA must be zero. F(1,J)            F(M+1,J) |
| 6 | `BDTS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TS.  When MBDCND = 3,4, or 8, (d/dTHETA)U(TS,R(J)), J = 1,2,...,N+1  . is a dummy variable. |
| 7 | `BDTF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TF.  When MBDCND = 2,3, or 6, (d/dTHETA)U(TF,R(J)), J = 1,2,...,N+1  . is a dummy variable. |
| 8 | `RS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | are changed from a previous call. are all unchanged from previous call to HWSCSP. must be less than must be less than negative. RF. = 5  If the solution is unspecified at R = RS = 0 (see note below) and the solution is specified at R = RF. = 6  If the solution is unspecified at R = RS = 0 (see note below) and the derivative of the solution with respect to R is specified at R = RF. NOTE:  NBDCND = 5 or 6 cannot be used with 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) |
| 9 | `RF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | are changed from a previous call. are all unchanged from previous call to HWSCSP. must be less than negative. RS)/N is the panel width. 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) |
| 10 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are changed from a previous call. are all unchanged from previous call to HWSCSP. The number of panels into which the interval (RS,RF) is subdivided.  Hence, there will be N+1 grid points in the R-direction given by R(J) = (J-1)DR+RS for J = 1,2,...,N+1, must be greater than 2 1 |
| 11 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are changed from a previous call. are all unchanged from previous call to HWSCSP. Indicates the type of boundary condition at R = RS and R = RF. = 1  If the solution is specified at R = RS and R = RF. = 2  If the solution is specified at R = RS and the derivative of the solution with respect to R is specified at R = RF. = 3  If the derivative of the solution with respect to R is specified at R = RS and R = RF. = 4  If the derivative of the solution with respect to R is 1 or 2  . 3 or 4, is a dummy variable. 2,3, or 6, is a dummy variable. 5 or 6 or F(I,1)            F(I,N+1) 2,4 or 6 define NUNK=N 1 3        define NUNK=N+1 Now set K=INT(log2(NUNK))+1 and L=2**(K+1) then W must be dimensioned at least (K-2)*L+K+5*(M+N)+MAX(2*N,6*M)+23 IMPORTANT** For purposes of checking, the required length of W is computed by HWSCSP and stored in W(1) in floating point format. |
| 12 | `BDRS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RS. (d/dR)U(THETA(I),RS), I = 1,2,...,M+1  . is a dummy variable. |
| 13 | `BDRF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RF. (d/dR)U(THETA(I),RF), I = 1,2,...,M+1  . is a dummy variable. |
| 14 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the Helmholtz equation.  If LAMBDA .GT. 0, a solution may not exist.  However, HWSCSP will |
| 15 | `F` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | A two-dimensional array that specifies the value of the right side of the Helmholtz equation and boundary values (if any). for I = 2,3,...,M and J = 2,3,...,N F(THETA(I),R(J)). On the boundaries F is defined by 1,2,...,N+1 6      F(0,R(J))         F(TF,R(J)) 7      U(TS,R(J))        F(PI,R(J)) 8      F(TS,R(J))        F(PI,R(J)) 9      F(0,R(J))         F(PI,R(J)) 1,2,...,M+1 5      F(TS,0)           U(THETA(I),RF) 6      F(TS,0)           F(THETA(I),RF) must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. must be at least M+1  . Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),R(J)), I = 1,2,...,M+1,   J = 1,2,...,N+1  . which ensures that a solution exists.  HWSCSP then computes this solution, which is a least squares solution to the original approximation. This solution is not unique and is unnormalized. The value of PERTRB should be small compared to the right side Otherwise , a solution is obtained to an essentially different problem. This comparison should always be made to insure that a meaningful solution has been obtained. |
| 16 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the array F as it appears in the program calling HWSCSP.  This parameter is used to specify the must be at least M+1  . |
| 17 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist.  PERTRB is a constant, calculated and subtracted from |
| 18 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters.  Except for numbers 0 and 10, a solution is not attempted. = 1  TS.LT.0. or TF.GT.PI = 2  TS.GE.TF = 3  M.LT.5 = 4  MBDCND.LT.1 or MBDCND.GT.9 = 5  RS.LT.0 = 6  RS.GE.RF = 7  N.LT.5 = 8  NBDCND.LT.1 or NBDCND.GT.6 = 9  ELMBDA.GT.0 = 10 IDIMF.LT.M+1 = 11 ELMBDA.NE.0 and MBDCND.GE.5 = 12 ELMBDA.NE.0 and NBDCND equals 5 or 6 = 13 MBDCND equals 5,6 or 9 and TS.NE.0 = 14 MBDCND.GE.7 and TF.NE.PI = 15 TS.EQ.0 and MBDCND equals 3,4 or 8 = 16 TF.EQ.PI and MBDCND equals 2,3 or 6 = 17 NBDCND.GE.5 and RS.NE.0 = 18 NBDCND.GE.5 and MBDCND equals 1,2,4,5 or 7 Since this is the only means of indicating a possibly incorrect call to HWSCSP, the user should test IERROR after a call. |
| 19 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space. Its length can be computed from the formula below which depends on the value of NBDCND. Contains intermediate values that must not be destroyed if |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: A one-dimensional array that must be provided by the user for work space. Its length can be computed from the formula below which depends on the value of NBDCND. Contains intermediate values that must not be destroyed if

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::pde::fishpack::hwscsp`. Native symbol: `hwscsp_`. Declaration feature: `fishpack`. Provider feature: `fishpack-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::pde::fishpack::hwscsp`
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
