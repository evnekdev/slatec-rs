# D1UPDT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DNSQ and DNSQE

## Description

Given an M by N lower trapezoidal matrix S, an M-vector U, and an N-vector V, the problem is to determine an orthogonal matrix Q such that t (S + U*V )*Q is again lower trapezoidal. This subroutine determines Q as the product of 2*(N - 1) transformations GV(N-1)*...*GV(1)*GW(1)*...*GW(N-1) where GV(I), GW(I) are Givens rotations in the (I,N) plane which eliminate elements in the I-th and N-th planes, respectively. Q itself is not accumulated, rather the information to recover the GV, GW rotations is returned. The SUBROUTINE statement is SUBROUTINE D1UPDT(M,N,S,LS,U,V,W,SING) where M is a positive integer input variable set to the number of rows of S. N is a positive integer input variable set to the number of columns of S. N must not exceed M. S is an array of length LS. On input S must contain the lower trapezoidal matrix S stored by columns. On output S contains the lower trapezoidal matrix produced as described above. LS is a positive integer input variable not less than (N*(2*M-N+1))/2. U is an input array of length M which must contain the vector U. V is an array of length N. On input V must contain the vector V. On output V(I) contains the information necessary to recover the Givens rotation GV(I) described above. W is an output array of length M. W(I) contains information necessary to recover the Givens rotation GW(I) described above. SING is a LOGICAL output variable. SING is set TRUE if any of the diagonal elements of the output S are zero. Otherwise SING is set FALSE.

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

- Canonical provider: `main-src/src/d1updt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/d1updt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/d1updt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/d1updt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
