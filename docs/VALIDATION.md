# Prototype validation

Local environment: macOS, Python 3.14.3, Tk 8.6; 2026-10-16, Europe/Kyiv.
This is evidence for the prototype, not a supported-platform or release claim.

| Check actually performed | Result |
| :--- | :--- |
| `BREAKFREE_GUI_TESTS=1 python3 -m unittest discover -s tests -v` | 46 tests passed, including 9 real-Tk tests; no skips |
| Default headless suite | Core, persistence and CLI/build tests passed; desktop tests intentionally opt-in |
| Parser/evaluator | Precedence, left associativity, unary minus, nested parentheses, ASCII references, syntax and resource bounds |
| Recalculation | Required 5 → 7 scenario; chains including 2600 cells; diamonds; direct/indirect cycles and recovery; blank/text semantics |
| Numbers/errors | Decimal 0.1+0.2, repeating division, half-even rounding, caller-context independence, overflow/underflow, division by signed zero |
| History | Typed source restoration, formula/input undo/redo, deletion, redo branch replacement, saved-state comparison |
| Persistence | Required round trip and further recalculation; Unicode and 00123; invalid/foreign/future/duplicate/unknown fields; failed write, flush and replace preserve prior bytes |
| GUI | Actual entry widgets, selection, generated keyboard event, visible errors, Save/Open/New prompts including unapplied drafts, failed Save/Open preservation |
| Open-current-file regression | Save during the Open prompt reloads newly saved bytes instead of showing a stale clean snapshot |
| CLI | One exact JSON response, version, no Tk import in capabilities, absolute paths including Unicode/spaces, invalid arguments/files, GUI routing |
| Standalone build | Byte-identical repeated zip builds; headless commands and real GUI initialization from an isolated directory with empty PYTHONPATH and embedded logo |
| Additional launch | Built executable launched with the synthetic workbook from `/private/tmp` without a startup exception |
| Source compatibility | All Python sources parsed using Python 3.10 grammar; this does not replace running Python 3.10 |
| Documentation | Local Markdown links and image paths checked by `python3 tools/check_docs.py`; Markdown anchors and remote links are not exhaustively checked |
| Repository/YAML | `git diff --check`; workflow and issue-template YAML syntax parsed by Ruby YAML |
| Brand | Approved shared raster inspected; original/PNG hashes and pinned provenance recorded; no Sheets-specific logo found |

The default suite deliberately skips `test_gui` unless `BREAKFREE_GUI_TESTS=1`.
No test success badge is fabricated. Tests are standard-library `unittest` and
require neither another project nor downloaded dependencies.

## Checks not completed locally

- Real Windows and Linux desktops, their launchers/file dialogs/keyboard behavior,
  runtime installation and native packaging: those operating systems are not
  available in this local environment.
- Real Python 3.10 execution: only Python 3.14.3 is available locally.
- Visual screenshot/accessibility-tree review: Computer Use reports that
  permissions are not granted. Functional Tk tests did run through the desktop;
  they do not prove visual layout or screen-reader accessibility.
- Office protocol reconciliation and live launcher integration: the inspected
  Office commit has no `contracts/v1/` or ADRs. Only the task's temporary baseline
  is verified. Nothing is fetched from Office during build/run.
- Power-cut recovery, filesystem directory-metadata durability, network filesystems,
  concurrent external writers, screen readers, performance beyond the bounded grid,
  installer signing and upgrade/uninstallation remain outside verified scope.

## Server CI and review

The workflow defines headless checks/builds for Windows, macOS and Linux on
Python 3.10 and 3.14, plus real Tk GUI tests under Linux Xvfb. Definition alone is
not a successful server run. For commit `71f009edb6e04220986f113f94c9ff380277cfcf`,
[PR CI](https://github.com/PerchAnton/breakfree-sheets/actions/runs/38868572048) and
[push CI](https://github.com/PerchAnton/breakfree-sheets/actions/runs/38868539526)
both completed successfully: six headless matrix jobs and one Linux/Xvfb GUI job
per run. This verifies actual Python 3.10 execution in CI, while interactive
Windows/Linux and visual review remain unverified. Commit-specific observations, review status
and unavailable checks are recorded separately in [PROJECT_LOG.md](PROJECT_LOG.md).
No branch is automatically merged and no release workflow is present.
