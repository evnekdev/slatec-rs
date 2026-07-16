# Routine catalogue

This directory contains routine-level documentation generated from official source prologues and checked against the SLATEC table of contents and package indexes. The pilot catalogue tests the metadata conventions before full-archive ingestion.

## Files

- [Pilot index](index.md)
- `pilot/`: one Markdown page per pilot routine
- `../../metadata/routines-pilot.toml`: machine-readable pilot records
- `../../metadata/routine-schema.md`: schema and evidence rules

## Status

The pilot is representative, not exhaustive. A field marked `inferred` or `unverified` must not be used for automatic unsafe binding generation without source review. The catalogue documents future FFI concerns but does not implement Rust bindings.
