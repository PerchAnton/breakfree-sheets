# ADR 0001 — Independent Python/Tk prototype

- Date: 2026-10-16
- Status: provisional, reversible prototype choice for owner review
- Scope: Sheets only; no suite-wide language or toolkit decision

## Context and decision

The owner requested a working independent first iteration and explicitly allowed
a reversible stack choice where no stack or Office ADR exists. Sheets at
`fd7428a` has documentation but no implementation, dependencies, ADRs or local
brand assets. Office at `cfaaf419e84d9981acd421d130113ca2265061fd` has a brand
guide and shared mark, but no architecture ADRs or `contracts/v1/` directory.

Use Python 3.10+ with standard-library Tk for the prototype. The development
host has Python 3.14.3 and Tk 8.6. No third-party Python package, package registry,
Office installation, account or server is required. The runtime must include Tk
for the GUI; headless calculation and contract tests do not import it.

This is not the owner's permanent architecture approval, a supported platform
matrix, a software license decision or a commitment to distribute Python/Tk.

## Module boundaries

| Module | Responsibility |
| :--- | :--- |
| `cells` | Immutable typed source, decimal validation, errors and display |
| `workbook` | Single-sheet model, canonical addresses and storage |
| `formula` | Bounded restricted grammar compiled to postfix instructions |
| `dependencies` | Dependency graph and iterative deterministic ordering |
| `calculation` | Decimal evaluation, propagation and recalculation |
| `history` | Edit commands, undo/redo, saved-state comparison |
| `persistence` | Versioned native envelope, validation and atomic saves |
| `gui` | Selection, entry, dialogs and rendering engine results |
| `office` / `cli` | Temporary suite adapter and process arguments |

The GUI makes edits through `Editor`; it never computes formulas. Changing a
cell, undoing or redoing rebuilds the graph and recalculates the whole bounded
worksheet in O(V + E), plus sorting/heap ordering. Incremental invalidation is a
later optimization, not a correctness shortcut. All graph traversal and formula
evaluation is iterative, including long chains and deeply parenthesized inputs.

## Packaging and reversibility

`python tools/build.py` creates a deterministic, uncompressed Python zip
application and a Windows command launcher. This is a real standalone *project*,
not a runtime-bundled native installer: Python and Tk remain prerequisites.
Sources, assets and the version are embedded; there are no build-time downloads.
`breakfree-sheets` is the executable entry point; Windows uses
`breakfree-sheets.cmd` with the platform suffix.

The pure model/engine and JSON fixtures can survive a toolkit change. The
native file version is separate from the application and integration versions.
No shared library or submodule is introduced. An Office launcher can start this
application through CLI arguments without becoming a runtime dependency.

## Tradeoffs and review gates

The first grid is fixed at A1–Z100 with single-cell edits and 500 undo commands.
Tk provides usable desktop controls quickly, but canvas cell accessibility and
screen-reader behavior need a separate review. Navigation and formula/source
editing are basic; range selection and copy reference adjustment are absent.

Before any public release, review this stack with the owner, test real Windows
and Linux desktops, decide licensing, establish a private security-reporting
route, and verify packaging and accessibility. No release is produced here.
