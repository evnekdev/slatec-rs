# Source reconciliation method

**Task:** E02 — archive, live-tree and relocated-subset reconciliation  
**Evidence run:** 2026-07-16  
**Canonical corpus selected:** no

## Evidence boundary

This revision replaces the earlier archive-unavailable E02 report. The four Netlib archives supplied by the repository owner were inspected locally as unmodified inputs. Their bytes were hashed before extraction. Live Netlib directory bytes and standalone upstream package archives were not supplied, so archive-to-live and archive-to-upstream identity remains unresolved except where direct archive evidence narrows the relationship.

## Inputs and measured identities

| Artifact ID | File | Bytes | SHA-256 | Regular files |
|---|---|---:|---|---:|
| `slatec-source-archive` | `slatec_src.tgz` | 1,768,291 | `4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc` | 741 |
| `slatec-linux-archive` | `slatec4linux.tgz` | 27,754 | `eef9234f8fcb49e7f4905a11eda8f453ec2ca314029a9ce303fdbc99cff42bf3` | 8 |
| `slatec-quick-check-archive` | `slatec_chk.tgz` | 324,049 | `a095f74665e165fa1a4bd3f9ab6a4573135e21b1d002c05607eb9394e1c0f2ca` | 406 |
| `slatec-browser-archive` | `slatecm.tgz` | 26,014 | `1bcaf724d92b50ea2a88604f714e61b1117e007fffc8b6476720aad3132d2b1c` | 16 |

The source archive's measured compressed size differs from the size previously transcribed from the mutable Netlib index. This is recorded as evidence that the download endpoint is not safely characterized by the 1993 release label alone; the archive contains a `changes` file with corrections dated 1994, 1999, and 2023.

## Reproducible procedure

1. Compute SHA-256 over each original archive before extraction.
2. List members with `tar -tzf` and reject absolute paths, `..` traversal, escaping links, devices, or other unsafe members.
3. Extract each archive into a separate source-set-qualified directory.
4. For every regular file calculate raw SHA-256, line-ending-normalized SHA-256, and fixed-form-normalized SHA-256.
5. For fixed-form Fortran, identify `PROGRAM`, `SUBROUTINE`, typed or untyped `FUNCTION`, `BLOCK DATA`, and `ENTRY` declarations. Record every unit with file and line.
6. Compare candidates by exact path, case-folded path, primary program-unit name, and manually reviewed rename evidence.
7. Classify differences as raw-identical, line-ending/whitespace-only, comment/prologue-only, declaration-only, machine configuration, error adaptation, precision conversion, executable-code change, split/combined/renamed unit, or unrelated.
8. Preserve every provider. Never overwrite a same-named unit merely because one source set appears newer.

## Parser and validation scope

The inventory run used a deterministic fixed-form declaration scanner over columns 7–72. It is sufficient to count ordinary one-unit-per-file SLATEC sources and detect duplicate declared names in these archives. It is not yet a full Fortran parser and must not be used to prove semantic equivalence, resolve preprocessor branches, or classify comment-only differences without review.

Measured source-archive results:

- 741 regular files under `src/`;
- 735 `.f` files;
- 735 detected program-unit occurrences and 735 unique program-unit names;
- no duplicate declared unit names within this archive;
- no case-folded filename collisions;
- no detected multi-unit `.f` files;
- six non-`.f` members: `.depend`, `MD5`, `changes`, `index`, `index.html`, and `sgeir.f.0`.

Measured quick-check results:

- 406 regular files, all fixed-form `.f`;
- 54 numbered drivers, `test01.f` through `test54.f`;
- 406 detected program units with no duplicate declared names within the archive.

## Status vocabulary

- `verified-identical`: compared bytes matched.
- `verified-different`: compared bytes or inspected content differed.
- `likely-identical`: strong direct evidence, but bytes were not compared.
- `possible-duplicate`: provider/name overlap without identity proof.
- `alternate-implementation`: intentional platform, machine, precision, or provider alternative.
- `unresolved`: insufficient evidence.

## Remaining evidence limitations

No complete live-directory snapshot was available for `src`, `lin`, `fishfft`, `fnlib`, `pchip`, `chk`, or `err`. No standalone upstream package archive was available. Therefore no source-archive member is claimed byte-identical to a live or upstream file in this revision.
