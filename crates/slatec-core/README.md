# slatec-core

Portable, provider-neutral `no_std` validation, status, workspace, and ABI-profile
types shared by the safe `slatec` facade. This crate contains no numerical
implementation, downloads, native linking, or backend selection. Enable `alloc`
or `std` only when the consuming safe API needs them; the GNU MinGW FFI profile
remains explicit and experimental outside its validated target.
