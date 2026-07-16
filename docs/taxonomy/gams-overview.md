# GAMS classification in SLATEC

## Scope

This page explains the Guide to Available Mathematical Software (GAMS) classification as it appears in the SLATEC Common Mathematical Library Version 4.1 table of contents. It is a mathematical taxonomy, not a statement of package ownership or implementation lineage.

## Official organization

The SLATEC table of contents states that Section I arranges user-callable routines by GAMS category, Section II lists subsidiary routines alphabetically, and Section III gives an alphabetical routine-to-category index. Every user-callable routine has at least one category; subsidiary routines are marked with an asterisk in the alphabetical index ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

The major categories represented in SLATEC 4.1 are:

| Code | Official broad subject | Typical SLATEC content |
|---|---|---|
| `A` | Arithmetic and error analysis | extended-range arithmetic and representation utilities |
| `C` | Elementary and special functions | elementary transcendental, gamma, Bessel, Airy, elliptic and related functions |
| `D` | Linear algebra | BLAS-like kernels, factorizations, eigenproblems, least squares and sparse solvers |
| `E` | Interpolation | polynomial, spline, Hermite and gridded interpolation |
| `F` | Solution of nonlinear equations | scalar and systems of nonlinear equations |
| `G` | Optimization | unconstrained, constrained and least-squares optimization |
| `H` | Differentiation and integration | numerical differentiation and quadrature |
| `I` | Differential and integral equations | ODE, BVP, DAE, PDE-related and integral-equation methods |
| `J` | Integral transforms | Fourier and related transforms |
| `K` | Approximation | approximation and fitting methods |
| `L` | Statistics and probability | distributions, random-number and statistical calculations |
| `N` | Data handling | searching, sorting and representation utilities |
| `R` | Service routines | error handling, machine constants and support infrastructure |
| `Z` | Other | routines not naturally assigned elsewhere |

Source: [`slatec-toc`](https://www.netlib.org/slatec/toc).

Codes are hierarchical. For example, `D1A7` identifies an elementary vector-operation class containing the AXPY operation, while `D2C1` identifies solution of general complex linear systems. A routine may have more than one code when it serves multiple mathematical roles. The classification therefore supports many-to-many mapping rather than a single directory-style hierarchy ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`gams-classification`](https://gams.nist.gov/)).

## Type markers are separate from GAMS codes

In the table of contents, a hyphen and letter after a routine name identify the data type or conversion family:

- `S`: single precision
- `D`: double precision
- `C`: complex
- `I`: integer
- `H`: character
- `L`: logical
- `A`: pseudo-type for routines that cannot reasonably be converted to another type

These markers are not part of the GAMS taxonomy. They describe routine variants and must be stored independently from mathematical classification ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## How `slatec-rs` should use GAMS

**Project interpretation.** GAMS is the most authoritative broad functional taxonomy available for SLATEC, but it is not sufficient by itself for Rust crate boundaries. A future routine record should support:

```text
routine -> zero or more GAMS leaf codes
routine -> one or more project domain IDs
routine -> zero or more historical package IDs
```

These relationships answer different questions:

- GAMS: what mathematical problem does the routine address?
- project domain: where should documentation, FFI and safe APIs be organized?
- package provenance: from which historical package or collection did the routine originate?

A routine may be in one package, several GAMS classes and a different project domain layer.

## Historical versus live GAMS

The current NIST GAMS service is an authoritative explanation of the classification concept, but the live hierarchy may have evolved since SLATEC 4.1. Historical SLATEC codes should therefore be transcribed from the 1993 SLATEC table of contents and source prologues, not silently normalized against a current web hierarchy ([`gams-classification`](https://gams.nist.gov/); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Unresolved questions

- A complete machine-readable comparison between `slatec/toc`, `slatec/gams` and source-prologue classifications has not yet been performed.
- It is not yet known how many routines carry multiple GAMS codes or inconsistent codes across official artifacts.
- The historical version of the GAMS hierarchy used during each SLATEC release has not been reconstructed.
- Some modern project domains intentionally combine or split GAMS branches; those mappings remain proposals until routine-level analysis is complete.

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc), *SLATEC Common Mathematical Library Version 4.1 Table of Contents*.
- [`slatec-guide`](https://www.netlib.org/slatec/guide), *Guide to the SLATEC Common Mathematical Library*.
- [`gams-classification`](https://gams.nist.gov/), NIST Guide to Available Mathematical Software.
