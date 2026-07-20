# DRC

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate a double precision approximation to DRC(X,Y) = Integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive.

## Description

1. DRC Standard FORTRAN function routine Double precision version The routine calculates an approximation result to DRC(X,Y) = integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. Logarithmic, inverse circular, and inverse hyperbolic functions can be expressed in terms of DRC. 2. Calling Sequence DRC( X, Y, IER ) Parameters On Entry Values assigned by the calling routine X - Double precision, nonnegative variable Y - Double precision, positive variable On Return (values assigned by the DRC routine) DRC - Double precision approximation to the integral IER - Integer to indicate normal or abnormal termination. IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X and Y are unaltered. 3. Error messages Value of IER assigned by the DRC routine Value assigned Error message printed IER = 1 X.LT.0.0D0.OR.Y.LE.0.0D0 = 2 X+Y.LT.LOLIM = 3 MAX(X,Y) .GT. UPLIM 4. Control parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X and Y LOLIM - Lower limit of valid arguments Not less than 5 * (machine minimum) . UPLIM - Upper limit of valid arguments Not greater than (machine maximum) / 5 . Acceptable values for: LOLIM UPLIM IBM 360/370 SERIES : 3.0D-78 1.0D+75 CDC 6000/7000 SERIES : 1.0D-292 1.0D+321 UNIVAC 1100 SERIES : 1.0D-307 1.0D+307 CRAY : 2.3D-2466 1.0D+2465 VAX 11 SERIES : 1.5D-38 3.0D+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - relative error due to truncation is less than 16 * ERRTOL ** 6 / (1 - 2 * ERRTOL). The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the truncation error there will be round-off error, but in practice the total error from both sources is usually less than the amount given in the table. Sample choices: ERRTOL Relative truncation error less than 1.0D-3 2.0D-17 3.0D-3 2.0D-14 1.0D-2 2.0D-11 3.0D-2 2.0D-8 1.0D-1 2.0D-5 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. *Long Description: DRC special comments Check: DRC(X,X+Z) + DRC(Y,Y+Z) = DRC(0,Z) where X, Y, and Z are positive and X * Y = Z * Z On Input: X, and Y are the variables in the integral DRC(X,Y). On Output: X and Y are unaltered. DRC(0,1/4)=DRC(1/16,1/8)=PI=3.14159... DRC(9/4,2)=LN(2) ******************************************************** WARNING: Changes in the program may improve speed at the expense of robustness. Special functions via DRC LN X X .GT. 0 2 LN(X) = (X-1) DRC(((1+X)/2) , X ) ARCSIN X -1 .LE. X .LE. 1 2 ARCSIN X = X DRC (1-X ,1 ) ARCCOS X 0 .LE. X .LE. 1 2 2 ARCCOS X = SQRT(1-X ) DRC(X ,1 ) ARCTAN X -INF .LT. X .LT. +INF 2 ARCTAN X = X DRC(1,1+X ) ARCCOT X 0 .LE. X .LT. INF 2 2 ARCCOT X = DRC(X ,X +1 ) ARCSINH X -INF .LT. X .LT. +INF 2 ARCSINH X = X DRC(1+X ,1 ) ARCCOSH X X .GE. 1 2 2 ARCCOSH X = SQRT(X -1) DRC(X ,1 ) ARCTANH X -1 .LT. X .LT. 1 2 ARCTANH X = X DRC(1,1-X ) ARCCOTH X X .GT. 1 2 2 ARCCOTH X = DRC(X ,X -1 )

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C14`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rc`

## Providers

- Canonical provider: `main-src/src/drc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/drc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/drc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/drc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | input/output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DRC Standard FORTRAN function routine Double precision version The routine calculates an approximation result to DRC(X,Y) = integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | input/output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DRC Standard FORTRAN function routine Double precision version The routine calculates an approximation result to DRC(X,Y) = integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Calling Sequence DRC( X, Y, IER ) Parameters On Entry Values assigned by the calling routine X - Double precision, nonnegative variable Y - Double precision, positive variable On Return (values assigned by the DRC routine) DRC - Double precision approximation to the integral IER - Integer to indicate normal or abnormal termination. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f64` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:f64(mut_f64,mut_f64,mut_i32)`.

### ABI and safety

Canonical path: `slatec_sys::special::drc`. Native symbol: `drc_`. Feature: `special`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::drc`
- Compatibility aliases: `slatec_sys::special::numerical::drc`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rc`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
