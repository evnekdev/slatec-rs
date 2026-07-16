# General utilities and cross-cutting support

## Scope

This page covers routines not naturally owned by a mathematical algorithm domain: machine constants, error handling, sorting/permutation, data input/output, representation conversion, searching and miscellaneous support. GAMS classes `N`, `R`, `A` and `Z` contain many such routines ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Utility groups

| Group | Representative routines | Notes |
|---|---|---|
| Machine constants | `I1MACH`, `R1MACH`, `D1MACH` | historical target-dependent numeric/environment constants |
| Error handling | `XERMSG`, `XERROR`, `XSETF`, `XSETUN`, `XSETUA`, `NUMXER`, `XERCLR`, `XERDMP`, `XERMAX`, `FDUMP` | shared, potentially stateful subsystem |
| Sorting | `SSORT/DSORT/ISORT`, `SPSORT/DPSORT/IPSORT`, `HPSORT` | direct or permutation-producing sorts |
| Permutation | `SPPERM/DPPERM/IPPERM/HPPERM` | rearrange arrays by permutation |
| Interval/search support | `INTRV/DINTRV` and package-specific helpers | locate intervals or indices |
| Sparse I/O | `SBHIN/DBHIN`, `STIN/DTIN`, `STOUT/DTOUT`, `SCPPLT/DCPPLT` | historical formatted input/output for SLAP matrices |
| Extended-range arithmetic | `XADD/DXADD`, `XSET/DXSET`, `XRED/DXRED` | arithmetic support beyond ordinary exponent range |
| Representation conversion | `R9PAK/D9PAK`, `R9UPAK/D9UPAK` | pack/unpack binary exponents |
| Documentation drivers | `FUNDOC`, `FFTDOC`, `QPDOC`, `PCHDOC`, `BSPDOC`, `SLPDOC` | executable/documentation entry points rather than numerical APIs |

## Machine constants and modern targets

The historical design centralizes machine-dependent values. Modern ports may compute constants from language/runtime facilities, but compatibility must preserve the exact meaning of each indexed value. Replacing source routines is a project decision requiring validation, not a documentation fact.

## Error subsystem

The error subsystem controls message routing, repetition limits, fatal/nonfatal behavior and current error state. This implies mutable process-level configuration in at least some configurations. Safe wrappers should not assume thread-local behavior.

Potential strategy:

1. retain raw error routines unchanged initially;
2. install a controlled nonterminating policy where supported;
3. serialize calls if global state cannot be isolated;
4. capture routine-specific status independently from global error messages;
5. never allow a Fortran `STOP` or site hook to terminate the host unexpectedly.

## Sorting and permutation

Sorting routines may optionally rearrange an auxiliary array or return permutation vectors. Safe APIs must clarify:

- stable versus unstable sorting;
- ascending/descending flags;
- handling of NaNs;
- one-based versus zero-based permutations;
- whether input is overwritten;
- character substring conventions.

Modern Rust sorting is usually preferable unless exact SLATEC behavior or a package dependency requires these routines.

## Sparse input/output

SLAP I/O routines understand historical Boeing/Harwell and SLAP formats and use Fortran logical units. Their direct exposure would introduce filesystem, formatting and unit-number concerns into the FFI boundary.

**Project proposal:** treat these as optional compatibility tools, not core sparse-solver APIs. A Rust parser/serializer may be safer, but must be validated against examples before replacing behavior.

## FFI risks

- Fortran logical-unit I/O and hidden runtime state;
- character argument length conventions;
- process-global error configuration;
- site-replaceable `FDUMP` and `XERHLT` hooks;
- integer-width assumptions in permutations;
- routines that overwrite arrays;
- `SAVE`, `COMMON` or `BLOCK DATA` initialization;
- symbolic names that collide with system numerical libraries.

## Tentative crate ownership

- Raw: `slatec-runtime-sys` for machine constants and errors; utility files may live with their consumers unless a stable shared closure emerges.
- Safe: `slatec-runtime` as an internal/support crate.
- Sorting and I/O should not become independent public crates without demonstrated demand.
- Extended-range arithmetic may justify a `slatec-numeric-support` module if used broadly.

## Open questions

- Which utility routines use `COMMON`, `SAVE` or `BLOCK DATA`?
- Can error state be isolated per thread or solver session?
- Which machine-constant sources are active in the archive?
- Are sparse I/O formats fully documented and portable?
- Which utilities are duplicated by imported packages?
- Can dead-code elimination omit required block-data initialization?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
