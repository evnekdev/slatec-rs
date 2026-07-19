# DFDJC1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DNSQ and DNSQE

## Description

This subroutine computes a forward-difference approximation to the N by N Jacobian matrix associated with a specified problem of N functions in N variables. If the Jacobian has a banded form, then function evaluations are saved by only approximating the nonzero terms. The subroutine statement is SUBROUTINE DFDJC1(FCN,N,X,FVEC,FJAC,LDFJAC,IFLAG,ML,MU,EPSFCN, WA1,WA2) where FCN is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) Calculate the functions at X and return this vector in FVEC. RETURN The value of IFLAG should not be changed by FCN unless the user wants to terminate execution of DFDJC1. In this case set IFLAG to a negative integer. N is a positive integer input variable set to the number of functions and variables. X is an input array of length N. FVEC is an input array of length N which must contain the functions evaluated at X. FJAC is an output N by N array which contains the approximation to the Jacobian matrix evaluated at X. LDFJAC is a positive integer input variable not less than N which specifies the leading dimension of the array FJAC. IFLAG is an integer variable which can be used to terminate the execution of DFDJC1. See description of FCN. ML is a nonnegative integer input variable which specifies the number of subdiagonals within the band of the Jacobian matrix. If the Jacobian is not banded, set ML to at least N - 1. EPSFCN is an input variable used in determining a suitable step length for the forward-difference approximation. This approximation assumes that the relative errors in the functions are of the order of EPSFCN. If EPSFCN is less than the machine precision, it is assumed that the relative errors in the functions are of the order of the machine precision. MU is a nonnegative integer input variable which specifies the number of superdiagonals within the band of the Jacobian matrix. If the Jacobian is not banded, set MU to at least N - 1. WA1 and WA2 are work arrays of length N. If ML + MU + 1 is at least N, then the Jacobian is considered dense, and WA2 is not referenced.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DNSQ, DNSQE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dfdjc1.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dfdjc1.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dfdjc1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dfdjc1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
