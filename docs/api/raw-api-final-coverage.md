# Final raw API coverage and disposition

Batch D completes the corpus-wide disposition audit for the unsafe
`slatec-sys` interface. Complete means that every retained identity has one
evidence-backed terminal disposition. It does not mean that every retained
identity is a public Rust function or that every raw function has a safe
wrapper.

The authoritative records are
[`final-disposition.json`](../../generated/raw-api/final-disposition.json),
with the compact counts in
[`final-disposition-summary.md`](../../generated/raw-api/final-disposition-summary.md).
The generator joins the existing catalogue, program-unit, ABI, callback,
complex/character/logical, provider, symbol, public-role, path, feature, test,
and source-hash evidence. It does not create a parallel inventory.

## Final coverage

The retained corpus has 1,517 identities. Of these, 812 have one canonical
public raw path: 776 preserved from the reviewed and Batch A-C tiers and 36
pre-existing family declarations requalified by Batch D. Those 812 identities
also have 280 compatibility re-export paths. Compatibility names never create
another `extern` declaration.

The non-public remainder is classified as 530 provider subsidiaries, 79 raw
internal units, 2 runtime support units, 17 error support units, 5 machine
support units, 2 demonstration programs, 26 catalogue-only identities, 13
missing symbols, 30 unsupported callback interfaces, and one other permanent
exclusion. The selected retained identities contain no callable `BLOCK DATA`,
`ENTRY`, or alternate-return unit. They also contain no unresolved long-string,
complex, COMMON-data, Fortran-I/O, or compiler-specific ABI after public-role
classification. Zero identities remain unexplained.

`public-api-coverage.json` lists canonical paths, compatibility paths, raw
features, and provider features. `permanent-exclusions.json` supplies the
evidence and reopen condition for every permanent decision;
`provider-gaps.json` separates selected-source symbol gaps; and
`legacy-interface-audit.json` records COMMON relationships, fixed-form I/O,
program units, and the dedicated BVSUP/DBVSUP audit.

## Batch A-D policy

- Batch A covers 459 mechanically eligible non-callback numerical interfaces.
- Batch B covers 47 callback-bearing interfaces with reconstructed outer and
  callback ABIs.
- Batch C covers 97 complex numerical and simple `CHARACTER*1`/`LOGICAL`
  interfaces with compiler-profile probes.
- Batch D source-hash-requalifies 36 already-declared family drivers exercised
  by existing safe-wrapper and native-family regressions, then assigns a
  terminal disposition to every remaining identity.

Batch D does not infer semantics from names, create safe closures, add source
providers, translate Fortran, or introduce duplicate declarations. No new shim
was justified. A shim remains permissible only for a genuinely public and
valuable interface whose direct ABI is impossible, when the transformation is
semantic-preserving, deterministic, provider-specific, documented, and tested
for equivalence. Shims may not implement safe ownership, validation, workspace
allocation, closure support, or a numerical rewrite.

## Exceptional and legacy interfaces

Long or variable character units in the retained corpus are error, support,
sorting, or tooling infrastructure rather than public numerical entry points.
They remain non-public; no filename, formatted-text buffer, or compiler string
descriptor is guessed. The fixed-form audit finds no retained `ENTRY` or
alternate-return identity. Ordinary callable routines that internally use
COMMON remain callable where already proven, but no COMMON object is exported
as Rust data and no provider `BLOCK DATA` unit is declared as a function.

Twenty-eight retained routines contain direct Fortran I/O statements. The
audit distinguishes public numerical routines whose documented implementation
uses the runtime from report, paging, installation, and support routines. No
new I/O-bearing API is promoted, and no interactive or unit-oriented wrapper is
added.

BVSUP and DBVSUP are permanent `unsupported-callback-abi` exclusions. Their
objects depend on FMAT/GVEC/UIVP/UVEC problem-definition procedures (and the
double-precision equivalents) that are not explicit procedure arguments. The
project cannot provide a faithful direct Rust callback ABI or user-data channel,
and provider linkage is therefore not reproducible for an ordinary caller.

## Provider and validation boundary

`slatec-sys` remains declarations-only and provider-neutral. Every public
identity records a declaration feature and a usable `slatec-src` provider
feature, while callers may instead supply a compatible external provider.
`slatec-sys/all` continues to include every public mathematical family without
selecting a backend.

Compile validation checks each new Batch D canonical path. Native link probes
reference all 36 requalified symbols through the existing exact family source
closures. Their runtime and numerical evidence is the already-maintained safe
family regression coverage; Batch D adds no numerical operation. Permanent
exclusions explicitly report non-applicable validation levels rather than
claiming tests that did not run.

The supported native ABI remains the observed GNU Fortran x86_64 MinGW profile.
Changes to a selected source hash, emitted symbol, provider closure, callback
call shape, compiler ABI, or public-role evidence can reopen a decision. They
must do so by updating authored evidence and regenerating the reports; an
exclusion never silently becomes public.

## Regeneration

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-api-inventory --offline
cargo run -p slatec-tools --bin slatec-corpus -- generate-final-raw-api-disposition
cargo run -p slatec-tools --bin slatec-corpus -- validate-final-raw-api-disposition
```

The final validator rejects missing or duplicate dispositions, public records
without canonical paths/features/providers/symbols/compile-link evidence,
duplicate symbols or paths, callable PROGRAM units, false link claims for
missing symbols, evidence-free permanent exclusions, and any Batch A-C count
regression.
