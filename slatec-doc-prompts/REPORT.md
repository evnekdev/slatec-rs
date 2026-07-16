# SLATEC-RS Documentation Programme

## Consolidated Report for Tasks 01–10

## 1. Executive summary

Tasks 01–10 established a substantial documentation and metadata foundation for `slatec-rs`. The work now covers:

* authoritative source registration and citation policy;
* SLATEC history and institutional context;
* Fortran source organization and calling conventions;
* GAMS taxonomy and historical package provenance;
* surveys of all major numerical domains;
* a machine-readable routine metadata schema and 20-routine pilot;
* a dependency-extraction and linker-validation specification;
* a proposed incremental Rust crate and FFI architecture;
* a final documentation audit and collection-stage index.

The programme has successfully answered the architectural question of **how the project should proceed**. It has not yet answered, at production quality, the routine-level questions of:

* exactly which source revision is authoritative;
* exactly which routines belong to each imported package;
* exactly what every routine calls;
* which symbols and state are shared;
* which interfaces are ABI-safe on each platform;
* which native components can genuinely be separated.

The recommended next phase is therefore **bulk evidence extraction and validation**, not immediate implementation of all Rust bindings.

---

# 2. Task-by-task report

## Task 01 — Source register and evidence policy

### Main outputs

Task 01 established:

* the authoritative source hierarchy;
* a source register in Markdown and TOML;
* citation and evidence-status conventions;
* terminology for facts, interpretations, inferences and project proposals;
* explicit warnings against assuming a uniform licence merely because material is hosted by Netlib.

The registered source base includes:

* the Netlib SLATEC index;
* the SLATEC guide;
* the Version 4.1 table of contents;
* the source archive and source directories;
* routine lists;
* Netlib dependency resources;
* quick-check archives;
* GAMS material;
* package-specific BLAS, LINPACK, EISPACK and related indexes.

### Unresolved discrepancies

1. **Canonical source snapshot remains unresolved.**
   It is not yet established whether the definitive project baseline should be:

   * `slatec_src.tgz`;
   * the live `slatec/src` tree;
   * the reorganized package subsets;
   * or a reconciled combination.

2. **Several registered sources were located but not fully ingested.**
   In particular:

   * `tree`;
   * `tree1`;
   * the SLATEC routine list;
   * the GAMS map;
   * some package-specific documentation.

3. **Rights remain artifact-specific.**
   The presence of SLATEC and imported packages on Netlib does not establish one common modern licence.

4. **Some source IDs represent indexes rather than exact evidence artifacts.**
   Index-level citations are useful for orientation but insufficient for detailed routine claims.

### Recommendations

* Download and checksum every primary source artifact.
* Create one canonical source manifest containing:

  * URL;
  * retrieval date;
  * checksum;
  * extracted file list;
  * source version;
  * rights status.
* Preserve the original archive unmodified.
* Store all project patches separately with reproducible patch IDs.
* Add automated source-ID validation across all metadata.

---

## Task 02 — History and project overview

### Main outputs

Task 02 documented:

* SLATEC’s institutional and cooperative origins;
* participating organisations;
* the role of the SLATEC Common Mathematical Library Subcommittee;
* known version and release information;
* the broad purpose and scope of the library;
* the distinction between the historical project and the proposed `slatec-rs` ecosystem.

### Unresolved discrepancies

1. **Release chronology is not yet fully authoritative.**
   Some dates are supported by release summaries or the guide, but a complete sequence of releases and corrections has not been reconstructed.

2. **Institutional participation and routine authorship can be conflated.**
   An institution contributing to SLATEC governance does not imply ownership of every routine associated with its staff.

3. **Post-Version-4.1 activity remains unclear.**
   Possible corrections, mirrors, package updates or independent modifications after the documented 4.1 release have not been comprehensively identified.

4. **Historical descriptions sometimes reflect different time periods.**
   The guide, Netlib index and individual routine prologues may describe the project at different stages.

