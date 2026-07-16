# Full SLATEC corpus completeness audit

`main-src` is the immutable, checksum-pinned `src/*.f` subset used by the
existing corpus, scanner, and prologue outputs. It contains 735 selected files
and 735 top-level program units. It must never be described as the complete
SLATEC numerical library merely because it is reproducible.

Run the separate audit once online to acquire SLATEC-hosted comparison evidence:

```text
cargo run -p slatec-tools --bin slatec-corpus -- audit-full-corpus \
  --output-dir generated/full-corpus
```

Thereafter, use the cached evidence without network access:

```text
cargo run -p slatec-tools --bin slatec-corpus -- audit-full-corpus --offline \
  --output-dir generated/full-corpus-recheck
```

The audit retrieves and hashes the official `list` and Version 4.1 `toc`, then
retrieves the SLATEC-hosted live `src`, `lin`, `fishfft`, `fnlib`, and `pchip`
directories. It also investigates the legacy `err` directory and the `spfun`
candidate. All downloaded bytes, directory indexes, and acquisition metadata
remain in ignored `evidence/full-corpus/`.

Committed output is limited to compact indexes:

- `artifact-index.json` records URLs, hashes, byte counts, and scope;
- `source-file-index.json` records paths, content hashes, and scanner counts;
- `program-unit-union.json` groups declared identities without source text;
- `provider-relationships.json` classifies duplicate, relocation, historical,
  and alternate providers without selecting one;
- `catalogue-comparison.json` records only factual identity/count relationships;
- `diagnostics.json`, `manifest.json`, and `audit-summary.md` record coverage,
  hashes, and unresolved review work.

The output classifications are provenance facts, not selection rules. A
byte-identical relocated copy remains a non-selected location, and a modified
copy remains unresolved until an explicit provider policy approves it. The
audit does not download, merge, or substitute standalone BLAS, LINPACK,
EISPACK, FFTPACK, FISHPACK, FN, PCHIP, or SLAP packages.

`normalized-identical` currently means identical after the reversible
comparison-only normalization of CRLF or CR line endings to LF. It does not
decode, reformat, patch, remove comments, or change the preserved raw bytes.

The Netlib distribution index describes Version 4.1 as containing more than
1,400 routines and identifies the relocated subset directories. The audit
compares its source union with those catalogues and with the documented 902
user-callable-routine figure; any unmatched identities remain visible as
catalogue-only or source-only records. It does not prove historical originality,
rights, ABI safety, dependency completeness, or build suitability.
