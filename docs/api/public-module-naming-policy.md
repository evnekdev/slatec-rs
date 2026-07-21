# Public module naming policy

The safe API is organized first by mathematical domain, not by historical
Fortran file names, raw FFI batches, or Cargo feature names.

- The frozen top-level domains are `linear_algebra`, `special`, `integration`,
  `equations`, `least_squares`, `differential_equations`, `optimization`,
  `transforms`, `interpolation`, and `roadmap`.
- Singular module names are preferred where they describe a mathematical
  family; established plurals such as `roots` and `transforms` are retained.
- Established abbreviations are allowed at meaningful leaves: BLAS, FFT, ODE,
  DAE, and PCHIP.
- Use `linear_programming`, never `lp`; use `least_squares`, never `lsq`.
- Roots, nonlinear systems, interpolation, approximation, ODEs, and DAEs are
  distinct domains even where historical source packages overlap.
- Dense, banded, packed, and sparse storage remain explicit because their
  layout and ownership guarantees differ.
- Scaled special functions remain explicitly named; no module or wrapper may
  silently choose a scaled form.
- Carlson symmetric forms belong to `special::elliptic::carlson`.
- Cargo feature labels and raw Fortran routine names are implementation
  metadata, not automatic public namespaces. In particular,
  `special-scalar-expanded` maps to mathematical integral and elliptic leaves.

Public APIs use the grouped domain tree after a reviewed source, ABI,
workspace, state, and error-contract audit.