### Recommendations

* Build an evidence-backed chronology table with:

  * date;
  * event;
  * primary source;
  * confidence;
  * affected source snapshot.
* Separate:

  * institutional governance;
  * package provenance;
  * routine authorship;
  * later Netlib preservation activity.
* Compare release archives and change records mechanically where available.
* Avoid unsupported claims that Version 4.1 was the final technical change unless proven.

---

## Task 03 — Library architecture and Fortran conventions

### Main outputs

Task 03 documented:

* fixed-form Fortran organization;
* routine prologue structure;
* user-callable versus subsidiary routines;
* argument-ordering conventions;
* argument intent markers;
* naming and precision conventions;
* machine constants;
* error handling;
* source organization;
* direct and transitive dependency concepts;
* implications for static linking and FFI.

### Unresolved discrepancies

1. **Not every routine uses the final prologue dialect.**
   Older QUADPACK and other incorporated routines use legacy formats.

2. **Documented argument intent may be absent or stale.**
   Prologue annotations cannot be accepted blindly as proof of safe Rust mutability.

3. **`ROUTINES CALLED` may not match executable code.**
   It may omit:

   * compiler runtime calls;
   * intrinsics;
   * callbacks;
   * stale or conditional dependencies;
   * common-block and block-data relationships.

4. **Machine constants may have multiple implementation variants.**

5. **Error handling is potentially global and stateful.**
   The exact interaction among `XERROR`, `XERMSG`, compatibility routines and fatal hooks remains unresolved.

6. **Complex, logical and character ABI details are compiler-specific.**

### Recommendations

* Implement a dialect-aware prologue parser.
* Preserve unrecognised fields rather than discarding them.
* Cross-check argument intent against executable source.
* Inventory:

  * `COMMON`;
  * `SAVE`;
  * `BLOCK DATA`;
  * `ENTRY`;
  * alternate returns;
  * character arguments;
  * complex function returns.
* Build compiler-specific ABI probes before generating production bindings.
* Treat all routines as not thread-safe until proven otherwise.

---

## Task 04 — Taxonomy and package provenance

### Main outputs

Task 04 established:

* an overview of GAMS classification;
* a proposed GAMS-to-project-domain map;
* a distinction between mathematical classification and package provenance;
* preliminary metadata for mathematical domains;
* preliminary metadata for imported or incorporated packages;
* analysis of Netlib’s reorganized subsets:

  * `slatec/lin`;
  * `slatec/fishfft`;
  * `slatec/fnlib`;
  * `slatec/pchip`.

### Unresolved discrepancies

1. **GAMS does not define package ownership.**
   A routine’s mathematical category cannot determine its raw crate.

2. **Netlib directory co-location does not imply one package.**
   For example:

   * FFTPACK and FISHPACK share a directory;
   * BLAS, LINPACK, EISPACK and SLAP share another.

3. **Exact package revisions are not established.**

4. **The relationship between FNLIB and Netlib `/fn` is explicitly unresolved.**

5. **Many routines have ambiguous family provenance.**
   This includes:

   * B-spline families;
   * constrained least-squares routines;
   * ODE/BVP families;
   * polynomial root solvers;
   * Amos-associated special functions.

6. **Package membership and functional domain overlap substantially.**

### Recommendations

* Compare every candidate imported package against its standalone Netlib source using checksums and source diffs.
* Store package provenance separately from:

  * GAMS classification;
  * safe Rust domain;
  * native component ownership.
* Allow confidence levels:

  * verified;
  * high;
  * medium;
  * low;
  * conflicting.
* Do not create package-specific native crates until exact membership and symbol ownership are known.

---

## Task 05 — Domain surveys A

### Domains covered

* dense and structured linear algebra;
* nonlinear equations and roots;
* optimization and nonlinear least squares;
* sparse linear algebra and iterative methods.

### Main findings

