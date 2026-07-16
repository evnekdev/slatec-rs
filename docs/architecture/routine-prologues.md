# Routine prologues and calling conventions

## Scope

This page is a technical reference to the final SLATEC prologue format and to the argument, workspace and callback information encoded in representative sources. It does not define individual Rust APIs.

## Purpose of the prologue

The 1993 guide states that source comments were the sole SLATEC-supplied user documentation for routines. The prologue format was deliberately rigid so programs could extract documentation and perform other source transformations automatically ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

A prologue begins immediately after the subprogram declaration with:

```fortran
C***BEGIN PROLOGUE  NAME
```

and ends with:

```fortran
C***END PROLOGUE  NAME
```

Before the first executable statement, every routine contains a `C***FIRST EXECUTABLE STATEMENT  NAME` marker. No unrelated comment may begin with `C***`, allowing those records to function as parseable sentinels ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## Ordered fields

The final guide specifies the following ordered sections:

| Field | User-callable | Subsidiary | Meaning |
|---|---:|---:|---|
| `BEGIN PROLOGUE` | required | required | Start sentinel and routine name |
| `SUBSIDIARY` | absent | required | Not intended as a primary user interface |
| `PURPOSE` | required | required | Short purpose statement |
| `LIBRARY` | required | required | SLATEC and optional package identities |
| `CATEGORY` | required | optional | GAMS categories, primary first |
| `TYPE` | required | required | Data type and equivalent routine family |
| `KEYWORDS` | required | optional | Retrieval terms |
| `AUTHOR` | required | required | Authorship and optional affiliation |
| `DESCRIPTION` | required | optional | Usage, arguments, method and other details |
| `SEE ALSO` | optional | optional | Related user-callable routines |
| `REFERENCES` | required | optional | Bibliographic sources or `(NONE)` |
| `ROUTINES CALLED` | required | required | Direct SLATEC references and passed externals |
| `COMMON BLOCKS` | iff present | iff present | Named common blocks used |
| `REVISION HISTORY` | required | required | Chronological source revisions |
| `END PROLOGUE` | required | required | End sentinel |

The guide notes that quick checks have a related but not identical requirement set ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## Argument ordering

The recommended formal-parameter order is:

1. external procedure names;
2. input variables;
3. input/output variables other than error flags;
4. output variables;
5. work arrays;
6. error flags.

Array dimensioning parameters should immediately follow the associated array. This is a recommendation, not proof that every historical routine follows it ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

Within the suggested `DESCRIPTION` layout, argument qualifiers have defined meanings:

- `:IN` — initialized by the caller and not changed;
- `:OUT` — produced by the routine;
- `:INOUT` — caller-provided and potentially changed;
- `:WORK` — working storage, normally not meaningful on return;
- `:EXT` — external subprogram supplied by the caller;
- `:DUMMY` — unused placeholder, explicitly discouraged.

The guide warns that some work arrays must retain contents between successive calls, which must be documented per routine ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## Array conventions

Fortran 77 passes array storage without intrinsic shape metadata. Routines therefore commonly accept explicit dimensions, leading dimensions, logical lengths, or maximum subdivision counts. Prologues describe minimum dimensions and which elements are significant.

The older QUADPACK routine `DQAG` illustrates this pattern. It accepts `LIMIT`, `LENW`, integer workspace `IWORK`, and double-precision workspace `WORK`; it requires `LENW >= 4*LIMIT`; and it partitions `WORK` into four logical regions containing interval endpoints, approximations and error estimates. On return, `LAST` determines how many entries are significant ([`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)).

**Implication for FFI:** A raw binding must preserve pointer-and-length parameters exactly. A safe wrapper must validate element counts, leading dimensions, aliasing and mutability and must not assume a work array is disposable until the routine documentation says so.

## Callback patterns

Fortran callbacks are passed as external procedure arguments and declared `EXTERNAL` in the calling program. The recommended argument order puts externals first, and the `:EXT` description is supposed to give the expected callback calling sequence ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

`DQAG` accepts an external double-precision function `F(X)` representing the integrand. The source declares `EXTERNAL F` and repeatedly passes it to lower-level integration routines ([`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)).

**Implication for FFI:** Callback ABI, hidden character-length arguments, captured user state, reentrancy, panic containment and lifetime must be handled explicitly. A Rust closure cannot generally be passed directly as a legacy Fortran external without a trampoline and a separately designed context strategy.

## `ROUTINES CALLED` and dependencies

The final guide defines `ROUTINES CALLED` as SLATEC routines directly referenced or declared `EXTERNAL` and passed onward. Intrinsics and formal external parameters are excluded. Entries should be alphabetized. This field is valuable call-graph evidence but is not equivalent to a complete linker dependency list because compiler runtime calls, block data, external libraries and stale prologue data may not appear ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

## Common blocks

A `COMMON BLOCKS` section appears if and only if the routine contains a common block, with `(BLANK)` representing blank common. The guide simultaneously tells authors to avoid common blocks and saved mutable variables because they obstruct multiprocessing; read-only initialized state was considered acceptable ([`slatec-guide`](https://www.netlib.org/slatec/guide)).

**Implication for FFI:** Presence of `COMMON` or mutable `SAVE` is a thread-safety signal. The common-block layout and all transitive users must be established before a wrapper can claim concurrent safety.

## Legacy prologue variants

Not every incorporated routine was normalized to the final template. `DQAG`, for example, uses an older lowercase prologue with `DATE WRITTEN`, `REVISION DATE`, `CATEGORY NO.` and the old `XERROR` service. Therefore parsers should first detect a prologue dialect and preserve unrecognized fields rather than discarding them ([`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)).

## Open questions

- How many prologue dialects occur in the full archive?
- Are argument qualifiers complete and trustworthy enough for automated safety classification?
- Which `ROUTINES CALLED` lists are stale relative to executable source?
- Which callbacks use reverse communication instead of direct external calls?
- Which arrays alias intentionally, and which routines prohibit overlap without saying so explicitly?
- Which character arguments cause compiler-specific hidden length parameters?

## Sources

- [`slatec-guide`](https://www.netlib.org/slatec/guide)
- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`netlib-quadpack`](https://www.netlib.org/quadpack/dqag.f)
