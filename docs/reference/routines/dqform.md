# DQFORM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DNSQ and DNSQE

## Description

This subroutine proceeds from the computed QR factorization of an M by N matrix A to accumulate the M by M orthogonal matrix Q from its factored form. The subroutine statement is SUBROUTINE DQFORM(M,N,Q,LDQ,WA) where M is a positive integer input variable set to the number of rows of A and the order of Q. N is a positive integer input variable set to the number of columns of A. Q is an M by M array. On input the full lower trapezoid in the first MIN(M,N) columns of Q contains the factored form. On output Q has been accumulated into a square matrix. LDQ is a positive integer input variable not less than M which specifies the leading dimension of the array Q. WA is a work array of length M.

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

- Canonical provider: `main-src/src/dqform.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqform.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqform.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
