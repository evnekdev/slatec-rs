# PCHDOC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Documentation for PCHIP, a Fortran package for piecewise cubic Hermite interpolation of data.

## Description

PCHIP: Piecewise Cubic Hermite Interpolation Package This document describes the contents of PCHIP, which is a Fortran package for piecewise cubic Hermite interpolation of data. It features software to produce a monotone and "visually pleasing" interpolant to monotone data. As is demonstrated in Reference 4, such an interpolant may be more reasonable than a cubic spline if the data contains both "steep" and "flat" sections. Interpolation of cumulative probability distribution functions is another application. (See References 2-4 for examples.) All piecewise cubic functions in PCHIP are represented in cubic Hermite form; that is, f(x) is determined by its values F(I) and derivatives D(I) at the breakpoints X(I), I=1(1)N. Throughout the package a PCH function is represented by the five variables N, X, F, D, INCFD: N - number of data points; X - abscissa values for the data points; F - ordinates (function values) for the data points; D - slopes (derivative values) at the data points; INCFD - increment between successive elements in the F- and D-arrays (more on this later). These appear together and in the same order in all calls. The double precision equivalents of the PCHIP routines are obtained from the single precision names by prefixing the single precision names with a D. For example, the double precision equivalent of PCHIM is DPCHIM. The contents of the package are as follows: 1. Determine Derivative Values. NOTE: These routines provide alternate ways of determining D if these values are not already known. PCHIM -- Piecewise Cubic Hermite Interpolation to Monotone data. Used if the data are monotonic or if the user wants to guarantee that the interpolant stays within the limits of the data. (See Reference 3.) PCHIC -- Piecewise Cubic Hermite Interpolation Coefficients. Used if neither of the above conditions holds, or if the user wishes control over boundary derivatives. Will generally reproduce monotonicity on subintervals over which the data are monotonic. PCHSP -- Piecewise Cubic Hermite Spline. Produces a cubic spline interpolator in cubic Hermite form. Provided primarily for easy comparison of the spline with other piecewise cubic interpolants. (A modified version of de Boor's CUBSPL, Reference 1.) 2. Evaluate, Differentiate, or Integrate Resulting PCH Function. NOTE: If derivative values are available from some other source, these routines can be used without calling any of the previous routines. CHFEV -- Cubic Hermite Function EValuator. Evaluates a single cubic Hermite function at an array of points. Used when the interval is known, as in graphing applications. Called by PCHFE. PCHFE -- Piecewise Cubic Hermite Function Evaluator. Used when the interval is unknown or the evaluation array spans more than one data interval. CHFDV -- Cubic Hermite Function and Derivative Evaluator. Evaluates a single cubic Hermite function and its first derivative at an array of points. Used when the interval is known, as in graphing applications. Called by PCHFD. PCHFD -- Piecewise Cubic Hermite Function and Derivative Evaluator. Used when the interval is unknown or the evaluation array spans more than one data interval. PCHID -- Piecewise Cubic Hermite Integrator, Data Limits. Computes the definite integral of a piecewise cubic Hermite function when the integration limits are data points. PCHIA -- Piecewise Cubic Hermite Integrator, Arbitrary Limits. Computes the definite integral of a piecewise cubic Hermite function over an arbitrary finite interval. 3. Utility routines. PCHBS -- Piecewise Cubic Hermite to B-Spline converter. Converts a PCH function to B-representation, so that it can be used with other elements of the B-spline package (see BSPDOC). PCHCM -- Piecewise Cubic Hermite, Check Monotonicity of. Checks the monotonicity of an arbitrary PCH function. Might be used with PCHSP to build a polyalgorithm for piecewise C-2 interpolation. 4. Internal routines. CHFIE -- Cubic Hermite Function Integral Evaluator. (Real function called by PCHIA.) CHFCM -- Cubic Hermite Function, Check Monotonicity of. (Integer function called by PCHCM.) PCHCE -- PCHIC End Derivative Setter. (Called by PCHIC.) PCHCI -- PCHIC Initial Derivative Setter. (Called by PCHIC.) PCHCS -- PCHIC Monotonicity Switch Derivative Setter. (Called by PCHIC.) PCHDF -- PCHIP Finite Difference Formula. (Real function called by PCHCE and PCHSP.) PCHST -- PCHIP Sign Testing Routine. (Real function called by various PCHIP routines.) PCHSW -- PCHCS Switch Excursion Adjuster. (Called by PCHCS.) The calling sequences for these routines are described in the prologues of the respective routines. INCFD, the increment between successive elements in the Fand D-arrays is included in the representation of a PCH function in this package to facilitate two-dimensional applications. For "normal" usage INCFD=1, and F and D are one-dimensional arrays. one would call PCHxx (where "xx" is "IM", "IC", or "SP") with N, X, F, D, 1 . Suppose, however, that one has data on a rectangular mesh, F2D(I,J) = value at (X(I), Y(J)), I=1(1)NX, J=1(1)NY. Assume the following dimensions: REAL X(NXMAX), Y(NYMAX) REAL F2D(NXMAX,NYMAX), FX(NXMAX,NYMAX), FY(NXMAX,NYMAX) where 2.LE.NX.LE.NXMAX AND 2.LE.NY.LE.NYMAX . To interpolate in X along the line Y = Y(J), call PCHxx with NX, X, F2D(1,J), FX(1,J), 1 . To interpolate along the line X = X(I), call PCHxx with NY, Y, F2D(I,1), FY(I,1), NXMAX . (This example assumes the usual columnwise storage of 2-D arrays in Fortran.)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E1A`
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

- Canonical provider: `pchip/pchdoc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchdoc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
