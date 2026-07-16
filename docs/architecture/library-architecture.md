# SLATEC library architecture

## Scope

This page describes the architectural model of SLATEC Common Mathematical Library (CML) Version 4.1 as documented in the July 1993 guide and preserved by Netlib. It separates official statements, observations from representative sources, implications for future Rust FFI, and questions that require later source-wide analysis.

## Official architecture

SLATEC is a broad collection of general-purpose mathematical and statistical Fortran 77 subprograms rather than a single algorithmic package. The official Netlib index describes Version 4.1 as containing more than 1,400 routines, while the guide records 902 user-callable routines in the July 1993 release. The difference is explained by the library's inclusion of both user-callable and subsidiary routines ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-guide`](https://www.netlib.org/slatec/guide)).

The library is organized around several interacting layers:

1. **User-callable numerical routines**, classified by GAMS mathematical category.
2. **Subsidiary routines**, intended primarily to support other library routines.
3. **Imported or incorporated packages**, including major bodies of code such as BLAS, LINPACK, EISPACK, FFTPACK, FISHPACK, FNLIB, PCHIP, QUADPACK and others identified in source prologues or Netlib sublibraries.
4. **Shared support infrastructure**, especially machine constants, error handling, documentation conventions, quick checks and documentation-extraction tools.
5. **Distribution metadata**, including the table of contents, routine lists, GAMS mappings, direct dependency records and transitive dependency records ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`slatec-toc`](https://www.netlib.org/slatec/toc); [`netlib-slatec-index`](https://www.netlib.org/slatec/)).

The guide required new routines to build on existing SLATEC routines unless there was a compelling reason not to do so. It specifically cited the existing LINPACK and EISPACK holdings as infrastructure that new code should reuse. This policy naturally produced a library-wide call graph rather than isolated, independently linkable routines ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## Public and subsidiary interfaces

The official table of contents has separate sections for user-callable and subsidiary routines. It states that subsidiary routines normally are not referenced directly by users, although they are listed to avoid name collisions and to help authors constructing new library routines. In the alphabetical index, an asterisk marks subsidiary routines ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

A user-callable designation is therefore a documentation and support boundary, not a language-level visibility rule. Fortran 77 object files do not inherently prevent a caller from invoking a subsidiary symbol. Conversely, some user-callable routines may only be usable when a nontrivial dependency closure, callback, work array or support subsystem is also linked.

## Cross-cutting support services

### Machine constants

SLATEC centralized machine-dependent numeric and I/O characteristics in `I1MACH`, `R1MACH` and `D1MACH`. The guide prohibited other library routines from embedding or calculating their own machine constants, and the Netlib index warns that the distributed copies require site-specific selection or replacement ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`netlib-slatec-index`](https://www.netlib.org/slatec/)).

### Error handling

The guide required detectable errors in user-callable routines to be represented by an integer error flag when appropriate. Messages were to be routed through `XERMSG`, not printed directly. The error package includes policy state, saved error numbers, message formatting, output-unit selection, optional traceback support and a fatal-stop hook ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`slatec-source-tree`](https://www.netlib.org/slatec/src/)).

### Documentation as structured source data

The guide says SLATEC's routine documentation is carried in source-code comments and prescribes a rigid prologue layout to enable automated extraction and transformation. The architecture therefore includes documentation metadata as part of each source unit, not as a separate manual layer ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

### Quick checks

Each user-callable routine, and subsidiary routines when appropriate, was expected to have a small demonstration or quick check. The Netlib distribution preserves 54 quick-check drivers, while warning that failures may reflect the machine, operating system, compiler, tests or external software rather than a defect in the numerical source ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`netlib-slatec-index`](https://www.netlib.org/slatec/)).

## Source-inspection findings

**Inferred:** SLATEC is best modelled as a federation of routine families and historical packages sharing conventions and support services, not as a uniform monolith. The `LIBRARY` prologue field can name an incorporated package, the `TYPE` field can connect precision-equivalent routines, and `ROUTINES CALLED` provides routine-local dependency evidence. However, older incorporated sources do not always use the final 1993 prologue format, so machine extraction must tolerate multiple generations of metadata ([`slatec-guide`](https://www.netlib.org/slatec/guide); representative older QUADPACK source: [`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)).

**Inferred:** Architectural boundaries overlap. A routine may belong simultaneously to a mathematical GAMS category, a historical source package, a precision family, a public/subsidiary classification and a dependency component. No single one of these should be treated as the definitive future Rust crate boundary.

## Implications for future Rust FFI

**Project implications:**

- Preserve exact native symbols and signatures in raw FFI crates, even when the eventual safe API groups routines differently.
- Record at least four independent classifications per routine: public/subsidiary status, source package, mathematical domain and dependency component.
- Treat support routines as explicit dependencies rather than invisible implementation details.
- Do not infer thread safety from purely numerical purpose. Calls involving error-package state, `COMMON`, `SAVE`, initialized work arrays or callbacks require individual analysis.
- Keep historical behavior and modern wrapper policy separate. A safe wrapper may suppress global printing or translate error flags, but the raw layer should document the native behavior precisely.
- Model callback signatures, workspaces and persistent initialization states in metadata before designing safe Rust abstractions.

## Open questions

- Which routines in the complete 4.1 inventory violate the final naming, prologue or argument-order recommendations?
- Which user-callable routines depend on global mutable state directly or transitively?
- Which Netlib sublibrary copies exactly match the routines included in the 4.1 archive?
- How many dependency components can be linked independently without duplicate symbols?
- Which quick checks depend on historical compiler extensions or machine-specific constants?
- Does the current `slatec_src.tgz` inventory exactly match the browsable Netlib source and moved sublibraries?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/)
- [`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)
