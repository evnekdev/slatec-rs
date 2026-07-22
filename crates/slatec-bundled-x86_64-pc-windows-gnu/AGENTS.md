# `slatec-bundled-x86_64-pc-windows-gnu` instructions

Scope: `crates/slatec-bundled-x86_64-pc-windows-gnu/**`.

This target-specific carrier exposes no numerical Rust API. It may contain a
generated static archive only after the generated source-level provenance gate
is `ready` and the archive checksum, source-unit
manifest, compiler recipe, symbol reports, runtime audit, and notices are all
present.

- Do not hand-edit `metadata/bundle-manifest.json`; regenerate it with
  `slatec-corpus generate-bundled-provider-evidence`.
- Do not add an archive, object, DLL, executable, compiler log, or source cache
  while the carrier manifest is provenance-blocked. Committed static archives
  must be listed in the ready manifest and have generated source and runtime
  receipts; failed build temporaries belong under `target/`, never here.
- This is a crates.io-publishable carrier. Package the workspace licenses,
  exact GNU runtime licence texts, source/relink instructions, notices, and
  deterministic evidence with every runtime archive.
- Do not add a numerical public API or a provider fallback here.
- Keep this package target-specific and independent from unrelated target
  downloads.
