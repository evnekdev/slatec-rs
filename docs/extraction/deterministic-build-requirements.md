# Deterministic build requirements

## Environment isolation

The extractor must run without network access after artifact acquisition. Locale, timezone, filesystem enumeration, CPU count, and host path must not affect semantic output. Use UTF-8 for metadata serialization while retaining source bytes independently.

## Pinned implementation

Record and pin:

- extractor version or commit;
- dependency lockfile hash;
- compiler/interpreter version used to build the extractor;
- schema bundle hash;
- grammar and prologue-rule hashes;
- configuration hash;
- canonical-corpus and rights-policy hashes.

## Input verification

Before parsing, verify archive SHA-256, member manifest, selected source-file hashes, and absence of unapproved patches. A mismatch is fatal and no prior cache entry may be reused.

For a clean worktree, set `SLATEC_EVIDENCE_CACHE` to the separately acquired
full-corpus `directories` cache used for exact Netlib-link verification. The
smaller `SLATEC_SOURCE_CACHE` provider closure may supplement it, but cannot
replace the full evidence cache. Neither cache path is serialized into
semantic output; every selected file is still accepted only when its recorded
source hash matches.

## Safe archive handling

Reject absolute paths, path traversal, links escaping extraction root, device nodes, duplicate normalized paths, case-fold collisions, and entries exceeding configured resource limits. Preserve original archive path bytes where representable and record decoding decisions.

## Resource limits

Set deterministic limits for archive members, file size, line length, continuation count, nesting depth, token count, parser recovery attempts, and diagnostic count. Exceeding a limit creates a fatal or review-blocking diagnostic according to policy.

## Reproducibility test

CI must execute the same input twice in clean directories and compare all semantic output bytes. A second supported host should reproduce the semantic dataset hash. Audit timestamps and cache-performance logs are compared separately or excluded.

## Canonical JSON

Use one documented implementation and number/string policy. No floating-point values are needed for source positions or hashes; where extracted numeric constants are represented, preserve source spelling and parse result separately.

## Paths

Semantic records use artifact-relative POSIX-style paths. Absolute local paths appear only in nonsemantic audit logs and must be redacted from distributable output.

## Stable errors

Parser errors and warnings use stable rule IDs and structured arguments. Stack traces, memory addresses, thread IDs, and host-specific wording are nonsemantic.

## Security

Treat archives and Fortran as untrusted input. Parsing performs no preprocessing command execution, include-file network access, compiler invocation, or shell expansion. Compiler probes are a separate sandboxed stage with separately reviewed inputs.
