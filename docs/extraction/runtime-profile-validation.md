# GNU MinGW runtime-profile validation

## Why this gate exists

The first broad special-function experiment was stopped before any safe API was
committed. Native calls to Airy `AI`, exponential integral `EI`, and reciprocal
gamma reached the SLATEC initialization and error subsystems and terminated the
probe processes. Degree-based sine and cosine completed, showing that raw ABI
linkage alone was not the missing evidence.

The selected `main-src` providers for `I1MACH`, `R1MACH`, and `D1MACH` are the
historical installation templates. They contain many machine-specific `DATA`
blocks, but every block is commented out in the acquired source. Compilation
succeeds while the saved arrays have no configured values. Under the supported
GNU Fortran profile the resulting zeros make FNLIB tolerance and limit
calculations invalid:

- `AI` and `EI` reach `INITS`, which reports that the Chebyshev series is too
  short for the derived accuracy;
- reciprocal gamma reaches `INITDS` and then `DGAMLM`, whose limit iteration
  cannot find `XMIN`;
- direct `DGAMLM` and `GAMLIM` probes fail for the same zero-limit reason.

The distributed `XERHLT` site hook then executes a bare `STOP`. On this profile
that terminates with status zero, so a link smoke test can mistake fatal
termination for success unless it also validates an expected result marker.

## Selected correction

The immutable selected-corpus providers and hashes remain unchanged. Native
archive construction applies four profile-scoped build-layer replacements from
`native/profile/gnu-mingw-x86_64/`:

- `I1MACH` derives numeric model values from Fortran inquiry intrinsics and
  obtains preconnected input, output, and error units from `ISO_FORTRAN_ENV`;
- `R1MACH` and `D1MACH` implement the documented PORT contracts using `TINY`,
  `HUGE`, `EPSILON`, and `RADIX`;
- `XERHLT` remains fatal but uses status 70, allowing a parent process to
  distinguish fatal termination from a clean return.

The Linux overlay was not selected. Its `D1MACH` and `R1MACH` implementations
add `DLAMCH`/`SLAMCH` dependencies, while its `I1MACH` is another hard-coded
platform selection. The live main-tree copies are byte-identical to the
unconfigured selected templates.

The explicit native build creates an ignored original archive for comparison
and the supported profile archive for Rust linking. The profile archive is
checked to contain exactly one definition of every replaced symbol. No Cargo
build script downloads or compiles Fortran.

## Machine-constant interpretation

All 16 valid `I1MACH` selectors and all five valid selectors for each of
`R1MACH` and `D1MACH` are compared with values printed by an authored Fortran
program using compiler inquiry intrinsics. `I1MACH(1)`, `(2)`, and `(4)` are
classified as operational unit values. Selector `(3)` is the historical punch
unit; the profile records zero and classifies it as historical but harmless,
not a numerical validation failure.

The compact results are in `generated/runtime-profile/`. Raw stdout, stderr,
executables, objects, archives, and elapsed-time observations remain below the
ignored `evidence/runtime-profile/` tree.

## Legacy error semantics

The selected `XERMSG` implementation behaves as follows:

| Level | Meaning | Profile behavior |
|---:|---|---|
| `-1` | print-once informational message | returns |
| `0` | warning | returns |
| `1` | recoverable error | returns only when recovery control permits it |
| `2` | fatal error | terminates through `XERHLT` |

`XSETF(1)` permits the authored level-1 probe to return. The default/control-2
level-1 probe and every level-2 probe terminate. Potentially fatal cases are
therefore tested only in child processes. Compact records retain exit status,
outcome class, and hashes of captured output; detailed messages remain ignored.

The nonlinear negative-`IFLAG` regression uses the same containment contract.
The parent captures the child status and both output streams, requires the
source-verified `DNSQE` driver / `DNSQ` XERROR diagnostic, accepts only the observed GNU
runtime outcomes (the historical status-zero `STOP` or the profile `XERHLT`
status 70), and rejects signals, access violations, illegal instructions, and
ordinary Rust panic returns. No fixed exit number is treated as the safety
contract; isolation and diagnostic evidence are.

`J4SAVE` stores control flags, output-unit settings, and last-error state in a
`SAVE`d array. `XERSVE` also maintains message counts. This state is
process-global for the selected implementation. Runtime-profile tests and
future wrappers that manipulate it must be serialized; this milestone does not
claim thread safety or define a public concurrency API.

## Commands

With the complete selected evidence and GNU MinGW compiler available:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-native-ffi --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-runtime-profile --offline
```

`SLATEC_FORTRAN_COMPILER` may name an explicitly reviewed compiler, but this
profile requires its reported target to be `x86_64-w64-mingw32`.

## Evidence boundary

Passing this gate separately establishes:

- the previously validated raw ABI profile;
- machine constants for the exact compiler profile;
- observed legacy error-level behavior;
- representative `INITDS`, `INITS`, `DGAMLM`, `GAMLIM`, `AI`, `EI`, and
  reciprocal-gamma initialization behavior.

It does not expose a safe special-function API, validate every SLATEC routine,
prove thread safety, or replace any numerical algorithm. Invalid-domain
preconditions and family-specific numerical contracts remain work for a later
safe-wrapper milestone.
