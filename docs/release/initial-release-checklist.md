# Initial release checklist

This checklist governs the coordinated `0.1.0` release candidate. It does not
authorize publishing or tagging. Record exact command output and the selected
release commit before checking an item.

The package-first independent evidence, target matrix, publication scripts,
and post-publication operations are in
[`0.1.0-publication-guide.md`](0.1.0-publication-guide.md). Its manual release
gates remain mandatory even when this checklist has no local failure.

## Public API and documentation

- [ ] Canonical mathematical namespaces are finalized.
- [ ] All 821 canonical raw routines resolve to exactly one authoritative extern declaration.
- [ ] Compatibility aliases are documented, deprecated where appropriate, and do not inflate routine counts.
- [ ] Public terminology contains no implementation-milestone labels.
- [ ] Documentation quality thresholds pass for every public routine.
- [ ] Every public argument has a structured row; unavailable semantics remain explicit.
- [ ] Return, callback, ABI, native-symbol, feature, provider, and Safety sections pass audit.
- [ ] Catalogue-to-export cross-check reports zero inconsistencies.
- [ ] Family and alphabetical indexes account for all 1,517 retained identities exactly once.

## Providers, native behavior, and licensing

- [ ] `slatec-sys` remains declaration-only and provider-neutral.
- [ ] `slatec-src` is the only owner of Cargo `links = "slatec"`.
- [ ] Source, system, and external provider modes are coherent; incompatible providers are rejected.
- [ ] GNU MinGW native link and representative runtime suites pass on `x86_64-pc-windows-gnu`.
- [ ] The XERROR child-process containment contract passes for documented GNU outcomes.
- [ ] Archive and safe-wrapper link granularity regressions pass.
- [ ] Workspace MIT/Apache licensing metadata and generated-code provenance are consistent.
- [ ] No unresolved-rights SLATEC source cache or bundled binary is included in a package.

## Package chain

- [ ] Coordinated version is finalized once; current candidate is `0.1.0`.
- [ ] Every publishable manifest has complete crates.io metadata and version-plus-path workspace dependencies.
- [ ] `validate-package-contents` passes for both target carriers, `slatec-sys`, `slatec-core`, `slatec-src`, and `slatec`.
- [ ] `cargo package` succeeds in publication-layer order.
- [ ] Registry-only downstream simulation succeeds without workspace path dependencies.
- [ ] Representative no-default, declaration-only, provider, narrow-safe-family, canonical-path, and compatibility-path features build.
- [ ] `cargo publish --dry-run` results and unpublished-dependency blockers are recorded honestly.

## Reproducibility and release operations

- [ ] Portable `release-check` passes without publication credentials.
- [ ] Same-root repeat generation is byte-identical.
- [ ] Clean-root generation has the same semantic hashes and expected bytes.
- [ ] Transactional release-readiness freshness passes without mutating the tree.
- [ ] `generator-drift-analysis.json` reports zero current differences and zero ownerless outputs; its historical partition has been reviewed before accepting any regenerated evidence.
- [ ] EOL audit and `git diff --check` pass after `git add --renormalize`.
- [ ] CI is green, including package, documentation/export, taxonomy, terminology, and documentation-quality jobs.
- [ ] Changelog is reviewed and the release commit is selected.
- [ ] crates.io ownership for all six publishable crates is confirmed.
- [ ] Publication order is confirmed: Windows carrier, Linux carrier, `slatec-sys`, `slatec-core`, `slatec-src`, then `slatec`.
- [ ] Tag name and signing policy are planned; no tag exists before successful publication.
- [ ] Rollback and yank policy is understood; a yanked crate version cannot be reused.

The generated publication graph and package-content evidence are under
[`generated/release-readiness`](../../generated/release-readiness/).
