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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [HWSCSP](https://www.netlib.org/slatec/fishfft/hwscsp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `INTL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = 0 On initial entry to HWSCSP or if any of the arguments 0 has been made then subsequent solutions corresponding to different F, BDTS, BDTF, BDRS, BDRF can be obtained faster with INTL = 1 since initialization is not repeated. |
| 2 | `TS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i. e. , TS. LE. THETA. TF. |
| 3 | `TF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The range of THETA (colatitude), i. e. , TS. LE. THETA. TF. |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The number of panels into which the interval (TS,TF) is subdivided. Hence, there will be M+1 grid points in the THETA-direction given by THETA(K) = (I-1)DTHETA+TS for I = 1,2,. ,M+1, where DTHETA = (TF-TS)/M is the panel width. |
| 5 | `MBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Indicates the type of boundary condition at THETA = TS and THETA = TF. = 1 If the solution is specified at THETA = TS and THETA = TF. = 2 If the solution is specified at THETA = TS and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 3 If the derivative of the solution with respect to THETA is specified at THETA = TS and THETA = TF (see notes 1,2 = 4 If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is specified at THETA = TF. = 5 If the solution is unspecified at THETA = TS = 0 and the = 6 If the solution is unspecified at THETA = TS = 0 and the = 7 If the solution is specified at THETA = TS and the solution is unspecified at THETA = TF = PI. = 8 If the derivative of the solution with respect to THETA is = 9 If the solution is unspecified at THETA = TS = 0 and NOTES: 1. |
| 6 | `BDTS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TS. When MBDCND = 3,4, or 8, (d/dTHETA)U(TS,R(J)), J = 1,2,. ,N+1. When MBDCND has any other value, BDTS is a dummy variable. |
| 7 | `BDTF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TF. When MBDCND = 2,3, or 6, (d/dTHETA)U(TF,R(J)), J = 1,2,. ,N+1. When MBDCND has any other value, BDTF is a dummy variable. |
| 8 | `RS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | changed from a previous call. = 1 If RS, RF, N, NBDCND are all unchanged from previous call to HWSCSP. NOTE A call with INTL=0 takes approximately 1. 5 times as much time as a call with INTL = 1. Once a call with The range of R, i. e. |
| 9 | `RF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | changed from a previous call. = 1 If RS, RF, N, NBDCND are all unchanged from previous call to HWSCSP. NOTE A call with INTL=0 takes approximately 1. 5 times as much time as a call with INTL = 1. Once a call with The range of R, i. e. |
| 10 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | changed from a previous call. = 1 If RS, RF, N, NBDCND are all unchanged from previous call to HWSCSP. NOTE A call with INTL=0 takes approximately 1. 5 times as much time as a call with INTL = 1. Once a call with The number of panels into which the interval (RS,RF) is subdivided. Hence, there will be N+1 grid points in the R-direction given by R(J) = (J-1)DR+RS for J = 1,2,. |
| 11 | `NBDCND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | changed from a previous call. = 1 If RS, RF, N, NBDCND are all unchanged from previous call to HWSCSP. NOTE A call with INTL=0 takes approximately 1. 5 times as much time as a call with INTL = 1. Once a call with Indicates the type of boundary condition at R = RS and R = RF. = 1 If the solution is specified at R = RS and R = RF. |
| 12 | `BDRS` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RS. When NBDCND = 3 or 4, (d/dR)U(THETA(I),RS), I = 1,2,. ,M+1. When NBDCND has any other value, BDRS is a dummy variable. |
| 13 | `BDRF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to R at R = RF. When NBDCND = 2,3, or 6, (d/dR)U(THETA(I),RF), I = 1,2,. ,M+1. When NBDCND has any other value, BDRF is a dummy variable. |
| 14 | `ELMBDA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | The constant LAMBDA in the Helmholtz equation. If LAMBDA. GT. 0, a solution may not exist. However, HWSCSP will attempt to find a solution. If NBDCND = 5 or 6 or. |
| 15 | `F` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (IDIMF, *) | A two-dimensional array that specifies the value of the right side of the Helmholtz equation and boundary values (if any). for I = 2,3,. ,M and J = 2,3,. ,N F(THETA(I),R(J)). On the boundaries F is defined by must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified. |
| 16 | `IDIMF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The row (or first) dimension of the array F as it appears in the program calling HWSCSP. This parameter is used to specify the variable dimension of F. IDIMF must be at least M+1. |
| 17 | `PERTRB` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HWSCSP then computes this solution, which is a least squares solution to the original approximation. This solution is not unique and is unnormalized. The value of PERTRB should be small compared to the right side F. Otherwise , a solution is obtained to an essentially different problem. |
| 18 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag that indicates invalid input parameters. Except for numbers 0 and 10, a solution is not attempted. = 1 TS. LT. 0. or TF. |
| 19 | `W` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A one-dimensional array that must be provided by the user for work space. Its length can be computed from the formula below which depends on the value of NBDCND. If NBDCND=2,4 or 6 define NUNK=N If NBDCND=1 or 5 define NUNK=N-1 If NBDCND=3 define NUNK=N+1 Now set K=INT(log2(NUNK))+1 and L=2**(K+1) then W must be dimensioned at least (K-2)*L+K+5*(M+N)+MAX(2*N,6*M)+23 IMPORTANT** For purposes of checking, the required length of W is computed by HWSCSP and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if HWSCSP will be called again with INTL = 1. W(1) contains the number of locations which W must have. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `1` | 1 TS.LT.0. or TF.GT.PI |
| `IERROR` | `2` | 2 TS.GE.TF |
| `IERROR` | `3` | 3 M.LT.5 |
| `IERROR` | `4` | 4 MBDCND.LT.1 or MBDCND.GT.9 |
| `IERROR` | `5` | 5 RS.LT.0 |
| `IERROR` | `6` | 6 RS.GE.RF |
| `IERROR` | `7` | 7 N.LT.5 |
| `IERROR` | `8` | 8 NBDCND.LT.1 or NBDCND.GT.6 |
| `IERROR` | `9` | 9 ELMBDA.GT.0 |
| `IERROR` | `10` | 10 IDIMF.LT.M+1 |
| `IERROR` | `11` | 11 ELMBDA.NE.0 and MBDCND.GE.5 |
| `IERROR` | `12` | 12 ELMBDA.NE.0 and NBDCND equals 5 or 6 |
| `IERROR` | `13` | 13 MBDCND equals 5,6 or 9 and TS.NE.0 |
| `IERROR` | `14` | 14 MBDCND.GE.7 and TF.NE.PI |
| `IERROR` | `15` | 15 TS.EQ.0 and MBDCND equals 3,4 or 8 |
| `IERROR` | `16` | 16 TF.EQ.PI and MBDCND equals 2,3 or 6 |
| `IERROR` | `17` | 17 NBDCND.GE.5 and RS.NE.0 |
| `IERROR` | `18` | 18 NBDCND.GE.5 and MBDCND equals 1,2,4,5 or 7 Since this is the only means of indicating a possibly incorrect call to HWSCSP, the user should test IERROR after a call. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

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
