# Repository map for coding agents

## Workspace crates

| Area | Owns | Must not own |
|---|---|---|
| `crates/slatec-sys` | Raw declarations, ABI types, canonical raw modules, declaration features | Provider selection, source build, safe validation |
| `crates/slatec-src` | Provider modes, exact native source closures, build/link directives | Public raw API design, safe wrappers |
| `crates/slatec-bundled-x86_64-pc-windows-gnu` | Target-specific bundled-provider carrier metadata and, only after provenance clearance, its verified archive | Numerical Rust APIs, provider fallback, unreviewed native artifacts |
| `crates/slatec-core` | Provider-neutral common types and infrastructure | Concrete provider selection, family-specific FFI |
| `crates/slatec` | Checked safe facade, user-facing numerical types | Source acquisition, generated corpus processing |
| `crates/slatec-tools` | Corpus processing, generation, audit, validation | Production numerical runtime APIs |
| `generated` | Committed deterministic output and evidence | Authored behaviour or hand edits |
| `docs` | Architecture and user-facing explanation | Hidden implementation state not enforced by code/tests |

## Typical change paths

### Add or promote a raw routine

1. Verify source identity, provider symbol, ABI, and dependencies.
2. Update authoritative raw inventory/correction metadata.
3. Generate the canonical mathematical-family declaration.
4. Reconcile `slatec-sys`, provider, and safe-facade features.
5. Add docs, canonical-path compilation, link/runtime tests, and status classification.
6. Regenerate and validate raw inventories.
7. Confirm `all` coverage if a new public family is introduced.

### Add a safe wrapper

1. Start from a reviewed canonical raw declaration.
2. Audit preconditions, workspace, state, callback, and concurrency behaviour.
3. Design checked types and error mapping.
4. Add independent numerical tests and native integration tests.
5. Confirm narrow feature compilation.
6. Confirm the wrapper does not retain unrelated raw/native routines.
7. Generate/update safe API metadata and docs.

### Change a provider/source closure

1. Identify the authored manifest/overlay.
2. Verify every source hash.
3. Update the exact dependency closure.
4. Preserve separate object/archive-member granularity.
5. Run symbol, link, archive, native regression, and deterministic metadata checks.
6. Keep source/system/external guarantees distinct.

### Change generated output

1. Find the generator in `crates/slatec-tools` and its authored inputs.
2. Do not patch output directly.
3. Regenerate, validate, rerun for determinism, and inspect unrelated churn.

### Change feature semantics

Trace the feature through:

```text
slatec public feature
-> slatec-sys declaration feature
-> slatec-src provider/source feature
-> authored registries/manifests
-> generated feature/provider reports
-> compile/link/native probes
-> documentation
```

Do not assume identically named features have identical responsibilities.

## Search order

For implementation questions, search in this order:

1. nearest `AGENTS.md`;
2. architecture index and relevant architecture document;
3. authored crate code and Cargo features;
4. authored generator registries/manifests;
5. generator implementation;
6. compact generated summaries;
7. large per-routine reports/pages only when needed as evidence.

## Evidence hierarchy

Prefer:

1. pristine selected source and verified source hash;
2. compiler/symbol/link/runtime evidence;
3. authoritative authored inventory/registry;
4. generated deterministic report;
5. prose documentation.

When sources disagree, do not guess. Record the discrepancy and fix the authoritative layer.

## Codex guidance

Read [Using Codex](USING-CODEX.md) for how root and subtree `AGENTS.md` files apply to a task, and use the [task checklist](TASK-CHECKLIST.md) before substantial changes.
