# Safe-facade link granularity

## Policy

A safe wrapper must not introduce a reference to an unrelated numerical
routine merely because both wrappers share a family, source file, generated
module, trait, table, or Cargo feature. A call may retain its own checked
wrapper, shared validation/runtime helpers, the provider anchor, and its real
native dependency closure. It must not retain unrelated drivers.

This rule applies to BLAS, special functions, roots, quadrature, nonlinear and
least-squares solvers, ODE/DAE, FFTPACK, FISHPACK, interpolation, linear
algebra, and every future safe family. It is a native-closure policy, not an
absolute executable-size promise: a `POIS3D` closure legitimately includes its
FFT solver support, while `SAXPY` must not pull `SDOT`, `SNRM2`, `SGEMM`,
`DGAMMA`, `FZERO`, or `HWSCRT`.

## Cause and design

On the supported GNU MinGW PE/COFF linker, archive member selection happens
before dormant sections in an extracted Rust object can be garbage-collected.
A monolithic safe wrapper object therefore made every `extern` reference in
that object visible to the native archive extractor. The native Fortran archive
was already correct: one verified original source per object and no partial or
whole-archive link was involved.

Safe BLAS Level 1 now uses one physical Rust module per operation and
precision. The sampled Level 2/3 matrix-vector/matrix-matrix operations use
the same layout; public paths are unchanged through module re-exports. The
release profile uses many codegen units, without LTO, so those operation
modules are independently extractable on the supported measurement profile.
The audit rejects observed module collisions rather than assuming a source-file
split is sufficient.

The reviewed real Airy surface follows the same rule: each of `AI`, `AIE`,
`BI`, `BIE`, `DAI`, `DAIE`, `DBI`, and `DBIE` has an operation-sized private
safe-wrapper module. The public `slatec::special::airy` re-exports remain
unchanged. Its `safe_airy_only` audit probe must retain `DAI` and its genuine
FNLIB support closure, but not the other Airy drivers.

Shared validation, integer conversion, locking, error mapping, character
conversion, and workspace checks remain centralized only when they do not
reference numerical routines. Production code contains no all-routine
function-pointer registry, address-taking table, constructor, or documentation
registry. Such metadata belongs in tooling, tests, generated JSON, or an
explicitly gated diagnostic facility.

## Features and providers

Family features remain the public capability boundary; this policy does not
create one feature per routine. Enabling a family makes declarations and the
provider source closure available but cannot itself root native code. The
`full` safe aggregate and raw `slatec-sys/all` remain no-call clean. The
provider anchor is retained so a real safe operation supplies its selected
native backend; it must not share a codegen object with raw references.

The guarantee is measured for the GNU MinGW source provider. `system` and
`external-backend` users must inspect their supplied archive/linker settings;
they may be monolithic or whole-archive linked outside this repository.

## Evidence and maintenance

Run, with `SLATEC_SOURCE_CACHE` and `SLATEC_GFORTRAN` configured:

```text
cargo run -p slatec-tools -- generate-native-link-audit
cargo run -p slatec-tools -- validate-native-link-audit
```

`generated/native-link/safe-facade-comparison.{json,md}` pairs direct raw and
safe probes. It records executable/section sizes, selected archive members,
native symbols, linker-map evidence, and compact demangled Rust-symbol groups.
The validator enforces required and forbidden native symbols for safe SAXPY,
DDOT, DGEMV, DGEMM, gamma, Airy Ai, roots, HWSCRT, and both no-call aggregates. Raw
maps and unabridged `nm` output remain ignored under `target/native-link/`.

When adding a safe family, add a representative probe or an explicit justified
deferment before adding production raw calls. Use semantic required/forbidden
groups, not complete compiler-sensitive symbol snapshots. Inspect a consumer
binary with `nm -C`, `size -A`, `objdump -p`, and a linker map when changing a
provider, linker, LTO, or profile setting.
