# Rust native iteration validation

Current local check: `0.1.0-alpha.3`, 2026-10-16.

Local host: macOS / aarch64, Rust 1.99.1. The owner requires Rust throughout;
Python/Tk results are historical evidence, not Rust GUI or Windows validation.

| Check actually performed | Result |
| :--- | :--- |
| `cargo test --locked --no-default-features` | 20 Rust core/files/interaction/CLI tests passed; no GUI dependency needed |
| `cargo test --locked` | 39 tests passed: 20 core/integration and 19 native egui component/event tests |
| Frozen engine conformance | 35 synthetic scenarios from the preserved Python v1 engine passed in Rust, including decimal rounding/bounds, grammar, text/blank rules, errors, cycles and dependent values |
| Required UI loop | Pointer/key events enter A1=2 and B1=3 in the grid; C1 formula entered in the real formula bar gives 5; grid replacement A1=4 gives 7; undo/redo, save/open and further formula-bar edit verified |
| Refined native chrome | Ribbon Copy/Paste/Clear/Undo/Redo, formula Apply/Cancel and zoom tested by pointer events; disabled formatting remains inert; File return/New/Save, pending-draft preservation and suspended cell keys verified |
| View and ribbon controls | Ctrl+F1, chevron and tab double click; draft handoff when Formula Bar hides; hidden-header coordinates; grid-stroke paint changes while selection/results remain; same serialized bytes and clean state after visibility toggles; bounded whole-sheet Clear/Undo |
| Narrow ribbon | Home and View at 1366/1093/980/910 logical widths checked for group bounds/caption visibility and formula/grid separation; scrollbar clipping found during real capture and corrected |
| Grid/interaction | Shift/drag rectangular selection, address/range entry, F2/Escape, separate edit/selection modes, header width/height drag, wheel scroll with pinned headers, bounded visible-cell painting |
| Clipboard | Native framework Copy/Cut/Paste events preserve typed identifiers; literal formula source, no range-reference translation claim |
| Unsaved state | Rendered Save/Discard/Cancel controls include pending input; failed Save prevents destructive New; failed Open leaves current model/draft intact |
| Persistence | Old `.bfsheet` example opens; Unicode/space paths; raw formulas survive; future/foreign/unknown/duplicate/malformed records rejected; oversized file rejected |
| Failed saves | Partial write, flush and replace failures injected; previous bytes unchanged and temporary files cleaned |
| Deep graph | All 2600 supported cells chained without recursion; correct further recalculation |
| Numeric model | 28 digits, exact rational intermediates, half-even per operation; no binary float cell calculation or financial-correctness guarantee |
| `cargo fmt --all -- --check` | Passed |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed, including the capture feature |
| `cargo build --locked --release --offline` | Native optimized macOS binary built after cached dependency fetch |
| Standalone executable | Copied release binary executes `--capabilities` and `--version` from an isolated directory outside the checkout; stderr empty |
| Actual native output | Six alpha.3 macOS framebuffer captures: Home 1.0/1.25/1.5, View 1.0/1.5 and collapsed ribbon 1.0; own logo, zones, group captions and formula source inspected; prior alpha.1 File/range evidence retained; see UI_SPEC |
| Logo | Exact approved dedicated WebP; transparent corner alpha and opaque interior tested |
| Historical baseline | Archived Python core 21 tests and persistence 10 tests still pass; Python is not needed by Rust builds/tests/runtime |
| Documentation | Local links/image paths, journal/table checks where applicable, YAML syntax and `git diff --check` checked |

Current native implementation is in `src/`, not a Python wrapper, JS table or
WebView. The calculation engine has no GUI dependency. The product does not
fetch another repository while building/running. Cargo.lock and the local
contract/brand snapshots are versioned. Dependency notice texts and font notices
are retained; the dependency inventory identifies remaining redistribution review.

## Required checks not available locally

- Actual Microsoft Excel 2024 for Windows: no accessible installation, exact build
  identification, measured default geometry or matched reference screenshots.
- Actual Windows display scaling at **100%, 125%, 150%**. The three diagnostic
  macOS captures and headless scale tests are explicitly not replacements.
- Interactive Windows/Linux file pickers, IME/system keyboard behavior, GPU-driver
  variations, screen readers and native installer/signing/upgrade behavior.
- Live Office launcher/spec reconciliation: its inspected commit remains
  `cfaaf419e84d9981acd421d130113ca2265061fd` without contracts/v1 or ADRs. Only
  the locally frozen task baseline is tested.
- Power-loss durability, directory metadata sync guarantees, network filesystems,
  concurrent external writers and crash recovery remain unverified.

[UI_SPEC](UI_SPEC.md) records verified documentary sources separately from
unavailable visual evidence and gives the Windows matched-capture procedure.
No visual identity, XLSX compatibility or supported-platform release is claimed.

## Server CI

The native workflow defines Rust tests/builds/lints for Windows, macOS and Linux,
plus a real Linux/Xvfb native-frame smoke test. The earlier Python CI cannot
validate this port. New commit-specific server observations belong in
[PROJECT_LOG.md](PROJECT_LOG.md); workflow definition is not server success.
No automatic merge or release is configured.
