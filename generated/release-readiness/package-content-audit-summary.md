# Workspace package audit

- Publishable crates: 6
- Package contents audited: 6
- Publication layers: 3
- Native implementation-provider `links` owner: `slatec-src` (`slatec`)
- Target carriers: `slatec-bundled-x86_64-pc-windows-gnu` and `slatec-bundled-x86_64-unknown-linux-gnu` (distinct metadata-only `links` namespaces)
- Result: `pass`

Publication order is layer-based: independent crates in the same layer may be published in either order, followed by each dependent layer.
