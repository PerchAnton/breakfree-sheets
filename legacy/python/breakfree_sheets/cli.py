"""Integration arguments are processed before importing any GUI module."""

import argparse
import json
from pathlib import Path
import sys

from . import __version__
from .office import capabilities
from .persistence import FileError, load


def main(argv=None) -> int:
  parser = argparse.ArgumentParser(prog="breakfree-sheets", description="BreakFree Sheets local spreadsheet prototype")
  group = parser.add_mutually_exclusive_group()
  group.add_argument("--new", action="store_true", help="Open a new workbook")
  group.add_argument("--open", metavar="ABSOLUTE_PATH", help="Open a native .bfsheet workbook")
  group.add_argument("--capabilities", action="store_true", help="Print the v1 capability JSON without a GUI")
  group.add_argument("--version", action="store_true", help="Print the actual build version without a GUI")
  args = parser.parse_args(argv)
  if args.version:
    print(f"BreakFree Sheets {__version__}")
    return 0
  if args.capabilities:
    print(json.dumps(capabilities(), separators=(",", ":")))
    return 0
  book, path = None, None
  if args.open:
    path = Path(args.open)
    if not path.is_absolute():
      parser.error("--open requires an absolute path")
    try:
      book = load(path)
    except FileError as exc:
      print(str(exc), file=sys.stderr)
      return 1
  try:
    from .gui import run
    run(book, path)
  except (ImportError, RuntimeError) as exc:
    print(f"Cannot start the GUI: {exc}. Install Python with Tk support.", file=sys.stderr)
    return 1
  return 0
