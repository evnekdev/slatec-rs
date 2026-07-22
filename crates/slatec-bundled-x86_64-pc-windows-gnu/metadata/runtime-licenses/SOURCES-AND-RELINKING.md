# GNU runtime source and relinking information

The carrier's static runtime archives are reduced, reproducible member closures
of the reviewed GNU MinGW 14.2.0 runtime archives. They are not modified source
implementations. The exact selected members, input archive hashes, output
archive hashes, and compiler target are in `../bundle-build-receipt.json`.

The licence text content in this directory is copied from the installed GNU
MinGW distribution used for the receipt. The package normalizes only terminal
line endings to the repository's one-final-LF rule; the source and packaged
SHA-256 values are both recorded below:

- `GPL-3.0.txt`: GNU GPL v3; source and packaged text are both
  `8ceb4b9ee5adedde47b31e975c1d90c73ad27b6b165a1dcd80c7c545eb65b903`.
- `GCC-RUNTIME-LIBRARY-EXCEPTION-3.1.txt`: GCC Runtime Library Exception 3.1,
  source `9d6b43ce4d8de0c878bf16b54d8e7a10d9bd42b75178153e3af6a815bdc90f74`,
  packaged `28e85c5aa4af9b4f1dfe6b4817aa3eefb3eaaee7fd735045016f29ccf50276a1`.
- `LGPL-2.1-or-later.txt`: GNU LGPL 2.1,
  source `a9bdde5616ecdd1e980b44f360600ee8783b1f99b8cc83a2beb163a0a390e861`,
  packaged `ce64d5f7b49ea6d80fdb6d4cdee6839d1a94274f7493dc797c3b55b65ec8e9ed`.

Canonical licence sources are <https://www.gnu.org/licenses/gpl-3.0.txt>,
<https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt>, and
<https://www.gnu.org/licenses/gcc-exception-3.1.html>. The exact reviewed
compiler is GNU Fortran 14.2.0 for `x86_64-w64-mingw32`.

To reproduce or replace a carrier runtime, obtain corresponding GCC 14.2.0
sources and a compatible GNU MinGW toolchain, set `SLATEC_SOURCE_CACHE` to the
verified source cache and `SLATEC_GFORTRAN` to that compiler, then run:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-bundled-provider --offline
```

The command verifies historical SLATEC source hashes, rebuilds the accepted
archive twice, derives runtime-member closures from the actual undefined-symbol
graph, validates a compiler-free consumer, and records every resulting hash.
An application distributor can relink ordinary static archives with a
compatible replacement runtime and rerun those checks. This practical guidance
is a technical reproduction record, not legal advice.
