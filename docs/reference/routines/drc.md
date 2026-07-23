# DRC

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate a double precision approximation to DRC(X,Y) = Integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive.

## Description

1. DRC Standard FORTRAN function routine Double precision version The routine calculates an approximation result to DRC(X,Y) = integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. Logarithmic, inverse circular, and inverse hyper- bolic functions can be expressed in terms of DRC. 2. Calling Sequence DRC( X, Y, IER ) Parameters On Entry Values assigned by the calling routine

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DRC](https://www.netlib.org/slatec/src/drc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision, nonnegative variable unaltered. 3. Error messages Value of IER assigned by the DRC routine Value assigned Error message printed IER = 1 X. LT. 0. 0D0. |
| 2 | `Y` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Double precision, positive variable On Return (values assigned by the DRC routine) DRC - Double precision approximation to the integral unaltered. 3. Error messages Value of IER assigned by the DRC routine Value assigned Error message printed IER = 1 X. LT. 0. 0D0. |
| 3 | `IER` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | to indicate normal or abnormal termination. IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_i32)`.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IER` | `0` | 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine |
| `IER` | `1` | 1 X.LT.0.0D0.OR.Y.LE.0.0D0 |
| `IER` | `2` | 2 X+Y.LT.LOLIM |
| `IER` | `3` | 3 MAX(X,Y) .GT. UPLIM 4. Control parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X and Y LOLIM - Lower limit of valid arguments Not less than 5 * (machine minimum) . UPLIM - Upper limit of valid arguments Not greater than (machine maximum) / 5 . Acceptable values for: LOLIM UPLIM IBM 360/370 SERIES : 3.0D-78 1.0D+75 CDC 6000/7000 SERIES : 1.0D-292 1.0D+321 UNIVAC 1100 SERIES : 1.0D-307 1.0D+307 CRAY : 2.3D-2466 1.0D+2465 VAX 11 SERIES : 1.5D-38 3.0D+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - relative error due to truncation is less than 16 * ERRTOL ** 6 / (1 - 2 * ERRTOL). The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the trunca- tion error there will be round-off error, but in prac- tice the total error from both sources is usually less than the amount given in the table. Sample choices: ERRTOL Relative truncation 1.0D-3 2.0D-17 3.0D-3 2.0D-14 1.0D-2 2.0D-11 3.0D-2 2.0D-8 1.0D-1 2.0D-5 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. |

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::drc`. Native symbol: `drc_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::drc`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::scalar_expanded::carlson_rc`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
