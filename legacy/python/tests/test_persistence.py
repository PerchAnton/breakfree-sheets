from decimal import Decimal
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from breakfree_sheets.cells import Cell, Kind
from breakfree_sheets.history import Editor
from breakfree_sheets.persistence import FileError, MAX_FILE_BYTES, decode, encode, load, save
from breakfree_sheets.workbook import Workbook


class PersistenceTests(unittest.TestCase):
  def setUp(self):
    self.temporary = tempfile.TemporaryDirectory()
    self.addCleanup(self.temporary.cleanup)
    self.path = Path(self.temporary.name) / "Workbook with spaces.bfsheet"

  def test_required_save_open_recalculate_workflow(self):
    editor = Editor()
    for name, kind, source in [("A1", Kind.NUMBER, "2"), ("B1", Kind.NUMBER, "3"),
                               ("C1", Kind.FORMULA, "=A1+B1"), ("D1", Kind.TEXT, "00123"),
                               ("E1", Kind.TEXT, "Україна 🌈")]:
      editor.set(name, Cell(kind, source))
    self.assertEqual(editor.values["C1"], Decimal(5))
    editor.set("A1", Cell(Kind.NUMBER, "4"))
    self.assertEqual(editor.values["C1"], Decimal(7))
    save(editor.book, self.path)
    reopened = Editor(load(self.path))
    self.assertEqual(reopened.book.cells, editor.book.cells)
    self.assertEqual(reopened.book.get("C1").source, "=A1+B1")
    self.assertEqual(reopened.values["D1"], "00123")
    reopened.set("A1", Cell(Kind.NUMBER, "6"))
    self.assertEqual(reopened.values["C1"], Decimal(9))
    self.assertFalse(any(self.path.parent.glob(".bfsheet-*")))

  def test_invalid_formulas_survive_round_trip_with_visible_error(self):
    editor = Editor(Workbook({"A1": Cell(Kind.FORMULA, "=SUM(A1)")}))
    reopened = Editor(decode(encode(editor.book)))
    self.assertEqual(reopened.book.get("A1").source, "=SUM(A1)")
    self.assertEqual(reopened.values["A1"].code, "#SYNTAX!")

  def test_malformed_foreign_future_and_unknown_fields_preserve_source(self):
    base = json.loads(encode(Workbook()))
    variants = [b"not JSON", b"\xff", b"[]", b"null", b'{"app_id":NaN}',
                b'{"app_id":"a","app_id":"b"}', b"[" * 2000]
    for key, value in [("app_id", "breakfree.docs"), ("format_id", "other"),
                       ("file_version", 2), ("file_version", True), ("extra", "unsupported")]:
      variants.append(json.dumps({**base, key: value}).encode())
    for data in variants:
      with self.subTest(data=data[:80]):
        self.path.write_bytes(data)
        with self.assertRaises(FileError):
          load(self.path)
        self.assertEqual(self.path.read_bytes(), data)

  def test_invalid_cells_and_unknown_worksheet_are_rejected(self):
    for cells in [{"A101": {"kind": "number", "source": "1"}},
                  {"a1": {"kind": "text", "source": "hello"}},
                  {"A1": {"kind": "number", "source": "NaN"}},
                  {"A1": {"kind": "number", "source": 3}},
                  {"A1": {"kind": "macro", "source": "anything"}},
                  {"A1": {"kind": "text", "source": "safe", "macro": "unsafe"}},
                  {"A1": {"kind": "text", "source": "\ud800"}}, []]:
      data = json.loads(encode(Workbook()))
      data["worksheet"]["cells"] = cells
      with self.subTest(cells=cells), self.assertRaises(FileError):
        decode(json.dumps(data).encode())
    data["worksheet"]["name"] = "Sheet2"
    with self.assertRaises(FileError):
      decode(json.dumps(data).encode())

  def test_oversized_input_is_refused(self):
    with self.assertRaises(FileError):
      decode(b" " * (MAX_FILE_BYTES + 1))

  def test_failed_replace_preserves_previous_file_and_cleans_temporary(self):
    save(Workbook({"A1": Cell(Kind.NUMBER, "2")}), self.path)
    previous = self.path.read_bytes()
    with patch("breakfree_sheets.persistence.os.replace", side_effect=OSError("disk failure")):
      with self.assertRaises(FileError):
        save(Workbook({"A1": Cell(Kind.NUMBER, "4")}), self.path)
    self.assertEqual(self.path.read_bytes(), previous)
    self.assertEqual(list(self.path.parent.iterdir()), [self.path])

  def test_failed_flush_preserves_previous_file(self):
    save(Workbook(), self.path)
    previous = self.path.read_bytes()
    with patch("breakfree_sheets.persistence.os.fsync", side_effect=OSError("flush failed")):
      with self.assertRaises(FileError):
        save(Workbook({"A1": Cell(Kind.NUMBER, "4")}), self.path)
    self.assertEqual(self.path.read_bytes(), previous)
    self.assertEqual(list(self.path.parent.iterdir()), [self.path])

  def test_failed_write_preserves_previous_file(self):
    save(Workbook(), self.path)
    previous = self.path.read_bytes()
    original_factory = tempfile.NamedTemporaryFile

    def failing_file(*args, **kwargs):
      stream = original_factory(*args, **kwargs)
      stream.write = lambda data: (_ for _ in ()).throw(OSError("disk full"))
      return stream

    with patch("breakfree_sheets.persistence.tempfile.NamedTemporaryFile", side_effect=failing_file):
      with self.assertRaises(FileError):
        save(Workbook({"A1": Cell(Kind.NUMBER, "4")}), self.path)
    self.assertEqual(self.path.read_bytes(), previous)
    self.assertEqual(list(self.path.parent.iterdir()), [self.path])

  def test_failed_first_save_and_missing_open_report_file_errors(self):
    with self.assertRaises(FileError):
      save(Workbook(), self.path / "missing" / "new.bfsheet")
    with self.assertRaises(FileError):
      load(self.path)

  def test_symlink_target_is_not_replaced(self):
    original = self.path.with_name("original.bfsheet")
    original.write_text("untouched", encoding="utf-8")
    try:
      self.path.symlink_to(original)
    except OSError:
      self.skipTest("Creating symlinks is not permitted on this host.")
    with self.assertRaises(FileError):
      save(Workbook(), self.path)
    self.assertTrue(self.path.is_symlink())
    self.assertEqual(original.read_text(), "untouched")
