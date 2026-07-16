# Machine constants and portability services

## Scope

This page documents `I1MACH`, `R1MACH`, `D1MACH`, related site-dependent routines and the distinction between historical machine models and modern language constants.

## Official policy

The guide says SLATEC uses `I1MACH`, `R1MACH` and `D1MACH` to centralize machine-dependent values. Other library routines should neither data-load machine constants independently nor attempt to derive them at run time. The routines originated in the Bell Laboratories PORT library ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

The historical rationale was broader than discovering nominal hardware limits. The constants describe a conservative subset of the machine's number system on which arithmetic operations behave reliably. The guide notes that hardware manuals may describe representation without fully characterizing arithmetic behavior near exponent extremes or powers of the radix ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## `I1MACH`

`I1MACH(N)`, for `1 <= N <= 16`, supplies integer, I/O and floating-model parameters:

| N | Meaning |
|---:|---|
| 1–4 | standard input, standard output, punch, and error-message unit numbers |
| 5 | bits per integer storage unit |
| 6 | characters per integer storage unit |
| 7 | integer radix |
| 8 | integer digits in that radix |
| 9 | largest magnitude integer with the required arithmetic behavior |
| 10 | floating-point radix `B` |
| 11–13 | single-precision significand digits `T`, minimum exponent `EMIN`, maximum exponent `EMAX` |
| 14–16 | double-precision `T`, `EMIN`, `EMAX` |

The I/O entries demonstrate that `I1MACH` is not merely a numeric-limits function; it also encodes historical runtime conventions ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## `R1MACH` and `D1MACH`

For the model `fraction * B**exponent`, `R1MACH` and `D1MACH` each provide five values:

| N | Meaning |
|---:|---|
| 1 | smallest positive value in the conservative arithmetic range, `B**(EMIN-1)` |
| 2 | largest positive value, `B**EMAX * (1-B**(-T))` |
| 3 | smallest relative spacing, `B**(-T)` |
| 4 | largest relative spacing, `B**(1-T)` |
| 5 | `log10(B)` |

The guide explicitly warns that entries 3 and 4 are not simply identical to the colloquial machine-epsilon definition. For iteration convergence it recommends entry 4 as a practical upper bound on theoretical epsilon ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## Historical installation model

The Netlib index states that the distributed copies contain machine-specific constant blocks commented out and require the installer to select the correct machine definitions, historically by uncommenting appropriate lines. It identifies five site-dependent routines in the main source: `D1MACH`, `I1MACH`, `R1MACH`, `FDUMP` and `XERHLT` ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

This is a build-configuration responsibility. A successful compilation of unconfigured sources does not prove that the returned constants are correct.

## Modern interpretation

**Interpretation:** Modern Fortran intrinsics and IEEE-oriented compiler environments can supply many corresponding values, but replacing the historical routines is not semantically trivial. SLATEC algorithms may have been tuned to the conservative PORT model, and `I1MACH` includes I/O units and assumptions about storage that are not direct equivalents of Rust's numeric associated constants.

**Interpretation:** On a target restricted to a known ABI and IEEE binary32/binary64 arithmetic, a generated or small compatibility implementation may be preferable to preserving large commented machine tables. That is a future build decision and must be verified against quick checks and representative edge cases.

## Rust FFI implications

- The raw native library must provide these external symbols because many routines depend on them.
- Their integer return type and symbol mangling are compiler-ABI concerns.
- A Rust-side constant cannot satisfy a Fortran external call unless an ABI-compatible exported function is supplied.
- Correctness tests should compare returned constants to compiler/runtime properties and to numerical behavior expected by SLATEC routines.
- `I1MACH(1..4)` output-unit semantics need a policy for embedded applications where Fortran units are undesirable.
- Do not translate `R1MACH(3/4)` or `D1MACH(3/4)` mechanically to a single `EPSILON` value without documenting the selected correspondence.

## Open questions

- Which modernized machine-constant implementation should be the canonical build input?
- Do all supported compilers use the same default integer width and real kinds?
- Which routines rely on I/O unit entries rather than numeric entries?
- Are any algorithms sensitive to conservative values different from exact IEEE limits?
- How should non-IEEE targets or extended-precision compiler modes be handled?
- Are there source copies of these routines in multiple Netlib sublibraries that would create duplicate symbols?

## Validated GNU MinGW profile

The `ffi-profile-gnu-mingw-x86_64` build does not use an uncommented historical
table. The selected archive templates remain immutable evidence, while explicit
profile compatibility providers derive the documented contracts from compiler
inquiry intrinsics. All valid selectors are compared with an independent
authored Fortran inquiry program. See the
[runtime-profile validation](../extraction/runtime-profile-validation.md) for
the provider decision, selector classifications, and FNLIB reprobes. This
resolution applies only to the supported GNU MinGW x86_64 profile; the broader
cross-platform questions remain open.

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/)
