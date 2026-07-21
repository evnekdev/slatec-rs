# FC

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. Equality and inequality constraints can be imposed on the fitted curve.

## Description

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. Equality and inequality constraints can be imposed on the fitted curve. For a description of the B-splines and usage instructions to evaluate them, see C. W. de Boor, Package for Calculating with B-Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further documentation and discussion of constrained curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/fc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/fc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/fc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/fc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [FC](https://www.netlib.org/slatec/src/fc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NDATA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0. (The constraints or variance function request can change in each call to FC( ).)  A new problem is anything other than an old problem. IW(1),IW(2) The amounts of working storage actually allocated for the working arrays W(*) and IW(*).  These quantities are compared with the actual amounts of storage needed in FC( ). Insufficient storage allocated for either |
| 2 | `XDATA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `YDATA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `SDDATA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `NORD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `NBKPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | NORD, and K=N+3 Let us assume that an equivalent least squares system RC=D has been obtained.  Here R is an N+1 by N matrix and D is a vector with N+1 components. The last row of R is zero.  The matrix R is upper triangular and banded.  At most NORD of the diagonals are nonzero. The contents of R and D can be copied to the working array W(*) as follows. The I-th diagonal of R, which has N-I+1 elements, is copied to W(*) starting at |
| 7 | `BKPT` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1,...,NBKPT. For example, this matrix could be the result of sequential accumulation of the least squares equations for a very large data set. The user writes this code in a manner convenient for the application.  For the discussion here let |
| 8 | `NCONST` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `XCONST` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `YCONST` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 11 | `NDERIV` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 12 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `COEFF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 14 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1)*K+1), for I=1,...,NORD. The vector D is copied to W(*) starting at W(NORD*K+1) The input value used for NDATA is arbitrary when an old problem is designated.  Because of the feature of FC( ) that checks the working storage array lengths, a value not exceeding NBKPT should be used.  For example, |
| 15 | `IW` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

