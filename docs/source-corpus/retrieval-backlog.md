# Candidate corpus retrieval backlog

This backlog is ordered to support E02 reconciliation. It does not imply a canonical choice.

## P0 — acquire and freeze bytes

1. Download `slatec_src.tgz`, `slatec4linux.tgz`, `slatec_chk.tgz`, and `slatecm.tgz`; record HTTP status, final URL, content type, byte length, retrieval timestamp, SHA-256, and archive member list.
2. Snapshot the live indexes and every file name under `slatec/src`, `slatec/lin`, `slatec/fishfft`, `slatec/fnlib`, `slatec/pchip`, `slatec/chk`, and `slatec/err`.
3. Retrieve `list`, `tree1`, `tree`, `gams`, `spfun`, `spfunchk`, `install`, `docinstall`, `slprep`, `sladoc`, and `subsid`; calculate checksums and identify formats.
4. Preserve redirect chains and server metadata; do not normalize line endings in the archived inputs.

## P1 — reconcile distribution forms

1. Compare archive members against live directories by normalized path, raw SHA-256, and text-normalized SHA-256.
2. Determine whether relocated subsets are present in `slatec_src.tgz`, absent, newer, older, or modified.
3. Compare `slatec_chk.tgz` with `chk/` and map all 54 drivers to routine families.
4. Diff `slatec4linux.tgz` against the complete archive and classify every modification as build, machine constant, error hook, source patch, test, or documentation.
5. Identify duplicate program-unit names across all candidate source sets.

## P1 — upstream package comparisons

- BLAS, LINPACK, EISPACK and SLAP versus `slatec/lin`.
- FISHPACK and FFTPACK versus `slatec/fishfft`, including upstream `changes`.
- `slatec/fnlib`, `/fn`, and `spfun`.
- `slatec/pchip` and `/pchip`.
- QUADPACK, MINPACK, DASSL, ODEPACK/related ODE families, and AMOS-associated routines versus matching SLATEC program units.

For each comparison, record exact file pairs, raw and normalized hashes, semantic differences, renamed routines, precision conversions, error/machine-support adaptations, and confidence.

## P1 — generated dependency evidence

1. Parse `tree1` and `tree`; infer their syntax without assuming correctness.
2. Select representative roots from each family and save Netlib plus-dependencies responses with request URLs and checksums.
3. Compare all generated products with parsed source calls, prologue `ROUTINES CALLED`, and later object-symbol evidence.
4. Determine whether products include callbacks, COMMON/BLOCK DATA, runtime references, alternate entries, and duplicate providers.

## P2 — rights and notices

- Extract notices from every archive and standalone package.
- Keep access, copyright, licence, attribution, modification and redistribution conclusions separate.
- Do not assign SPDX identifiers by analogy or Netlib co-location.

## P2 — unresolved endpoints

- Retry and inspect `/pchip/` and `/dassl/`, which did not render in this pass.
- Verify the existence and current structure of `/amos/`.
- Locate authoritative change/correction notices for SLATEC itself, beyond package-level `changes` files.
- Determine whether there are official post-4.1 corrections or mirrors with modified files.
