# Native archive summary

- Archive members: `1317`
- Compiled objects: `1317`
- Archive bytes: `5497482`
- Partial linking (`ld -r`): `false`
- Whole-archive linking: `false`
- Generated amalgamated source: `false`
- Compiler command: `gfortran -x f77 -std=legacy -ffixed-line-length-none -c SOURCE -o OBJECT`
- Archive construction: `ar cr`, chunked `ar q`, then `ar s`

`slatec-sys` itself emits no native link directive. `slatec-src` provides the ordinary static archive and static GNU Fortran runtime libraries. Archive extraction, rather than the number of Rust declarations or enabled API features, determines which native members reach a final executable.
