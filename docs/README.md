# `slatec-rs` knowledge base

## Purpose

This documentation area is a provenance-first knowledge base for the SLATEC Common Mathematical Library. Its immediate purpose is to describe the historical library, its sources, routines, packages, taxonomy, dependencies, and design constraints accurately enough to support later automated cataloguing and Rust-interface planning. It is not yet documentation for a working Rust implementation.

The project follows a strict distinction between:

- **sourced historical or technical facts**;
- **modern interpretation** of those facts;
- **proposals for `slatec-rs`**, which are project decisions rather than claims about SLATEC.

See the [citation and distillation policy](sources/citation-policy.md) before adding or generating pages.

## Start here

1. [Historical and conceptual overview](history/overview.md) — why SLATEC was created, how it operated, and what is known about its present preservation.
2. [Chronology](history/chronology.md) — supported dates and release counts from 1974 through Version 4.1.
3. [Participating institutions](history/participating-institutions.md) — founding sites, later participants, and the July 1993 voting membership.
4. [Releases and changes](history/releases-and-changes.md) — numbered releases, routine-level revision evidence, and unresolved post-4.1 questions.
5. [Authoritative source register](sources/source-register.md) — primary materials, stable source IDs, coverage, limitations, and retrieval status.
6. [Citation policy](sources/citation-policy.md) — source precedence, uncertainty labels, quotation limits, and provenance requirements.
7. [Terminology](sources/terminology.md) — controlled historical and project terminology.

## Planned documentation areas

| Area | Intended content | Current status |
|---|---|---|
| `history/` | Origins, institutions, releases, maintenance, and preservation | Initial overview present |
| `sources/` | Source register, citation policy, terminology, rights evidence | Initial foundation present |
| `architecture/` | Historical library structure and later `slatec-rs` architectural proposals | Planned |
| `domains/` | Evidence-based mathematical-domain groupings | Planned |
| `packages/` | Imported and internal package histories and provenance | Planned |
| `taxonomy/` | GAMS categories and normalized classifications | Planned |
| `routines/` | Generated and reviewed routine catalogue | Planned |
| `rust/` | FFI and safe-API planning, clearly labelled as project proposals | Planned |
| `../metadata/` | Machine-readable sources, routines, dependencies, and provenance | Source register present; expansion planned |

## Evidence model

The exact routine source and prologue are the preferred authority for routine-level facts. Canonical SLATEC distribution records and the 1993 Guide govern library-level history and policy. Original package sources and publications govern imported-package intent and algorithms. Secondary material is used only for discovery or explicitly labelled context ([citation policy](sources/citation-policy.md)).

Every generated record should retain stable source IDs from `metadata/sources.toml`, direct artifact locations, retrieval dates, and checksums where files have been downloaded. Conflicts and failed retrievals must remain visible rather than being silently resolved.

## Current historical baseline

The strongest opened historical source is the July 1993 *Guide to the SLATEC Common Mathematical Library*. It states that the technical exchange committee was formed in 1974, that the common-library project was decided in 1977, and that Version 4.1 was released on 1 July 1993. Netlib currently preserves the Version 4.1 distribution, individual routine sources, archives, catalogue material, and dependency resources ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`netlib-slatec-index`](https://www.netlib.org/slatec/)).

No authoritative current statement of active SLATEC committee maintenance has yet been verified. Preservation on Netlib and active upstream governance must therefore be treated as separate questions.

## Contribution expectations

Documentation contributions should:

- cite every nontrivial factual paragraph;
- prefer primary artifacts and original publications;
- preserve historical names and date precision;
- label inference and project proposals;
- avoid long copied passages;
- avoid assuming that Netlib hosting gives every artifact one uniform licence;
- record unresolved questions rather than guess;
- keep repository-relative links and paths.

Before merging generated documentation, apply the review checklist in [citation-policy.md](sources/citation-policy.md).
