# Package provenance overview

## Purpose

This section records historical package provenance independently from mathematical taxonomy. Its purpose is to support exact source lineage, acknowledgements, licensing review, routine-catalogue generation and future Rust packaging decisions.

## Three independent classifications

Every routine may eventually have:

1. **Functional classification:** GAMS codes and project domain IDs.
2. **Historical provenance:** package, smaller family, individual contribution or support subsystem.
3. **Dependency role:** kernel, helper, driver, callback interface or cross-cutting runtime service.

These classifications must not be collapsed. For example, FFTPACK and FISHPACK share a relocated Netlib directory but serve transforms and elliptic PDEs respectively; QUADPACK routines share a functional quadrature domain but may call support routines outside that package ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

## Evidence levels

Package assignments in `metadata/packages.toml` use:

- `verified`: the official SLATEC index or exact source explicitly names the package.
- `high`: official package and SLATEC indexes strongly agree, but an exact archive diff has not been completed.
- `medium`: family identity is well supported, but exact SLATEC membership or revision is incomplete.
- `low`: plausible attribution awaiting source-level proof.
- `unknown`: no reliable package boundary is currently established.

A package record does not imply that every file in a Netlib directory is an unchanged copy of an upstream release.

## Provenance workflow

For each proposed package:

1. identify the canonical upstream collection and original documentation;
2. inventory the SLATEC copy;
3. compare filenames, routine names, prologue authorship and revision histories;
4. compare source content or checksums where possible;
5. record renames, precision conversions and local patches;
6. identify dependencies crossing the package boundary;
7. preserve ambiguous candidates rather than choosing one silently;
8. review file-level rights before redistribution.

## Package versus family

A named package generally has an identifiable distribution, documentation and coherent routine set. A family may only share authorship, algorithmic lineage or naming. The project should use package IDs only when evidence supports a package identity.

Examples:

- `blas`, `linpack`, `eispack`, `fftpack`, `quadpack`, `fishpack`, `slap`, `pchip` and `fnlib` are treated as package/collection candidates.
- complex Bessel and Airy routines associated with Amos algorithms may be represented as a routine family until the exact distribution identity is established.
- central error handling and machine constants are support subsystems, not mathematical packages.

## Implications for Rust organization

**Project interpretation:** provenance-based crates could simplify upstream comparison and acknowledgements, while domain-based crates could provide a more coherent user API. These goals can conflict.

A likely hybrid is:

- raw FFI build units based on linkable source/dependency closure;
- metadata preserving original package provenance;
- safe wrappers organized primarily by function/domain;
- a facade that exposes coherent user workflows.

No crate boundary should be finalized until routine inventory, symbol collisions, dependency strongly connected components and licensing constraints are known.

## Unresolved questions

- Which package copies were modified by SLATEC, and at what revision?
- Are package labels in the reorganized Netlib tree identical to the historical 4.1 archive boundaries?
- Which routines were precision-converted or renamed by SLATEC tooling?
- Which routines belong to multiple historical collections?
- Which source files have independent rights or attribution conditions?
- Should compatibility implementations remain separate even when functionally redundant?

## Sources

See [Imported packages](imported-packages.md), the [SLATEC subpackage taxonomy](../taxonomy/slatec-subpackages.md) and the authoritative [source register](../sources/source-register.md).
