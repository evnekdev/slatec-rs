# DFC

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. Equality and inequality constraints can be imposed on the fitted curve.

## Description

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. Equality and inequality constraints can be imposed on the fitted curve. For a description of the B-splines and usage instructions to evaluate them, see C. W. de Boor, Package for Calculating with B-Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further documentation and discussion of constrained curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978). Input.. All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). No sorting of XDATA(*) is required. Any non-negative value of NDATA is allowed. A negative value of NDATA is an error. A zero value for any entry of SDDATA(*) will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry. NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,..., NORD-1 and I=NBKPT-NORD+2,...,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required. Internal to DFC( ) the extreme end knots may be reduced and increased respectively to accommodate any data values that are exterior to the given knot values. The contents of BKPT(*) is not changed. NORD must be in the range 1 .LE. NORD .LE. 20. The value of NBKPT must satisfy the condition NBKPT .GE. 2*NORD. Other values are considered errors. (The order of the spline is one more than the degree of the piecewise polynomial defined on each interval. This is consistent with the B-spline package convention. For example, NORD=4 when we are using piecewise cubics.) NCONST,XCONST(*), YCONST(*),NDERIV(*) The number of conditions that constrain the B-spline is NCONST. A constraint is specified by an (X,Y) pair in the arrays XCONST(*) and YCONST(*), and by the type of constraint and derivative value encoded in the array NDERIV(*). No sorting of XCONST(*) is required. The value of NDERIV(*) is determined as follows. Suppose the I-th constraint applies to the J-th derivative of the B-spline. (Any non-negative value of J < NORD is permitted. In particular the value J=0 refers to the B-spline itself.) For this I-th constraint, set XCONST(I)=X, YCONST(I)=Y, and NDERIV(I)=ITYPE+4*J, where ITYPE = 0, if (J-th deriv. at X) .LE. Y. = 1, if (J-th deriv. at X) .GE. Y. = 2, if (J-th deriv. at X) .EQ. Y. = 3, if (J-th deriv. at X) .EQ. (J-th deriv. at Y). (A value of NDERIV(I)=-1 will cause this constraint to be ignored. This subprogram feature is often useful when temporarily suppressing a constraint while still retaining the source code of the calling program.) MODE An input flag that directs the least squares solution method used by DFC( ). The variance function, referred to below, defines the square of the probable error of the fitted curve at any point, XVAL. This feature of DFC( ) allows one to use the square root of this variance function to determine a probable error band around the fitted curve. =1 a new problem. No variance function. =2 a new problem. Want variance function. =3 an old problem. No variance function. =4 an old problem. Want variance function. Any value of MODE other than 1-4 is an error. The user with a new problem can skip directly to the description of the input parameters IW(1), IW(2). If the user correctly specifies the new or old problem status, the subprogram DFC( ) will perform more efficiently. By an old problem it is meant that subprogram DFC( ) was last called with this same set of knots, data points and weights. Another often useful deployment of this old problem designation can occur when one has previously obtained a Q-R orthogonal decomposition of the matrix resulting from B-spline fitting of data (without constraints) at the breakpoints BKPT(I), I=1,...,NBKPT. For example, this matrix could be the result of sequential accumulation of the least squares equations for a very large data set. The user writes this code in a manner convenient for the application. For the discussion here let N=NBKPT-NORD, and K=N+3 Let us assume that an equivalent least squares system RC=D has been obtained. Here R is an N+1 by N matrix and D is a vector with N+1 components. The last row of R is zero. The matrix R is upper triangular and banded. At most NORD of the diagonals are nonzero. The contents of R and D can be copied to the working array W(*) as follows. The I-th diagonal of R, which has N-I+1 elements, is copied to W(*) starting at W((I-1)*K+1), for I=1,...,NORD. The vector D is copied to W(*) starting at W(NORD*K+1) The input value used for NDATA is arbitrary when an old problem is designated. Because of the feature of DFC( ) that checks the working storage array lengths, a value not exceeding NBKPT should be used. For example, use NDATA=0. (The constraints or variance function request can change in each call to DFC( ).) A new problem is anything other than an old problem. IW(1),IW(2) The amounts of working storage actually allocated for the working arrays W(*) and IW(*). These quantities are compared with the actual amounts of storage needed in DFC( ). Insufficient storage allocated for either W(*) or IW(*) is an error. This feature was included in DFC( ) because misreading the storage formulas for W(*) and IW(*) might very well lead to subtle and hard-to-find programming bugs. The length of W(*) must be at least NB=(NBKPT-NORD+3)*(NORD+1)+ 2*MAX(NDATA,NBKPT)+NBKPT+NORD**2 Whenever possible the code uses banded matrix processors DBNDAC( ) and DBNDSL( ). These are utilized if there are no constraints, no variance function is required, and there is sufficient data to uniquely determine the B-spline coefficients. If the band processors cannot be used to determine the solution, then the constrained least squares code DLSEI is used. In this case the subprogram requires an additional block of storage in W(*). For the discussion here define the integers NEQCON and NINCON respectively as the number of equality (ITYPE=2,3) and inequality (ITYPE=0,1) constraints imposed on the fitted curve. Define L=NBKPT-NORD+1 and note that NCONST=NEQCON+NINCON. When the subprogram DFC( ) uses DLSEI( ) the length of the working array W(*) must be at least LW=NB+(L+NCONST)*L+ 2*(NEQCON+L)+(NINCON+L)+(NINCON+2)*(L+6) The length of the array IW(*) must be at least IW1=NINCON+2*L in any case. Output.. All TYPE REAL variables are DOUBLE PRECISION MODE An output flag that indicates the status of the constrained curve fit. =-1 a usage error of DFC( ) occurred. The offending condition is noted with the SLATEC library error processor, XERMSG. In case the working arrays W(*) or IW(*) are not long enough, the minimal acceptable length is printed. = 0 successful constrained curve fit. = 1 the requested equality constraints are contradictory. = 2 the requested inequality constraints are contradictory. = 3 both equality and inequality constraints are contradictory. COEFF(*) If the output value of MODE=0 or 1, this array contains the unknowns obtained from the least squares fitting process. These N=NBKPT-NORD parameters are the B-spline coefficients. For MODE=1, the equality constraints are contradictory. To make the fitting process more robust, the equality constraints are satisfied in a least squares sense. In this case the array COEFF(*) contains B-spline coefficients for this extended concept of a solution. If MODE=-1,2 or 3 on output, the array COEFF(*) is undefined. Working Arrays.. All Type REAL variables are DOUBLE PRECISION W(*),IW(*) These arrays are respectively typed DOUBLE PRECISION and INTEGER. Their required lengths are specified as input parameters in IW(1), IW(2) noted above. The contents of W(*) must not be modified by the user if the variance function is desired. Evaluating the Variance Function.. To evaluate the variance function (assuming that the uncertainties of the Y values were provided to DFC( ) and an input value of MODE=2 or 4 was used), use the function subprogram DCV( ) VAR=DCV(XVAL,NDATA,NCONST,NORD,NBKPT, BKPT,W) Here XVAL is the point where the variance is desired. The other arguments have the same meaning as in the usage of DFC( ). For those users employing the old problem designation, let MDATA be the number of data points in the problem. (This may be different from NDATA if the old problem designation feature was used.) The value, VAR, should be multiplied by the quantity DBLE(MAX(NDATA-N,1))/DBLE(MAX(MDATA-N,1)) The output of this subprogram is not defined if an input value of MODE=1 or 3 was used in FC( ) or if an output value of MODE=-1, 2, or 3 was obtained. The variance function, except for the scaling factor noted above, is given by VAR=(transpose of B(XVAL))*C*B(XVAL) The vector B(XVAL) is the B-spline basis function values at X=XVAL. The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints. This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. Evaluating the Fitted Curve.. To evaluate derivative number IDER at XVAL, use the function subprogram DBVALU( ) F = DBVALU(BKPT,COEFF,NBKPT-NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be defined unless an output value of MODE=0 or 1 was obtained from DFC( ), XVAL is in the data interval, and IDER is nonnegative and .LT. NORD. The first time DBVALU( ) is called, INBV=1 must be specified. This value of INBV is the overwritten by DBVALU( ). The array WORKB(*) must be of length at least 3*NORD, and must not be the same as the W(*) array used in the call to DFC( ). DBVALU( ) expects the breakpoint array BKPT(*) to be sorted.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1A1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dfc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dfc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dfc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dfc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Approximation](../families/approximation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `NDATA` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XDATA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YDATA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SDDATA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NORD` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBKPT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BKPT` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NCONST` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | For example, NORD=4 when we are using piecewise cubics.) NCONST,XCONST(*), YCONST(*),NDERIV(*) The number of conditions that constrain the B-spline is NCONST. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XCONST` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | For example, NORD=4 when we are using piecewise cubics.) NCONST,XCONST(*), YCONST(*),NDERIV(*) The number of conditions that constrain the B-spline is NCONST. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YCONST` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | For example, NORD=4 when we are using piecewise cubics.) NCONST,XCONST(*), YCONST(*),NDERIV(*) The number of conditions that constrain the B-spline is NCONST. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NDERIV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | For example, NORD=4 when we are using piecewise cubics.) NCONST,XCONST(*), YCONST(*),NDERIV(*) The number of conditions that constrain the B-spline is NCONST. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This subprogram feature is often useful when temporarily suppressing a constraint while still retaining the source code of the calling program.) MODE An input flag that directs the least squares solution method used by DFC( ). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `COEFF` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | COEFF(*) If the output value of MODE=0 or 1, this array contains the unknowns obtained from the least squares fitting process. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IW` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user with a new problem can skip directly to the description of the input parameters IW(1), IW(2). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::approximation::dfc`. Native symbol: `dfc_`. Feature: `approximation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::dfc`
- Compatibility aliases: `slatec_sys::approximation::numerical::dfc`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
