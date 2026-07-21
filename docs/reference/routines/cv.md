# CV

[Family: Probability and statistics](../families/probability-and-statistics.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram FC.

## Description

CV( ) is a companion function subprogram for FC( ). The documentation for FC( ) has complete usage instructions. CV( ) is used to evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram, FC( ).

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

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [CV](https://www.netlib.org/slatec/src/cv.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `XVAL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `NDATA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `NCONST` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `NORD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `NBKPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `BKPT` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NBKPT) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

of the fitted curve at any point, XVAL.  One can use the square around the fitted curve. CV( ) is used after a call to FC( ).  MODE, an input variable to FC( ), is used to indicate if the variance function is desired. In order to use CV( ), MODE must equal 2 or 4 on input to FC( ). MODE is also used as an output flag from FC( ).  Check to make sure that MODE = 0 after calling FC( ), indicating a successful constrained curve fit.  The array SDDATA, as input to FC( ), must also be defined with the standard deviation or uncertainty of the Y values to use CV( ). To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. The vector B(XVAL) is the B-spline basis function values at X=XVAL.  The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints.  This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. All the variables in the calling sequence for CV( ) are used in FC( ) except the variable XVAL.  Do not change the values of these variables between the call to FC( ) and the use of CV( ). The following is a brief description of the variables XVAL    The point where the variance is desired. NDATA   The number of discrete (X,Y) pairs for which FC( ) calculated a piece-wise polynomial curve. NCONST  The number of conditions that constrained the B-spline in FC( ). NORD    The order of the B-spline used in FC( ). The value of NORD must satisfy 1 < NORD < 20 . (The order of the spline is one more than the degree of the piece-wise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For example, NORD=4 when we are using piece-wise cubics.) NBKPT   The number of knots in the array BKPT(*). The value of NBKPT must satisfy NBKPT .GE. 2*NORD. BKPT(*) The real array of knots.  Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1).  The additional end knots BKPT(I),I=1,...,NORD-1 and I=NBKPT-NORD+2,...,NBKPT, are required by FC( ) to compute the functions used to fit the data. W(*)    Real work array as used in FC( ).  See FC( ) for the required length of W(*).  The contents of W(*) must not be modified by the user if the variance function is desired.

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::statistics::cv`. Native symbol: `cv_`. Declaration feature: `statistics`. Provider feature: `statistics-core`. ABI fingerprint: `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::statistics::cv`
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