Task 05 showed that:

* linear algebra spans BLAS, LINPACK, EISPACK and SLATEC-specific drivers;
* nonlinear systems and nonlinear least squares share MINPACK-derived infrastructure;
* optimization includes several unrelated historical families;
* SLAP is a substantial sparse iterative package with storage, solver and callback conventions;
* safe API boundaries should follow mathematical problems rather than source package directories.

### Unresolved discrepancies

1. **Exact imported revisions remain unknown.**

2. **LINPACK/EISPACK symbols may collide with system BLAS/LAPACK installations.**

3. **Complex Fortran ABI remains unverified.**

4. **SLAP storage invariants are not fully extracted.**

5. **Callback reentrancy and thread safety remain unknown.**

6. **The precise provenance of constrained least-squares and sparse LP routines remains incomplete.**

7. **Routine family tables are representative, not exhaustive.**

### Recommendations

* Extract exact storage formulas and mutation rules from every source prologue.
* Build typed Rust storage abstractions for:

  * band matrices;
  * packed symmetric matrices;
  * Hermitian matrices;
  * sparse SLAP formats.
* Preserve solver-specific statuses rather than introducing one generic error.
* Use one shared callback implementation layer for MINPACK-derived routines only after signature compatibility is demonstrated.
* Keep raw provenance boundaries and safe domain boundaries separate.

---

## Task 06 — Domain surveys B

### Domains covered

* quadrature and cubature;
* ODEs, DAEs, BVPs and related PDE support;
* interpolation and approximation;
* Fourier and related transforms;
* special functions;
* probability and statistics;
* general utilities.

### Main findings

Task 06 showed that:

* QUADPACK is callback- and workspace-driven;
* DASSL and several ODE families require persistent continuation state;
* PCHIP cleanly separates construction, evaluation and integration;
* FFTPACK initialization arrays behave like reusable transform plans;
* special functions comprise multiple historical families, not one package;
* statistics is relatively small and strongly overlaps with fitting and special functions;
* machine constants and error handling form a shared runtime layer.

### Unresolved discrepancies

1. **General multidimensional cubature coverage is unclear.**

2. **ODE, DEPAC, DRIVE, BVP and integral-equation family boundaries remain incomplete.**

3. **FFTPACK precision coverage is not fully established.**

4. **FFTPACK workspace mutability and thread safety are unknown.**

5. **FNLIB versus `/fn` remains unresolved.**

6. **Historical random generators have not been analysed for:**

   * algorithm;
   * period;
   * seeding;
   * hidden state;
   * reproducibility;
   * thread safety.

7. **Special-function branch conventions and accuracy guarantees need family-specific validation.**

### Recommendations

* Model ODE/DAE wrappers as stateful session objects.
* Model FFTPACK wrappers as plan objects.
* Keep PCHIP, B-spline and piecewise-polynomial representations distinct.
* Avoid a standalone statistics crate until routine inventory demonstrates sufficient cohesion.
* Treat historical RNGs as compatibility functionality, not modern recommended generators.
* Extract and test scaled versus unscaled special-function behavior explicitly.

---

## Task 07 — Routine metadata schema and pilot catalogue

### Main outputs

Task 07 produced:

* a routine metadata schema;
* a parseable TOML example;
* a 20-routine pilot catalogue;
* one Markdown page per pilot routine;
* YAML front matter aligned with the metadata model.

The pilot spans:

* error handling;
* machine constants;
* BLAS/LINPACK;
* quadrature;
* interpolation;
* roots and nonlinear systems;
* nonlinear least squares;
* DAEs;
* special functions;
* FFTPACK;
* SLAP.

### Unresolved discrepancies

1. **Several official source files were located but not fully rendered during extraction.**

2. **Some pilot fields remain `unverified` or `inferred`.**

3. **The pilot was manually assembled rather than generated by the future production parser.**

4. **YAML/TOML parity has not yet been automatically validated.**

