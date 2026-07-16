# Open research questions

## Purpose

This register covers unresolved collection-wide questions. Rust-specific decisions remain in [Rust open questions](rust/open-questions.md). Questions should be resolved through primary-source evidence, source comparison or reproducible experiments.

## Source corpus and rights

### SRC-1 — canonical source snapshot

Which exact archive, date and reorganized Netlib subsets define the `slatec-rs` baseline?

**Needed evidence:** archive checksum, extracted inventory and comparison with `slatec/src`, `slatec/lin`, `slatec/fishfft`, `slatec/fnlib` and `slatec/pchip`.

### SRC-2 — rights by artifact

What redistribution and modification conditions apply to each source file, incorporated package, guide, test and manual?

**Needed evidence:** file notices, package records, institutional statements and legal review. Do not infer a common licence from Netlib hosting.

### SRC-3 — changed or missing files

Does the complete archive contain routines omitted from the reorganized indexes, or vice versa?

## History and releases

### HIST-1 — release chronology

Can every stated version and release date be tied to a primary release record or change log?

### HIST-2 — institutional participation

Which institutions participated at which dates, and which statements describe governance versus routine authorship?

### HIST-3 — post-4.1 history

Were there official corrections, mirrored variants or package updates after the documented Version 4.1 release?

## Package provenance

### PKG-1 — exact imported revisions

Which BLAS, LINPACK, EISPACK, FFTPACK, FISHPACK, QUADPACK, SLAP, PCHIP, FNLIB, MINPACK-derived and DASSL versions are present?

### PKG-2 — SLATEC modifications

Which imported routines were renamed, precision-converted, patched, reformatted or adapted to SLATEC error/machine support?

### PKG-3 — FNLIB relationship

Netlib states that the relationship between `slatec/fnlib` and the separate `/fn` collection is unresolved. Which is newer and which fixes exist only in one copy?

### PKG-4 — ambiguous family ownership

How should Amos-associated complex special functions, B-spline collections, ODE/BVP families, constrained least squares and polynomial-root routines be represented when no standalone package boundary is verified?

## Routine catalogue

### ROUT-1 — complete user-callable/subsidiary inventory

Do the table of contents, `list`, source prologues and source declarations agree?

### ROUT-2 — argument intent quality

How often are `:IN`, `:OUT`, `:INOUT`, `:WORK` and `:EXT` missing or inconsistent with executable source?

### ROUT-3 — prologue dialects

How many legacy/final prologue formats exist, and which fields require parser-specific treatment?

### ROUT-4 — precision families

Which apparent S/D/C/Z/I/H/L pairs are true semantic equivalents, and which differ materially?

### ROUT-5 — aliases and alternate entries

Which program units define `ENTRY` points or aliases that must share source/native ownership?

## Dependencies and linkage

### DEP-1 — Netlib dependency snapshots

What source snapshot and rules produced `tree1`, `tree` and plus-dependencies output?

### DEP-2 — source versus prologue disagreement

Which `ROUTINES CALLED` lists are stale or incomplete relative to parsed source and object undefined symbols?

### DEP-3 — strongly connected components

Which routine groups form cycles that constrain native component boundaries?

### DEP-4 — duplicate symbols

Which symbols occur in the main archive, relocated subsets, compatibility error package and external package copies?

### DEP-5 — common and block data

Which routines read or write common blocks, and which `BLOCK DATA` units require forced linker retention?

### DEP-6 — compiler runtime

Which external references are Fortran runtime, system math, I/O, BLAS or unresolved application callbacks?

## Numerical domains

### DOM-1 — multidimensional cubature

What verified cubature capability exists beyond one-dimensional QUADPACK and specialized integration helpers?

### DOM-2 — differential-equation families

What are the exact boundaries and revisions of DEPAC/LSODE-related, DRIVE, BVP and integral-equation collections?

### DOM-3 — interpolation families

Which B-spline and fitting routines belong to identifiable upstream collections?

### DOM-4 — FFTPACK precision coverage

Which single/double and real/complex transform variants are actually present in the selected source snapshot?

### DOM-5 — statistical functionality

What algorithms, periods, seeding and state models are used by `RAND`, `RUNIF` and `RGAUSS`, and is there enough coherent statistical functionality for a safe statistics crate?

### DOM-6 — special-function accuracy

What authoritative test values, error bounds and branch conventions apply to each special-function family?

## ABI and runtime behavior

### ABI-1 — compiler matrix

What are the actual symbol, integer, logical, complex, character-length and callback conventions for supported compilers and targets?

### ABI-2 — error-state isolation

Does the selected error subsystem use mutable process-global state, and can fatal paths be intercepted?

### ABI-3 — thread safety

Which routines contain mutable `SAVE`, common blocks or retained callbacks?

### ABI-4 — callback cancellation

Which callback interfaces provide a documented immediate cancellation/error channel?

### ABI-5 — workspace persistence

Which work arrays must remain unchanged between continuation calls, plans or evaluations?

## Validation and documentation

### DOC-1 — complete link audit

Run a repository-local Markdown link checker, including fragment targets and generated routine pages.

### DOC-2 — source-ID lint

Verify every source ID referenced by metadata exists exactly once in `metadata/sources.toml`.

### DOC-3 — terminology lint

Enforce canonical uses of “user-callable”, “subsidiary”, “package”, “family”, “domain”, “raw crate”, “native component” and evidence-status values.

### DOC-4 — claim locators

Add field- or paragraph-level source locators to claims currently supported only by broad source lists.

### DOC-5 — quantity/date checks

Regenerate routine counts, file counts, package inventories and dates from pinned artifacts rather than hand-maintained prose.

## Resolution policy

For every resolved question record:

- evidence source and locator;
- retrieval/checksum date;
- method or experiment;
- confidence;
- affected metadata and pages;
- whether the result changes a Rust proposal.

Uncertainty should be narrowed, not removed by editorial wording alone.
