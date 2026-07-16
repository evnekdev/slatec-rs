# Error-handling infrastructure

## Scope

This page describes the documented SLATEC error model, the transition from older `XERROR` calls to `XERMSG`, global-state implications and requirements for a future Rust interface.

## Required routine-level behavior

The guide says a user-callable routine capable of detecting errors should have an integer error flag, preferably last in its argument list. Zero indicates successful completion; positive values identify distinct warning or failure conditions and should be documented in the routine prologue. The preferred error-number range is 1–999 ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

A routine may set the flag and return without printing. If it emits a message, it must call `XERMSG` rather than use direct `WRITE` or `PRINT`. The guide generally prefers recoverable errors so the caller can choose another strategy ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

These are final project guidelines, not proof that every incorporated routine follows them. Older package sources can call `XERROR`; `DQAG` is a representative example ([`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)).

## `XERMSG` interface

The documented call is conceptually:

```fortran
CALL XERMSG (LIBRARY, ROUTINE, MESSAGE, ERROR_NUMBER, LEVEL)
```

All arguments are inputs. `LIBRARY` and `ROUTINE` identify the reporting component; `MESSAGE` carries text; `ERROR_NUMBER` is local to the routine; and `LEVEL` selects severity ([`slatec-guide`](https://www.netlib.org/slatec/guide); [`slatec-source-tree`](https://www.netlib.org/slatec/src/xermsg.f)).

Severity levels are:

| Level | Meaning | Documented control flow |
|---:|---|---|
| 0 | warning | returns |
| 1 | recoverable error | may return or terminate according to user policy |
| 2 | fatal error | does not return |

The guide says level 2 should be rare and gives repeated reverse-communication calls with irrecoverably invalid input as one possible justification ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## Message formatting and output

`XERMSG` delegates formatted output to `XERPRN`. The guide describes 72-character wrapping, a `" * "` prefix and `$$` as an explicit line-break sentinel. Numeric details can be formatted into character variables using internal Fortran I/O and concatenated into the message ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

The error subsystem can select output units and control whether recoverable errors terminate. The Netlib index identifies `FDUMP` and `XERHLT` as machine/site-dependent hooks: `FDUMP` may provide traceback output, while `XERHLT` is invoked when termination is required. The distributed `FDUMP` does not provide traceback, and the distributed `XERHLT` uses `STOP`; sites historically replaced them as needed ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

## Saved error state

The guide says the package remembers the most recent error number and exposes entry points through which users can retrieve or clear it, including `NUMXER` and `XERCLR`. A later call overwrites the saved value. Consequently, after reporting a recoverable error, a routine should return without issuing another recoverable message, so the caller has an opportunity to inspect or clear the state ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

The guide recommends that subsidiary routines return error flags and let the highest user-callable level issue the final `XERMSG`. This centralizes documented conditions and avoids overwriting a serious saved error ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

**Inferred:** The saved number, message-count tables, control flags and output-unit configuration make at least part of the traditional error subsystem process-global or library-global mutable state. Exact storage must be confirmed by source-wide inspection of `COMMON`, `SAVE` and entry points.

## Old and new interfaces

Netlib preserves an `err` sublibrary described as `xersav and other error handling from old version of SLATEC`, because other Netlib packages still use it. The main source contains the later standard error handler. This means a combined build may need compatibility symbols as well as the later interface, and duplicate implementations must be detected ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

`DQAG` calls `XERROR` with a Hollerith-style message argument. This older calling convention is materially different from the character-argument `XERMSG` interface and may require compiler compatibility options or source adaptation in a modern build ([`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)).

## Rust FFI implications

**Project implications:**

- Raw bindings should expose native error flags and documented error-service symbols without pretending they are Rust `Result` values.
- Safe wrappers may translate routine-local error flags into typed errors, but should preserve the numerical outputs and partial-result semantics documented by each routine.
- Calls capable of invoking global error state or output should initially be treated as non-reentrant until tested and audited.
- Fatal paths using `STOP` cannot be safely represented as ordinary Rust errors. A build intended for embedding may need a reviewed replacement for `XERHLT`.
- Message interception, output-unit configuration and error-policy changes require synchronization if they affect global state.
- A callback from Fortran into Rust must never unwind across the FFI boundary, including during error reporting.
- Compatibility for `XERROR`/Hollerith callers should be a deliberate build-layer decision, not hidden in generated bindings.

## Open questions

- Which exact routines and common blocks store error state in Version 4.1?
- Is the error package safe under concurrent calls when only warnings occur?
- Can a site-supplied `XERHLT` reliably return, or do callers assume non-return after level 2?
- Which routines call `XERROR`, `XERRWV`, `XERMSG` or print directly?
- Which error flags represent warnings with usable results versus complete failure?
- Are output-unit setters and control setters thread-local on any compiler, or always global?

## Validated GNU MinGW profile behavior

For `ffi-profile-gnu-mingw-x86_64`, authored child-process probes confirm that
levels `-1` and `0` return, level `1` follows `XSETF` recovery control, and
level `2` terminates. The profile supplies the documented site hook `XERHLT`
with a deterministic nonzero status while preserving fatal semantics. Saved
control and last-error state are process-global and must be serialized. See the
[runtime-profile validation](../extraction/runtime-profile-validation.md).

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`slatec-source-tree`](https://www.netlib.org/slatec/src/)
- [`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)