5. **Direct dependencies have not yet been reconciled with compiled object symbols.**

6. **The schema is parseable but lacks a formal machine-validation layer such as JSON Schema, CUE or Taplo schema.**

7. **Evidence-status vocabularies differ from source retrieval-status and package-confidence vocabularies.**

### Recommendations

* Re-run all 20 pilot routines through the production parser.
* Add automated validation for:

  * unique IDs;
  * valid domains;
  * valid package IDs;
  * valid source IDs;
  * argument-name uniqueness;
  * YAML/TOML parity.
* Add field-level source locators:

  * prologue section;
  * source lines;
  * source checksum.
* Never generate a safe wrapper from inferred argument intent alone.
* Preserve raw unparsed prologue fields.

---

## Task 08 — Dependency extraction and validation specification

### Main outputs

Task 08 defined:

* typed dependency graph nodes and edges;
* direct and transitive dependency outputs;
* unresolved-reference reporting;
* strongly connected components;
* duplicate-symbol reporting;
* source and native ownership mapping;
* validation through object files and linker behaviour;
* a worked `DQAG` example;
* a native build strategy;
* proposed domain-boundary rules.

### Unresolved discrepancies

1. **Netlib `tree` and `tree1` contents and snapshots remain unverified.**

2. **The worked `DQAG` example verifies `DQAG → DQAGE` and `DQAG → XERROR`, but deeper `DQAGE` closure remained partly provisional.**

3. **Source-call parsing may disagree with prologues or Netlib graphs.**

4. **Conditional calls and alternate implementations need representation.**

5. **Object-file and linker evidence has not yet been generated.**

6. **`BLOCK DATA` retention and common-block compatibility are unresolved.**

7. **Cargo ergonomics for one native `links` owner serving several logical raw crates remain untested.**

### Recommendations

* Keep all evidence sources separate:

  * prologue;
  * parsed source;
  * Netlib graph;
  * object undefined symbols;
  * linker result.
* Never overwrite one source with another during reconciliation.
* Collapse strongly connected components before assigning native ownership.
* Compile one source file per object where practical.
* Generate:

  * defined symbols;
  * undefined symbols;
  * common symbols;
  * runtime dependencies;
  * component closures.
* Validate every proposed component through an isolated minimal link test.

---

## Task 09 — Future Rust architecture

### Main outputs

Task 09 proposed a staged architecture:

1. one monolithic `slatec-sys` native owner;
2. logical internal modules;
3. generated native component manifests;
4. multiple archives under one owner;
5. focused raw re-export crates;
6. independent native `*-sys` crates only where proven safe;
7. domain-oriented safe crates;
8. a final `slatec` facade.

It also documented:

* FFI type policy;
* array and workspace handling;
* callback safety;
* global-state handling;
* BLAS/LAPACK provider policy;
* versioning and publication order;
* an incremental migration roadmap.

### Unresolved discrepancies

1. **The bootstrap owner is described with slightly varying names.**

   * `slatec-sys`;
   * `slatec-native-sys`;
   * one native core.

2. **The permanent role of the monolithic owner is unresolved.**

3. **Cargo feature unification complicates mutually exclusive BLAS providers.**

4. **The best callback context mechanism remains unresolved.**

5. **The minimum compiler and platform matrix remains undefined.**

6. **Whether prebuilt native libraries should be distributed remains open.**

7. **The ideal safe crate count remains provisional.**

8. **No Cargo prototype has yet validated the architecture.**

### Recommendations

* Standardize the initial native owner name as `slatec-sys`.
* Let `slatec-sys` be the sole `links` owner initially.
* Treat domain-specific `*-sys` crates as re-export/namespace crates before allowing independent native builds.
* Use bundled reference BLAS as the reproducibility baseline.
* Add system BLAS later through an explicit provider mechanism.
* Never silently replace LINPACK/EISPACK with LAPACK.
* Begin safe wrappers with low-risk areas:

  * PCHIP;
  * selected special functions;
  * possibly scalar roots.
