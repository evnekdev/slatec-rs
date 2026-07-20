# DEFC

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense.

## Description

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. The data can be processed in groups of modest size. The size of the group is chosen by the user. This feature may be necessary for purposes of using constrained curve fitting with subprogram DFC( ) on a very large data set. For a description of the B-splines and usage instructions to evaluate them, see C. W. de Boor, Package for Calculating with B-Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978). Input.. All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). No sorting of XDATA(*) is required. Any non-negative value of NDATA is allowed. A negative value of NDATA is an error. A zero value for any entry of SDDATA(*) will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry. NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,..., NORD-1 and I=NBKPT-NORD+2,...,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required. Internal to DEFC( ) the extreme end knots may be reduced and increased respectively to accommodate any data values that are exterior to the given knot values. The contents of BKPT(*) is not changed. NORD must be in the range 1 .LE. NORD .LE. 20. The value of NBKPT must satisfy the condition NBKPT .GE. 2*NORD. Other values are considered errors. (The order of the spline is one more than the degree of the piecewise polynomial defined on each interval. This is consistent with the B-spline package convention. For example, NORD=4 when we are using piecewise cubics.) MDEIN An integer flag, with one of two possible values (1 or 2), that directs the subprogram action with regard to new data points provided by the user. =1 The first time that DEFC( ) has been entered. There are NDATA points to process. =2 This is another entry to DEFC(). The subprogram DEFC( ) has been entered with MDEIN=1 exactly once before for this problem. There are NDATA new additional points to merge and process with any previous points. (When using DEFC( ) with MDEIN=2 it is important that the set of knots remain fixed at the same values for all entries to DEFC( ).) LW The amount of working storage actually allocated for the working array W(*). This quantity is compared with the actual amount of storage needed in DEFC( ). Insufficient storage allocated for W(*) is an error. This feature was included in DEFC because misreading the storage formula for W(*) might very well lead to subtle and hard-to-find programming bugs. The length of the array W(*) must satisfy LW .GE. (NBKPT-NORD+3)*(NORD+1)+ (NBKPT+1)*(NORD+1)+ 2*MAX(NDATA,NBKPT)+NBKPT+NORD**2 Output.. All TYPE REAL variables are DOUBLE PRECISION MDEOUT An output flag that indicates the status of the curve fit. =-1 A usage error of DEFC( ) occurred. The offending condition is noted with the SLATEC

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

- Canonical provider: `main-src/src/defc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/defc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/defc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/defc.f) — `verified_cached`
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
| `NDATA` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XDATA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YDATA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SDDATA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | All TYPE REAL variables are DOUBLE PRECISION NDATA,XDATA(*), YDATA(*), SDDATA(*) The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and SDDATA(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NORD` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBKPT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BKPT` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDEIN` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | For example, NORD=4 when we are using piecewise cubics.) MDEIN An integer flag, with one of two possible values (1 or 2), that directs the subprogram action with regard to new data points provided by the user. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDEOUT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | All TYPE REAL variables are DOUBLE PRECISION MDEOUT An output flag that indicates the status of the curve fit. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `COEFF` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LW` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | (When using DEFC( ) with MDEIN=2 it is important that the set of knots remain fixed at the same values for all entries to DEFC( ).) LW The amount of working storage actually allocated for the working array W(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::approximation::defc`. Native symbol: `defc_`. Feature: `approximation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::defc`
- Compatibility aliases: `slatec_sys::approximation::numerical::defc`
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
