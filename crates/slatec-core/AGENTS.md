# `slatec-core` instructions

Scope: `crates/slatec-core/**`.

## Responsibility

This crate contains provider-neutral shared types and infrastructure used across layers.

## Rules

- Keep the crate independent of a concrete native provider.
- Do not add raw SLATEC declarations or safe family-specific numerical wrappers here.
- Preserve `no_std`/`alloc` boundaries and feature minimality where supported.
- Avoid dependencies that force `std`, filesystem, process, network, or provider selection into core APIs.
- Shared error, indexing, shape, workspace, and conversion types must remain deterministic and broadly reusable.
- Do not introduce global mutable state unless required, documented, synchronized, and tested.
- Changes to shared public types require a cross-crate compatibility review.

## Required checks

Run focused unit tests, no-default-feature checks, relevant `no_std` checks, workspace check/test, strict Clippy, and rustdoc.