* Delay:

  * DAE;
  * SLAP callbacks;
  * sparse LP;
  * complex special functions;
    until callback and ABI infrastructure is validated.

---

## Task 10 — Final documentation audit

### Main outputs

Task 10 produced:

* the collection index;
* collection readiness status;
* a consolidated open-research register;
* the final audit report;
* machine-readable coverage metadata.

### Audit conclusion

The documentation stage is **complete with conditions**.

It is ready to proceed to:

* source pinning;
* bulk parsing;
* metadata generation;
* dependency extraction;
* compilation and symbol validation.

It is not ready for:

* comprehensive generated FFI;
* native crate splitting;
* thread-safety claims;
* uniform licensing claims;
* automatic safe wrappers;
* cross-compiler compatibility promises.

### Major audit discrepancies

1. **No executable full-repository Markdown link check has yet been run.**

2. **File counts are reconstructed from prompt manifests, not a canonical repository tree manifest.**

3. **Source-ID references have not been comprehensively linted.**

4. **Evidence and confidence vocabularies require central documentation.**

5. **Some family tables are more confident than current package evidence permits.**

6. **Historical dates and quantities require generated verification.**

7. **Routine metadata and documentation can drift until generated from one source.**

8. **The current documentation has high breadth but variable verification depth.**

### Recommendations

* Add CI checks for:

  * TOML parsing;
  * schema validation;
  * YAML/TOML parity;
  * Markdown links and fragments;
  * source-ID existence;
  * domain/package ID existence;
  * duplicate IDs;
  * generated-file reproducibility.
* Generate indexes and family tables from metadata.
* Keep manually authored interpretation separate from generated facts.
* Add audit/coverage regeneration to every major metadata update.

---

# 3. Cross-cutting unresolved discrepancies

## 3.1 Canonical source identity

The most important unresolved issue is that the exact source corpus has not been frozen.

The project must determine:

* which archive is canonical;
* which relocated subsets supersede or duplicate archive files;
* which package copies are newer;
* which machine-specific alternatives are selected;
* which corrections are included.

Until this is resolved, all package membership, dependency and symbol conclusions remain provisional.

## 3.2 Historical package revision matching

The documentation identifies likely provenance for major packages, but not exact revisions.

This affects:

* reproducibility;
* attribution;
* source selection;
* bug history;
* numerical comparison;
* crate versioning.

Source-level comparisons are required for every imported package.

## 3.3 Dependency evidence disagreement

Potential dependency sources include:

* prologue `ROUTINES CALLED`;
* parsed executable source;
* `EXTERNAL` forwarding;
* Netlib `tree1`;
* Netlib `tree`;
* plus-dependency bundles;
* object-file undefined symbols;
* actual linker behaviour.

These may disagree. The project must preserve each evidence type and report conflicts.

## 3.4 Shared global state

The following may introduce shared state:

* error subsystem;
* machine constants;
* `COMMON`;
* mutable `SAVE`;
* block data;
* callback registries;
* FFT initialization workspaces;
* random-number generators.

Thread safety cannot be claimed until these are inventoried and tested.

## 3.5 ABI uncertainty

Unresolved ABI areas include:

* symbol mangling;
* default integer width;
* `LOGICAL`;
* complex function returns;
* character hidden-length arguments;
* callback calling convention;
* common-block layout;
* Fortran runtime linkage.

A successful build on one compiler does not prove portability.

## 3.6 Native and safe crate boundaries

Historical package, mathematical domain, native link component and safe Rust crate are four different classification systems.

The project should resist attempts to force them into one hierarchy.

## 3.7 Evidence vocabulary

The documentation currently uses several related vocabularies:

* `verified`, `derived`, `inferred`, `unverified`, `conflicting`;
* source retrieval statuses;
* package confidence values;
* collection coverage levels.

