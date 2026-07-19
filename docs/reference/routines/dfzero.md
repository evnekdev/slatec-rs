# DFZERO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Search for a zero of a function F(X) in a given interval (B,C). It is designed primarily for problems where F(B) and F(C) have opposite signs.

## Description

DFZERO searches for a zero of a DOUBLE PRECISION function F(X) between the given DOUBLE PRECISION values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B-C) .LE. 2.*(RW*ABS(B)+AE). The method used is an efficient combination of bisection and the secant rule and is due to T. J. Dekker. Description Of Arguments F :EXT - Name of the DOUBLE PRECISION external function. This name must be in an EXTERNAL statement in the calling program. F must be a function of one DOUBLE PRECISION argument. B :INOUT - One end of the DOUBLE PRECISION interval (B,C). The value returned for B usually is the better approximation to a zero of F. C :INOUT - The other end of the DOUBLE PRECISION interval (B,C) R :IN - A (better) DOUBLE PRECISION guess of a zero of F which could help in speeding up convergence. If F(B) and F(R) have opposite signs, a root will be found in the interval (B,R); if not, but F(R) and F(C) have opposite signs, a root will be found in the interval (R,C); otherwise, the interval (B,C) will be searched for a possible root. When no better guess is known, it is recommended that R be set to B or C, since if R is not interior to the interval (B,C), it will be ignored. RE :IN - Relative error used for RW in the stopping criterion. If the requested RE is less than machine precision, then RW is set to approximately machine precision. AE :IN - Absolute error used in the stopping criterion. If the given interval (B,C) contains the origin, then a nonzero value should be chosen for AE. IFLAG :OUT - A status code. User must check IFLAG after each call. Control returns to the user from DFZERO in all cases. 1 B is within the requested tolerance of a zero. The interval (B,C) collapsed to the requested tolerance, the function changes sign in (B,C), and F(X) decreased in magnitude as (B,C) collapsed. 2 F(B) = 0. However, the interval (B,C) may not have collapsed to the requested tolerance. 3 B may be near a singular point of F(X). The interval (B,C) collapsed to the requested tolerance and the function changes sign in (B,C), but F(X) increased in magnitude as (B,C) collapsed, i.e. ABS(F(B out)) .GT. MAX(ABS(F(B in)),ABS(F(C in))) 4 No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether B is near a local minimum of F(X), or B is near a zero of even multiplicity, or neither of these. 5 Too many (.GT. 500) function evaluations used.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F1B`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::roots::find_root`

## Providers

- Canonical provider: `main-src/src/dfzero.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dfzero.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dfzero.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dfzero.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.
