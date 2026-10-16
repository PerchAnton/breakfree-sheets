# Temporary Office integration contract v1

Status: local baseline, **not reconciled with an Office specification**.

The Office repository tree was inspected at commit
[`cfaaf419e84d9981acd421d130113ca2265061fd`](https://github.com/PerchAnton/breakfree-office/tree/cfaaf419e84d9981acd421d130113ca2265061fd)
on 2026-10-16 (Europe/Kyiv). It contains neither `contracts/v1/` nor architecture
decision records. There is no upstream protocol document to vendor at this commit.
The owner-provided task's baseline v1 is recorded here instead. This is not an
Office approval or an integration test against a running Office application.

## Commands and response

- `breakfree-sheets --new`: launch an independent window with a blank workbook.
- `breakfree-sheets --open <absolute_path>`: validate and open a native workbook.
  The path is one argument, including spaces and Unicode. Relative paths fail.
- `breakfree-sheets --capabilities`: write exactly one UTF-8 JSON object and a
  newline to stdout, exit successfully, and never import or start the GUI.
- `breakfree-sheets --version`: print the build's application version and exit.

Actions are mutually exclusive. No action defaults to `--new`. Unknown arguments
and relative paths exit 2; invalid or unreadable documents exit 1 with diagnostics
on stderr. Successful headless commands leave stderr empty.

The capabilities response has these required fields:

| Field | Value or type |
| :--- | :--- |
| `protocol_version` | Integer `1` |
| `app_id` | `breakfree.sheets` |
| `app_version` | Actual version from `breakfree_sheets.__version__` |
| `display_name` | `BreakFree Sheets` |
| `capabilities` | Array of implemented launch actions: `new`, `open` |
| `formats` | Array of `{id, extensions, read, write}` records |

The only format is `breakfree.sheets.native`, extensions `['bfsheet']`, read and
write both true. No CSV, XLSX, macros, remote formulas or external links are
advertised. `file_version` inside a saved workbook is independent of
`protocol_version` and `app_version`.

See [the exact response fixture](capabilities.example.json). Contract tests compare
it to the executable response, including the actual build version. Version changes
must update that fixture. Build and launch use only this repository; no build
script fetches a mutable branch or any other repository.