SDDATA(*) will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry. NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*).  Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,..., NORD-1 and I=NBKPT-NORD+2,...,NBKPT, are required to compute the functions used to fit the data.  No sorting of BKPT(*) is required. Internal to  FC( ) the extreme end knots may be reduced and increased respectively to accommodate any data values that are exterior to the given knot values.  The contents of BKPT(*) is not changed. NORD must be in the range 1 .LE. NORD .LE. 20. The value of NBKPT must satisfy the condition NBKPT .GE. 2*NORD. (The order of the spline is one more than the degree of the piecewise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For example, NORD=4 when we are using piecewise cubics.) NCONST,XCONST(*), YCONST(*),NDERIV(*) The number of conditions that constrain the B-spline is NCONST.  A constraint is specified by an (X,Y) pair in the arrays XCONST(*) and YCONST(*), and by the type of constraint and derivative value encoded in the array NDERIV(*).  No sorting of XCONST(*) is required.  The value of NDERIV(*) is determined as follows.  Suppose the I-th constraint applies to the J-th derivative of the B-spline.  (Any non-negative value of J < NORD is permitted.  In particular the value J=0 refers to the B-spline itself.) For this I-th constraint, set XCONST(I)=X, YCONST(I)=Y, and NDERIV(I)=ITYPE+4*J, where ITYPE = 0,      if (J-th deriv. at X) .LE. Y. = 1,      if (J-th deriv. at X) .GE. Y. = 2,      if (J-th deriv. at X) .EQ. Y. = 3,      if (J-th deriv. at X) .EQ. (J-th deriv. at Y). (A value of NDERIV(I)=-1 will cause this constraint to be ignored.  This subprogram feature is often useful when temporarily suppressing a constraint while still retaining the source code of the calling program.) MODE An input flag that directs the least squares solution method used by FC( ). The variance function, referred to below, the fitted curve at any point, XVAL. This feature of  FC( ) allows one to use the square root of this variance function to fitted curve. =1  a new problem.  No variance function. =2  a new problem.  Want variance function. =3  an old problem.  No variance function. =4  an old problem.  Want variance function. The user with a new problem can skip directly included in FC( ) because misreading the storage formulas for W(*) and IW(*) might very well lead to subtle and hard-to-find programming bugs. The length of W(*) must be at least NB=(NBKPT-NORD+3)*(NORD+1)+ 2*MAX(NDATA,NBKPT)+NBKPT+NORD**2 Whenever possible the code uses banded matrix processors BNDACC( ) and BNDSOL( ).  These are utilized if there are no constraints, no variance function is required, and there is sufficient data to uniquely determine the B-spline coefficients.  If the band processors cannot be used to determine the solution, then the constrained least squares code LSEI is used.  In this case the subprogram requires an additional block of storage in W(*).  For the discussion here define the integers NEQCON and NINCON respectively as the number of equality (ITYPE=2,3) and inequality (ITYPE=0,1) constraints imposed on the fitted curve.  Define L=NBKPT-NORD+1 and note that NCONST=NEQCON+NINCON. When the subprogram FC( ) uses LSEI( ) the length of the working array W(*) must be at least LW=NB+(L+NCONST)*L+ 2*(NEQCON+L)+(NINCON+L)+(NINCON+2)*(L+6) The length of the array IW(*) must be at least IW1=NINCON+2*L in any case. offending condition is noted with the In case the working arrays W(*) or IW(*) are not long enough, the minimal acceptable length is printed. = 0  successful constrained curve fit. = 1  the requested equality constraints are contradictory. = 2  the requested inequality constraints are contradictory. = 3  both equality and inequality constraints are contradictory. COEFF(*) If the output value of MODE=0 or 1, this array contains the unknowns obtained from the least squares fitting process.  These N=NBKPT-NORD parameters are the B-spline coefficients. For MODE=1, the equality constraints are contradictory.  To make the fitting process more robust, the equality constraints are satisfied in a least squares sense.  In this case the array COEFF(*) contains B-spline coefficients for this extended concept of a solution.  If MODE=-1,2 or 3 on output, the array COEFF(*) is undefined. Working Arrays.. W(*),IW(*) These arrays are respectively typed REAL and INTEGER. Their required lengths are specified as input parameters in IW(1), IW(2) noted above.  The contents of W(*) must not be modified by the user if the variance function is desired. Evaluating the Variance Function.. To evaluate the variance function (assuming that the uncertainties of the Y values were provided to  FC( ) and an input value of MODE=2 or 4 was used), use the function subprogram  CV( ) VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT, BKPT,W) Here XVAL is the point where the variance is desired.  The other arguments have the same meaning as in the usage of FC( ). For those users employing the old problem designation, let MDATA be the number of data points in the problem.  (This may be different from NDATA if the old problem designation feature was used.)  The value, VAR, should be multiplied by the quantity REAL(MAX(NDATA-N,1))/MAX(MDATA-N,1) The output of this subprogram is not defined if an input value of MODE=1 or 3 was used in FC( ) or if an output value of MODE=-1, 2, or 3 was obtained.  The variance function, except for the scaling factor noted above, is given by VAR=(transpose of B(XVAL))*C*B(XVAL) The vector B(XVAL) is the B-spline basis function values at X=XVAL. The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints.  This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. Evaluating the Fitted Curve.. To evaluate derivative number IDER at XVAL, use the function subprogram BVALU( ). F = BVALU(BKPT,COEFF,NBKPT-NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be defined unless an output value of MODE=0 or 1 was obtained from  FC( ), XVAL is in the data interval, and IDER is nonnegative and .LT. NORD. The first time BVALU( ) is called, INBV=1 must be specified.  This value of INBV is the overwritten by BVALU( ).  The array WORKB(*) must be of length at least 3*NORD, and must not be the same as the W(*) array used in the call to FC( ). BVALU( ) expects the breakpoint array BKPT(*) to be sorted.

### Storage and workspace requirements

`W`: 1)*K+1), for I=1,...,NORD. The vector D is copied to W(*) starting at W(NORD*K+1) The input value used for NDATA is arbitrary when an old problem is designated.  Because of the feature of FC( ) that checks the working storage array lengths, a value not exceeding NBKPT should be used.  For example,

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::fc`. Native symbol: `fc_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::fc`
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