These should remain separate but be described in one central evidence model.

---

# 4. Recommended next programme of work

## Phase 1 — Freeze the source corpus

Priority: critical.

Deliver:

* downloaded canonical archive;
* relocated subset downloads;
* SHA-256 checksums;
* extracted file manifests;
* duplicate-file report;
* selected baseline policy;
* rights register.

## Phase 2 — Build the extraction toolchain

Use Codex High to implement deterministic tools for:

* fixed-form Fortran program-unit parsing;
* prologue dialect detection;
* argument extraction;
* callback extraction;
* call/reference extraction;
* `COMMON`, `SAVE`, `ENTRY` and `BLOCK DATA` extraction;
* source and symbol identity generation;
* YAML/Markdown generation;
* schema validation.

## Phase 3 — Generate the complete routine catalogue

For every routine, generate:

* canonical ID;
* source file and checksum;
* kind and signature;
* GAMS classification;
* public/subsidiary status;
* package provenance;
* arguments and evidence status;
* callbacks;
* workspace;
* errors;
* direct dependencies;
* related routines;
* authorship and revision;
* unresolved ABI issues.

All manually written pilot records should then be replaced or reconciled with generated records.

## Phase 4 — Generate and validate the dependency graph

Produce:

* direct graph;
* transitive closure;
* SCCs;
* unresolved references;
* common-state graph;
* duplicate symbols;
* alternate implementations;
* source-file ownership;
* candidate native components.

Then compile sources and compare predicted references with object symbols.

## Phase 5 — Prototype monolithic `slatec-sys`

Start with:

* one GNU Fortran toolchain;
* one primary platform;
* one selected source set;
* one native library;
* one `links` owner;
* generated raw declarations;
* ABI probes;
* machine/error support tests;
* selected quick checks.

Do not split native crates during this phase.

## Phase 6 — Implement one low-risk safe pilot

Preferred first candidates:

1. PCHIP;
2. a small set of real special functions;
3. simple BLAS Level 1 routines;
4. bracketed scalar roots after callback infrastructure.

Avoid beginning with DASSL, SLAP or sparse LP.

## Phase 7 — Callback and concurrency infrastructure

Implement:

* per-signature trampolines;
* panic containment;
* scoped callback context;
* callback error transport;
* cancellation;
* nested-call tests;
* concurrent-call tests;
* error subsystem locking or isolation.

## Phase 8 — Validate native component boundaries

Only after complete graph and linker validation should the project consider:

* separate runtime archive;
* BLAS component;
* QUADPACK component;
* PCHIP component;
* FFTPACK component;
* SLAP component;
* special-function components.

## Phase 9 — Publish safe crates and facade

Suggested publication sequence:

1. `slatec-sys`;
2. `slatec-interpolation`;
3. selected `slatec-special`;
4. `slatec-roots`;
5. `slatec-quadrature`;
6. `slatec-transforms`;
7. `slatec-nonlinear`;
8. `slatec-linalg`;
9. `slatec-sparse`;
10. `slatec-diffeq`;
11. `slatec` facade.

---

# 5. Final recommendation

Tasks 01–10 have successfully completed the **knowledge-collection and architecture-design stage**.

The project should now stop expanding manually authored surveys and move to a reproducible evidence pipeline.

The highest-value next action is:

> Pin the complete SLATEC source corpus and use Codex High to implement deterministic source, routine, dependency and symbol extraction.

The preferred implementation strategy remains:

> Start with one monolithic `slatec-sys` native owner, build metadata-driven logical modules and safe APIs above it, and split native ownership only after source, dependency and linker evidence demonstrates that the split is valid.

This strategy minimizes the risks of:

* duplicate symbols;
* incompatible package copies;
* missing dependency closures;
* fragmented global error state;
* Cargo dependency cycles;
* premature public API commitments;
* incorrect claims of thread or ABI safety.
