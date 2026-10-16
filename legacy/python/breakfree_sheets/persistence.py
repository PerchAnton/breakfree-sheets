"""Strict versioned JSON loading and same-directory atomic file replacement."""

import json
import os
from pathlib import Path
import tempfile

from .cells import Cell, Kind, address
from .workbook import Workbook

APP_ID = "breakfree.sheets"
FORMAT_ID = "breakfree.sheets.native"
FILE_VERSION = 1
MAX_FILE_BYTES = 16 * 1024 * 1024


class FileError(ValueError):
  pass


def _object(pairs):
  result = {}
  for key, value in pairs:
    if key in result:
      raise FileError(f"Duplicate JSON key: {key}.")
    result[key] = value
  return result


def _reject_constant(value):
  raise FileError(f"Non-finite JSON number: {value}.")


def encode(book: Workbook) -> bytes:
  document = {
    "app_id": APP_ID,
    "format_id": FORMAT_ID,
    "file_version": FILE_VERSION,
    "worksheet": {
      "name": "Sheet1",
      "cells": {name: {"kind": cell.kind.value, "source": cell.source}
                for name, cell in sorted(book.cells.items())},
    },
  }
  data = (json.dumps(document, ensure_ascii=False, indent=2) + "\n").encode("utf-8")
  if len(data) > MAX_FILE_BYTES:
    raise FileError("Workbook exceeds the 16 MiB file limit.")
  return data


def decode(data: bytes) -> Workbook:
  if len(data) > MAX_FILE_BYTES:
    raise FileError("Workbook exceeds the 16 MiB file limit.")
  try:
    document = json.loads(data.decode("utf-8"), object_pairs_hook=_object, parse_constant=_reject_constant)
    if not isinstance(document, dict) or set(document) != {"app_id", "format_id", "file_version", "worksheet"}:
      raise FileError("Invalid workbook envelope; unknown fields are not discarded.")
    if document["app_id"] != APP_ID or document["format_id"] != FORMAT_ID:
      raise FileError("This document belongs to another application or format.")
    if type(document["file_version"]) is not int or document["file_version"] != FILE_VERSION:
      raise FileError("Unsupported workbook content version.")
    sheet = document["worksheet"]
    if not isinstance(sheet, dict) or set(sheet) != {"name", "cells"} or sheet["name"] != "Sheet1":
      raise FileError("Expected the single worksheet Sheet1.")
    if not isinstance(sheet["cells"], dict) or len(sheet["cells"]) > 2600:
      raise FileError("Invalid or oversized cell collection.")
    cells = {}
    for name, content in sheet["cells"].items():
      if address(name) != name:
        raise FileError("Stored addresses must be canonical uppercase references.")
      if not isinstance(content, dict) or set(content) != {"kind", "source"}:
        raise FileError("Invalid cell record; unknown fields are not discarded.")
      cells[name] = Cell(Kind(content["kind"]), content["source"])
    return Workbook(cells)
  except (ValueError, TypeError, KeyError, UnicodeError, RecursionError) as exc:
    if isinstance(exc, FileError):
      raise
    raise FileError(f"Invalid workbook: {exc}") from exc


def load(path: str | Path) -> Workbook:
  try:
    with Path(path).open("rb") as stream:
      data = stream.read(MAX_FILE_BYTES + 1)
    return decode(data)
  except OSError as exc:
    raise FileError(f"Cannot open workbook: {exc}") from exc


def save(book: Workbook, path: str | Path):
  data = encode(book)
  target = Path(path).absolute()
  temporary = None
  try:
    if target.is_symlink():
      raise FileError("Saving through a symbolic link is unsupported; choose a regular file.")
    with tempfile.NamedTemporaryFile(mode="wb", dir=target.parent, prefix=".bfsheet-", delete=False) as stream:
      temporary = Path(stream.name)
      stream.write(data)
      stream.flush()
      os.fsync(stream.fileno())
    # The old file is untouched until a complete, flushed replacement exists.
    os.replace(temporary, target)
    temporary = None
  except OSError as exc:
    raise FileError(f"Cannot save workbook: {exc}") from exc
  finally:
    if temporary is not None:
      temporary.unlink(missing_ok=True)
