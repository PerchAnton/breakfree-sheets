# ADR 0002 — Calculation semantics and native content v1

- Date: 2026-10-16
- Status: implemented for the prototype; reviewable and versioned
- Integration protocol: temporary v1, recorded in [the snapshot](../../contracts/v1/SNAPSHOT.md)

## Cell content and display

One worksheet named `Sheet1` has 26 columns and 100 rows. Address spelling is
case-insensitive in formulas/API, normalized to uppercase; persisted addresses
must already be canonical. References outside A1–Z100 produce `#REF!`.

A stored cell has exactly `{kind, source}`. `kind` is `text`, `number` or
`formula`; `source` is a valid Unicode string of at most 4096 characters. Missing
cells are blank. Empty *text* is still text. The entry type is explicit and
defaults to Text for a blank cell. `00123` stays text; choosing Number is an
intentional conversion. Dates, booleans and locale-based inference are absent.

Numeric sources allow a sign, ASCII decimal digits and a dot, without grouping,
whitespace, exponent notation, NaN or Infinity. Sources preserve trailing zeros
and leading zeros if the user explicitly chooses Number. Display may remove
trailing fractional zeros and use scientific notation for very small/large
results; it never rewrites the stored source. The grid truncates long displays;
the entry contains the full source and the result line shows up to 600 characters.

## Numeric model

Use decimal floating-point with **28 significant digits**, `ROUND_HALF_EVEN`,
`Emin=-999` and `Emax=999`. Numeric source/literals with more than 28 significant
digits or adjusted exponents outside -999 through 999 are rejected. Each formula
uses a fresh local decimal context independent of the caller's decimal settings.
Each arithmetic operation rounds in that context, not just the final result.

Exact subnormal arithmetic results can extend to exponent -1026. Inexact
underflow, overflow and invalid decimal operations return `#NUM!`. Positive and
negative zero both display as `0` and both are zero divisors. `0.1+0.2` is `0.3`;
`1/3` is `0.3333333333333333333333333333`.

This explicitly chosen decimal model is not a claim of financial correctness,
currency rounding rules, arbitrary precision or compatibility with another
spreadsheet's numeric semantics. See Python's [decimal context documentation](https://docs.python.org/3.14/library/decimal.html).

## Restricted formula grammar

```ebnf
formula    = "=", expression ;
expression = term, { ("+" | "-"), term } ;
term       = unary, { ("*" | "/"), unary } ;
unary      = "-", unary | primary ;
primary    = decimal | reference | "(", expression, ")" ;
decimal    = digits, [".", [digits]] | ".", digits ;
reference  = letter, row ;
```

Whitespace is allowed between tokens. Unary minus binds most tightly, then
multiplication/division, then addition/subtraction; binary operators associate
left to right. Unary plus, exponent notation, implicit multiplication, strings,
functions, ranges, `$` references and other sheets/books are unsupported.
Formulas are limited to 1024 characters and 512 tokens. The parser produces
only decimal constants, validated references and five arithmetic instructions;
there is no general-purpose evaluation or executable document content.

## References, errors and recalculation

Only numbers participate in arithmetic. Blank references yield numeric zero.
Text references, including `"123"` and empty text, yield `#VALUE!`; no implicit
numeric coercion occurs. Syntax errors remain stored and editable.

| Visible error | Meaning |
| :--- | :--- |
| `#SYNTAX!` | Malformed or unsupported grammar |
| `#DIV/0!` | Division by either signed zero, including `0/0` |
| `#CYCLE!` | Participates in or depends on a circular dependency |
| `#REF!` | Invalid or out-of-grid reference |
| `#VALUE!` | Text used as a numeric operand |
| `#NUM!` | Numeric bound or operation failure |
| `#LIMIT!` | Formula character/token bound exceeded |

Parse errors are reported at the first encountered invalid token/reference.
Well-formed formulas contribute graph edges. An iterative topological pass
evaluates prerequisites before dependents; lexical address ordering breaks ties.
Unresolved graph nodes (cycles and their descendants) get `#CYCLE!`, which takes
precedence over arithmetic in those cells. Other errors propagate; when a binary
operation sees two errors, its left operand's error wins. Independent cells still
calculate. There is no network access, macro interpreter or external link loader.

## Undo and saved state

An edit records address, old content and new content. Undo/redo restore source
and rerun the same engine. A fresh edit drops the redo branch. The most recent
500 edits are retained in memory. Persistence does not include undo history.
Dirty state compares actual stored cells with the last successfully saved state;
undo can return to a clean state. An unapplied entry also counts as unsaved in
the GUI. New, Open and Close offer Save / Discard / Cancel, including that entry.
Cancelling a save or a failed save cancels the requested destructive action.

## Native envelope and safe writes

UTF-8 JSON with `app_id: breakfree.sheets`,
`format_id: breakfree.sheets.native`, integer `file_version: 1` and
`worksheet: {name: Sheet1, cells: {...}}`. Extension: `.bfsheet`.
Only raw sources are persisted; caches and display values are recalculated.
See [the synthetic example](../../examples/first-workbook.bfsheet).

Loads validate the entire envelope, cell types and addresses before replacing
the active model. Duplicate JSON keys, unknown fields, unknown versions, foreign
documents, invalid Unicode and files over 16 MiB are rejected. Invalid formulas
are supported cell content and reopen with a visible error. Opening never writes
to the source. The current workbook and pending entry survive a failed Open.

Save serializes first, writes a same-directory temporary file, flushes and
`fsync`s its contents, then uses `os.replace`. A failed write/flush/replacement
leaves a prior valid destination untouched; tests inject failures. Temporary
files are removed on handled failures. Saving through a symlink is refused.
This uses the host's [atomic replacement semantics](https://docs.python.org/3.14/library/os.html#os.replace).
No claim is made about power loss durability of directory metadata, network
filesystems, preserving filesystem metadata, external concurrent writers or
recovering unsaved work after a crash. Those need additional design and tests.
