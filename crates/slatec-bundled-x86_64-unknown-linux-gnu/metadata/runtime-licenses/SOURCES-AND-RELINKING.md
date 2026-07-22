# Runtime sources and relinking

This carrier was built with GNU Fortran 11.4.0 for `x86_64-unknown-linux-gnu`. Its receipt records the exact static `libgfortran` and `libquadmath` source archives, retained members, checksums, and external symbols. To relink, replace the ordinary static archives under `native/` with compatible rebuilt archives and rebuild the consuming Rust application. Preserve the GNU runtime notices shipped beside this file.
