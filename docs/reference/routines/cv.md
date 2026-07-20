# CV

[Family: Probability and statistics](../families/probability-and-statistics.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram FC.

## Description

CV( ) is a companion function subprogram for FC( ). The documentation for FC( ) has complete usage instructions. CV( ) is used to evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram, FC( ). The variance function defines the square of the probable error of the fitted curve at any point, XVAL. One can use the square root of this variance function to determine a probable error band around the fitted curve. CV( ) is used after a call to FC( ). MODE, an input variable to FC( ), is used to indicate if the variance function is desired. In order to use CV( ), MODE must equal 2 or 4 on input to FC( ). MODE is also used as an output flag from FC( ). Check to make sure that MODE = 0 after calling FC( ), indicating a successful constrained curve fit. The array SDDATA, as input to FC( ), must also be defined with the standard deviation or uncertainty of the Y values to use CV( ). To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. The vector B(XVAL) is the B-spline basis function values at X=XVAL. The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints. This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. All the variables in the calling sequence for CV( ) are used in FC( ) except the variable XVAL. Do not change the values of these variables between the call to FC( ) and the use of CV( ). The following is a brief description of the variables XVAL The point where the variance is desired. NDATA The number of discrete (X,Y) pairs for which FC( ) calculated a piece-wise polynomial curve. NCONST The number of conditions that constrained the B-spline in FC( ). NORD The order of the B-spline used in FC( ). The value of NORD must satisfy 1 < NORD < 20 . (The order of the spline is one more than the degree of the piece-wise polynomial defined on each interval. This is consistent with the B-spline package convention. For example, NORD=4 when we are using piece-wise cubics.) NBKPT The number of knots in the array BKPT(*). The value of NBKPT must satisfy NBKPT .GE. 2*NORD. BKPT(*) The real array of knots. Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,...,NORD-1 and I=NBKPT-NORD+2,...,NBKPT, are required by FC( ) to compute the functions used to fit the data. W(*) Real work array as used in FC( ). See FC( ) for the required length of W(*). The contents of W(*) must not be modified by the user if the variance function is desired.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Probability and statistics`
- Mathematical domain: `probability-statistics`
- Package provenance: `unknown`
- GAMS classifications: `L7A3`
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

- Canonical provider: `main-src/src/cv.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cv.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cv.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Probability and statistics](../families/probability-and-statistics.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `XVAL` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | The variance function defines the square of the probable error of the fitted curve at any point, XVAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NDATA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NCONST` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NORD` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBKPT` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BKPT` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (NBKPT) | To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f32` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

### ABI and safety

Canonical path: `slatec_sys::statistics::cv`. Native symbol: `cv_`. Feature: `statistics`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::statistics::cv`
- Compatibility aliases: `slatec_sys::statistics::numerical::cv`
- Public declaration feature: `statistics`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
