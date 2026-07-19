# XSETF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Set the error control flag.

## Description

Abstract XSETF sets the error control flag value to KONTRL. (KONTRL is an input parameter only.) The following table shows how each message is treated, depending on the values of KONTRL and LEVEL. (See XERMSG for description of LEVEL.) If KONTRL is zero or negative, no information other than the message itself (including numeric values, if any) will be printed. If KONTRL is positive, introductory messages, trace-backs, etc., will be printed in addition to the message. ABS(KONTRL) LEVEL 0 1 2 value 2 fatal fatal fatal 1 not printed printed fatal 0 not printed printed printed -1 not printed printed printed only only once once

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Error handling`
- Mathematical domain: `runtime-support`
- Package provenance: `slatec-error`
- GAMS classifications: `R3A`
- Family evidence: `package_provenance` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/xsetf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xsetf.f` (`live-main-source`)
  - `err/xsetf.f` (`legacy-error-directory`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xsetf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xsetf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
