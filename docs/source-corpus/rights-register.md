# Rights, attribution, and provenance register

**Task:** E04 — rights, attribution, and provenance  
**Evidence date:** 2026-07-16  
**Legal status:** risk documentation only; not legal advice

## Governing rule

Rights are recorded per artifact or source family. Netlib hosting, SLATEC directory co-location, government sponsorship, or historical use does not establish one common licence.

## Selected and supporting artifacts

| Record | Artifact or family | Status | Direct evidence | Packaging consequence |
|---|---|---|---|---|
| `RIGHT-001` | Checksum-pinned `slatec_src.tgz` numerical source archive | `no-explicit-license-found` | The archive contains `AAAAAA`, whose notice says the material was prepared as an account of work sponsored by the United States Government and gives warranty/liability and private-rights disclaimers. No explicit permission to copy, modify, redistribute, sublicense, or use commercially was found by archive-wide rights-term search. | Keep archive bytes out of ordinary distribution until review. Metadata, checksums, file names, and factual extraction may be published. |
| `RIGHT-002` | Individual source files in selected corpus | `requires-review` | Files have varied authors, institutions, package provenance, and references. Most contain SLATEC prologues but no file-specific licence. Some originated in separately developed packages. | Do not infer one licence from the archive-level disclaimer. Preserve file-level provenance and notices. |
| `RIGHT-003` | `slatec_chk.tgz` quick-check archive | `no-explicit-license-found` | Archive-wide search found no explicit copyright or redistribution notice. Tests may include authored expected values and package-derived code. | Do not redistribute by default; publish generated test metadata and hashes separately. |
| `RIGHT-004` | `slatec4linux.tgz` Linux overlay | `conflicting` | Its readme states “General Public License” and separately says SLATEC is “Freely distributable,” but includes no GPL version or licence text and cannot license upstream SLATEC files on behalf of their rights holders. The overlay's own modifications may have separate authorship. | Do not label the combined corpus GPL. Treat overlay files as separately authored, unresolved material; exclude by default. |
| `RIGHT-005` | `slatecm.tgz` browser/tool archive | `requires-review` | An explicit Regents of the University of Michigan notice permits copying and internal use and says redistribution outside the institution is allowed, but prohibits commercial marketing, inclusion in commercial products, and commercial derivative works without written authorization. Notice reproduction is required. | Do not include in an open-source or commercial distribution without dedicated review. Never apply its notice to numerical SLATEC source. |
| `RIGHT-006` | SLATEC guide, table of contents, list, GAMS map, installation notes, and dependency products | `no-explicit-license-found` | Publicly accessible documentation and generated metadata were identified, but no common explicit licence was verified. Manuals and expressive descriptions may be copyrighted even when factual fields are reusable. | Link and paraphrase. Avoid wholesale reproduction. Preserve citations and source IDs. |
| `RIGHT-007` | Reference BLAS upstream family | `redistribution-permitted` | The official BLAS page says the reference BLAS is freely available, may be included in commercial software with proper credit, and asks that modified routines be renamed and changes documented. It also states that the software is copyrighted. | Apply only after exact file/revision matching. Record credit and modification/renaming requirements. Do not extend this statement to LINPACK, EISPACK, or all `slatec/lin` files. |
| `RIGHT-008` | LINPACK family | `no-explicit-license-found` | The inspected Netlib package index and existing source register establish provenance but not a blanket licence. The associated users' guide is a copyrighted publication. | Exact source files and notices require review before redistribution. Cite the guide rather than reproducing it. |
| `RIGHT-009` | EISPACK family | `no-explicit-license-found` | Package identity and institutional provenance are documented, but no blanket source licence was verified. Guide volumes are copyrighted publications. | Review exact files; keep book/manual rights separate from source-code rights. |
| `RIGHT-010` | SLAP family | `no-explicit-license-found` | Netlib provides archives and package metadata, but no explicit licence was verified in this pass. | Review archive notices and individual files before packaging. |
| `RIGHT-011` | FFTPACK and FISHPACK families | `no-explicit-license-found` | Official Netlib indexes identify authors and package scope but do not establish a common redistribution licence for the historical source copies. | Review exact upstream and SLATEC files independently. Preserve NCAR/authorship attribution. |
| `RIGHT-012` | FNLIB and standalone `/fn` | `requires-review` | Relative revision/correctness is unresolved, and no common explicit licence was verified. | Rights and provenance must be resolved per selected provider; do not merge copies before review. |
| `RIGHT-013` | PCHIP family | `no-explicit-license-found` | SLATEC and standalone copies are known, but their exact identity and rights text have not been reconciled. | Review the exact selected files and preserve authorship/revision metadata. |
| `RIGHT-014` | QUADPACK family | `requires-review` | Source files are publicly served, while the package monograph is copyrighted. No blanket code licence was verified. | Keep source and book rights separate; cite the monograph, inspect file notices, and avoid reproducing book text. |
| `RIGHT-015` | MINPACK, DASSL, ODEPACK-related, and AMOS families | `requires-review` | These are identifiable upstream candidates with varied authorship and institutional origins. Exact SLATEC lineage and file-level rights remain unresolved. | Resolve exact provider and notices before any family is redistributed or relicensed. |
| `RIGHT-016` | Project-authored metadata, policies, extraction tools, and patches | `requires-review` until project licence is declared | These are new project works rather than historical SLATEC material. Their rights can be set by the project, subject to employer/contributor obligations and without purporting to relicense upstream source. | Add an explicit project licence and contribution policy before publication; keep patch authorship and sign-off records. |

## Government-work interpretation boundary

The `AAAAAA` notice establishes government sponsorship and a disclaimer. It does **not** state that every file is in the public domain, that every author was a federal employee, or that contractor-authored and third-party package material is a United States Government work. Therefore the selected archive is not classified `public-domain-stated` or `government-work-stated` as a whole.

## Evidence distinctions

- **Verified text fact:** the exact notice or rights statement was found in the inspected artifact.
- **Absence finding:** a defined archive-wide search found no explicit licence terms; this is not proof that no rights exist.
- **Interpretation:** packaging risk derived from the evidence.
- **Decision:** project policy pending legal or institutional review.

## Sources

- Selected archive `slatec_src.tgz`, SHA-256 `4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc`, especially `src/aaaaaa.f`.
- Quick-check archive `slatec_chk.tgz`, SHA-256 `a095f74665e165fa1a4bd3f9ab6a4573135e21b1d002c05607eb9394e1c0f2ca`.
- Linux overlay `slatec4linux.tgz`, SHA-256 `eef9234f8fcb49e7f4905a11eda8f453ec2ca314029a9ce303fdbc99cff42bf3`, especially `readme`.
- Browser archive `slatecm.tgz`, SHA-256 `1bcaf724d92b50ea2a88604f714e61b1117e007fffc8b6476720aad3132d2b1c`, especially `slatecm/COPYRIGHT`.
- Netlib SLATEC distribution index.
- Official reference BLAS licensing statement.
