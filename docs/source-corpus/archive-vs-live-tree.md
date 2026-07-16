# Main archive versus live Netlib trees

## Measured archive inventory

`slatec_src.tgz` was successfully downloaded and inspected.

- SHA-256: `4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc`
- compressed bytes: 1,768,291
- archive members: 742
- regular files: 741
- top-level directory: `src/`
- Fortran files: 735
- detected unique program-unit names: 735
- duplicate declared names within the archive: none detected

The archive is not a frozen untouched 1993 payload. Its `changes` member records:

- an `rd.f` comment correction dated 16 July 1994;
- an executable conditional correction to `sgeir.f` dated 18 November 1999, with `sgeir.f.0` retained as another member;
- coefficient corrections to `dqk15w.f` and `qk15w.f` dated 14 November 2023.

This directly establishes that the current download is a maintained Netlib snapshot containing post-4.1 corrections. It must be identified by checksum and retrieval date rather than by “Version 4.1” alone.

## Archive versus live directory

| Left | Right | Status | Evidence | Missing comparison |
|---|---|---|---|---|
| `slatec-source-archive` | `slatec-source-directory` | `unresolved` | Archive bytes and complete member inventory now exist; Netlib describes `src/` as the live per-file surface. | Download every live file and compare raw and normalized hashes. |
| `slatec-source-archive` | `slatec-lin-directory` | `possible-duplicate` | Archive contains many BLAS, LINPACK, EISPACK, and SLAP-named units; Netlib says the subset was relocated from `src/`. | Complete `lin/` snapshot and unit pairing. |
| `slatec-source-archive` | `slatec-fishfft-directory` | `possible-duplicate` | Archive contains FISHPACK/FFTPACK-named units; distribution relocation is documented. | Complete `fishfft/` snapshot and hashes. |
| `slatec-source-archive` | `slatec-fnlib-directory` | `possible-duplicate` | Archive contains FNLIB-style special-function units and post-release changes. | Complete `fnlib/` snapshot, `/fn` snapshot, and semantic diff. |
| `slatec-source-archive` | `slatec-pchip-directory` | `possible-duplicate` | Archive contains PCHIP single- and double-precision units. | Complete `pchip/` and standalone `/pchip` snapshots. |

## Quick-check archive versus live checks

`slatec_chk.tgz` is now verified to contain 406 Fortran files, including exactly 54 numbered drivers from `test01.f` through `test54.f`. Its SHA-256 is `a095f74665e165fa1a4bd3f9ab6a4573135e21b1d002c05607eb9394e1c0f2ca`.

The relationship with the live `chk/` directory remains `possible-duplicate`: the archive side is fully inventoried, but live files were not downloaded and compared.

## Linux adaptation

`slatec4linux.tgz` is not a full source distribution. Its own readme instructs the user to obtain and extract `slatec_src.tgz` first, then overlay the Linux archive. It contains eight regular files: three replacement machine-constant sources, three makefiles, `makedoc.sed`, and a readme.

All three overlapping Fortran files are `verified-different` from the source archive:

| Unit | Source archive SHA-256 | Linux SHA-256 | Classification |
|---|---|---|---|
| `D1MACH` | `8ef737d5f74cb1bf083d6ee4506429b6d18edb7067c804386f1eec21a3a5d8db` | `3aeb2c0576006564efd462bd0408bde91bebbb9060a7667746538964d8234794` | executable machine-constant adaptation; calls external `DLAMCH` and caches results |
| `I1MACH` | `f63ea5c0dc7324a3a576ae3640e8a2801e1ba7d2fdad669892ad0f97795f3fba` | `7a9e027ddb98a3a484b980fb9e3f5878684bb36b7723b8475740e016ff4281f6` | executable machine-selection change; IBM-PC constants uncommented |
| `R1MACH` | `b22dd08760e1c3166805469f1652e42588898872117b3fa6ec7279e6b3cb3478` | `08e611a0a1e238e67acc1eca0eadb310a6a1dc61a4584487448aa927bee2df70` | executable machine-constant adaptation; calls external `SLAMCH` and caches results |

The Linux readme states that linking additionally requires LAPACK because of `SLAMCH`/`DLAMCH`. Therefore this archive changes both source providers and external dependency requirements; it cannot be treated as a harmless build-script wrapper.
