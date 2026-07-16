# Canonical corpus tooling

`slatec-corpus` is the first extraction stage. It acquires, verifies, safely inventories, and extracts the policy-v1 archive, then writes deterministic factual manifests. It does not parse Fortran, compile native code, generate FFI, or make ABI, dependency, licensing, or historical-authority conclusions.

## Install and run

Run the tool through Cargo from the repository root:

```text
cargo run -p slatec-tools --bin slatec-corpus -- prepare --offline
```

The default archive path is `evidence/downloads/slatec_src.tgz`, the local evidence root is `evidence/`, and the committed manifest path is `generated/corpus/`. Each operation accepts the same bounded options:

```text
cargo run -p slatec-tools --bin slatec-corpus -- <acquire|verify|inspect|extract|manifest|prepare> \
  --artifact-path PATH --evidence-dir PATH --output-dir PATH --offline
```

Use `acquire` without `--offline` only when the artifact is absent locally. It downloads to a temporary file, validates the policy-controlled size and SHA-256, and atomically promotes only verified bytes. If a file already exists with the wrong size or digest, the tool fails and leaves it in place for investigation; it never replaces it silently.

`verify`, `inspect`, `extract`, `manifest`, and `prepare` work with a valid local archive and do not need network access. `prepare` runs acquire-or-verify, inspection, raw extraction, and manifest generation in policy order.

## Evidence layout and Git boundary

The local evidence store is intentionally ignored:

```text
evidence/
  downloads/slatec_src.tgz       # downloaded archive, never committed
  acquisition/                   # retrieval audit records, never committed
  extracted/<snapshot>/...       # byte-for-byte raw archive members, never committed
  work/                          # incomplete extraction staging, never committed
```

The optional `.evidence/` root is also ignored for callers that want a separate local store. The committed `generated/corpus/` directory contains only checksums, names, paths, selection facts, diagnostics, and deterministic manifest metadata. It contains no SLATEC source bytes.

## Generated outputs

`prepare` produces:

- `artifact-manifest.json` — pinned artifact and tool/configuration identity;
- `archive-members.json` — every archive member, raw-content hash for regular files, and selection state;
- `source-files.json` — raw source-file inventory with explicit no-parser scope;
- `selected-providers.json` — selected `src/*.f` files and filename-level uniqueness result;
- `diagnostics.json` — stable diagnostics and review items;
- `manifest.json` and `manifest-summary.md` — output hashes, semantic snapshot identity, and a compact projection.

The snapshot ID derives from policy version and semantic hash, archive SHA-256, selection rule, raw-extraction configuration, and the tool semantic version. It excludes paths, timestamps, users, host names, temporary directories, and scheduling. All committed outputs use a fixed audit timestamp so equal inputs produce byte-identical files.

The policy-v1 checks require the exact 1,768,291-byte `slatec_src.tgz` with SHA-256 `4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc`, 741 regular members, and 735 selected `src/*.f` members. The current `sgeir.f`, `qk15w.f`, and `dqk15w.f` must be selected; `sgeir.f.0` and support records are inventoried but excluded.

## Safety and failures

Archive inspection rejects absolute or traversal paths, path-normalization ambiguity, symbolic and hard links, devices, FIFOs and other unsupported entries, duplicate paths, and case-folding collisions. Extraction uses a fresh staging tree, checks every written regular-file hash, then atomically promotes a complete tree. A failed extraction cannot become an authoritative extracted tree. Existing differing manifests are preserved; choose a new output path after investigation rather than overwriting them.

The command reports `success`, `success_with_review_items`, or fails. Policy-v1 preparation intentionally returns `success_with_review_items`: program-unit uniqueness is pending the later fixed-form source scanner. Filename-level provider uniqueness is checked now; this tool does not claim program-unit uniqueness.

## Reproducibility and later stages

Run `prepare --offline` twice against the same archive and policy; the generated semantic files must be identical. CI uses synthetic tar/gzip archives only, exercises the same safety, hashing, extraction, atomicity, offline, and determinism paths, and does not download or redistribute the real archive.

Later parser stages consume `archive-members.json` and `source-files.json` as raw, source-qualified inputs. They must create separate parser views and retain the raw artifact/member hashes and snapshot ID. They may not rewrite the extracted files in place.

## Rights warning

Successful preparation proves reproducibility against the pinned archive only. It does not prove that the archive is the historically original Version 4.1 source, the latest source for every imported package, or redistributable. It also does not prove ABI safety, complete dependency metadata, or valid native crate splitting. See the [rights register](../source-corpus/rights-register.md) and [redistribution-risk assessment](../source-corpus/redistribution-risk.md) before distributing any upstream-derived material.
